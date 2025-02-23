use std::time::Instant;

pub struct ScopeProfiler {
    name: String,
    start_time: Instant,
}

impl ScopeProfiler {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            start_time: Instant::now(),
        }
    }

    fn stop(&self) {
        let duration = self.start_time.elapsed();
        log::debug!(
            "{} in {} ms ({} ns)",
            self.name,
            duration.as_millis(),
            duration.as_nanos()
        );
    }
}

impl Drop for ScopeProfiler {
    fn drop(&mut self) {
        self.stop();
    }
}
