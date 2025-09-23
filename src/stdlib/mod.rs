pub mod list;
pub mod list_mutations;
pub mod list_queries;
pub mod list_advanced;
pub mod set_operations;
pub mod string;
pub mod math;
pub mod helpers;
pub mod object;
pub mod collection;
pub mod date;
pub mod system;
pub mod type_checking;
pub mod json;
pub mod encoding;
pub mod crypto;
pub mod filesystem;
pub mod object_manipulation;
mod utils;

use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Get all standard library function names
pub fn get_stdlib_function_names() -> &'static [&'static str] {
    &[
        // List functions
        "map", "filter", "reduce", "sort", "reverse", "list",
        // Advanced list functions with predicates
        "find", "find-index", "find-last", "every", "some", "remove-if", "count-if",
        "take-while", "drop-while", "partition", "group-by", "sort-by",
        // List mutation functions
        "remove", "remove-at", "insert", "set-at", "pop", "shift", "unshift",
        // List query functions
        "index-of", "contains", "slice", "concat", "take", "drop",
        // List advanced functions
        "flatten", "unique", "zip", "chunk", "transpose",
        // Set operations
        "union", "difference", "intersection",
        // String functions
        "split", "join", "trim", "uppercase", "lowercase",
        "starts-with", "ends-with", "substring", "replace", "repeat", "pad-left", "pad-right",
        // Math functions
        "absolute", "square-root", "random",
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        "log", "log10", "exp", "pow", "round", "floor", "ceil", "pi", "e",
        // Date functions
        "now", "date", "date-add", "date-subtract", "date-diff", "date-format", "date-parse",
        "date-year", "date-month", "date-day", "date-hour", "date-minute", "date-second", "date-weekday",
        "date-before", "date-after", "date-equal",
        // Object functions
        "keys", "values", "has",
        // Collection functions
        "length", "append",
        // Common helper functions - predicates
        "is-even", "is-odd", "is-positive", "is-negative", "is-zero",
        // Common helper functions - transformations
        "double", "triple", "quadruple", "half", "square", "increment", "decrement",
        // Common helper functions - reductions
        "add", "multiply", "max", "min",
        // System functions
        "env",
        // Type checking functions
        "is-number", "is-string", "is-boolean", "is-list", "is-object", "is-null", "is-empty", "is-defined",
        // JSON functions
        "to-json", "from-json",
        // Encoding functions
        "base64-encode", "base64-decode", "url-encode", "url-decode",
        // Cryptography functions
        "sha256", "md5", "hmac-sha256",
        // Filesystem functions
        "file-exists", "dir-exists", "file-size",
        // Object manipulation functions
        "merge", "pick", "omit", "object-get", "object-set", "deep-merge",
    ]
}

