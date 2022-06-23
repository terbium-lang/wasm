use terbium::{AstBody, AstNode, AstParseInterface, BcTransformer, DefaultInterpreter};
use terbium::grammar::Source;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! ast {
    ($content:ident, $asttype:ty) => {{
        match <$asttype>::from_string(Source::default(), $content) {
            Ok(t) => t,
            Err(e) => {
                return e
                    .iter()
                    .map(|e| format!("[{:?}]{}\n", e.span, e.message))
                    .collect::<String>()
                    .into();
            }
        }
    }};
}

macro_rules! program {
    ($body:ident) => {{
        let mut transformer = BcTransformer::new();
        transformer.interpret_body(None, $body);

        let mut program = transformer.program();
        program.resolve();

        program
    }};
}

#[must_use]
#[wasm_bindgen]
pub fn ast(content: String) -> JsValue {
    let node = ast!(content, AstNode);

    format!("{:#?}", node).into()
}

#[must_use]
#[wasm_bindgen]
pub fn dis(code: String) -> JsValue {
    let body = ast!(code, AstBody);
    let program = program!(body);

    let mut output = Vec::new();
    drop(program.dis(&mut output));

    String::from_utf8(output).unwrap_or_else(|_| unreachable!()).into()
}

#[must_use]
#[wasm_bindgen]
pub fn interpret(code: String) -> JsValue {
    let body = ast!(code, AstBody);
    let program = program!(body);

    let mut interpreter = DefaultInterpreter::new();
    interpreter.run_bytecode(&program);

    let popped = interpreter.ctx.pop_ref();
    let popped = interpreter.ctx.store.resolve(popped);
    
    interpreter.get_object_repr(popped).into()
}
