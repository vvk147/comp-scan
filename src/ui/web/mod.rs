pub mod server;
pub mod templates;

use crate::storage::Database;
use anyhow::Result;

pub async fn run_web_server(db: &Database, port: u16) -> Result<()> {
    server::start(db.clone(), port).await
}
