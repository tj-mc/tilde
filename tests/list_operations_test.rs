use tilde::evaluator::Evaluator;
use tilde::parser::Parser;
use tilde::value::Value;

fn eval_expression(input: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    let mut parser = Parser::new(input);
    let program = parser.parse()?;
    evaluator.eval_program(program)
}

#[cfg(test)]
mod predicate_tests {
    use super::*;

    #[test]
    fn test_filter_with_stdlib_predicates() {
        let result = eval_expression("filter [1, 2, 3, 4, 5, 6] is-even").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0)
            ])
        );

        let result = eval_expression("filter [1, 2, 3, 4, 5] is-odd").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(5.0)
            ])
        );

        let result = eval_expression("filter [1, 2, 0, -1, 3] is-positive").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
        );
    }

    #[test]
    fn test_find_with_stdlib_predicates() {
        let result = eval_expression("find [1, 2, 3, 4] is-even").unwrap();
        assert_eq!(result, Value::Number(2.0));

        let result = eval_expression("find [2, 4, 6] is-odd").unwrap();
        assert_eq!(result, Value::Null);

        let result = eval_expression("find [-1, 0, 1, 2] is-positive").unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_find_index_with_stdlib_predicates() {
        let result = eval_expression("find-index [1, 2, 3, 4] is-even").unwrap();
        assert_eq!(result, Value::Number(1.0));

        let result = eval_expression("find-index [1, 3, 5] is-even").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_find_last_with_stdlib_predicates() {
        let result = eval_expression("find-last [1, 2, 3, 4, 6] is-even").unwrap();
        assert_eq!(result, Value::Number(6.0));

        let result = eval_expression("find-last [2, 4, 6] is-odd").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_every_with_stdlib_predicates() {
        let result = eval_expression("every [1, 3, 5] is-odd").unwrap();
        assert_eq!(result, Value::Boolean(true));

        let result = eval_expression("every [1, 2, 3] is-odd").unwrap();
        assert_eq!(result, Value::Boolean(false));

        let result = eval_expression("every [2, 4, 6] is-even").unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_some_with_stdlib_predicates() {
        let result = eval_expression("some [1, 2, 3] is-even").unwrap();
        assert_eq!(result, Value::Boolean(true));

        let result = eval_expression("some [1, 3, 5] is-even").unwrap();
        assert_eq!(result, Value::Boolean(false));

        let result = eval_expression("some [-1, 0, 1] is-positive").unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_remove_if_with_stdlib_predicates() {
        let result = eval_expression("remove-if [1, 2, 3, 4, 5] is-even").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(5.0)
            ])
        );

        let result = eval_expression("remove-if [1, 2, 3, 4, 5] is-odd").unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(2.0), Value::Number(4.0)])
        );
    }

    #[test]
    fn test_count_if_with_stdlib_predicates() {
        let result = eval_expression("count-if [1, 2, 3, 4, 5] is-even").unwrap();
        assert_eq!(result, Value::Number(2.0));

        let result = eval_expression("count-if [1, 2, 3, 4, 5] is-odd").unwrap();
        assert_eq!(result, Value::Number(3.0));

        let result = eval_expression("count-if [-1, 0, 1, 2] is-positive").unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_take_while_with_stdlib_predicates() {
        let result = eval_expression("take-while [1, 3, 5, 2, 7] is-odd").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(5.0)
            ])
        );

        let result = eval_expression("take-while [2, 4, 6, 1] is-even").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0)
            ])
        );
    }

    #[test]
    fn test_drop_while_with_stdlib_predicates() {
        let result = eval_expression("drop-while [1, 3, 5, 2, 7] is-odd").unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(2.0), Value::Number(7.0)])
        );

        let result = eval_expression("drop-while [2, 4, 6, 1] is-even").unwrap();
        assert_eq!(result, Value::List(vec![Value::Number(1.0)]));
    }

    #[test]
    fn test_map_with_stdlib_transformers() {
        let result = eval_expression("map [1, 2, 3] double").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(2.0),
                Value::Number(4.0),
                Value::Number(6.0)
            ])
        );

        let result = eval_expression("map [2, 4, 6] half").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
        );

        let result = eval_expression("map [2, 3, 4] square").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(4.0),
                Value::Number(9.0),
                Value::Number(16.0)
            ])
        );
    }
}

#[cfg(test)]
mod new_list_functions_tests {
    use super::*;

