use std::future::Future;
use std::task::{
    Context, Poll, RawWaker, RawWakerVTable, Waker
};

fn noop_waker() -> Waker {
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(
        clone, wake, wake_by_ref, drop
    );
    let raw = clone(std::ptr::null());
    unsafe {
        Waker::from_raw(raw)
    }
}

pub fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = noop_waker();
    let mut ctx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    // let mut fut = unsafe { Pin::new_unchecked(&mut fut)};
    // NOTE:    the commented code does exactly the same thing as
    //          std::pin::pin!, but the pin! macro moves fut and
    //          hide it, and the lifetime is kept (these become possible
    //          by 'super let')
    // NOTE:    'super let' is an unstable feature for internal use now

    loop {
        match fut.as_mut().poll(&mut ctx) {
            Poll::Pending => std::thread::yield_now(),
            Poll::Ready(res) => return res,
        }
    }
}

