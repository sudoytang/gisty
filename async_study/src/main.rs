pub mod async_ctx_naive;
pub mod async_ctx_awakenable;
pub mod future;

use future::NPollsToBeReady;

async fn difficult_hello() {
    NPollsToBeReady::new(5).await;
    println!("Hello from async!!");
}


fn main() {
    async_ctx_naive::block_on(difficult_hello());
}
