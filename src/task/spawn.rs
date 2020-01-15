use core::future::Future;

use crate::task::{Builder, JoinHandle};

/// Spawns a task.
///
/// This function is similar to [`core::thread::spawn`], except it spawns an asynchronous task.
///
/// [`core::thread`]: https://doc.rust-lang.org/core/thread/fn.spawn.html
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::task;
///
/// let handle = task::spawn(async {
///     1 + 2
/// });
///
/// assert_eq!(handle.await, 3);
/// #
/// # })
/// ```
pub fn spawn<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(future).expect("cannot spawn task")
}
