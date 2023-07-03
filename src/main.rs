use onnxruntime::{environment::Environment, ndarray::Array, GraphOptimizationLevel, LoggingLevel};
use std::env::var;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

type Error = Box<dyn std::error::Error>;

fn run() -> Result<(), Error> {}

fn main() {}
