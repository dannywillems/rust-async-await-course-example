use std::time::Duration;
use tokio::time::sleep;

/// Example 1: Simple async state machine
///
/// This async function demonstrates how Rust transforms async functions into state machines.
/// When compiled, this becomes a state machine with states for:
/// - Initial state
/// - Suspended state (waiting on sleep)
/// - Completed state
///
/// The compiler generates code that can be paused at await points and resumed later.
pub async fn async_state_machine_example() {
    println!("  Starting async state machine...");

    // State transition 1: Before await
    let start_time = std::time::Instant::now();

    // Await point - function suspends here and yields control
    sleep(Duration::from_millis(100)).await;

    // State transition 2: After await
    let elapsed = start_time.elapsed();
    println!("  Completed after {:?}", elapsed);
}

/// Example 2: Multiple await points
///
/// This demonstrates how a single async function can have multiple suspension points.
/// Each await creates a new state in the generated state machine.
///
/// State machine states:
/// 1. Initial
/// 2. After first sleep
/// 3. After second sleep
/// 4. After third sleep
/// 5. Completed
pub async fn multiple_awaits_example() {
    println!("  Starting task with multiple awaits...");

    // First await point
    println!("  Awaiting first operation...");
    sleep(Duration::from_millis(50)).await;
    println!("  First operation completed");

    // Second await point
    println!("  Awaiting second operation...");
    sleep(Duration::from_millis(50)).await;
    println!("  Second operation completed");

    // Third await point
    println!("  Awaiting third operation...");
    sleep(Duration::from_millis(50)).await;
    println!("  Third operation completed");

    println!("  All operations finished!");
}

/// Example 3: Variable scoping across await boundaries
///
/// This demonstrates how variables are handled across await points.
/// Variables that live across awaits must be stored in the generated Future's state.
///
/// Important concepts:
/// - Variables before an await that aren't used after can be dropped
/// - Variables needed after an await are moved into the Future's state
/// - This affects what types can be used (must be Send for multi-threaded runtimes)
pub async fn variable_scoping_example() {
    println!("  Demonstrating variable scoping across awaits...");

    // Variable defined before await, used after
    let important_value = 42;
    println!("  Before await: important_value = {}", important_value);

    {
        // Variable scoped to this block, dropped before await
        let temporary_value = "temporary";
        println!("  Temporary value: {}", temporary_value);
    } // temporary_value dropped here

    // Await point - important_value must be stored in Future state
    sleep(Duration::from_millis(50)).await;

    // important_value is still available after await
    println!("  After await: important_value = {}", important_value);

    // New variable created after await
    let result = important_value * 2;
    println!("  Computed result: {}", result);
}

/// Example 4: Complex async function with error handling
///
/// This demonstrates:
/// - Async functions with parameters
/// - Returning Result types from async functions
/// - Multiple await points with error propagation
/// - Generic types in async functions
pub async fn complex_async_function(
    id: u32,
    data: String,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("  Processing request with id: {}, data: {}", id, data);

    // Simulate async validation
    sleep(Duration::from_millis(30)).await;

    if id == 0 {
        return Err("Invalid ID: cannot be zero".into());
    }

    // Simulate async processing
    sleep(Duration::from_millis(30)).await;

    let processed_data = format!("Processed(id={}, data={})", id, data);

    // Simulate async finalization
    sleep(Duration::from_millis(30)).await;

    Ok(processed_data)
}

/// Example 5: Real-world async HTTP request
///
/// This demonstrates using async with external libraries (reqwest).
/// Shows how async/await integrates with I/O operations.
///
/// Note: This makes a real network request. For tests, you might want to mock this.
pub async fn fetch_data_from_api(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("  Fetching data from: {}", url);

    // Create HTTP client
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // Make async HTTP request
    let response = client.get(url).send().await?;

    // Check response status
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    // Get response body as text
    let body = response.text().await?;

    println!("  Successfully fetched {} bytes", body.len());
    Ok(body)
}

/// Helper function to demonstrate concurrent execution
///
/// This shows how multiple async tasks can run concurrently using tokio::join!
pub async fn concurrent_execution_example() {
    println!("  Starting concurrent tasks...");

    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        println!("  Task 1 completed");
        1
    };

    let task2 = async {
        sleep(Duration::from_millis(50)).await;
        println!("  Task 2 completed");
        2
    };

    let task3 = async {
        sleep(Duration::from_millis(75)).await;
        println!("  Task 3 completed");
        3
    };

    // All tasks run concurrently and complete when all are done
    let (result1, result2, result3) = tokio::join!(task1, task2, task3);

    println!(
        "  All tasks completed: {} + {} + {} = {}",
        result1,
        result2,
        result3,
        result1 + result2 + result3
    );
}

/// Example showing async function that returns a custom Future
///
/// This demonstrates that async functions are syntactic sugar for functions
/// returning impl Future<Output = T>
pub async fn async_sugar_example() -> i32 {
    sleep(Duration::from_millis(10)).await;
    42
}

// The above is equivalent to:
// pub fn async_sugar_example() -> impl Future<Output = i32> {
//     async {
//         sleep(Duration::from_millis(10)).await;
//         42
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_state_machine() {
        // This should complete without panicking
        async_state_machine_example().await;
    }

    #[tokio::test]
    async fn test_multiple_awaits() {
        // This should complete all awaits successfully
        multiple_awaits_example().await;
    }

    #[tokio::test]
    async fn test_variable_scoping() {
        // Variables should be properly scoped across awaits
        variable_scoping_example().await;
    }

    #[tokio::test]
    async fn test_complex_async_function_success() {
        let result = complex_async_function(42, "test".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Processed"));
    }

    #[tokio::test]
    async fn test_complex_async_function_error() {
        let result = complex_async_function(0, "test".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_execution() {
        concurrent_execution_example().await;
    }

    #[tokio::test]
    async fn test_async_sugar() {
        let result = async_sugar_example().await;
        assert_eq!(result, 42);
    }

    // Note: We don't test fetch_data_from_api in unit tests as it requires network access
    // In a real project, you'd use mocking or integration tests for this
}
