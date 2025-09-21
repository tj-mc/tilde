use crate::evaluator::Evaluator;
use crate::value::{Value, ErrorValue};
use crate::ast::Expression;
use std::collections::HashMap;
use std::io::Read;

/// HTTP response object containing status, headers, and body
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub url: String,
    pub response_time_ms: u64,
}

impl HttpResponse {
    pub fn to_tails_value(&self) -> Value {
        let mut response_map = HashMap::new();

        response_map.insert("status".to_string(), Value::Number(self.status as f64));
        response_map.insert("status_text".to_string(), Value::String(self.status_text.clone()));
        response_map.insert("url".to_string(), Value::String(self.url.clone()));
        response_map.insert("response_time_ms".to_string(), Value::Number(self.response_time_ms as f64));

        // Convert headers to Tails object
        let mut headers_map = HashMap::new();
        for (key, value) in &self.headers {
            headers_map.insert(key.clone(), Value::String(value.clone()));
        }
        response_map.insert("headers".to_string(), Value::Object(headers_map));

        // Try to parse body as JSON, fall back to string
        match serde_json::from_str::<serde_json::Value>(&self.body) {
            Ok(json) => {
                response_map.insert("body".to_string(), Self::json_to_tails_value(json));
                response_map.insert("body_text".to_string(), Value::String(self.body.clone()));
            }
            Err(_) => {
                response_map.insert("body".to_string(), Value::String(self.body.clone()));
                response_map.insert("body_text".to_string(), Value::String(self.body.clone()));
            }
        }

        // Add convenience fields
        response_map.insert("ok".to_string(), Value::Boolean(self.status >= 200 && self.status < 300));
        response_map.insert("success".to_string(), Value::Boolean(self.status >= 200 && self.status < 300));

        Value::Object(response_map)
    }

    fn json_to_tails_value(json: serde_json::Value) -> Value {
        match json {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_f64() {
                    Value::Number(i)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                let tails_list: Vec<Value> = arr.into_iter().map(Self::json_to_tails_value).collect();
                Value::List(tails_list)
            }
            serde_json::Value::Object(obj) => {
                let mut tails_map = HashMap::new();
                for (key, value) in obj {
                    tails_map.insert(key, Self::json_to_tails_value(value));
                }
                Value::Object(tails_map)
            }
        }
    }
}

/// HTTP request configuration
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout_ms: u64,
    pub follow_redirects: bool,
}

impl HttpRequest {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_uppercase(),
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            timeout_ms: 30000, // 30 seconds default
            follow_redirects: true,
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    pub fn with_json_body(mut self, json_value: &Value) -> Result<Self, String> {
        let json_str = self.value_to_json_string(json_value)?;
        self.headers.insert("content-type".to_string(), "application/json".to_string());
        self.body = Some(json_str);
        Ok(self)
    }

    pub fn with_bearer_token(mut self, token: &str) -> Self {
        self.headers.insert("authorization".to_string(), format!("Bearer {}", token));
        self
    }

    pub fn with_basic_auth(mut self, username: &str, password: &str) -> Self {
        let credentials = base64::encode(format!("{}:{}", username, password));
        self.headers.insert("authorization".to_string(), format!("Basic {}", credentials));
        self
    }

    fn value_to_json_string(&self, value: &Value) -> Result<String, String> {
        let json_value = self.tails_value_to_json(value)?;
        serde_json::to_string(&json_value).map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    fn tails_value_to_json(&self, value: &Value) -> Result<serde_json::Value, String> {
        match value {
            Value::Null => Ok(serde_json::Value::Null),
            Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            Value::Number(n) => Ok(serde_json::Value::Number(
                serde_json::Number::from_f64(*n).ok_or("Invalid number")?
            )),
            Value::String(s) => Ok(serde_json::Value::String(s.clone())),
            Value::List(list) => {
                let mut json_array = Vec::new();
                for item in list {
                    json_array.push(self.tails_value_to_json(item)?);
                }
                Ok(serde_json::Value::Array(json_array))
            }
            Value::Object(map) => {
                let mut json_obj = serde_json::Map::new();
                for (key, val) in map {
                    json_obj.insert(key.clone(), self.tails_value_to_json(val)?);
                }
                Ok(serde_json::Value::Object(json_obj))
            }
            Value::Date(dt) => Ok(serde_json::Value::String(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())),
            Value::Error(_) => Err("Cannot serialize Error values to JSON".to_string()),
        }
    }
}

/// HTTP client with robust error handling
pub struct HttpClient;

impl HttpClient {
    pub fn execute(request: HttpRequest) -> Result<HttpResponse, Value> {
        #[cfg(target_arch = "wasm32")]
        {
            Self::execute_wasm(request)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::execute_native(request)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute_native(request: HttpRequest) -> Result<HttpResponse, Value> {
        let start_time = std::time::Instant::now();

        // Build ureq agent with timeout
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_millis(request.timeout_ms)))
            .max_redirects(if request.follow_redirects { 10 } else { 0 })
            .build();
        let agent: ureq::Agent = config.into();

        // Create request builder
        let mut req_builder = match request.method.as_str() {
            "GET" => agent.get(&request.url),
            "POST" => agent.post(&request.url),
            "PUT" => agent.put(&request.url),
            "DELETE" => agent.delete(&request.url),
            "PATCH" => agent.request("PATCH", &request.url),
            "HEAD" => agent.head(&request.url),
            "OPTIONS" => agent.request("OPTIONS", &request.url),
            _ => return Err(Self::create_error(
                format!("Unsupported HTTP method: {}", request.method),
                Some("unsupported_method".to_string()),
                Some(request.url.clone()),
                HashMap::new(),
            )),
        };

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.set(&key, &value);
        }

