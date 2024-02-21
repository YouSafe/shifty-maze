use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type", content = "value")]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(value: std::result::Result<T, E>) -> Self {
        match value {
            Ok(ok) => Result::Ok(ok),
            Err(err) => Result::Err(err),
        }
    }
}
