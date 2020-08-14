use wasm_bindgen::prelude::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// creates static buffer
// what size do i make it?
const WASM_MEMORY_BUFFER_SIZE: usize = 64000;
static mut WASM_MEMORY_BUFFER: [f32; WASM_MEMORY_BUFFER_SIZE] = [0.0; WASM_MEMORY_BUFFER_SIZE];

// adds all x and y vals to buffer stack
#[wasm_bindgen]
pub fn transform(seq: &str) {
    let mut x_coord: f32 = 0.0;
    let mut y_coord: f32 = 0.0;

    unsafe {
        for (i, base) in seq.chars().enumerate() {

            WASM_MEMORY_BUFFER[2 * i + 1] = x_coord + 0.5;
            WASM_MEMORY_BUFFER[2 * i + 2] = x_coord + 1.0;
            x_coord += 1.0;
    
            match base {
                'A' => {
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i)] = y_coord + 0.5;
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i) + 1] = y_coord;
                }
                'C' => {
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i)] = y_coord - 0.5;
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i) + 1] = y_coord;
                }
                'T' | 'U' => {
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i)] = y_coord - 0.5;
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i) + 1] = y_coord - 1.0;
                    y_coord -= 1.0;
                }
                'G' => {
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i)] = y_coord + 0.5;
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i) + 1] = y_coord + 1.0;
                    y_coord += 1.0;
                }
                _ => {
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i)] = y_coord;
                    WASM_MEMORY_BUFFER[2 * (seq.len() + 1 + i) + 1] = y_coord;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn buffer_pointer() -> *const f32 {
  let pointer: *const f32;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}

