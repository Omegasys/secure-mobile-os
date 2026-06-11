```rust id="perf01"
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub cpu_usage_percent: f32,
    pub memory_used_mb: u64,
    pub timestamp_ms: u128,
}

pub struct PerformanceMonitor {
    snapshots: Vec<PerformanceSnapshot>,
    start_time: Instant,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn record_snapshot(
        &mut self,
        cpu_usage_percent: f32,
        memory_used_mb: u64,
    ) {
        let snapshot = PerformanceSnapshot {
            cpu_usage_percent,
            memory_used_mb,
            timestamp_ms: self.start_time.elapsed().as_millis(),
        };

        self.snapshots.push(snapshot);
    }

    pub fn get_latest(&self) -> Option<&PerformanceSnapshot> {
        self.snapshots.last()
    }

    pub fn print_summary(&self) {
        println!("Performance Summary");
        println!("-------------------");

        for snap in &self.snapshots {
            println!(
                "[{} ms] CPU: {:.2}% | RAM: {} MB",
                snap.timestamp_ms,
                snap.cpu_usage_percent,
                snap.memory_used_mb
            );
        }
    }

    pub fn average_cpu(&self) -> f32 {
        if self.snapshots.is_empty() {
            return 0.0;
        }

        let sum: f32 = self.snapshots
            .iter()
            .map(|s| s.cpu_usage_percent)
            .sum();

        sum / self.snapshots.len() as f32
    }
}
```
