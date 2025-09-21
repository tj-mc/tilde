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
mod utils;

use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Get all standard library function names
pub fn get_stdlib_function_names() -> &'static [&'static str] {
    &[
        // List functions
        "map", "filter", "reduce", "sort", "reverse",
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
        // Math functions
        "absolute", "square-root", "random",
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

        // Math functions
        "absolute" => Some(math::eval_absolute),
        "square-root" => Some(math::eval_square_root),
        "random" => Some(crate::random::eval_random_positional_wrapper),

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

        _ => None,
    }
}