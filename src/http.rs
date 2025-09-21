use crate::value::{Value, ErrorValue};
use std::collections::HashMap;

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

        // Convert headers to Tilde object
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
        let json_value = self.tilde_value_to_json(value)?;
        serde_json::to_string(&json_value).map_err(|e| format!("Failed to serialize to JSON: {}", e))
    }

    fn tilde_value_to_json(&self, value: &Value) -> Result<serde_json::Value, String> {
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
                    json_array.push(self.tilde_value_to_json(item)?);
                }
                Ok(serde_json::Value::Array(json_array))
            }
            Value::Object(map) => {
                let mut json_obj = serde_json::Map::new();
                for (key, val) in map {
                    json_obj.insert(key.clone(), self.tilde_value_to_json(val)?);
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

        // Execute request based on method
        let response_result = match request.method.as_str() {
            "GET" => {
                let mut req = agent.get(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                req.call()
            }
            "POST" => {
                let mut req = agent.post(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                if let Some(body) = &request.body {
                    req.send(body.as_bytes())
                } else {
                    req.send_empty()
                }
            }
            "PUT" => {
                let mut req = agent.put(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                if let Some(body) = &request.body {
                    req.send(body.as_bytes())
                } else {
                    req.send_empty()
                }
            }
            "DELETE" => {
                let mut req = agent.delete(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                req.call()
            }
            "PATCH" => {
                let mut req = agent.patch(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                if let Some(body) = &request.body {
                    req.send(body.as_bytes())
                } else {
                    req.send_empty()
                }
            }
            "HEAD" => {
                let mut req = agent.head(&request.url);
                for (key, value) in &request.headers {
                    req = req.header(key.as_str(), value.as_str());
                }
                req.call()
            }
            _ => return Err(Self::create_error(
                format!("Unsupported HTTP method: {}", request.method),
                Some("unsupported_method".to_string()),
                Some(request.url.clone()),
                HashMap::new(),
            )),
        };

        let response_time_ms = start_time.elapsed().as_millis() as u64;

        match response_result {
            Ok(mut response) => {
                // Extract metadata first
                let status = response.status().as_u16();
                let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();

                // Extract headers
                let mut headers = HashMap::new();
                for (name, value) in response.headers() {
                    headers.insert(name.to_string().to_lowercase(), value.to_str().unwrap_or("").to_string());
                }

                // Read response body
                let body = match response.body_mut().read_to_string() {
                    Ok(body_str) => body_str,
                    Err(e) => {
                        return Err(Self::create_error(
                            format!("Failed to read response body: {}", e),
                            Some("body_read_error".to_string()),
                            Some(request.url),
                            [("response_time_ms".to_string(), Value::Number(response_time_ms as f64))].into(),
                        ));
                    }
                };

                let http_response = HttpResponse {
                    status,
                    status_text: status_text.clone(),
                    headers: headers.clone(),
                    body: body.clone(),
                    url: request.url.clone(),
                    response_time_ms,
                };

                // Check if this is a client or server error (4xx or 5xx)
                if status >= 400 {
                    let mut error_context = HashMap::new();
                    error_context.insert("status".to_string(), Value::Number(status as f64));
                    error_context.insert("status_text".to_string(), Value::String(status_text));
                    error_context.insert("response_time_ms".to_string(), Value::Number(response_time_ms as f64));
                    error_context.insert("body".to_string(), Value::String(http_response.body.clone()));

                    // Include headers in error context
                    let mut headers_map = HashMap::new();
                    for (key, value) in &http_response.headers {
                        headers_map.insert(key.clone(), Value::String(value.clone()));
                    }
                    error_context.insert("headers".to_string(), Value::Object(headers_map));

                    Err(Self::create_error(
                        format!("http status: {}", status),
                        Some("http_error".to_string()),
                        Some(request.url),
                        error_context,
                    ))
                } else {
                    Ok(http_response)
                }
            }
            Err(error) => {
                // Handle ureq errors
                let mut error_context = HashMap::new();
                error_context.insert("response_time_ms".to_string(), Value::Number(response_time_ms as f64));
                error_context.insert("timeout_ms".to_string(), Value::Number(request.timeout_ms as f64));

                // Handle different types of ureq errors
                let (error_message, error_code) = match error {
                    ureq::Error::StatusCode(status) => {
                        // This is an HTTP status code error (4xx, 5xx)
                        error_context.insert("status".to_string(), Value::Number(status as f64));
                        (format!("http status: {}", status), "http_error")
                    },
                    ureq::Error::Timeout(_) => {
                        (error.to_string(), "timeout")
                    },
                    ureq::Error::HostNotFound => {
                        (error.to_string(), "dns_error")
                    },
                    ureq::Error::ConnectionFailed => {
                        (error.to_string(), "connection_failed")
                    },
                    _ => {
                        (error.to_string(), "network_error")
                    }
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

/// Parse HTTP options from Tilde value
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