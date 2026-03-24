//use tracing::{instrument, warn};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn setup_tracing() {
    let logfile = tracing_appender::rolling::hourly("./log", "auth");
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(stdout.and(logfile))
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    //tracing::info!("INFO TEXT");
    //tracing::warn!("WARNING TEXT");
    //tracing::error!("ERROR TEXT");
    //tracing::debug!("DEBUG TEXT");
}

#[cfg(test)]
mod tests {
    //use super::*;
}
