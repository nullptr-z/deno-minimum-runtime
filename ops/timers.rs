use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

struct DelayFn<F> {
    delay: Option<Pin<Box<dyn Future<Output = ()>>>>,
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

fn delay_fn<F: FnOnce() + Send + 'static>(duration: Duration, f: F) -> impl Future<Output = ()> {
    DelayFn {
        delay: Some(Box::pin(async_delay_fn(duration, || {}))),
        f: Some(Box::new(f)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_delay_fn() {
        let delay_long = Duration::from_secs(5);
        let delay_short = Duration::from_secs(3);
        let start = Instant::now();

        delay_fn(delay_short, || {
            println!("Delayed execution delay_short");
        })
        .await;

        delay_fn(delay_long, || {
            println!("Delayed execution delay_long");
        })
        .await;

        let elapsed = start.elapsed();
        assert!(elapsed >= delay_long);
    }
}
