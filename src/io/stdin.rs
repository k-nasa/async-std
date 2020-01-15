use core::future::Future;
use core::pin::Pin;
use core::sync::Mutex;

use crate::future;
use crate::io::{self, Read};
use crate::task::{spawn_blocking, Context, JoinHandle, Poll};
use crate::utils::Context as _;

cfg_unstable! {
    use once_cell::sync::Lazy;
    use core::io::Read as _;
}

/// Constructs a new handle to the standard input of the current process.
///
/// This function is an async version of [`core::io::corein`].
///
/// [`core::io::corein`]: https://doc.rust-lang.org/core/io/fn.corein.html
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return
/// an error.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
/// #
/// use async_core::io;
///
/// let corein = io::corein();
/// let mut line = String::new();
/// corein.read_line(&mut line).await?;
/// #
/// # Ok(()) }) }
/// ```
pub fn corein() -> Stdin {
    Stdin(Mutex::new(State::Idle(Some(Inner {
        corein: core::io::corein(),
        line: String::new(),
        buf: Vec::new(),
        last_op: None,
    }))))
}

/// A handle to the standard input of the current process.
///
/// This reader is created by the [`corein`] function. See its documentation for
/// more.
///
/// ### Note: Windows Portability Consideration
///
/// When operating in a console, the Windows implementation of this stream does not support
/// non-UTF-8 byte sequences. Attempting to write bytes that are not valid UTF-8 will return
/// an error.
///
/// [`corein`]: fn.corein.html
#[derive(Debug)]
pub struct Stdin(Mutex<State>);

/// A locked reference to the Stdin handle.
///
/// This handle implements the [`Read`] traits, and is constructed via the [`Stdin::lock`] method.
///
/// [`Read`]: trait.Read.html
/// [`Stdin::lock`]: struct.Stdin.html#method.lock
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[cfg(feature = "unstable")]
#[derive(Debug)]
pub struct StdinLock<'a>(core::io::StdinLock<'a>);

#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
unsafe impl Send for StdinLock<'_> {}

/// The state of the asynchronous corein.
///
/// The corein can be either idle or busy performing an asynchronous operation.
#[derive(Debug)]
enum State {
    /// The corein is idle.
    Idle(Option<Inner>),

    /// The corein is blocked on an asynchronous operation.
    ///
    /// Awaiting this operation will result in the new state of the corein.
    Busy(JoinHandle<State>),
}

/// Inner representation of the asynchronous corein.
#[derive(Debug)]
struct Inner {
    /// The blocking corein handle.
    corein: core::io::Stdin,

    /// The line buffer.
    line: String,

    /// The write buffer.
    buf: Vec<u8>,

    /// The result of the last asynchronous operation on the corein.
    last_op: Option<Operation>,
}

/// Possible results of an asynchronous operation on the corein.
#[derive(Debug)]
enum Operation {
    ReadLine(io::Result<usize>),
    Read(io::Result<usize>),
}

impl Stdin {
    /// Reads a line of input into the specified buffer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
    /// #
    /// use async_core::io;
    ///
    /// let corein = io::corein();
    /// let mut line = String::new();
    /// corein.read_line(&mut line).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn read_line(&self, buf: &mut String) -> io::Result<usize> {
        future::poll_fn(|cx| {
            let state = &mut *self.0.lock().unwrap();

            loop {
                match state {
                    State::Idle(opt) => {
                        let inner = opt.as_mut().unwrap();

                        // Check if the operation has completed.
                        if let Some(Operation::ReadLine(res)) = inner.last_op.take() {
                            let n = res?;

                            // Copy the read data into the buffer and return.
                            buf.push_str(&inner.line);
                            return Poll::Ready(Ok(n));
                        } else {
                            let mut inner = opt.take().unwrap();

                            // Start the operation asynchronously.
                            *state = State::Busy(spawn_blocking(move || {
                                inner.line.clear();
                                let res = inner.corein.read_line(&mut inner.line);
                                inner.last_op = Some(Operation::ReadLine(res));
                                State::Idle(Some(inner))
                            }));
                        }
                    }
                    // Poll the asynchronous operation the corein is currently blocked on.
                    State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
                }
            }
        })
        .await
        .context(|| String::from("could not read line on corein"))
    }

    /// Locks this handle to the standard input stream, returning a readable guard.
    ///
    /// The lock is released when the returned lock goes out of scope. The returned guard also implements the Read trait for accessing the underlying data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
    /// #
    /// use async_core::io;
    /// use async_core::prelude::*;
    ///
    /// let mut buffer = String::new();
    ///
    /// let corein = io::corein();
    /// let mut handle = corein.lock().await;
    ///
    /// handle.read_to_string(&mut buffer).await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    #[cfg(any(feature = "unstable", feature = "docs"))]
    pub async fn lock(&self) -> StdinLock<'static> {
        static STDIN: Lazy<core::io::Stdin> = Lazy::new(core::io::corein);

        spawn_blocking(move || StdinLock(STDIN.lock())).await
    }
}

impl Read for Stdin {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let state = &mut *self.0.lock().unwrap();

        loop {
            match state {
                State::Idle(opt) => {
                    let inner = opt.as_mut().unwrap();

                    // Check if the operation has completed.
                    if let Some(Operation::Read(res)) = inner.last_op.take() {
                        let n = res?;

                        // If more data was read than fits into the buffer, let's retry the read
                        // operation.
                        if n <= buf.len() {
                            // Copy the read data into the buffer and return.
                            buf[..n].copy_from_slice(&inner.buf[..n]);
                            return Poll::Ready(Ok(n));
                        }
                    } else {
                        let mut inner = opt.take().unwrap();

                        // Set the length of the inner buffer to the length of the provided buffer.
                        if inner.buf.len() < buf.len() {
                            inner.buf.reserve(buf.len() - inner.buf.len());
                        }
                        unsafe {
                            inner.buf.set_len(buf.len());
                        }

                        // Start the operation asynchronously.
                        *state = State::Busy(spawn_blocking(move || {
                            let res = core::io::Read::read(&mut inner.corein, &mut inner.buf);
                            inner.last_op = Some(Operation::Read(res));
                            State::Idle(Some(inner))
                        }));
                    }
                }
                // Poll the asynchronous operation the corein is currently blocked on.
                State::Busy(task) => *state = futures_core::ready!(Pin::new(task).poll(cx)),
            }
        }
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, RawFd};

    impl AsRawFd for Stdin {
        fn as_raw_fd(&self) -> RawFd {
            core::io::corein().as_raw_fd()
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{AsRawHandle, RawHandle};

    impl AsRawHandle for Stdin {
        fn as_raw_handle(&self) -> RawHandle {
            core::io::corein().as_raw_handle()
        }
    }
}

#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
impl Read for StdinLock<'_> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(self.0.read(buf))
    }
}
