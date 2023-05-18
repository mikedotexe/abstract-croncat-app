use crate::msg::CronCatMigrateMsg;
use crate::{
    dependencies::CRON_CAT_DEPS,
    error::CronCatError,
    handlers,
    msg::{CronCatExecuteMsg, CronCatInstantiateMsg, CronCatQueryMsg},
    replies::{self, INSTANTIATE_REPLY_ID},
    CRON_CAT_ID,
};
use abstract_app::AppContract;
use cosmwasm_std::Response;
use cw20::Cw20ReceiveMsg;

/// The version of your module to be uploaded
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The type of the result returned by your app's entrypoints.
pub type CronCatResult<T = Response> = Result<T, CronCatError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type CronCatApp = AppContract<
    CronCatError,
    CronCatInstantiateMsg,
    CronCatExecuteMsg,
    CronCatQueryMsg,
    CronCatMigrateMsg,
    Cw20ReceiveMsg,
>;

const CRON_CAT_APP: CronCatApp = CronCatApp::new(CRON_CAT_ID, MODULE_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_receive(handlers::receive_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_dependencies(CRON_CAT_DEPS);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(CRON_CAT_APP, CronCatApp);
