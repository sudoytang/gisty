use std::future::Future;
use std::task::{
    Context, Poll, RawWaker, RawWakerVTable, Waker
};

use std::sync::Arc;
use std::thread::{self, Thread};

fn awakenable_waker() -> Waker {
    let handle = Arc::new(thread::current());
    fn clone(raw_arc: *const ()) -> RawWaker {
        // construct Arc back from raw
        let reborn_arc = unsafe { Arc::from_raw(raw_arc as *const Thread) };
        let cloned_arc = reborn_arc.clone();

        // NOTE(mem): this convert the reborn_arc back to unmanaged state
        // and discard the result, because we have the raw pointer already.
        // otherwise reborn_arc will be unexpectedly deleted
        std::mem::forget(reborn_arc);

        RawWaker::new(Arc::into_raw(cloned_arc) as *const (), &VTABLE)
    }
    fn wake(raw_arc: *const ()) {
        let reborn = unsafe { Arc::from_raw(raw_arc as *const Thread) };
        reborn.unpark();
        // this function should consume the waker
        // so we should not convert back to unmanaged state
        // after this line the refcount will be decreased
    }
    fn wake_by_ref(raw_arc: *const ()) {
        let reborn = unsafe { Arc::from_raw(raw_arc as *const Thread) };
        reborn.unpark();
        std::mem::forget(reborn);
    }

    fn drop(raw_arc: *const ()) {
        unsafe { Arc::from_raw(raw_arc as *const Thread)};
        // after this line the refcount will be decreased
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(
        clone, wake, wake_by_ref, drop
    );
    let raw = RawWaker::new(
        Arc::into_raw(handle) as *const (), &VTABLE
    );

    unsafe {
        Waker::from_raw(raw)
    }

}

pub fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = awakenable_waker();
    let mut ctx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);

    loop {
        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(res) => return res,
            Poll::Pending => {
                thread::park();
            }
        }
    }

    
}