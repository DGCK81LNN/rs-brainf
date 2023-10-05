#[macro_use]
mod int;
pub mod compile;
mod debug;
mod kvvec;
mod runner;
mod simple_loop;
pub mod tokenize;
mod utils;

use crate::compile::*;
use crate::int::BfValue;
use crate::tokenize::tokenize;
use paste::paste;
use serde::Serialize;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(who: &str) {
    utils::set_panic_hook();
    alert(format!("Hello, {who}!").as_str());
}

#[wasm_bindgen]
pub fn greet_world() {
    greet("world");
}

#[wasm_bindgen]
pub fn greet_world_times(times: i32) {
    for _ in 0..times {
        greet_world();
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize)]
pub enum Status {
    Running,
    Exited,
    AwaitingInput,
    Hanged,
    Debugger,
}

#[derive(Serialize)]
pub struct Report<T: BfValue> {
    pub status: Status,
    pub output: Box<[T]>,
}

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_DECL: &'static str = r#"
type BfValue =
  | Int8Array
  | Uint8Array
  | Int16Array
  | Uint16Array
  | Int32Array
  | Uint32Array
  | BigInt64Array
  | BigUint64Array
interface Report<T extends BfValue> {
  status: Status;
  output: T;
}
"#;

macro_rules! define_fns_for_ttype {
    ($ty: ident, $tsty: literal) => {
        paste! {
            #[wasm_bindgen]
            pub fn [<compile_ $ty>](source: &str) -> Result<*const Box<[BfInstruction<$ty>]>, JsError> {
                let tokens = tokenize(source);
                match compile_from_tokens::<$ty>(&tokens) {
                    Ok(program) => Ok(Rc::into_raw(Rc::new(program))),
                    Err(_) => Err(JsError::new("Brainf compile error")),
                }
            }

            #[wasm_bindgen]
            pub fn [<input_ $ty>](_data: Vec<$ty>) {
                todo!();
            }

            #[wasm_bindgen]
            extern {
                #[wasm_bindgen(typescript_type = $tsty)]
                pub type [<Report $ty:camel>];
            }

            #[wasm_bindgen]
            pub fn [<run_ $ty>](
                _program: *const Box<[BfInstruction<$ty>]>,
                _time: f64
            ) -> Result<[<Report $ty:camel>], JsValue> {
                Ok(serde_wasm_bindgen::to_value(&Report::<$ty> {
                    status: Status::Running,
                    output: Box::new([]),
                })?.into())
            }

            #[wasm_bindgen]
            pub fn [<destroy_ $ty>](program: *const Box<[BfInstruction<$ty>]>) {
                drop(unsafe { Rc::from_raw(program) });
            }
        }
    };
}

define_fns_for_ttype!(i8, "Report<Int8Array>");
define_fns_for_ttype!(u8, "Report<Uint8Array>");
define_fns_for_ttype!(i16, "Report<Int16Array>");
define_fns_for_ttype!(u16, "Report<Uint16Array>");
define_fns_for_ttype!(i32, "Report<Int32Array>");
define_fns_for_ttype!(u32, "Report<Uint32Array>");
define_fns_for_ttype!(i64, "Report<BigInt64Array>");
define_fns_for_ttype!(u64, "Report<BigUint64Array>");
