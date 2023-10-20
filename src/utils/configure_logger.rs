pub fn configure_logger() {
    let file_appender = tracing_appender::rolling::daily("./logs", "log_of_day");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ENTER)
        .init();
}
