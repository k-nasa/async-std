/// Resolves to the provided value.
///
/// This function is an async version of [`core::convert::identity`].
///
/// [`core::convert::identity`]: https://doc.rust-lang.org/core/convert/fn.identity.html
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::future;
///
/// assert_eq!(future::ready(10).await, 10);
/// #
/// # })
/// ```
pub async fn ready<T>(val: T) -> T {
    val
}
