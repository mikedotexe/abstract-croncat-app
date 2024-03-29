pub mod contract;
pub mod dependencies;
pub mod error;
mod handlers;
#[cfg(feature = "interface")]
pub mod interface;
pub mod msg;
mod replies;
pub mod state;

pub const CRON_CAT_ID: &str = "yournamespace:template";
