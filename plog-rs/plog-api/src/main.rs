//! Plog CMS API Server

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(plog_api::run())
        .unwrap();
}
