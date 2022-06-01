mod msg;
mod querier;
mod query;
mod route;

pub use msg::{create_swap_msg, create_swap_send_msg, PalomaMsg, PalomaMsgWrapper};
pub use querier::PalomaQuerier;
pub use query::{
    ContractInfoResponse, ExchangeRateItem, ExchangeRatesResponse, PalomaQuery, PalomaQueryWrapper,
    SwapResponse, TaxCapResponse, TaxRateResponse,
};
pub use route::PalomaRoute;

// This export is added to all contracts that import this package, signifying that they require
// "paloma" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_paloma() {}
