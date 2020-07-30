use wasm_bindgen::prelude::*;
// use serde::{Serialize, Deserialize};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn squiggle(seq: &str) -> JsValue {
    // define vectors and starting coord
    let mut x: Vec<f32> = vec![0.0];
    let mut y: Vec<f32> = vec![0.0];
    let mut y_coord: f32 = 0.0;

    // loop through entire seq
    for character in seq.chars() {
        x.push(x[x.len() - 1] + 0.5);
        x.push(x[x.len() - 1] + 0.5);
        // utilizes pattern match syntax for supposed higher performance
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
            // squiggle by default takes all values
            _ => {
                y.push(y_coord);
                y.push(y_coord);
            }
        }
    }

    let result: Vec<Vec<f32>> = vec![x, y];
    JsValue::from_serde(&result).unwrap()
    // convert x and y vectors into arrays
    // let x_final: Array = x.into_iter().map(JsValue::from).collect();
    // let y_final: Array = y.into_iter().map(JsValue::from).collect();
    // declare nested vector
    // let mut ptr: Vec<Array> = vec![];
    // ptr.push(x_final);
    // ptr.push(y_final);
    // convert vec<array> into nested array
    // let result: Array = ptr.into_iter().map(JsValue::from).collect();
    // result 
}