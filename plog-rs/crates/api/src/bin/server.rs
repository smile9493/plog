//! Plog API 服务入口

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    plog_api::run().await
}
