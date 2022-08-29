use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calc(a: i32, b: i32, op: char) -> i32 {
    if op == '+' {
        return a + b;
    } else if op == '-' {
        return a - b;
    } else if op == '*' {
        return a * b;
    } else if op == '/' {
        return a / b;
    } else {
        return 0;
    }
}
