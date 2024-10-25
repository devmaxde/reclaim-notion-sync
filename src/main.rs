use crate::config::SyncConfig;
use crate::sync::DatabaseSync;

mod config;
mod page;
mod sync;

#[tokio::main]
async fn main() {
    let sync_config = SyncConfig::from_config_file();

    let sync_bot = DatabaseSync::new(sync_config.unwrap());
    sync_bot.sync().await;
}