    #[test]
    fn test_partition_with_stdlib_predicates() {
        let result = eval_expression("partition [1, 2, 3, 4, 5, 6] is-even").unwrap();

        if let Value::Object(map) = result {
            let matched = map.get("matched").unwrap();
            let unmatched = map.get("unmatched").unwrap();

            assert_eq!(
                *matched,
                Value::List(vec![
                    Value::Number(2.0),
                    Value::Number(4.0),
                    Value::Number(6.0)
                ])
            );
            assert_eq!(
                *unmatched,
                Value::List(vec![
                    Value::Number(1.0),
                    Value::Number(3.0),
                    Value::Number(5.0)
                ])
            );
        } else {
            panic!("partition should return an object");
        }
    }

    #[test]
    fn test_partition_with_all_matching() {
        let result = eval_expression("partition [2, 4, 6] is-even").unwrap();

        if let Value::Object(map) = result {
            let matched = map.get("matched").unwrap();
            let unmatched = map.get("unmatched").unwrap();

            assert_eq!(
                *matched,
                Value::List(vec![
                    Value::Number(2.0),
                    Value::Number(4.0),
                    Value::Number(6.0)
                ])
            );
            assert_eq!(*unmatched, Value::List(vec![]));
        } else {
            panic!("partition should return an object");
        }
    }

    #[test]
    fn test_partition_with_none_matching() {
        let result = eval_expression("partition [1, 3, 5] is-even").unwrap();

        if let Value::Object(map) = result {
            let matched = map.get("matched").unwrap();
            let unmatched = map.get("unmatched").unwrap();

            assert_eq!(*matched, Value::List(vec![]));
            assert_eq!(
                *unmatched,
                Value::List(vec![
                    Value::Number(1.0),
                    Value::Number(3.0),
                    Value::Number(5.0)
                ])
            );
        } else {
            panic!("partition should return an object");
        }
    }

