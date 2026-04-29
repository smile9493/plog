//! Plog CMS API Server
//!
//! Production-ready entry point with panic hook and graceful shutdown

use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = std::backtrace::Backtrace::capture();
        eprintln!("\n{}", "=".repeat(60));
        eprintln!("PANIC: {}", panic_info);
        eprintln!("Backtrace:\n{:?}", backtrace);
        eprintln!("{}\n", "=".repeat(60));
        std::process::exit(101);
    }));
}

fn setup_signal_handlers(cancel_token: tokio_util::sync::CancellationToken) {
    let cancel_token_clone = cancel_token.clone();
    
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        
        tokio::spawn(async move {
            let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
            let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
            
            tokio::select! {
                _ = sigterm.recv() => {
                    tracing::info!("Received SIGTERM, initiating graceful shutdown");
                }
                _ = sigint.recv() => {
                    tracing::info!("Received SIGINT, initiating graceful shutdown");
                }
            }
            
            SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
            cancel_token_clone.cancel();
        });
    }
    
    #[cfg(not(unix))]
    {
        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to register Ctrl+C handler");
            tracing::info!("Received Ctrl+C, initiating graceful shutdown");
            SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
            cancel_token_clone.cancel();
        });
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_panic_hook();
    
    let cancel_token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(cancel_token.clone());
    
    plog_api::run(cancel_token).await
}
