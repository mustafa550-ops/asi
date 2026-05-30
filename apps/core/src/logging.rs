use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn init_logging() {
    let log_dir = "./logs";
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("adler-core")
        .build(log_dir)
        .expect("Failed to initialize rolling file appender");

    // Non-blocking file writer
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // Note: We are deliberately leaking the _guard in this simple setup so the logs flush at exit.
    // In a real production setup, the guard should be kept in the main function.
    std::mem::forget(_guard); 

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .json()
        .with_filter(EnvFilter::from_default_env().add_directive("adler_core=debug".parse().unwrap()));

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env().add_directive("adler_core=info".parse().unwrap()));

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();
}