    #[test]
    fn test_group_by_with_stdlib_functions() {
        // Group by length - using string length
        let result = eval_expression(r#"group-by ["cat", "dog", "bird", "cow"] length"#).unwrap();

        if let Value::Object(map) = result {
            let group_3 = map.get("3").unwrap();
            let group_4 = map.get("4").unwrap();

            if let Value::List(items) = group_3 {
                assert_eq!(items.len(), 3); // cat, dog, cow
                assert!(items.contains(&Value::String("cat".to_string())));
                assert!(items.contains(&Value::String("dog".to_string())));
                assert!(items.contains(&Value::String("cow".to_string())));
            } else {
                panic!("group should contain a list");
            }

            if let Value::List(items) = group_4 {
                assert_eq!(items.len(), 1); // bird
                assert!(items.contains(&Value::String("bird".to_string())));
            } else {
                panic!("group should contain a list");
            }
        } else {
            panic!("group-by should return an object");
        }
    }

    #[test]
    fn test_group_by_with_numbers() {
        // Group by parity (even/odd using modulo)
        let result = eval_expression("group-by [1, 2, 3, 4, 5, 6] is-even").unwrap();

        if let Value::Object(map) = result {
            let true_group = map.get("true").unwrap();
            let false_group = map.get("false").unwrap();

            assert_eq!(
                *true_group,
                Value::List(vec![
                    Value::Number(2.0),
                    Value::Number(4.0),
                    Value::Number(6.0)
                ])
            );
            assert_eq!(
                *false_group,
                Value::List(vec![
                    Value::Number(1.0),
                    Value::Number(3.0),
                    Value::Number(5.0)
                ])
            );
        } else {
            panic!("group-by should return an object");
        }
    }

    #[test]
    fn test_sort_by_with_stdlib_functions() {
        // Sort strings by length
        let result =
            eval_expression(r#"sort-by ["apple", "pie", "banana", "kiwi"] length"#).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("pie".to_string()),
                Value::String("kiwi".to_string()),
                Value::String("apple".to_string()),
                Value::String("banana".to_string())
            ])
        );
    }

    #[test]
    fn test_sort_by_with_numbers() {
        // Sort by absolute value (when we have negative numbers)
        let result = eval_expression("sort-by [-3, 1, -1, 4, -2] absolute").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(-1.0),
                Value::Number(-2.0),
                Value::Number(-3.0),
                Value::Number(4.0)
            ])
        );
    }

    #[test]
    fn test_union_basic() {
        let result = eval_expression("union [1, 2, 3] [3, 4, 5]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0)
            ])
        );
    }

    #[test]
    fn test_union_with_duplicates() {
        let result = eval_expression("union [1, 2, 2, 3] [3, 4, 4, 5]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0)
            ])
        );
    }

    #[test]
    fn test_union_mixed_types() {
        let result = eval_expression(r#"union [1, "hello", true] [2, "hello", false]"#).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::String("hello".to_string()),
                Value::Boolean(true),
                Value::Number(2.0),
                Value::Boolean(false)
            ])
        );
    }

    #[test]
    fn test_difference_basic() {
        let result = eval_expression("difference [1, 2, 3, 4] [2, 4, 6]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(1.0), Value::Number(3.0)])
        );
    }

    #[test]
    fn test_difference_no_overlap() {
        let result = eval_expression("difference [1, 2, 3] [4, 5, 6]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
        );
    }

    #[test]
    fn test_difference_complete_overlap() {
        let result = eval_expression("difference [1, 2, 3] [1, 2, 3]").unwrap();
        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_intersection_basic() {
        let result = eval_expression("intersection [1, 2, 3, 4] [2, 4, 6]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(2.0), Value::Number(4.0)])
        );
    }

    #[test]
    fn test_intersection_no_overlap() {
        let result = eval_expression("intersection [1, 2, 3] [4, 5, 6]").unwrap();
        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_intersection_with_duplicates() {
        let result = eval_expression("intersection [1, 2, 2, 3] [2, 2, 3, 4]").unwrap();
        // Should remove duplicates and only include each element once
        assert_eq!(
            result,
            Value::List(vec![Value::Number(2.0), Value::Number(3.0)])
        );
    }

    #[test]
    fn test_set_operations_error_handling() {
        // Test with non-list arguments
        assert!(eval_expression("union 123 [1, 2, 3]").is_err());
        assert!(eval_expression("difference [1, 2, 3] 456").is_err());
        assert!(eval_expression("intersection 789 abc").is_err());

        // Test with wrong number of arguments
        assert!(eval_expression("union [1, 2, 3]").is_err());
        assert!(eval_expression("difference [1, 2, 3] [4, 5, 6] [7, 8, 9]").is_err());
    }

    #[test]
    fn test_new_functions_error_handling() {
        // Test partition errors
        assert!(eval_expression("partition 123 is-even").is_err());
        assert!(eval_expression("partition [1, 2, 3]").is_err());

        // Test group-by errors
        assert!(eval_expression("group-by 123 length").is_err());
        assert!(eval_expression("group-by [1, 2, 3]").is_err());

        // Test sort-by errors
        assert!(eval_expression("sort-by 123 length").is_err());
        assert!(eval_expression("sort-by [1, 2, 3]").is_err());
    }

    #[test]
    fn test_empty_lists() {
        // Test with empty lists
        let result = eval_expression("partition [] is-even").unwrap();
        if let Value::Object(map) = result {
            assert_eq!(*map.get("matched").unwrap(), Value::List(vec![]));
            assert_eq!(*map.get("unmatched").unwrap(), Value::List(vec![]));
        }

        let result = eval_expression("group-by [] length").unwrap();
        if let Value::Object(map) = result {
            assert!(map.is_empty());
        }

        let result = eval_expression("sort-by [] length").unwrap();
        assert_eq!(result, Value::List(vec![]));

        let result = eval_expression("union [] [1, 2, 3]").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
        );

        let result = eval_expression("difference [] [1, 2, 3]").unwrap();
        assert_eq!(result, Value::List(vec![]));

        let result = eval_expression("intersection [] [1, 2, 3]").unwrap();
        assert_eq!(result, Value::List(vec![]));
    }
}

#[cfg(test)]
mod list_mutations_tests {
    use super::*;

