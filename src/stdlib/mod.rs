pub mod list;
pub mod string;
pub mod math;
pub mod helpers;
mod utils;

use crate::ast::Expression;
use crate::evaluator::Evaluator;
use crate::value::Value;

/// Register all standard library functions
pub fn get_stdlib_function(name: &str) -> Option<fn(Vec<Expression>, &mut Evaluator) -> Result<Value, String>> {
    match name {
        // List functions
        "map" => Some(list::eval_map),
        "filter" => Some(list::eval_filter),
        "reduce" => Some(list::eval_reduce),
        "sort" => Some(list::eval_sort),
        "reverse" => Some(list::eval_reverse),

        // String functions
        "split" => Some(string::eval_split),
        "join" => Some(string::eval_join),
        "trim" => Some(string::eval_trim),
        "uppercase" => Some(string::eval_uppercase),
        "lowercase" => Some(string::eval_lowercase),

        // Math functions
        "absolute" => Some(math::eval_absolute),
        "square-root" => Some(math::eval_square_root),

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