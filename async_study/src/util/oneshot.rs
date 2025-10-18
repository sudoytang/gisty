use std::{pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, Waker}};
use std::future::Future;


struct OneshotData<T> {
    value: Option<T>,       // value:   checked by rx, set by tx
    waker: Option<Waker>,   // waker:   set by rx, use by tx. 
                            //          (When the tx has set value, 
                            //          it has to wake rx up to let rx know
                            //          and let rx continue to do something.
}

impl<T> Default for OneshotData<T> {
    fn default() -> Self {
        Self { value: None, waker: None }
    }
}

pub struct OneshotTx<T> {
    inner: Arc<Mutex<OneshotData<T>>>
}

pub struct OneshotRx<T> {
    inner: Arc<Mutex<OneshotData<T>>>
}

pub fn oneshot<T>() -> (OneshotTx<T>, OneshotRx<T>) {
    let inner = Arc::new(Mutex::new(Default::default()));
    (
        OneshotTx { inner: inner.clone() },
        OneshotRx { inner: inner }
    )
}

impl<T> OneshotTx<T> {
    pub fn send(self, value: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.value = Some(value);

        // if the waker is Some(_), it means that the Rx side
        // has been polled and returned Pending.
        // we need to wake it up.
        // If it is None, it means that the Rx side has not been polled yet.
        // then we can just set the value
        // And when the Rx is polled, it will return ready because there
        // is an available value.
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for OneshotRx<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(value) = inner.value.take() {
            Poll::Ready(value)
        } else {
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

