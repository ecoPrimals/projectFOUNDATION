//! sunCloud radiating attribution (Layer 6 of the gAIa sovereignty stack).
//!
//! Reads sweetGrass braids to extract contributor graphs and calculates
//! proportional value distribution along the attribution chain. Per
//! whitePaper `economics/SUNCLOUD_ECONOMIC_MODEL.md`, sunCloud is NOT a
//! cryptocurrency — it is an autonomous, transparent protocol that
//! radiates value backward through braids to every contributor.

mod manifest;
mod metabolic;

pub use manifest::{Contributor, ContributionRole, DistributionManifest};
pub use metabolic::{MetabolicSchedule, RateRange};

use crate::attribution::manifest::ShareEntry;

/// Calculate attribution shares for a set of contributors.
///
/// Applies the metabolic schedule to divide the total value among
/// contributors according to their roles and weights. Contributions
/// with unreachable beneficiaries revert to the gAIa commons pool.
///
/// # Errors
///
/// Returns an error if the total value is zero or contributors are empty.
pub fn calculate_attribution(
    contributors: &[Contributor],
    total_value: u64,
    schedule: &MetabolicSchedule,
) -> Result<DistributionManifest, AttributionError> {
    if contributors.is_empty() {
        return Err(AttributionError::NoContributors);
    }
    if total_value == 0 {
        return Err(AttributionError::ZeroValue);
    }

    let mut shares = Vec::with_capacity(contributors.len());
    let mut distributed: u64 = 0;
    let mut gaia_revert: u64 = 0;

    let total_weight: f64 = contributors.iter().map(|c| c.weight).sum();
    if total_weight <= 0.0 {
        return Err(AttributionError::ZeroWeight);
    }

    for contributor in contributors {
        let category_rate = schedule.rate_for_role(&contributor.role);
        #[expect(clippy::cast_precision_loss, reason = "total_value fits f64 mantissa for realistic values")]
        let raw_share = (contributor.weight / total_weight) * category_rate * total_value as f64;
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "raw_share is non-negative and within u64 range"
        )]
        let share_units = raw_share.round() as u64;

        if contributor.reachable {
            shares.push(ShareEntry {
                contributor: contributor.clone(),
                amount: share_units,
            });
            distributed += share_units;
        } else {
            gaia_revert += share_units;
        }
    }

    Ok(DistributionManifest {
        total_value,
        distributed,
        gaia_commons_revert: gaia_revert,
        remainder: total_value.saturating_sub(distributed + gaia_revert),
        shares,
        schedule: schedule.clone(),
    })
}

/// Errors from attribution calculation.
#[derive(Debug, thiserror::Error)]
pub enum AttributionError {
    /// No contributors provided.
    #[error("no contributors in attribution graph")]
    NoContributors,
    /// Total value is zero.
    #[error("total value is zero — nothing to distribute")]
    ZeroValue,
    /// All contributor weights sum to zero or negative.
    #[error("total contributor weight is zero or negative")]
    ZeroWeight,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_contributors() -> Vec<Contributor> {
        vec![
            Contributor {
                identity: "experimenter-alice".to_owned(),
                role: ContributionRole::Science,
                weight: 3.0,
                reachable: true,
            },
            Contributor {
                identity: "compute-node-irongate".to_owned(),
                role: ContributionRole::Infrastructure,
                weight: 1.0,
                reachable: true,
            },
            Contributor {
                identity: "dataset-ncbi-genbank".to_owned(),
                role: ContributionRole::Science,
                weight: 1.0,
                reachable: true,
            },
        ]
    }

    #[test]
    fn basic_attribution() {
        let schedule = MetabolicSchedule::default();
        let result = calculate_attribution(&sample_contributors(), 10_000, &schedule).unwrap();
        assert!(result.distributed > 0);
        assert_eq!(result.gaia_commons_revert, 0);
        assert_eq!(result.shares.len(), 3);
    }

    #[test]
    fn unreachable_reverts_to_gaia() {
        let mut contributors = sample_contributors();
        contributors[2].reachable = false;
        let schedule = MetabolicSchedule::default();
        let result = calculate_attribution(&contributors, 10_000, &schedule).unwrap();
        assert!(result.gaia_commons_revert > 0);
        assert_eq!(result.shares.len(), 2);
    }

    #[test]
    fn empty_contributors_errors() {
        let schedule = MetabolicSchedule::default();
        assert!(calculate_attribution(&[], 10_000, &schedule).is_err());
    }

    #[test]
    fn zero_value_errors() {
        let schedule = MetabolicSchedule::default();
        assert!(calculate_attribution(&sample_contributors(), 0, &schedule).is_err());
    }

    #[test]
    fn higher_weight_gets_more() {
        let schedule = MetabolicSchedule::default();
        let result = calculate_attribution(&sample_contributors(), 10_000, &schedule).unwrap();
        let alice_share = result.shares[0].amount;
        let infra_share = result.shares[1].amount;
        assert!(
            alice_share > infra_share,
            "alice (w=3.0) should get more than infra (w=1.0)"
        );
    }

    #[test]
    fn manifest_json_roundtrip() {
        let schedule = MetabolicSchedule::default();
        let result = calculate_attribution(&sample_contributors(), 10_000, &schedule).unwrap();
        let json = serde_json::to_string(&result).unwrap();
        let parsed: DistributionManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.total_value, 10_000);
        assert_eq!(parsed.shares.len(), 3);
    }

    #[test]
    fn all_unreachable_fully_reverts() {
        let contributors: Vec<Contributor> = sample_contributors()
            .into_iter()
            .map(|mut c| {
                c.reachable = false;
                c
            })
            .collect();
        let schedule = MetabolicSchedule::default();
        let result = calculate_attribution(&contributors, 10_000, &schedule).unwrap();
        assert_eq!(result.distributed, 0);
        assert!(result.gaia_commons_revert > 0);
        assert!(result.shares.is_empty());
    }
}