/// Register all standard library functions
pub fn get_stdlib_function(name: &str) -> Option<fn(Vec<Expression>, &mut Evaluator) -> Result<Value, String>> {
    match name {
        // List functions
        "map" => Some(list::eval_map),
        "filter" => Some(list::eval_filter),
        "reduce" => Some(list::eval_reduce),
        "sort" => Some(list::eval_sort),
        "reverse" => Some(list::eval_reverse),
        "list" => Some(list::eval_list),

        // Advanced list functions with predicates
        "find" => Some(list::eval_find),
        "find-index" => Some(list::eval_find_index),
        "find-last" => Some(list::eval_find_last),
        "every" => Some(list::eval_every),
        "some" => Some(list::eval_some),
        "remove-if" => Some(list::eval_remove_if),
        "count-if" => Some(list::eval_count_if),
        "take-while" => Some(list::eval_take_while),
        "drop-while" => Some(list::eval_drop_while),
        "partition" => Some(list::eval_partition),
        "group-by" => Some(list::eval_group_by),
        "sort-by" => Some(list::eval_sort_by),

        // List mutation functions
        "remove" => Some(list_mutations::eval_remove),
        "remove-at" => Some(list_mutations::eval_remove_at),
        "insert" => Some(list_mutations::eval_insert),
        "set-at" => Some(list_mutations::eval_set_at),
        "pop" => Some(list_mutations::eval_pop),
        "shift" => Some(list_mutations::eval_shift),
        "unshift" => Some(list_mutations::eval_unshift),

        // List query functions
        "index-of" => Some(list_queries::eval_index_of),
        "contains" => Some(list_queries::eval_contains),
        "slice" => Some(list_queries::eval_slice),
        "concat" => Some(list_queries::eval_concat),
        "take" => Some(list_queries::eval_take),
        "drop" => Some(list_queries::eval_drop),

        // List advanced functions
        "flatten" => Some(list_advanced::eval_flatten),
        "unique" => Some(list_advanced::eval_unique),
        "zip" => Some(list_advanced::eval_zip),
        "chunk" => Some(list_advanced::eval_chunk),
        "transpose" => Some(list_advanced::eval_transpose),

        // Set operations
        "union" => Some(set_operations::eval_union),
        "difference" => Some(set_operations::eval_difference),
        "intersection" => Some(set_operations::eval_intersection),

        // String functions
        "split" => Some(string::eval_split),
        "join" => Some(string::eval_join),
        "trim" => Some(string::eval_trim),
        "uppercase" => Some(string::eval_uppercase),
        "lowercase" => Some(string::eval_lowercase),
        "starts-with" => Some(string::eval_starts_with),
        "ends-with" => Some(string::eval_ends_with),
        "substring" => Some(string::eval_substring),
        "replace" => Some(string::eval_replace),
        "repeat" => Some(string::eval_repeat),
        "pad-left" => Some(string::eval_pad_left),
        "pad-right" => Some(string::eval_pad_right),

        // Math functions
        "absolute" => Some(math::eval_absolute),
        "square-root" => Some(math::eval_square_root),
        "random" => Some(crate::random::eval_random_positional_wrapper),
        "sin" => Some(math::eval_sin),
        "cos" => Some(math::eval_cos),
        "tan" => Some(math::eval_tan),
        "asin" => Some(math::eval_asin),
        "acos" => Some(math::eval_acos),
        "atan" => Some(math::eval_atan),
        "atan2" => Some(math::eval_atan2),
        "log" => Some(math::eval_log),
        "log10" => Some(math::eval_log10),
        "exp" => Some(math::eval_exp),
        "pow" => Some(math::eval_pow),
        "round" => Some(math::eval_round),
        "floor" => Some(math::eval_floor),
        "ceil" => Some(math::eval_ceil),
        "pi" => Some(math::eval_pi),
        "e" => Some(math::eval_e),

        // Date functions
        "now" => Some(date::eval_now),
        "date" => Some(date::eval_date),
        "date-add" => Some(date::eval_date_add),
        "date-subtract" => Some(date::eval_date_subtract),
        "date-diff" => Some(date::eval_date_diff),
        "date-format" => Some(date::eval_date_format),
        "date-parse" => Some(date::eval_date_parse),
        "date-year" => Some(date::eval_date_year),
        "date-month" => Some(date::eval_date_month),
        "date-day" => Some(date::eval_date_day),
        "date-hour" => Some(date::eval_date_hour),
        "date-minute" => Some(date::eval_date_minute),
        "date-second" => Some(date::eval_date_second),
        "date-weekday" => Some(date::eval_date_weekday),
        "date-before" => Some(date::eval_date_before),
        "date-after" => Some(date::eval_date_after),
        "date-equal" => Some(date::eval_date_equal),

        // Object functions
        "keys" => Some(object::eval_keys),
        "values" => Some(object::eval_values),
        "has" => Some(object::eval_has),

        // Collection functions
        "length" => Some(collection::eval_length),
        "append" => Some(collection::eval_append),

        // Common helper functions - predicates
        "is-even" => Some(helpers::eval_is_even),
        "is-odd" => Some(helpers::eval_is_odd),
        "is-positive" => Some(helpers::eval_is_positive),
        "is-negative" => Some(helpers::eval_is_negative),
        "is-zero" => Some(helpers::eval_is_zero),

        // Common helper functions - transformations
        "double" => Some(helpers::eval_double),
        "triple" => Some(helpers::eval_triple),
        "quadruple" => Some(helpers::eval_quadruple),
        "half" => Some(helpers::eval_half),
        "square" => Some(helpers::eval_square),
        "increment" => Some(helpers::eval_increment),
        "decrement" => Some(helpers::eval_decrement),

        // Common helper functions - reductions
        "add" => Some(helpers::eval_add),
        "multiply" => Some(helpers::eval_multiply),
        "max" => Some(helpers::eval_max),
        "min" => Some(helpers::eval_min),

        // System functions
        "env" => Some(system::eval_env),

        // Type checking functions
        "is-number" => Some(type_checking::eval_is_number),
        "is-string" => Some(type_checking::eval_is_string),
        "is-boolean" => Some(type_checking::eval_is_boolean),
        "is-list" => Some(type_checking::eval_is_list),
        "is-object" => Some(type_checking::eval_is_object),
        "is-null" => Some(type_checking::eval_is_null),
        "is-empty" => Some(type_checking::eval_is_empty),
        "is-defined" => Some(type_checking::eval_is_defined),

        // JSON functions
        "to-json" => Some(json::eval_to_json),
        "from-json" => Some(json::eval_from_json),

        // Encoding functions
        "base64-encode" => Some(encoding::eval_base64_encode),
        "base64-decode" => Some(encoding::eval_base64_decode),
        "url-encode" => Some(encoding::eval_url_encode),
        "url-decode" => Some(encoding::eval_url_decode),

        // Cryptography functions
        "sha256" => Some(crypto::eval_sha256),
        "md5" => Some(crypto::eval_md5),
        "hmac-sha256" => Some(crypto::eval_hmac_sha256),

        // Filesystem functions
        "file-exists" => Some(filesystem::eval_file_exists),
        "dir-exists" => Some(filesystem::eval_dir_exists),
        "file-size" => Some(filesystem::eval_file_size),

        // Object manipulation functions
        "merge" => Some(object_manipulation::eval_merge),
        "pick" => Some(object_manipulation::eval_pick),
        "omit" => Some(object_manipulation::eval_omit),
        "object-get" => Some(object_manipulation::eval_object_get),
        "object-set" => Some(object_manipulation::eval_object_set),
        "deep-merge" => Some(object_manipulation::eval_deep_merge),

        _ => None,
    }
}