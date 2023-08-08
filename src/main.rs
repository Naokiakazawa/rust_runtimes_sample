use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod execute;

fn main() {
    let subscriber: FmtSubscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    if let Err(e) = execute::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
