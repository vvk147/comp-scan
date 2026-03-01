pub mod server;
pub mod templates;

use anyhow::Result;
use crate::storage::Database;

pub async fn run_web_server(db: &Database, port: u16) -> Result<()> {
    server::start(db.clone(), port).await
}
