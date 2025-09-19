pub mod list;
pub mod string;
pub mod math;

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

        // Math functions
        "absolute" => Some(math::eval_absolute),
        "square-root" => Some(math::eval_square_root),

        _ => None,
    }
}