    #[test]
    fn test_remove_integration() {
        // Test what assignment returns (should be the assigned value)
        let result = eval_expression("~result is remove [1, 2, 3, 2, 4] 2").unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(2.0),
                Value::Number(4.0),
            ])
        );

        // Basic remove with separate assignment and give
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 2, 4]
            ~result is remove ~list 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(3.0),
                Value::Number(2.0),
                Value::Number(4.0),
            ])
        );

        // Remove non-existent element
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is remove ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );

        // Remove from empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is remove ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_remove_at_integration() {
        // Basic remove-at
        let result = eval_expression(
            "
            ~list is [\"a\", \"b\", \"c\", \"d\"]
            ~result is remove-at ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".to_string()),
                Value::String("c".to_string()),
                Value::String("d".to_string()),
            ])
        );

        // Remove at index 0
        let result = eval_expression(
            "
            ~list is [10, 20, 30]
            ~result is remove-at ~list 0
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(20.0), Value::Number(30.0),])
        );

        // Remove last element
        let result = eval_expression(
            "
            ~list is [10, 20, 30]
            ~result is remove-at ~list 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(10.0), Value::Number(20.0),])
        );

        // Out of bounds error
        let result = eval_expression(
            "
            ~list is [1, 2]
            ~result is remove-at ~list 5
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }

    #[test]
    fn test_insert_integration() {
        // Basic insert
        let result = eval_expression(
            "
            ~list is [1, 3, 4]
            ~result is insert ~list 1 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Insert at beginning
        let result = eval_expression(
            "
            ~list is [2, 3, 4]
            ~result is insert ~list 0 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Insert at end
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is insert ~list 3 4
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Insert in empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is insert ~list 0 \"hello\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::String("hello".to_string()),])
        );
    }

    #[test]
    fn test_set_at_integration() {
        // Basic set-at
        let result = eval_expression(
            "
            ~list is [\"a\", \"b\", \"c\"]
            ~result is set-at ~list 1 \"x\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("a".to_string()),
                Value::String("x".to_string()),
                Value::String("c".to_string()),
            ])
        );

        // Set first element
        let result = eval_expression(
            "
            ~list is [10, 20, 30]
            ~result is set-at ~list 0 100
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(100.0),
                Value::Number(20.0),
                Value::Number(30.0),
            ])
        );

        // Set last element
        let result = eval_expression(
            "
            ~list is [10, 20, 30]
            ~result is set-at ~list 2 300
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(10.0),
                Value::Number(20.0),
                Value::Number(300.0),
            ])
        );
    }

    #[test]
    fn test_pop_integration() {
        // Basic pop
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is pop ~list
            give ~result
        ",
        )
        .unwrap();
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("value"), Some(&Value::Number(3.0)));
            assert_eq!(
                obj.get("list"),
                Some(&Value::List(vec![Value::Number(1.0), Value::Number(2.0),]))
            );
        } else {
            panic!("Expected object result from pop");
        }

        // Pop single element
        let result = eval_expression(
            "
            ~list is [42]
            ~result is pop ~list
            give ~result
        ",
        )
        .unwrap();
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("value"), Some(&Value::Number(42.0)));
            assert_eq!(obj.get("list"), Some(&Value::List(vec![])));
        } else {
            panic!("Expected object result from pop");
        }

        // Pop from empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is pop ~list
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty list"));
    }

    #[test]
    fn test_shift_integration() {
        // Basic shift
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is shift ~list
            give ~result
        ",
        )
        .unwrap();
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("value"), Some(&Value::Number(1.0)));
            assert_eq!(
                obj.get("list"),
                Some(&Value::List(vec![Value::Number(2.0), Value::Number(3.0),]))
            );
        } else {
            panic!("Expected object result from shift");
        }
    }

    #[test]
    fn test_unshift_integration() {
        // Basic unshift
        let result = eval_expression(
            "
            ~list is [2, 3, 4]
            ~result is unshift ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Unshift to empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is unshift ~list \"first\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::String("first".to_string()),])
        );
    }
}

#[cfg(test)]
mod list_queries_tests {
    use super::*;

