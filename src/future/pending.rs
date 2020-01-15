use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;

use crate::task::{Context, Poll};

/// Never resolves to a value.
///
/// # Examples
///
/// ```
/// # async_core::task::block_on(async {
/// #
/// use core::time::Duration;
///
/// use async_core::future;
/// use async_core::io;
///
/// let dur = Duration::from_secs(1);
/// let fut = future::pending();
///
/// let res: io::Result<()> = io::timeout(dur, fut).await;
/// assert!(res.is_err());
/// #
/// # })
/// ```
pub async fn pending<T>() -> T {
    let fut = Pending {
        _marker: PhantomData,
    };
    fut.await
}

struct Pending<T> {
    _marker: PhantomData<T>,
}

impl<T> Future for Pending<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}
