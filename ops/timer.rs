use deno_core::{anyhow::Result, op};
use std::future::{self, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

// #[op]
async fn op_setTimeout(delay: u64) -> Result<bool> {
    // spawn(move || {
    //     std::thread::sleep(std::time::Duration::from_millis(delay as u64));
    //     println!("【 setTimeout 】==> {}", delay);
    // });
    block_on(move || {
        let client = reqwest::Client::new();
        let res = client.get("https://dummyjson.com/products/1").send();

        res
    });

    std::thread::sleep(std::time::Duration::from_millis(delay as u64));

    Ok(true)
}

fn block_on<F: Future + std::marker::Unpin>(mut future: F) -> F::Output {
    let waker = futures::task::noop_waker_ref();
    let mut cx = Context::from_waker(waker);

    loop {
        match Pin::new(&mut future).poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => (),
        }
    }
}
