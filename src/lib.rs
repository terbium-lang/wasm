use terbium::{AstNode, AstParseInterface, AstBody, BcTransformer, DefaultInterpreter, TerbiumObject};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

    let output = match interpreter.stack().pop() {
        TerbiumObject::Integer(n) => n.to_string(),
        TerbiumObject::Float(f) => f.0.to_string(),
        TerbiumObject::String(s_id) => format!("{:?}", interpreter.string_lookup(s_id)),
        TerbiumObject::Bool(b) => b.to_string(),
        TerbiumObject::Null => "null".to_string(),
    };

    if !errors.is_empty() {
        return format!("{:?}\nErrors: {:?}", output, errors).into();
    }

    output.into()
}
