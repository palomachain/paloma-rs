use cosmwasm_std::{Coin, QuerierWrapper, StdResult};

use crate::query::{
    ContractInfoResponse, ExchangeRatesResponse, PalomaQuery, PalomaQueryWrapper, SwapResponse,
    TaxCapResponse, TaxRateResponse,
};
use crate::route::PalomaRoute;

/// This is a wrapper to easily use our custom queries
pub struct PalomaQuerier<'a> {
    querier: &'a QuerierWrapper<'a, PalomaQueryWrapper>,
}

impl<'a> PalomaQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, PalomaQueryWrapper>) -> Self {
        PalomaQuerier { querier }
    }

    pub fn query_swap<T: Into<String>>(
        &self,
        offer_coin: Coin,
        ask_denom: T,
    ) -> StdResult<SwapResponse> {
        let request = PalomaQueryWrapper {
            route: PalomaRoute::Market,
            query_data: PalomaQuery::Swap {
                offer_coin,
                ask_denom: ask_denom.into(),
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_tax_cap<T: Into<String>>(&self, denom: T) -> StdResult<TaxCapResponse> {
        let request = PalomaQueryWrapper {
            route: PalomaRoute::Treasury,
            query_data: PalomaQuery::TaxCap {
                denom: denom.into(),
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_tax_rate(&self) -> StdResult<TaxRateResponse> {
        let request = PalomaQueryWrapper {
            route: PalomaRoute::Treasury,
            query_data: PalomaQuery::TaxRate {},
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_exchange_rates<T: Into<String>>(
        &self,
        base_denom: T,
        quote_denoms: Vec<T>,
    ) -> StdResult<ExchangeRatesResponse> {
        let request = PalomaQueryWrapper {
            route: PalomaRoute::Oracle,
            query_data: PalomaQuery::ExchangeRates {
                base_denom: base_denom.into(),
                quote_denoms: quote_denoms.into_iter().map(|x| x.into()).collect(),
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        let request = PalomaQueryWrapper {
            route: PalomaRoute::Wasm,
            query_data: PalomaQuery::ContractInfo {
                contract_address: contract_address.into(),
            },
        }
        .into();

        self.querier.query(&request)
    }
}
