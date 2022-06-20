use terbium::{AstBody, AstNode, AstParseInterface, BcTransformer, DefaultInterpreter};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! handle_output {
    ($output:ident, $errors:ident) => {{
        if !$errors.is_empty() {
            return format!(
                "{}\nErrors: {:?}",
                $output,
                $errors
                    .into_iter()
                    .map(|e| format!("{:?}\n", e))
                    .collect::<String>()
            )
            .into();
        }

        $output.into()
    }};
    ($output:ident, $errors:ident, $errfmt:literal, $outputfmt:literal) => {{
        if !$errors.is_empty() {
            return format!(
                $errfmt,
                $output,
                $errors
                    .into_iter()
                    .map(|e| format!("{:?}\n", e))
                    .collect::<String>()
            )
            .into();
        }

        format!($outputfmt, $output).into()
    }};
}

macro_rules! ast {
    ($content:ident, $asttype:ty) => {{
        match <$asttype>::from_string($content) {
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
    let (node, errors) = ast!(content, AstNode);

    handle_output!(node, errors, "{:#?}\nErrors: {:?}", "{:#?}")
}

#[must_use]
#[wasm_bindgen]
pub fn dis(code: String) -> JsValue {
    let (body, errors) = ast!(code, AstBody);
    let program = program!(body);

    let mut output = Vec::new();
    drop(program.dis(&mut output));

    let final_output = String::from_utf8(output).unwrap_or_else(|_| unreachable!());

    handle_output!(final_output, errors)
}

#[must_use]
#[wasm_bindgen]
pub fn interpret(code: String) -> JsValue {
    let (body, errors) = ast!(code, AstBody);
    let program = program!(body);

    let mut interpreter = DefaultInterpreter::new();
    interpreter.run_bytecode(program);

    let popped = interpreter.ctx.pop_ref();
    let popped = interpreter.ctx.store.resolve(popped);
    let output = interpreter.get_object_repr(popped);

    handle_output!(output, errors)
}
