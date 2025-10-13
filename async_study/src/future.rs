use std::{sync::{atomic::AtomicBool, Arc}, task::Poll, thread, time::Duration};

pub struct NPollsToBeReady {
    awake: Arc<AtomicBool>,
    left: usize,
}

impl NPollsToBeReady {
    pub fn new(n: usize) -> Self {
        Self {
            awake: Arc::new(AtomicBool::new(true)),
            left: n,
        }
    }
}

impl Future for NPollsToBeReady {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if !self.awake.load(std::sync::atomic::Ordering::Relaxed) {
            // not awake!
            return Poll::Pending;
        }
        println!("NPollsToBeReady is polled, remaining {} times.", self.left);
        match self.left {
            0 => Poll::Ready(()),
            _ => {
                self.left -= 1;
                self.awake.store(false, std::sync::atomic::Ordering::Relaxed);
                let wk = cx.waker().clone();
                let atom = self.awake.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(1));
                    atom.store(true, std::sync::atomic::Ordering::Relaxed);
                    wk.wake();
                });
                Poll::Pending
            }
        }
    }
}