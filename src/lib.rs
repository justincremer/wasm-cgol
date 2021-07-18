// #![feature(wasm_custom_section, wasm_import_module)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn setInner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn _alert(s: &str) {
    alert(&format!("This is from Rust\n{}", s));
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn _createElement() {
    let div = document.createElement("div");
    let p = document.createElement("p");

    p.setInner("Hello from Rust");
    div.append(p);

    document.body().append(div);
}
