use crate::contract::{CronCatApp, CronCatResult};

use abstract_sdk::features::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply, Response};

/// An example of a reply handler mapped to a reply id.
pub fn instantiate_reply(
    _deps: DepsMut,
    _env: Env,
    app: CronCatApp,
    reply: Reply,
) -> CronCatResult {
    let _data = reply.result.unwrap().data.unwrap();

    Ok(app.tag_response(Response::default(), "instantiate_reply"))
}