    #[test]
    fn test_index_of_integration() {
        // Found case
        let result = eval_expression(
            "
            ~list is [\"apple\", \"banana\", \"cherry\", \"banana\"]
            ~result is index-of ~list \"banana\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Number(1.0));

        // Not found case
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is index-of ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Null);

        // Empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is index-of ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_contains_integration() {
        // Contains case
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4]
            ~result is contains ~list 3
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Does not contain case
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4]
            ~result is contains ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Boolean(false));

        // String contains
        let result = eval_expression(
            "
            ~list is [\"hello\", \"world\", \"test\"]
            ~result is contains ~list \"world\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_slice_integration() {
        // Basic slice with start and end
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5, 6]
            ~result is slice ~list 1 4
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Slice to end
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5]
            ~result is slice ~list 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0),
            ])
        );

        // Slice beyond bounds
        let result = eval_expression(
            "
            ~list is [1, 2]
            ~result is slice ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));

        // Slice entire list
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is slice ~list 0
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_concat_integration() {
        // Concat multiple lists
        let result = eval_expression(
            "
            ~list1 is [1, 2]
            ~list2 is [3, 4]
            ~list3 is [5]
            ~result is concat ~list1 ~list2 ~list3
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0),
            ])
        );

        // Concat with empty lists
        let result = eval_expression(
            "
            ~list1 is []
            ~list2 is [1, 2]
            ~list3 is []
            ~result is concat ~list1 ~list2 ~list3
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(1.0), Value::Number(2.0),])
        );
    }

    #[test]
    fn test_take_integration() {
        // Basic take
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5]
            ~result is take ~list 3
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );

        // Take more than available
        let result = eval_expression(
            "
            ~list is [1, 2]
            ~result is take ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(1.0), Value::Number(2.0),])
        );

        // Take zero
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is take ~list 0
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_drop_integration() {
        // Basic drop
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5]
            ~result is drop ~list 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0),
            ])
        );

        // Drop more than available
        let result = eval_expression(
            "
            ~list is [1, 2]
            ~result is drop ~list 5
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));

        // Drop zero
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is drop ~list 0
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }
}

#[cfg(test)]
mod list_advanced_tests {
    use super::*;

    #[test]
    fn test_flatten_integration() {
        // Basic flatten
        let result = eval_expression(
            "
            ~list is [1, [2, 3], 4, [5, [6, 7]]]
            ~result is flatten ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
                Value::Number(5.0),
                Value::Number(6.0),
                Value::Number(7.0),
            ])
        );

        // Flatten with depth
        let result = eval_expression(
            "
            ~list is [1, [2, [3, 4]], 5]
            ~result is flatten ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::List(vec![Value::Number(3.0), Value::Number(4.0)]),
                Value::Number(5.0),
            ])
        );

        // Already flat list
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is flatten ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_unique_integration() {
        // Basic unique
        let result = eval_expression(
            "
            ~list is [1, 2, 2, 3, 1, 4, 3]
            ~result is unique ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Mixed types unique
        let result = eval_expression(
            "
            ~list is [1, \"hello\", 1, true, \"hello\", false, true]
            ~result is unique ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::String("hello".to_string()),
                Value::Boolean(true),
                Value::Boolean(false),
            ])
        );

        // Already unique
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is unique ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_zip_integration() {
        // Basic zip
        let result = eval_expression(
            "
            ~list1 is [1, 2, 3]
            ~list2 is [\"a\", \"b\", \"c\"]
            ~result is zip ~list1 ~list2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0), Value::String("a".to_string())]),
                Value::List(vec![Value::Number(2.0), Value::String("b".to_string())]),
                Value::List(vec![Value::Number(3.0), Value::String("c".to_string())]),
            ])
        );

        // Different lengths
        let result = eval_expression(
            "
            ~list1 is [1, 2, 3, 4]
            ~list2 is [\"a\", \"b\"]
            ~result is zip ~list1 ~list2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0), Value::String("a".to_string())]),
                Value::List(vec![Value::Number(2.0), Value::String("b".to_string())]),
            ])
        );

        // Empty lists
        let result = eval_expression(
            "
            ~list1 is []
            ~list2 is [1, 2]
            ~result is zip ~list1 ~list2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_chunk_integration() {
        // Basic chunk
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5, 6, 7]
            ~result is chunk ~list 3
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![
                    Value::Number(1.0),
                    Value::Number(2.0),
                    Value::Number(3.0)
                ]),
                Value::List(vec![
                    Value::Number(4.0),
                    Value::Number(5.0),
                    Value::Number(6.0)
                ]),
                Value::List(vec![Value::Number(7.0)]),
            ])
        );

        // Exact division
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4]
            ~result is chunk ~list 2
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0), Value::Number(2.0)]),
                Value::List(vec![Value::Number(3.0), Value::Number(4.0)]),
            ])
        );

        // Single element chunks
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is chunk ~list 1
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0)]),
                Value::List(vec![Value::Number(2.0)]),
                Value::List(vec![Value::Number(3.0)]),
            ])
        );
    }

    #[test]
    fn test_transpose_integration() {
        // Basic transpose
        let result = eval_expression(
            "
            ~matrix is [[1, 2, 3], [4, 5, 6]]
            ~result is transpose ~matrix
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0), Value::Number(4.0)]),
                Value::List(vec![Value::Number(2.0), Value::Number(5.0)]),
                Value::List(vec![Value::Number(3.0), Value::Number(6.0)]),
            ])
        );

        // Transpose single row
        let result = eval_expression(
            "
            ~matrix is [[1, 2, 3]]
            ~result is transpose ~matrix
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0)]),
                Value::List(vec![Value::Number(2.0)]),
                Value::List(vec![Value::Number(3.0)]),
            ])
        );

        // Empty matrix
        let result = eval_expression(
            "
            ~matrix is []
            ~result is transpose ~matrix
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::List(vec![]));
    }
}

