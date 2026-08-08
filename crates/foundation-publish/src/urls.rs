// SPDX-License-Identifier: AGPL-3.0-or-later
//! Centralized URL resolution for sporePrint gallery generation.
//!
//! URLs are resolved from environment variables with sensible defaults.
//! This follows the `env_keys` pattern: no bare string literals in
//! production routing.

/// Env var for pseudoSpore download base URL.
pub const DOWNLOAD_BASE_ENV: &str = "FOUNDATION_DOWNLOAD_BASE";

/// Env var for Forgejo base URL.
pub const FORGEJO_BASE_ENV: &str = "FOUNDATION_FORGEJO_BASE";

/// Default download base URL (used when env var is not set).
const DOWNLOAD_BASE_DEFAULT: &str = "https://primals.eco/lab/spores";

/// Default Forgejo base URL (used when env var is not set).
const FORGEJO_BASE_DEFAULT: &str = "https://git.primals.eco";

/// Resolve the pseudoSpore download base URL.
///
/// Priority: `FOUNDATION_DOWNLOAD_BASE` env var -> default.
#[must_use]
pub fn download_base() -> String {
    std::env::var(DOWNLOAD_BASE_ENV).unwrap_or_else(|_| DOWNLOAD_BASE_DEFAULT.to_owned())
}

/// Resolve the Forgejo base URL.
///
/// Priority: `FOUNDATION_FORGEJO_BASE` env var -> default.
#[must_use]
pub fn forgejo_base() -> String {
    std::env::var(FORGEJO_BASE_ENV).unwrap_or_else(|_| FORGEJO_BASE_DEFAULT.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_base_returns_nonempty() {
        let url = download_base();
        assert!(!url.is_empty());
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn forgejo_base_returns_nonempty() {
        let url = forgejo_base();
        assert!(!url.is_empty());
        assert!(url.starts_with("https://"));
    }

    #[test]
    fn env_var_names_follow_convention() {
        assert!(DOWNLOAD_BASE_ENV.starts_with("FOUNDATION_"));
        assert!(FORGEJO_BASE_ENV.starts_with("FOUNDATION_"));
    }
}
