use semver::Version;

pub trait Bump {
    /// Perform a patch bump.
    ///
    /// Example:
    /// ```
    /// # use semver::Version;
    /// fn main() {
    ///     let mut version = Version::new(0, 0, 0);
    ///     version.bump_patch();
    ///     assert_eq!(version.patch, 1);
    /// }
    fn bump_patch(&mut self);

    /// Perform a minor bump. This will reset the patch version to 0.
    ///
    /// Example:
    /// ```
    /// # use semver::Version;
    /// fn main() {
    ///     let mut version = Version::new(0, 0, 1);
    ///     version.bump_minor();
    ///     assert_eq!(version.patch, 0);
    ///     assert_eq!(version.minor, 1);
    /// }
    fn bump_minor(&mut self);

    /// Perform a major bump. This will reset the patch and minor versions to 0.
    ///
    /// Example:
    /// ```
    /// # use semver::Version;
    /// fn main() {
    ///     let mut version = Version::new(0, 1, 1);
    ///     version.bump_major();
    ///     assert_eq!(version.patch, 0);
    ///     assert_eq!(version.minor, 0);
    ///     assert_eq!(version.major, 1);
    /// }
    fn bump_major(&mut self);
}

impl Bump for Version {
    fn bump_patch(&mut self) {
        self.patch += 1;
    }

    fn bump_minor(&mut self) {
        self.patch = 0;
        self.minor += 1;
    }

    fn bump_major(&mut self) {
        self.patch = 0;
        self.minor = 0;
        self.major += 1;
    }
}
