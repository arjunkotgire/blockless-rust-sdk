#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

static mut MEMORY: [i32; 4] = [0; 4];

#[no_mangle]
pub extern "C" fn write_memory(index: i32, value: i32) {
    unsafe {
        if index >= 0 && (index as usize) < MEMORY.len() {
            MEMORY[index as usize] = value;
        }
    }
}

#[no_mangle]
pub extern "C" fn read_memory(index: i32) -> i32 {
    unsafe {
        if index >= 0 && (index as usize) < MEMORY.len() {
            return MEMORY[index as usize];
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn long_running_task() -> i32 {
    let mut sum = 0;
    for i in 0..1000000 {
        sum += i;
    }
    sum
}
