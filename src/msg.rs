use abstract_core::app;
use abstract_sdk::base::{ExecuteEndpoint, InstantiateEndpoint, MigrateEndpoint, QueryEndpoint};
use cosmwasm_schema::QueryResponses;

use crate::contract::CronCatApp;

/// Abstract App instantiate msg
pub type InstantiateMsg = <CronCatApp as InstantiateEndpoint>::InstantiateMsg;
pub type ExecuteMsg = <CronCatApp as ExecuteEndpoint>::ExecuteMsg;
pub type QueryMsg = <CronCatApp as QueryEndpoint>::QueryMsg;
pub type MigrateMsg = <CronCatApp as MigrateEndpoint>::MigrateMsg;

impl app::AppExecuteMsg for CronCatExecuteMsg {}
impl app::AppQueryMsg for CronCatQueryMsg {}

/// CronCat instantiate message
#[cosmwasm_schema::cw_serde]
pub struct CronCatInstantiateMsg {}

/// CronCat execute messages
#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[cfg_attr(feature = "interface", impl_into(ExecuteMsg))]
pub enum CronCatExecuteMsg {
    UpdateConfig {},
}

#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[cfg_attr(feature = "interface", impl_into(QueryMsg))]
#[derive(QueryResponses)]
pub enum CronCatQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub enum CronCatMigrateMsg {}

#[cosmwasm_schema::cw_serde]
pub enum Cw20HookMsg {
    Deposit {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}
