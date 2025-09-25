use std::process::Command;

#[test]
fn test_traveling_salesman_problem() {
    let tsp_program = r#"
say "🗺️ Traveling Salesman Problem Integration Test"
say "=============================================="

~distance_matrix is [
    [0, 10, 15, 20],
    [10, 0, 35, 25],
    [15, 35, 0, 30],
    [20, 25, 30, 0]
]

function calculate-distance ~path ~distances (
    ~total is 0
    ~i is 0
    ~path_len is length ~path
    loop (
        ~next_i is ~i + 1
        if ~next_i >= ~path_len break-loop
        ~from is ~path.~i
        ~to is ~path.~next_i
        ~from_row is ~distances.~from
        ~dist is ~from_row.~to
        ~total is ~total + ~dist
        ~i is ~next_i
    )
    give ~total
)

say "Testing distance calculation..."
~test_path is [0, 1, 3, 2]
~test_distance is *calculate-distance ~test_path ~distance_matrix
say "Path [0, 1, 3, 2] distance: " ~test_distance

say ""
say "Testing all 4-city routes:"

~all_paths is [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1]
]

~best_distance is 999999
~best_path is []
~i is 0
~paths_len is length ~all_paths

loop (
    if ~i >= ~paths_len break-loop
    ~path is ~all_paths.~i
    ~distance is *calculate-distance ~path ~distance_matrix

    say "Path " ~path " = distance " ~distance

    if ~distance < ~best_distance (
        ~best_distance is ~distance
        ~best_path is ~path
    )
    ~i up 1
)

say ""
say "OPTIMAL SOLUTION:"
say "Best path: " ~best_path
say "Total distance: " ~best_distance

# Verify the expected result
if ~best_distance == 65 (
    say "✅ TEST PASSED: Found optimal distance 65"
) else (
    say "❌ TEST FAILED: Expected distance 65, got " ~best_distance
)

if ~best_path.0 == 0 and ~best_path.1 == 1 and ~best_path.2 == 3 and ~best_path.3 == 2 (
    say "✅ TEST PASSED: Found optimal path [0, 1, 3, 2]"
) else (
    say "❌ TEST FAILED: Expected path [0, 1, 3, 2], got " ~best_path
)
"#;

    // Write the TSP program to a temporary file
    let temp_file = "temp_tsp_test.tde";
    std::fs::write(temp_file, tsp_program).expect("Failed to write test file");

    // Execute the Tilde interpreter
    let output = Command::new("cargo")
        .args(&["run", "--", temp_file])
        .output()
        .expect("Failed to execute tilde");

    // Clean up
    std::fs::remove_file(temp_file).ok();

    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    // Verify the program executed successfully
    assert!(output.status.success(), "Tilde program failed to execute");

    // Verify the output contains expected results
    assert!(stdout.contains("✅ TEST PASSED: Found optimal distance 65"));
    assert!(stdout.contains("✅ TEST PASSED: Found optimal path [0, 1, 3, 2]"));
    assert!(stdout.contains("Path [0, 1, 3, 2] distance: 65"));
    assert!(stdout.contains("Best path: [0, 1, 3, 2]"));
    assert!(stdout.contains("Total distance: 65"));

    // Verify all paths were tested
    assert!(stdout.contains("Path [0, 1, 2, 3] = distance 75"));
    assert!(stdout.contains("Path [0, 2, 3, 1] = distance 70"));
    assert!(stdout.contains("Path [0, 3, 2, 1] = distance 85"));
}

#[test]
fn test_dynamic_property_access() {
    let dynamic_access_program = r#"
say "🔧 Dynamic Property Access Test"
say "==============================="

function test-dynamic ~path ~index (
    say "Testing dynamic access: path[" ~index "] = " ~path.~index
    give ~path.~index
)

~test_array is [10, 20, 30, 40, 50]

~result0 is *test-dynamic ~test_array 0
~result1 is *test-dynamic ~test_array 1
~result4 is *test-dynamic ~test_array 4

if ~result0 == 10 and ~result1 == 20 and ~result4 == 50 (
    say "✅ Dynamic property access working correctly"
) else (
    say "❌ Dynamic property access failed"
    say "Expected: 10, 20, 50"
    say "Got: " ~result0 ", " ~result1 ", " ~result4
)
"#;

    let temp_file = "temp_dynamic_test.tde";
    std::fs::write(temp_file, dynamic_access_program).expect("Failed to write test file");

    let output = Command::new("cargo")
        .args(&["run", "--", temp_file])
        .output()
        .expect("Failed to execute tilde");

    std::fs::remove_file(temp_file).ok();

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Dynamic access test output:\n{}", stdout);

    assert!(
        output.status.success(),
        "Dynamic access test failed to execute"
    );
    assert!(stdout.contains("✅ Dynamic property access working correctly"));
}

#[test]
fn test_recursive_functions() {
    let recursive_program = r#"
say "🔄 Recursive Function Test"
say "=========================="

function factorial ~n (
    if ~n <= 1 (
        give 1
    ) else (
        ~n_minus_one is ~n - 1
        ~recursive_result is *factorial ~n_minus_one
        give ~n * ~recursive_result
    )
)

~fact_5 is *factorial 5
say "5! = " ~fact_5

if ~fact_5 == 120 (
    say "✅ Recursive factorial working correctly"
) else (
    say "❌ Recursive factorial failed: expected 120, got " ~fact_5
)
"#;

    let temp_file = "temp_recursive_test.tde";
    std::fs::write(temp_file, recursive_program).expect("Failed to write test file");

    let output = Command::new("cargo")
        .args(&["run", "--", temp_file])
        .output()
        .expect("Failed to execute tilde");

    std::fs::remove_file(temp_file).ok();

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Recursive test output:\n{}", stdout);

    assert!(output.status.success(), "Recursive test failed to execute");
    assert!(stdout.contains("✅ Recursive factorial working correctly"));
    assert!(stdout.contains("5! = 120"));
}
