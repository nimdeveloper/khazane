// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     // invoke without arguments
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
//     async fn invoke_no_args(cmd: &str) -> JsValue;

//     // invoke with arguments (default)
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;

//     // They need to have different names!
// }

fn main() {
    tauri_build::build()
}
