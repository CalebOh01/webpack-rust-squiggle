use wasm_bindgen::prelude::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
pub fn squiggle(seq: &str) -> JsValue {
    let mut x: Vec<f32> = vec![0.0];
    let mut y: Vec<f32> = vec![0.0];
    let mut y_coord: f32 = 0.0;
    for character in seq.chars() {
        x.push(x[x.len() - 1] + 0.5);
        x.push(x[x.len() - 1] + 0.5);
        match character {
            'A' => {
                y.push(y_coord + 0.5);
                y.push(y_coord);
            }
            'C' => {
                y.push(y_coord - 0.5);
                y.push(y_coord);
            }
            'T' | 'U' => {
                y.push(y_coord - 0.5);
                y.push(y_coord - 1.0);
                y_coord -= 1.0;
            }
            'G' => {
                y.push(y_coord + 0.5);
                y.push(y_coord + 1.0);
                y_coord += 1.0;
            }
            _ => {
                y.push(y_coord);
                y.push(y_coord);
            }
        }
    }
    let result: Vec<Vec<f32>> = vec![x, y];
    JsValue::from_serde(&result).unwrap()
}
