//! Terbium Playground
//! Returns: [result: String / null, ANSI Error: String / null]

#![feature(let_chains)]

use terbium::analyzer::{run_analysis, AnalyzerMessageKind, AnalyzerSet, Context};
use terbium::grammar::{ParseInterface, Source, Span};
use terbium::{sources, AstToken};
use terbium::{AstBody, AstNode, BcProgram, BcTransformer, DefaultInterpreter};
use wasm_bindgen::prelude::*;

const NULL: JsValue = JsValue::NULL;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn parse_ast<T>(content: &str) -> Result<T, String>
where
    T: ParseInterface,
{
    console_error_panic_hook::set_once();
    let source = Source::default();

    let res = T::from_string(source.clone(), content.to_string());
    if let Err(e) = res {
        let mut err = Vec::new();

        for error in e {
            let cache = sources::<Source, String, _>(vec![(source.clone(), content.to_string())]);

            error.write(cache, &mut err);
        }
        return Err(String::from_utf8_lossy(&err).into_owned());
    }
    // SAFETY: `res` is not Err because is already checked
    Ok(unsafe { res.unwrap() })
}

fn analyze<T>(code: &str) -> Result<(T, Option<String>), String>
where
    T: ParseInterface,
{
    let tokens = parse_ast::<Vec<(AstToken, Span)>>(code)?;

    let src = vec![(Source::default(), code.to_string())];
    let ctx = Context::from_tokens(src.clone(), tokens.clone());
    let analyzers = AnalyzerSet::default();

    let messages = run_analysis(&analyzers, ctx)?;

    let mut should_return = false;

    let count = messages.len();
    let mut info_count = 0;
    let mut warning_count = 0;
    let mut error_count = 0;

    let mut final_message = Vec::new();

    for message in messages {
        match message.kind {
            AnalyzerMessageKind::Info => info_count += 1,
            AnalyzerMessageKind::Alert(k) => {
                if k.is_error() {
                    error_count += 1;
                    should_return = true;
                } else {
                    warning_count += 1;
                }
            }
        }

        message.write(sources(src.clone()), &mut final_message);
    }

    let result = {
        if !final_message.is_empty() {
            let mut result = String::from_utf8_lossy(&final_message).into_owned();

            result.push_str(&format!(
                "{} message{} ({} info, {} warning{}, {} error{})\n",
                count,
                if count == 1 { "" } else { "s" },
                info_count,
                warning_count,
                if warning_count == 1 { "" } else { "s" },
                error_count,
                if error_count == 1 { "" } else { "s" }
            ));

            Some(result)
        } else {
            None
        }
    };

    if should_return {
        // SAFETY: `should_return` is only `true` if there is an error,
        // that mean there has to be at least one item.
        return Err(unsafe { result.unwrap() });
    }

    if tokens.is_empty() {
        return Err("Expected input".to_string());
    }

    // SAFETY: Analyzer already checked for us so is safe to unwrap.
    Ok((unsafe { T::parse(tokens).unwrap() }, result))
}

fn program(body: AstBody) -> BcProgram {
    let mut transformer = BcTransformer::default();
    transformer.interpret_body(None, body);

    let mut program = transformer.program();
    program.resolve();

    program
}

#[must_use]
#[wasm_bindgen]
pub fn ast(content: &str) -> Vec<JsValue> {
    let result = analyze::<AstNode>(content);
    if let Err(e) = result {
        return vec![NULL, e.into()];
    }

    // SAFETY: `node` is not Err because is already checked.
    let (node, warnings) = unsafe { result.unwrap() };

    vec![
        format!("{:#?}", node).into(),
        if let Some(warnings) = warnings {
            warnings.into()
        } else {
            NULL
        },
    ]
}

#[must_use]
#[wasm_bindgen]
pub fn dis(code: &str) -> Vec<JsValue> {
    let result = analyze::<AstBody>(code);
    if let Err(e) = result {
        return vec![NULL, e.into()];
    }

    // SAFETY: `node` is not Err because is already checked.
    let (body, warnings) = unsafe { result.unwrap() };

    let program = program(body);

    let mut output = Vec::new();
    drop(program.dis(&mut output));

    vec![
        String::from_utf8(output)
            .unwrap_or_else(|_| unreachable!())
            .into(),
        if let Some(warnings) = warnings {
            warnings.into()
        } else {
            NULL
        },
    ]
}

#[must_use]
#[wasm_bindgen]
pub fn interpret(code: &str) -> Vec<JsValue> {
    let result = analyze::<AstBody>(code);
    if let Err(e) = result {
        return vec![NULL, e.into()];
    }

    // SAFETY: `node` is not Err because is already checked.
    let (body, warnings) = unsafe { result.unwrap() };

    let program = program(body);

    let mut interpreter = DefaultInterpreter::default();
    interpreter.run_bytecode(&program);

    if interpreter.ctx.stack_is_empty() {
        if let Some(warnings) = warnings {
            return vec![NULL, warnings.into()];
        } else {
            return vec![false.into(), NULL];
        }
    }

    let popped = interpreter.ctx.pop_ref();
    let popped = interpreter.ctx.store.resolve(popped);

    vec![
        interpreter.get_object_repr(popped).into(),
        if let Some(warnings) = warnings {
            warnings.into()
        } else {
            NULL
        },
    ]
}