        // Execute request
        let response_result = if let Some(body) = &request.body {
            req_builder.send_string(body)
        } else {
            req_builder.call()
        };

        let response_time_ms = start_time.elapsed().as_millis() as u64;

        match response_result {
            Ok(mut response) => {
                // Read response body
                let mut body = String::new();
                if let Err(e) = response.body_mut().read_to_string(&mut body) {
                    return Err(Self::create_error(
                        format!("Failed to read response body: {}", e),
                        Some("body_read_error".to_string()),
                        Some(request.url),
                        [("response_time_ms".to_string(), Value::Number(response_time_ms as f64))].into(),
                    ));
                }

                // Extract headers
                let mut headers = HashMap::new();
                for name in response.headers_names() {
                    if let Some(value) = response.header(&name) {
                        headers.insert(name.to_lowercase(), value.to_string());
                    }
                }

                let http_response = HttpResponse {
                    status: response.status(),
                    status_text: response.status_text().to_string(),
                    headers,
                    body,
                    url: request.url,
                    response_time_ms,
                };

                Ok(http_response)
            }
            Err(error) => {
                // Handle ureq errors
                let mut error_context = HashMap::new();
                error_context.insert("response_time_ms".to_string(), Value::Number(response_time_ms as f64));
                error_context.insert("timeout_ms".to_string(), Value::Number(request.timeout_ms as f64));

                // Try to extract response information if available
                if let Some(response) = error.response() {
                    let status = response.status();
                    let status_text = response.status_text();

                    // Read response body if possible
                    let body = if let Ok(body_str) = response.into_string() {
                        body_str
                    } else {
                        "Failed to read response body".to_string()
                    };

                    error_context.insert("status".to_string(), Value::Number(status as f64));
                    error_context.insert("status_text".to_string(), Value::String(status_text.to_string()));
                    error_context.insert("response_body".to_string(), Value::String(body));

                    Err(Self::create_error(
                        format!("HTTP {} {}", status, status_text),
                        Some(status.to_string()),
                        Some(request.url),
                        error_context,
                    ))
                } else {
                    // Network/transport error
                    let error_message = error.to_string();
                    let error_code = if error_message.contains("timeout") {
                        "timeout"
                    } else if error_message.contains("DNS") || error_message.contains("dns") {
                        "dns_error"
                    } else if error_message.contains("connection") || error_message.contains("Connection") {
                        "connection_failed"
                    } else {
                        "network_error"
                    };

                    error_context.insert("error_details".to_string(), Value::String(error_message.clone()));

                    Err(Self::create_error(
                        error_message,
                        Some(error_code.to_string()),
                        Some(request.url),
                        error_context,
                    ))
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn execute_wasm(_request: HttpRequest) -> Result<HttpResponse, Value> {
        // TODO: Implement WASM HTTP client using web-sys fetch
        Err(Self::create_error(
            "WASM HTTP client not yet implemented".to_string(),
            Some("not_implemented".to_string()),
            None,
            HashMap::new(),
        ))
    }

    fn create_error(message: String, code: Option<String>, source: Option<String>, context: HashMap<String, Value>) -> Value {
        Value::Error(ErrorValue {
            message,
            code,
            source,
            context,
        })
    }
}

/// Parse HTTP options from Tails value
pub fn parse_http_options(options_value: Option<Value>) -> Result<(HashMap<String, String>, Option<String>, u64), String> {
    let mut headers = HashMap::new();
    let mut body = None;
    let mut timeout_ms = 30000;

    if let Some(Value::Object(options)) = options_value {
        // Parse headers
        if let Some(Value::Object(headers_obj)) = options.get("headers") {
            for (key, value) in headers_obj {
                if let Value::String(header_value) = value {
                    headers.insert(key.to_lowercase(), header_value.clone());
                } else {
                    return Err(format!("Header '{}' must be a string", key));
                }
            }
        }

        // Parse body
        if let Some(body_value) = options.get("body") {
            match body_value {
                Value::String(s) => body = Some(s.clone()),
                _ => {
                    // Convert to JSON
                    let request = HttpRequest::new("POST", "");
                    body = Some(request.value_to_json_string(body_value)?);
                    headers.insert("content-type".to_string(), "application/json".to_string());
                }
            }
        }

        // Parse timeout
        if let Some(Value::Number(timeout)) = options.get("timeout") {
            timeout_ms = *timeout as u64;
        }

        // Parse auth
        if let Some(Value::String(token)) = options.get("bearer_token") {
            headers.insert("authorization".to_string(), format!("Bearer {}", token));
        }

        if let Some(Value::Object(auth)) = options.get("basic_auth") {
            if let (Some(Value::String(username)), Some(Value::String(password))) =
                (auth.get("username"), auth.get("password")) {
                let credentials = base64::encode(format!("{}:{}", username, password));
                headers.insert("authorization".to_string(), format!("Basic {}", credentials));
            }
        }
    }

    Ok((headers, body, timeout_ms))
}

// Add base64 encoding for basic auth
mod base64 {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(input: String) -> String {
        let bytes = input.as_bytes();
        let mut result = String::new();

        for chunk in bytes.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }

            let b1 = buf[0] as usize;
            let b2 = buf[1] as usize;
            let b3 = buf[2] as usize;

            result.push(CHARS[b1 >> 2] as char);
            result.push(CHARS[((b1 & 0x03) << 4) | (b2 >> 4)] as char);

            if chunk.len() > 1 {
                result.push(CHARS[((b2 & 0x0f) << 2) | (b3 >> 6)] as char);
            } else {
                result.push('=');
            }

            if chunk.len() > 2 {
                result.push(CHARS[b3 & 0x3f] as char);
            } else {
                result.push('=');
            }
        }

        result
    }
}