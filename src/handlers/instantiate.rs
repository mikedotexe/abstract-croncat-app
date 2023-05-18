use cosmwasm_std::{wasm_execute, DepsMut, Env, MessageInfo, Response, SubMsg};

use crate::contract::{CronCatApp, CronCatResult};
use crate::msg::CronCatInstantiateMsg;
use crate::replies::INSTANTIATE_REPLY_ID;
use crate::state::{Config, CONFIG};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: CronCatApp,
    _msg: CronCatInstantiateMsg,
) -> CronCatResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;

    // Example reply that doesn't do anything
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        wasm_execute(_env.contract.address, &cosmwasm_std::Empty {}, vec![])?,
        INSTANTIATE_REPLY_ID,
    )))
}
