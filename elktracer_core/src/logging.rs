use env_logger::{Env, WriteStyle};
use log::trace;

pub fn initialize() {
    let env = Env::default().filter_or("ELK_LOG", "trace");
    env_logger::builder()
        .parse_env(env)
        .write_style(WriteStyle::Always)
        .init();

    trace!("Logging initialized");
}
