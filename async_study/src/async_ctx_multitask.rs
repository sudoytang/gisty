use std::cell::{Cell, RefCell};
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::AtomicBool;
use std::collections::{
    HashMap, VecDeque
};
use std::future::Future;
use std::task::{
    Context, Poll, RawWaker, RawWakerVTable, Waker
};

use std::sync::{Arc, Condvar, Mutex};

struct AsyncTask {
    future: Pin<Box<dyn Future<Output = ()>>>,
}


pub struct Executor {
    tasks: RefCell<HashMap<usize, AsyncTask>>,
    next_task_id: Cell<usize>,
    queue: Arc<Mutex<VecDeque<usize>>>,
    cv: Arc<Condvar>,
    main_woken: Arc<AtomicBool>,
}

struct MainWakerInner {
    woken: Arc<AtomicBool>,
    cv: Arc<Condvar>,
}

impl MainWakerInner {
    fn wake(&self) {
        self.woken.store(true, std::sync::atomic::Ordering::Relaxed);
        self.cv.notify_one();
    }
}


struct AsyncTaskWakerInner {
    task_id: usize,
    queue: Arc<Mutex<VecDeque<usize>>>,
    cv: Arc<Condvar>,
}

impl AsyncTaskWakerInner {
    fn wake(&self) {
        self.queue.lock().unwrap().push_back(self.task_id);
        self.cv.notify_one();
    }
}



thread_local! {
    static CURRENT_EXECUTOR: RefCell<Option<NonNull<Executor>>> = RefCell::new(None);
}

impl Executor {
    fn get_main_waker(&self) -> Waker {
        let woken = self.main_woken.clone();
        let cv = self.cv.clone();
        let arc = Arc::new(MainWakerInner { woken, cv });
        fn clone(raw_arc: *const ()) -> RawWaker {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const MainWakerInner )};
            let cloned = reborn.clone();

            std::mem::forget(reborn);
            RawWaker::new(Arc::into_raw(cloned) as *const(), &VTABLE)
        }

        fn wake(raw_arc: *const ()) {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const MainWakerInner) };
            reborn.wake();
        }

        fn wake_by_ref(raw_arc: *const ()) {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const MainWakerInner) };
            reborn.wake();
            std::mem::forget(reborn);
        }

        fn drop(raw_arc: *const ()) {
            unsafe {
                Arc::from_raw(raw_arc as *const MainWakerInner)
            };
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            clone, wake, wake_by_ref, drop
        );

        let raw = RawWaker::new(
            Arc::into_raw(arc) as *const (), &VTABLE
        );

        unsafe {
            Waker::from_raw(raw)
        }
    }
    fn get_async_task_waker(&self, task_id: usize) -> Waker {
        let queue = self.queue.clone();
        let cv = self.cv.clone();
        let arc = Arc::new(AsyncTaskWakerInner { task_id, queue, cv });
        fn clone(raw_arc: *const ()) -> RawWaker {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const AsyncTaskWakerInner )};
            let cloned = reborn.clone();

            std::mem::forget(reborn);
            RawWaker::new(Arc::into_raw(cloned) as *const(), &VTABLE)
        }

        fn wake(raw_arc: *const ()) {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const AsyncTaskWakerInner) };
            reborn.wake();
        }

        fn wake_by_ref(raw_arc: *const ()) {
            let reborn = unsafe { Arc::from_raw(raw_arc as *const AsyncTaskWakerInner) };
            reborn.wake();
            std::mem::forget(reborn);
        }

        fn drop(raw_arc: *const ()) {
            unsafe {
                Arc::from_raw(raw_arc as *const AsyncTaskWakerInner)
            };
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            clone, wake, wake_by_ref, drop
        );

        let raw = RawWaker::new(
            Arc::into_raw(arc) as *const (), &VTABLE
        );

        unsafe {
            Waker::from_raw(raw)
        }
    }

    pub fn block_on<F: Future>(main_fut: F) -> F::Output {

        let executor = Executor {
            tasks: RefCell::new(HashMap::new()),
            next_task_id: Cell::new(0),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            cv: Arc::new(Condvar::new()),
            main_woken: Arc::new(AtomicBool::new(true)),
        };

        CURRENT_EXECUTOR.with(|c| {
            *c.borrow_mut() = Some(NonNull::from_ref(&executor));
        });

        let mut main_fut = std::pin::pin!(main_fut);
        let main_waker = executor.get_main_waker();
        let mut main_cx = Context::from_waker(&main_waker);

        let output = loop {
            // Poll main future
            if executor.main_woken.load(std::sync::atomic::Ordering::Relaxed) {
                match main_fut.as_mut().poll(&mut main_cx) {
                    Poll::Ready(output) => break output,
                    Poll::Pending => executor.main_woken.store(false, std::sync::atomic::Ordering::Relaxed),
                }
            }

            // lock queue
            let mut queue = executor.queue.lock().unwrap();

            // handle all ready tasks
            while let Some(task_id) = queue.pop_front() {
                // unlock to allow spawn within poll to append task
                drop(queue);

                // Poll tasks
                let option_task = executor.tasks.borrow_mut().remove(&task_id);
                // ^^^ I have to split this line before if let because the lifetime
                //     of RefMut is very confusing.
                if let Some(mut task) = option_task {
                    let waker = executor.get_async_task_waker(task_id);
                    let mut cx = Context::from_waker(&waker);
                    match task.future.as_mut().poll(&mut cx) {
                        Poll::Ready(()) => {
                            // Ok the task is over
                        }
                        Poll::Pending => {
                            // Put back to map
                            executor.tasks.borrow_mut().insert(task_id, task);
                        }
                    }
                }

                // Relock for next task
                queue = executor.queue.lock().unwrap();
            }

            // when main task is not woken and queue is empty, fall asleep
            if !executor.main_woken.load(std::sync::atomic::Ordering::Relaxed) {
                // we don't need to re-check if queue is empty,
                // because before cv.wait(...), the queue is locked by us
                // and the former while loop guarantees that the queue is empty.
                // and no one can put anything into the queue
                let _guard = executor.cv.wait(queue).unwrap();
            }
        };

        CURRENT_EXECUTOR.with(|c| {
            *c.borrow_mut() = None;
        });
        output
    }

    pub fn spawn<F>(fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        CURRENT_EXECUTOR.with(|exec| {
            let exec = exec.borrow().expect("Executor::spawn must be called within executor context!");
            let exec = unsafe { exec.as_ref() };
            let task_id = exec.next_task_id.get();
            exec.next_task_id.set(task_id + 1);
            let task = AsyncTask {
                future: Box::pin(fut)
            };

            exec.tasks.borrow_mut().insert(task_id, task);
            exec.queue.lock().unwrap().push_back(task_id);
            exec.cv.notify_one();
        })
    }
}

