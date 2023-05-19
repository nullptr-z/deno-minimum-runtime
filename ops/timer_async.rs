use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

struct DelayFn<F> {
    delay: Option<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    f: Option<Box<F>>,
}

impl<F: FnOnce() + 'static> Future for DelayFn<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(delay) = self.delay.as_mut() {
            match Pin::new(delay).poll(cx) {
                Poll::Ready(()) => {
                    self.delay = None;
                    let f = self.f.take().unwrap();
                    f();
                    Poll::Ready(())
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(())
        }
    }
}

async fn async_delay_fn<F: FnOnce() + 'static>(duration: Duration, f: F) {
    sleep(duration).await;
    f();
}

fn delay_fn<F: FnOnce() + 'static>(duration: Duration, f: F) -> impl Future<Output = ()> + 'static {
    DelayFn {
        delay: Some(Box::pin(async_delay_fn(duration, || {}))),
        f: Some(Box::new(f)),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::task::{spawn, spawn_local, LocalSet};

    use super::*;

    #[tokio::test]
    async fn test_delay_fn() {
        let delay_long = Duration::from_secs(4);
        let delay_short = Duration::from_secs(3);
        let start: Instant = Instant::now();

        // spawn_local运行在当前线程中，它没有线程切换的开销，因此在处理高并发或者密集计算的情况下，spawn_local通常比spawn更高效。
        // 但是因单线程原因,spawn_local不适合阻塞任务和长时间占用算力的任务
        let local_set = LocalSet::new();
        local_set
            .run_until(async {
                let task_long: tokio::task::JoinHandle<()> =
                    spawn_local(delay_fn(delay_long, || {
                        println!("Delayed execution delay_long");
                    }));

                let task_short = spawn_local(delay_fn(delay_short, || {
                    println!("Delayed execution delay_short");
                }));
                // --及其像Promise::All；并发地等待多个异步操作的完成，并获取它们的结果
                tokio::try_join!(task_short, task_long).unwrap();
            })
            .await;
        let elapsed = start.elapsed();
        assert!(elapsed >= delay_long);
    }
}
