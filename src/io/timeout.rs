use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use core::future::Future;

use futures_timer::Delay;
use pin_project_lite::pin_project;

use crate::io;

/// Awaits an I/O future or times out after a duration of time.
///
/// If you want to await a non I/O future consider using
/// [`future::timeout`](../future/fn.timeout.html) instead.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
/// #
/// use core::time::Duration;
///
/// use async_core::io;
///
/// io::timeout(Duration::from_secs(5), async {
///     let corein = io::corein();
///     let mut line = String::new();
///     let n = corein.read_line(&mut line).await?;
///     Ok(())
/// })
/// .await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn timeout<F, T>(dur: Duration, f: F) -> io::Result<T>
where
    F: Future<Output = io::Result<T>>,
{
    Timeout {
        timeout: Delay::new(dur),
        future: f,
    }
    .await
}

pin_project! {
    /// Future returned by the `FutureExt::timeout` method.
    #[derive(Debug)]
    pub struct Timeout<F, T>
    where
        F: Future<Output = io::Result<T>>,
    {
        #[pin]
        future: F,
        #[pin]
        timeout: Delay,
    }
}

impl<F, T> Future for Timeout<F, T>
where
    F: Future<Output = io::Result<T>>,
{
    type Output = io::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Pending => {}
            other => return other,
        }

        if this.timeout.poll(cx).is_ready() {
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out"));
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}
