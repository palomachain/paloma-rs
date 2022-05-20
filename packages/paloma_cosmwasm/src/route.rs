use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents paloma query route path.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PalomaRoute {
    Market,
    Treasury,
    Oracle,
    Wasm,
}
