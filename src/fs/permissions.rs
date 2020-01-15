cfg_not_docs! {
    pub use core::fs::Permissions;
}

cfg_docs! {
    /// A set of permissions on a file or directory.
    ///
    /// This type is a re-export of [`core::fs::Permissions`].
    ///
    /// [`core::fs::Permissions`]: https://doc.rust-lang.org/core/fs/struct.Permissions.html
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Permissions {
        _private: (),
    }

    impl Permissions {
        /// Returns the read-only flag.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
        /// #
        /// use async_core::fs;
        ///
        /// let perm = fs::metadata("a.txt").await?.permissions();
        /// println!("{:?}", perm.readonly());
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn readonly(&self) -> bool {
            unreachable!("this impl only appears in the rendered docs")
        }

        /// Configures the read-only flag.
        ///
        /// [`fs::set_permissions`]: fn.set_permissions.html
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> core::io::Result<()> { async_core::task::block_on(async {
        /// #
        /// use async_core::fs;
        ///
        /// let mut perm = fs::metadata("a.txt").await?.permissions();
        /// perm.set_readonly(true);
        /// fs::set_permissions("a.txt", perm).await?;
        /// #
        /// # Ok(()) }) }
        /// ```
        pub fn set_readonly(&mut self, readonly: bool) {
            unreachable!("this impl only appears in the rendered docs")
        }
    }
}
