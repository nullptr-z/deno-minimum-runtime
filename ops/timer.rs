use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

struct DelayFn<F> {
    delay: Option<std::time::Duration>,
    f: Option<Box<F>>,
}

impl<F: FnOnce() + Send + 'static> Future for DelayFn<F> {
    type Output = ();

    /// Output 为 Future 的完成类型；
    /// Poll::Ready(())的意思是 Future 已经完成。返回了一个值为 () 的Output。
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll ");
        // task()会返回self.delay的值，然后将self.delay设置为None。
        // 这样，下一次调用poll()时，也就是waker唤醒时，就会进入else分支。
        if let Some(delay) = self.delay.take() {
            let waker = cx.waker().clone();
            thread::spawn(move || {
                // 等待 delay 指定的时间间隔
                thread::sleep(delay);
                // 唤醒 waker
                waker.wake_by_ref();
                println!("waker ");
            });
            println!("Poll::Pending ");
            Poll::Pending
        } else {
            // 延时时间到，调用 f()回调
            let f = self.f.take().unwrap();
            f();
            // 异步任务完成
            println!("Poll::Ready ");
            Poll::Ready(())
        }
    }
}

fn delay_fn<F: FnOnce() + Send + 'static>(duration: Duration, f: F) -> impl Future<Output = ()> {
    DelayFn {
        delay: Some(duration),
        f: Some(Box::new(f)),
    }
}

async fn async_delay_fn<F: FnOnce() + Send + 'static>(duration: Duration, f: F) {
    sleep(duration).await; // 内部使用了 waker 的唤醒机制，可以简化代码，无需手动调用 waker.wake_by_ref()
    f();
}

// 通过轮询 Future 来实现异步等待；关键字 loop waker Context Pin Poll
fn _block_on<F: Future + std::marker::Unpin>(mut future: F) -> F::Output {
    let waker = futures::task::noop_waker_ref();
    let mut cx = Context::from_waker(waker);

    loop {
        match Pin::new(&mut future).poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // 在此处可以添加适当的线程调度策略，
                // 例如通过调用 thread::yield_now() 来让出 CPU 时间片，
                // 避免忙等待造成过多的 CPU 使用。
                thread::yield_now();
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_delay_fn() {
        let delay_long = Duration::from_secs(5);
        let delay_short = Duration::from_secs(4);
        let start = std::time::Instant::now();

        delay_fn(delay_long, || {
            println!("Delayed execution delay_long");
        })
        .await;

        delay_fn(delay_short, || {
            println!("Delayed execution delay_short");
        })
        .await;

        // 计算 Instant::now() 到现在的时间间隔
        let elapsed = start.elapsed();
        // 至少已经超过了 delay 指定的时间间隔
        assert!(elapsed >= delay_long);
    }
}
