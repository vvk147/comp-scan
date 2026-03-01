use std::time::Duration;
use tokio::time;

pub struct TaskScheduler {
    tasks: Vec<ScheduledTask>,
}

struct ScheduledTask {
    name: String,
    interval: Duration,
    task: Box<dyn Fn() + Send + Sync>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task<F>(&mut self, name: &str, interval: Duration, task: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.tasks.push(ScheduledTask {
            name: name.to_string(),
            interval,
            task: Box::new(task),
        });
    }

    pub async fn run(self) {
        let mut handles = Vec::new();

        for task in self.tasks {
            let handle = tokio::spawn(async move {
                let mut interval = time::interval(task.interval);
                loop {
                    interval.tick().await;
                    tracing::debug!("Running scheduled task: {}", task.name);
                    (task.task)();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}
