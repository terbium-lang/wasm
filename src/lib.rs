#![no_std]

extern crate alloc;
extern crate console_error_panic_hook;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use terbium::{AstNode, AstParseInterface, AstBody, BcTransformer, DefaultInterpreter};
use wasm_bindgen::prelude::*;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn ast(content: String) -> JsValue {
    let (node, errors) = match AstNode::from_string(content) {
        Ok(t) => t,
        Err(e) => {
            return e.iter().map(|e| format!("[{:?}]{}\n", e.span, e.message)).collect::<String>().into();
        }
    };

    if !errors.is_empty() {
        return format!("{:#?}\nErrors: {:?}", node, errors).into();
    }

    format!("{:#?}", node).into()
}

#[wasm_bindgen]
pub fn dis(code: String) -> JsValue {
    let (body, errors) = match AstBody::from_string(code) {
        Ok(t) => t,
        Err(e) => {
            return e.iter().map(|e| format!("[{:?}]{}\n", e.span, e.message)).collect::<String>().into();
        }
    };

    let mut transformer = BcTransformer::new();
    transformer.interpret_body(None, body);

    let mut program = transformer.program();
    program.resolve();

    let mut output = Vec::new();
    drop(program.dis(&mut output));

    let final_output = String::from_utf8(output).unwrap();

    if !errors.is_empty() {
        return format!("{:?}\nErrors: {:?}", final_output, errors).into();
    }

    final_output.into()
}

#[wasm_bindgen]
pub fn interpret(code: String) -> JsValue {
    let (body, errors) = match AstBody::from_string(code) {
        Ok(t) => t,
        Err(e) => {
            return e.iter().map(|e| format!("[{:?}]{}\n", e.span, e.message)).collect::<String>().into();
        }
    };

    let mut transformer = BcTransformer::new();
    transformer.interpret_body(None, body);

    let mut program = transformer.program();
    program.resolve();

    let mut interpreter = DefaultInterpreter::new();
    interpreter.run_bytecode(program);
    
    let final_input = format!("{:?}", interpreter.stack().pop());

    if !errors.is_empty() {
        return format!("{:?}\nErrors: {:?}", final_input, errors).into();
    }

    final_input.into()
}
