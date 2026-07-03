use std::sync::mpsc::{self, Receiver, Sender};

type Task = Box<dyn FnOnce() + Send>;

pub struct Scheduler {
    receiver: Receiver<Task>
}

impl Scheduler {
    pub fn update(&mut self) {
        while let Ok(task) = self.receiver.try_recv() {
            task();
        }
    }
}

#[derive(Clone)]
pub struct SchedulerContext {
    sender: Sender<Task>
}

impl SchedulerContext {
    /// Add task to execute on main thread
    pub fn add(&self, task: Task) {
        let _ = self.sender.send(task);
    }
}

pub fn create_scheduler() -> (Scheduler, SchedulerContext) {
    let (sender, receiver) = mpsc::channel();
    (Scheduler { receiver }, SchedulerContext { sender })
}

#[cfg(test)]
mod tests {
    use std::{sync::{Arc, atomic::AtomicI8}, thread, time::Duration};

    use super::*;
    
    #[test]
    fn test_schedule() {
        let mut tries: u32 = 0;

        let (mut scheduler, context) = create_scheduler();

        let context_clone = context.clone();

        let number: Arc<AtomicI8> = Arc::new(AtomicI8::new(0));

        let number_clone = number.clone();

        let _ = thread::spawn(move || {
            context_clone.add(Box::new(move || { let _ = number_clone.fetch_add(5, std::sync::atomic::Ordering::SeqCst); }));
        });

        while tries <= 5 {
            scheduler.update();

            if number.load(std::sync::atomic::Ordering::SeqCst) == 5 {
                break;
            }

            tries += 1;

            thread::sleep(Duration::from_millis(300));
        }

        assert_eq!(number.load(std::sync::atomic::Ordering::SeqCst), 5);

    }
}
