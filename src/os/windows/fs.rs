//! Windows-specific filesystem extensions.

use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// Creates a new directory symbolic link on the filesystem.
///
/// The `dst` path will be a directory symbolic link pointing to the `src` path.
///
/// This function is an async version of [`core::os::windows::fs::symlink_dir`].
///
/// [`core::os::windows::fs::symlink_dir`]: https://doc.rust-lang.org/core/os/windows/fs/fn.symlink_dir.html
///
/// # Examples
///
/// ```no_run
/// # fn main() -> core::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::os::windows::fs::symlink_dir;
///
/// symlink_dir("a", "b").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    spawn_blocking(move || core::os::windows::fs::symlink_dir(&src, &dst)).await
}

/// Creates a new file symbolic link on the filesystem.
///
/// The `dst` path will be a file symbolic link pointing to the `src` path.
///
/// This function is an async version of [`core::os::windows::fs::symlink_file`].
///
/// [`core::os::windows::fs::symlink_file`]: https://doc.rust-lang.org/core/os/windows/fs/fn.symlink_file.html
///
/// # Examples
///
/// ```no_run
/// # fn main() -> core::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::os::windows::fs::symlink_file;
///
/// symlink_file("a.txt", "b.txt").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref().to_owned();
    let dst = dst.as_ref().to_owned();
    spawn_blocking(move || core::os::windows::fs::symlink_file(&src, &dst)).await
}
