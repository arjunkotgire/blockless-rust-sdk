use wasmtime::*;
use crate::execution::ExecutionFlow;

pub struct WasmModule {
    #[allow(dead_code)]
    engine: Engine,
    store: Store<()>,
    instance: Instance,
    execution_flow: ExecutionFlow,
}

impl WasmModule {
    // The constructor for WasmModule, initializing with the provided WASM bytes.
    pub fn new(wasm_bytes: &[u8]) -> Result<Self, String> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| e.to_string())?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).map_err(|e| e.to_string())?;
        
        // Hardcoding `num_threads` or setting a default value for execution flow.
        let num_threads = 4;
        let execution_flow = ExecutionFlow::new(num_threads);
        
        // Returning a new instance of WasmModule.
        Ok(WasmModule { engine, store, instance, execution_flow })
    }

    // Method to invoke a function within the WASM module.
    pub fn invoke_function(&mut self, func_name: &str, args: (i32, i32)) -> Result<i32, String> {
        let func = self.instance.get_func(&mut self.store, func_name)
            .ok_or_else(|| format!("Function '{}' not found", func_name))?;
        
        let typed_func = func.typed::<(i32, i32), i32, _>(&self.store)
            .map_err(|e| e.to_string())?;

        let result = typed_func.call(&mut self.store, args)
            .map_err(|e| e.to_string())?;
        
        Ok(result)
    }

    // Method to read a specified portion of memory from the WASM module.
    pub fn read_memory(&mut self, offset: usize, len: usize) -> Result<Vec<u8>, String> {
        let memory = self.instance.get_memory(&mut self.store, "memory")
            .ok_or("Memory not found")?;
        let data = memory.data(&self.store);
        
        if offset + len > data.len() {
            return Err("Memory access out of bounds".to_string());
        }
        
        Ok(data[offset..offset + len].to_vec())
    }

    // Method to manage the execution flow within the module.
    pub async fn manage_execution_flow(&self) {
        self.execution_flow.manage_flow().await;
    }
}

// Function to load a WASM module, returning a WasmModule instance.
pub fn load_wasm_module(wasm_bytes: &[u8]) -> Result<WasmModule, String> {
    WasmModule::new(wasm_bytes) // Only passing `wasm_bytes` to the constructor.
}
