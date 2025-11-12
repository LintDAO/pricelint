use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, CandidType)]
pub struct WasmFile {
    pub wasm_name: String,
    pub wasm_version: String,
    pub wasm_bin: Option<Vec<u8>>,
    pub upload_time: u64,
    pub update_type: UpdateType, //功能性更新或者模型更新
}
#[derive(Deserialize, Serialize, Clone, CandidType, PartialEq)]
pub enum UpdateType {
    FunctionUpdate,
    ModelUpdate,
}