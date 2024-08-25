use blockless_rust_sdk::wasm::load_wasm_module;
use blockless_rust_sdk::execution::ExecutionFlow;
use blockless_rust_sdk::http::BlocklessAPI;
use blockless_rust_sdk::safety::SafeData;
use std::fs::File;
use std::io::Read;
use std::path::Path;


#[tokio::test]
async fn test_load_wasm_module() {
    // Specify the path to your .wasm file using a raw string literal
    let wasm_file_path = Path::new(r"C:\Users\mallikarjun\OneDrive\blockchain\blockless_rust_sdk  copy version\wasm_demo\target\wasm32-unknown-unknown\release\wasm_demo.wasm");

    // Read the WASM file into a byte vector
    let mut file = File::open(wasm_file_path).expect("Failed to open WASM file");
    let mut wasm_bytes = Vec::new();
    file.read_to_end(&mut wasm_bytes).expect("Failed to read WASM file");

    let result = load_wasm_module(&wasm_bytes);
    assert!(result.is_ok(), "Failed to load WASM module: {:?}", result.err());
}


#[tokio::test]
    async fn test_execute_task() {
        let execution_flow = ExecutionFlow::new(4); // Create with 4 threads
        execution_flow.add_task(1, "Low priority task".to_string());
        execution_flow.add_task(5, "High priority task".to_string());
        execution_flow.manage_flow().await;
        assert!(true);
    }


#[tokio::test]
async fn test_http_get() {
    let api = BlocklessAPI::new("https://example.com");
    let response = api.get("endpoint").await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_memory_safety() {
    let safe_data = SafeData::new();
    safe_data.update_data(42).await;
    let value = safe_data.read_data().await;
    assert_eq!(value, 42);
}
