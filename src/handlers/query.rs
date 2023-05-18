use crate::contract::{CronCatApp, CronCatResult};
use crate::msg::{ConfigResponse, CronCatQueryMsg};
use crate::state::CONFIG;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _etf: &CronCatApp,
    msg: CronCatQueryMsg,
) -> CronCatResult<Binary> {
    match msg {
        CronCatQueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}
