use wasm_bindgen::prelude::*;
use terbium::{AstNode, AstParseInterface};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ast(content: String) -> JsValue {
    let (node, errors) = match AstNode::from_string(content) {
        Ok(t) => t,
        Err(e) => {
            return e.iter().map(|e| format!("[{:?}]{}", e.span, e.message)).collect::<String>().into();
        }
    };

    if !errors.is_empty() {
        return format!("{:?}\nErrors: {:?}", node, errors).into();
    }

    format!("{:?}", node).into()
}
