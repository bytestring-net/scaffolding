use tokio_util::sync::CancellationToken;

/// Waits for either Ctrl+C (SIGINT) or Docker stop (SIGTERM).
pub async fn shutdown_signal() -> CancellationToken {
    
    // Create SIGINT async block
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    // Create SIGTERM async block
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler").recv().await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();


    // Create a cancellation token for graceful shutdown
    let cancel_token = CancellationToken::new();
    let token_clone = cancel_token.clone();

    // Spawn the signal listener as a background Tokio task
    tokio::spawn(async move {
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        token_clone.cancel();
    });

    // Return cancel token
    cancel_token
}
