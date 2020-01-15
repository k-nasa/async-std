use core::time::Duration;

use crate::future;
use crate::io;

/// Sleeps for the specified amount of time.
///
/// This function might sleep for slightly longer than the specified duration but never less.
///
/// This function is an async version of [`core::thread::sleep`].
///
/// [`core::thread::sleep`]: https://doc.rust-lang.org/core/thread/fn.sleep.html
///
/// See also: [`stream::interval`].
///
/// [`stream::interval`]: ../stream/fn.interval.html
///
/// # Examples
///
/// ```
/// # async_core::task::block_on(async {
/// #
/// use core::time::Duration;
///
/// use async_core::task;
///
/// task::sleep(Duration::from_secs(1)).await;
/// #
/// # })
/// ```
pub async fn sleep(dur: Duration) {
    let _: io::Result<()> = io::timeout(dur, future::pending()).await;
}
