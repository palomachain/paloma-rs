use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use paloma_cosmwasm::{
    ExchangeRatesResponse, PalomaMsg, PalomaMsgWrapper, PalomaQuery, PalomaQueryWrapper,
    PalomaRoute, SwapResponse, TaxCapResponse, TaxRateResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(PalomaMsgWrapper), &out_dir);
    export_schema(&schema_for!(PalomaMsg), &out_dir);
    export_schema(&schema_for!(PalomaQueryWrapper), &out_dir);
    export_schema(&schema_for!(PalomaQuery), &out_dir);
    export_schema(&schema_for!(PalomaRoute), &out_dir);
    export_schema(&schema_for!(SwapResponse), &out_dir);
    export_schema(&schema_for!(TaxCapResponse), &out_dir);
    export_schema(&schema_for!(TaxRateResponse), &out_dir);
    export_schema(&schema_for!(ExchangeRatesResponse), &out_dir);
}
