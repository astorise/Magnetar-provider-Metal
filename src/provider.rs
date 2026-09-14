//! `MetalProvider`: a structural placeholder for an Apple Metal execution
//! Provider, deliberately not a real device-discovery skeleton the way
//! `providers/rocm`'s `RocmProvider` is for ROCm.
//!
//! # Why this is unconditionally unavailable
//!
//! Unlike CUDA or ROCm -- vendor GPU runtimes that *could* be installed
//! on any of Windows/Linux/macOS, so "the shared library is missing" is a
//! real, dynamically-loadable-and-testable absence -- Metal is an Apple
//! operating-system framework, not a redistributable driver library.
//! There is no `dlopen`-and-see-if-it-fails story for Metal on a non-
//! macOS host: the framework, its headers, and its Objective-C runtime
//! dependencies simply do not exist there at all. This crate has never
//! been developed, compiled, or run on macOS (no macOS environment or CI
//! runner exists anywhere in this repository's current tooling), so
//! writing `#[cfg(target_os = "macos")]`-gated real Metal FFI here would
//! be code this repository's own tooling could never even type-check,
//! let alone verify against real Apple Silicon hardware -- the one thing
//! this repository's development practice consistently avoids (every
//! other Provider/Kernel in this workspace is verified against something
//! real before being considered done).
//!
//! `MetalProvider::new()` therefore always constructs successfully and
//! always reports [`ProviderHealth::Unavailable`] and zero Devices,
//! unconditionally, on every platform -- a real, honest, fully-verified
//! statement of what this crate can actually do today, not a stand-in
//! for real Metal support. See this crate's own README for what a future
//! contributor with real macOS/Apple Silicon hardware would need to do.

use magnetar_runtime::affinity::ProviderHealth;
use magnetar_runtime::provider::{Provider, ProviderError, ProviderMetadata, ProviderRegistry};

const METAL_PROVIDER_NAME: &str = "metal";
const METAL_PROVIDER_VERSION: &str = env!("CARGO_PKG_VERSION");
const METAL_PROVIDER_VENDOR: &str = "Apple";

pub fn metal_provider_metadata() -> ProviderMetadata {
    ProviderMetadata::new(
        METAL_PROVIDER_NAME,
        METAL_PROVIDER_VERSION,
        METAL_PROVIDER_VENDOR,
        "Structural placeholder for an Apple Metal Provider -- unconditionally \
         unavailable; no macOS development or CI environment exists in this \
         repository's tooling to implement or verify real Metal support against \
         (see this crate's own README)",
    )
}

/// The Metal Provider itself -- see this module's own doc comment for why
/// it is unconditionally unavailable on every platform today.
pub struct MetalProvider {
    metadata: ProviderMetadata,
}

impl MetalProvider {
    pub fn new() -> Self {
        Self {
            metadata: metal_provider_metadata(),
        }
    }

    /// Always `false` -- see this module's own doc comment.
    pub fn is_available(&self) -> bool {
        false
    }
}

impl Default for MetalProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Provider for MetalProvider {
    fn metadata(&self) -> ProviderMetadata {
        self.metadata.clone()
    }

    fn register(&self, _registry: &mut ProviderRegistry) -> Result<(), ProviderError> {
        Ok(())
    }

    fn health(&self) -> ProviderHealth {
        ProviderHealth::Unavailable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_unconditionally_unavailable() {
        let provider = MetalProvider::new();
        assert!(!provider.is_available());
        assert_eq!(provider.health(), ProviderHealth::Unavailable);
        assert!(provider.devices().is_empty());
    }

    #[test]
    fn metadata_is_well_formed() {
        let provider = MetalProvider::new();
        let metadata = provider.metadata();
        assert_eq!(metadata.name, METAL_PROVIDER_NAME);
        assert_eq!(metadata.vendor, METAL_PROVIDER_VENDOR);
        assert!(!metadata.description.is_empty());
    }
}
