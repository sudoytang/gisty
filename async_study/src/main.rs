
#[allow(unused)]
use async_study::exec::e01_naive;
#[allow(unused)]
use async_study::exec::e02_wake;
#[allow(unused)]
use async_study::exec::e03_multitask;
#[allow(unused)]
use async_study::exec::e04_join;

use async_study::future;

use future::NPollsToBeReady;

async fn difficult_hello() {
    let join_handle = e04_join::Executor::spawn(async {
        NPollsToBeReady::new(10, "#spawned".into()).await;
        return "Hello!";
    });

    NPollsToBeReady::new(5, "#1".into()).await;
    println!("Now I will join (wait) for this spawned task.");
    let res = join_handle.await;
    println!("Hello from async!! -> {}", res);
}


fn main() {
    e04_join::Executor::block_on(difficult_hello());
}
