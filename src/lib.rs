//pub mod app;
//pub mod calendar;
//pub mod event;

#[cfg(feature = "ui")]
pub mod ui;

use tracing::{Level, subscriber};
use tracing_subscriber::FmtSubscriber;

// Enable Logging
pub fn enable_logging() {
    let subscriber = FmtSubscriber::builder()
                    .with_max_level(Level::TRACE)
                    .finish();

    subscriber::set_global_default(subscriber)
        .expect("Setting global default subscriber failed");
}
