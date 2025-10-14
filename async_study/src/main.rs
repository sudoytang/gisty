
#[allow(unused)]
use async_study::async_ctx_naive;
#[allow(unused)]
use async_study::async_ctx_awakenable;
#[allow(unused)]
use async_study::async_ctx_multitask;

use async_study::future;

use future::NPollsToBeReady;

async fn difficult_hello() {
    async_ctx_multitask::Executor::spawn(NPollsToBeReady::new(3, "spawned".into()));

    NPollsToBeReady::new(5, "#1".into()).await;
    println!("Hello from async!!");
}


fn main() {
    async_ctx_multitask::Executor::block_on(difficult_hello());
}
