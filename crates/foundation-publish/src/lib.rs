// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
//! Publication bridge and sunCloud attribution for the gAIa sovereignty stack.
//!
//! This crate handles two responsibilities:
//!
//! 1. **Gallery generation**: Bridges lithoSpore pseudoSpore artifacts to
//!    sporePrint's Zola static site (Markdown gallery pages with front matter).
//!
//! 2. **Radiating attribution** (Layer 6): Reads sweetGrass braids to extract
//!    contributor graphs and calculates proportional value distribution along
//!    the attribution chain per sunCloud economics.

pub mod attribution;
pub mod domain_profile;
pub mod gallery;
pub mod registry;
pub mod urls;

pub use attribution::{
    calculate_attribution, AttributionError, Contributor, ContributionRole, DistributionManifest,
    MetabolicSchedule, RateRange,
};
pub use domain_profile::{DomainProfileHeader, ProfileIndex};
pub use gallery::GalleryGenerator;
pub use registry::{PseudoSporeEntry, SporeRegistry};
