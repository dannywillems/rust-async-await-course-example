use rust_async_await_course_example::{
    async_state_machine_example, complex_async_function, fetch_data_from_api,
    multiple_awaits_example, variable_scoping_example,
};

/// Main entry point demonstrating various async/await patterns in Rust.
///
/// This example showcases:
/// - State machine transformation of async functions
/// - Multiple await points in a single function
/// - Variable scoping across await boundaries
/// - Real-world async patterns with tokio runtime
#[tokio::main]
async fn main() {
    println!("=== Rust Async/Await Course Examples ===\n");

    // Example 1: Simple async state machine
    println!("1. Async State Machine Example:");
    async_state_machine_example().await;
    println!();

    // Example 2: Multiple await points
    println!("2. Multiple Awaits Example:");
    multiple_awaits_example().await;
    println!();

    // Example 3: Variable scoping across awaits
    println!("3. Variable Scoping Example:");
    variable_scoping_example().await;
    println!();

    // Example 4: Complex async function with error handling
    println!("4. Complex Async Function Example:");
    match complex_async_function(42, "test-data".to_string()).await {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 5: Real HTTP request (optional - commented to avoid network dependency)
    // Uncomment to test with real network calls
    println!("5. HTTP Request Example (simulated):");
    match fetch_data_from_api("https://api.github.com/repos/rust-lang/rust").await {
        Ok(data) => println!(
            "Fetched data (first 100 chars): {}...",
            &data[..data.len().min(100)]
        ),
        Err(e) => println!("Failed to fetch data: {}", e),
    }
    println!();

    println!("=== All examples completed ===");
}
