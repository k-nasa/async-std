use crate::fs::Metadata;
use crate::io;
use crate::path::Path;
use crate::task::spawn_blocking;

/// Reads metadata for a path without following symbolic links.
///
/// If you want to follow symbolic links before reading metadata of the target file or directory,
/// use [`metadata`] instead.
///
/// This function is an async version of [`core::fs::symlink_metadata`].
///
/// [`metadata`]: fn.metadata.html
/// [`core::fs::symlink_metadata`]: https://doc.rust-lang.org/core/fs/fn.symlink_metadata.html
///
/// # Errors
///
/// An error will be returned in the following situations:
///
/// * `path` does not point to an existing file or directory.
/// * The current process lacks permissions to read metadata for the path.
/// * Some other I/O error occurred.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
/// #
/// use async_core::fs;
///
/// let perm = fs::symlink_metadata("a.txt").await?.permissions();
/// #
/// # Ok(()) }) }
/// ```
pub async fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || core::fs::symlink_metadata(path)).await
}
