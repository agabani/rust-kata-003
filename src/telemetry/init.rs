use tracing::{subscriber, Subscriber};

pub fn init(subscriber: impl Subscriber + Send + Sync) {
    subscriber::set_global_default(subscriber).expect("setting tracing default failed.");
}
