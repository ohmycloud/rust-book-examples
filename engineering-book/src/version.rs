// src/version.rs - consuming the metadata
pub struct BuildInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub git_hash: &'static str,
    pub build_epoch: &'static str,
    pub build_target: &'static str,
    pub build_profile: &'static str,
}

pub const BUILD_INFO: BuildInfo = BuildInfo {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
    git_hash: env!("APP_GIT_HASH"),
    build_epoch: env!("APP_BUILD_EPOCH"),
    build_target: env!("APP_BUILD_TARGET"),
    build_profile: env!("APP_BUILD_PROFILE"),
};

impl BuildInfo {
    /// Parse the epoch at runtime when needed (const &str -> u64 is not possible
    /// on stable Rust - There is no const fn for str-to-int).
    pub fn build_epoch_secs(&self) -> u64 {
        self.build_epoch.parse().unwrap_or(0)
    }

    pub fn new(
        name: &'static str,
        version: &'static str,
        git_hash: &'static str,
        build_epoch: &'static str,
        build_target: &'static str,
        build_profile: &'static str,
    ) -> Self {
        Self {
            name,
            version,
            git_hash,
            build_epoch,
            build_target,
            build_profile,
        }
    }
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DiagTool v{} (git: {} target: {}, build epoch: {}, build profile: {})",
            self.version, self.git_hash, self.build_target, self.build_epoch, self.build_profile
        )
    }
}
