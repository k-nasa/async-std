use core::fmt;
use core::sync::atomic::{AtomicU64, Ordering};

/// A unique identifier for a task.
///
/// # Examples
///
/// ```
/// use async_core::task;
///
/// task::block_on(async {
///     println!("id = {:?}", task::current().id());
/// })
/// ```
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct TaskId(pub(crate) u64);

impl TaskId {
    /// Generates a new `TaskId`.
    pub(crate) fn generate() -> TaskId {
        static COUNTER: AtomicU64 = AtomicU64::new(1);

        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > u64::max_value() / 2 {
            core::process::abort();
        }
        TaskId(id)
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
