use wasm_bindgen::prelude::*;
use js_sys::Array;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
pub fn squiggle(seq: &str) -> Array {
    let mut ptr = vec![0.0; 4 * (seq.len()) + 2];
    let mut x_coord: f32 = 0.0;
    let mut y_coord: f32 = 0.0;
    
    for (i, character) in seq.chars().enumerate() {
        ptr[i * 2 + 1] = x_coord + 0.5;
        ptr[i * 2 + 2] = x_coord + 1.0;
        x_coord += 1.0;
        match character {
            'A' => {
                ptr[2 * (seq.len() + 1 + i)] = y_coord + 0.5;
                ptr[2 * (seq.len() + 1 + i) + 1] = y_coord;
            }
            'C' => {
                ptr[2 * (seq.len() + 1 + i)] = y_coord - 0.5;
                ptr[2 * (seq.len() + 1 + i) + 1] = y_coord;
            }
            'T' | 'U' => {
                ptr[2 * (seq.len() + 1 + i)] = y_coord - 0.5;
                ptr[2 * (seq.len() + 1 + i) + 1] = y_coord - 1.0;
                y_coord -= 1.0;
            }
            'G' => {
                ptr[2 * (seq.len() + 1 + i)] = y_coord + 0.5;
                ptr[2 * (seq.len() + 1 + i) + 1] = y_coord + 1.0;
                y_coord += 1.0;
            }
            _ => {
                ptr[2 * (seq.len() + 1 + i)] = y_coord;
                ptr[2 * (seq.len() + 1 + i) + 1] = y_coord;
            }
        }
    }
    let result: Array = ptr.into_iter().map(JsValue::from).collect();
    result
}
