use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::task::spawn;
use tokio::time::sleep;

struct DelayFn<F> {
    delay: Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    f: Option<Box<F>>,
}

impl<F: FnOnce() + Send + 'static> Future for DelayFn<F> {
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

async fn async_delay_fn<F: FnOnce() + Send + 'static>(duration: Duration, f: F) {
    sleep(duration).await;
    f();
}

fn delay_fn<F: FnOnce() + Send + 'static>(
    duration: Duration,
    f: F,
) -> impl Future<Output = ()> + Send + 'static {
    DelayFn {
        delay: Some(Box::pin(async_delay_fn(duration, || {}))),
        f: Some(Box::new(f)),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::task::spawn;

    use super::*;

    #[tokio::test]
    async fn test_delay_fn() {
        let delay_long = Duration::from_secs(4);
        let delay_short = Duration::from_secs(3);
        let start: Instant = Instant::now();
        let task_long = spawn(delay_fn(delay_long, || {
            println!("Delayed execution delay_long");
        }));

        let task_short = spawn(delay_fn(delay_short, || {
            println!("Delayed execution delay_short");
        }));

        let (res_short, res_long) = tokio::try_join!(task_short, task_long).unwrap();

        let elapsed = start.elapsed();
        assert!(elapsed >= delay_long);
    }
}
