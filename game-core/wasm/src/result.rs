use serde::Serialize;
use tsify::Tsify;

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Result<T, E> {
    ok: bool,
    #[tsify(type = "T | E")]
    value: std::result::Result<T, E>,
}

impl<T: Serialize, E: Serialize> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(value: std::result::Result<T, E>) -> Self {
        let ok = value.is_ok();

        Self { ok, value }
    }
}
