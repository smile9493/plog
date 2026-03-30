//! Plog CMS API 入口

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    plog_api::run().await
}