#[cfg(test)]
mod edge_cases_and_error_handling {
    use super::*;

    #[test]
    fn test_type_errors() {
        // Non-list first argument
        let result = eval_expression(
            "
            ~result is remove 123 1
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a list"));

        // Non-number index
        let result = eval_expression(
            "
            ~result is remove-at [1, 2, 3] \"not-a-number\"
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a number"));

        // Negative index
        let result = eval_expression(
            "
            ~result is slice [1, 2, 3] -1
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-negative"));
    }

    #[test]
    fn test_functional_immutability() {
        // Verify original lists are not modified
        let result = eval_expression(
            "
            ~original is [1, 2, 3]
            ~modified is remove ~original 2
            give ~original
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );

        let result = eval_expression(
            "
            ~original is [1, 2, 3]
            ~modified is insert ~original 1 99
            give ~original
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_complex_chaining() {
        // Chain multiple operations
        let result = eval_expression(
            "
            ~data is [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            ~evens is filter ~data is-even
            ~doubled is map ~evens double
            ~sliced is slice ~doubled 1 3
            ~unique is unique ~sliced
            give ~unique
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Number(8.0), Value::Number(12.0),])
        );

        // Complex processing pipeline
        let result = eval_expression(
            "
            ~nested is [[1, 2], [3, 4], [2, 1]]
            ~flattened is flatten ~nested
            ~uniqued is unique ~flattened
            ~sorted is sort ~uniqued
            ~chunked is chunk ~sorted 2
            give ~chunked
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::List(vec![Value::Number(1.0), Value::Number(2.0)]),
                Value::List(vec![Value::Number(3.0), Value::Number(4.0)]),
            ])
        );
    }
}

#[cfg(test)]
mod collection_functions_tests {
    use super::*;

    #[test]
    fn test_append_as_stdlib_function() {
        // Basic append functionality
        let result = eval_expression(
            "
            ~list is [1, 2, 3]
            ~result is append ~list 4
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ])
        );

        // Append to empty list
        let result = eval_expression(
            "
            ~list is []
            ~result is append ~list \"first\"
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::String("first".to_string()),])
        );

        // Test that original list is unchanged (functional)
        let result = eval_expression(
            "
            ~original is [1, 2, 3]
            ~modified is append ~original 4
            give ~original
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_length_as_stdlib_function() {
        // Length of list
        let result = eval_expression(
            "
            ~list is [1, 2, 3, 4, 5]
            ~result is length ~list
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Number(5.0));

        // Length of string
        let result = eval_expression(
            "
            ~str is \"hello world\"
            ~result is length ~str
            give ~result
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Number(11.0));
    }

    #[test]
    fn test_append_length_chaining() {
        // Chain append and length operations
        let result = eval_expression(
            "
            ~list is [1, 2]
            ~step1 is append ~list 3
            ~step2 is append ~step1 4
            ~final_length is length ~step2
            give ~final_length
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Number(4.0));

        // Complex chaining with new operations
        let result = eval_expression(
            "
            ~base is [1, 2, 3]
            ~appended is append ~base 4
            ~inserted is insert ~appended 0 0
            ~final_length is length ~inserted
            give ~final_length
        ",
        )
        .unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_collection_error_handling() {
        // append on non-list
        let result = eval_expression(
            "
            ~not_list is 42
            ~result is append ~not_list \"item\"
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("append can only be used on lists")
        );

        // Wrong number of arguments
        let result = eval_expression(
            "
            ~result is append [1, 2, 3]
            give ~result
        ",
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("append requires exactly two arguments")
        );
    }
}
