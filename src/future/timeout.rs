use core::error::Error;
use core::fmt;
use core::pin::Pin;
use core::time::Duration;
use core::future::Future;

use futures_timer::Delay;
use pin_project_lite::pin_project;

use crate::task::{Context, Poll};

/// Awaits a future or times out after a duration of time.
///
/// If you want to await an I/O future consider using
/// [`io::timeout`](../io/fn.timeout.html) instead.
///
/// # Examples
///
/// ```
/// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
/// #
/// use core::time::Duration;
///
/// use async_core::future;
///
/// let never = future::pending::<()>();
/// let dur = Duration::from_millis(5);
/// assert!(future::timeout(dur, never).await.is_err());
/// #
/// # Ok(()) }) }
/// ```
pub async fn timeout<F, T>(dur: Duration, f: F) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    let f = TimeoutFuture {
        future: f,
        delay: Delay::new(dur),
    };
    f.await
}

pin_project! {
    /// A future that times out after a duration of time.
    pub struct TimeoutFuture<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Delay,
    }
}

impl<F> TimeoutFuture<F> {
    #[allow(dead_code)]
    pub(super) fn new(future: F, dur: Duration) -> TimeoutFuture<F> {
        TimeoutFuture { future: future, delay: Delay::new(dur) }
    }
}

impl<F: Future> Future for TimeoutFuture<F> {
    type Output = Result<F::Output, TimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(v) => Poll::Ready(Ok(v)),
            Poll::Pending => match this.delay.poll(cx) {
                Poll::Ready(_) => Poll::Ready(Err(TimeoutError { _private: () })),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

/// An error returned when a future times out.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeoutError {
    _private: (),
}

impl Error for TimeoutError {}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "future has timed out".fmt(f)
    }
}
