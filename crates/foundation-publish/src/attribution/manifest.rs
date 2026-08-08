use super::metabolic::MetabolicSchedule;

/// Role categories for contribution classification.
///
/// Maps to the metabolic rate schedule from sunCloud economics.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ContributionRole {
    /// Compute nodes, storage hosts, networking infrastructure.
    Infrastructure,
    /// Experimenters, data curators, cited authors, natural data sources.
    Science,
    /// Product teams (earn from users, not from braids).
    Product,
    /// Commons maintenance, bounties, education.
    Ecosystem,
}

impl std::fmt::Display for ContributionRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infrastructure => write!(f, "infrastructure"),
            Self::Science => write!(f, "science"),
            Self::Product => write!(f, "product"),
            Self::Ecosystem => write!(f, "ecosystem"),
        }
    }
}

/// A contributor in the attribution graph.
///
/// Contributors are identified by their `BearDog` identity (or a descriptive
/// string for natural data sources like "Yellowstone National Park Fund").
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contributor {
    /// `BearDog` identity or descriptive identifier.
    pub identity: String,
    /// Contribution category (determines metabolic rate).
    pub role: ContributionRole,
    /// Relative weight in the attribution graph (higher = more contribution).
    pub weight: f64,
    /// Whether the beneficiary is reachable for distribution.
    /// Unreachable shares revert to gAIa commons per sunCloud model.
    pub reachable: bool,
}

/// An individual share in a distribution manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShareEntry {
    /// The contributor receiving this share.
    pub contributor: Contributor,
    /// Amount in the distribution unit (abstract — units defined by context).
    pub amount: u64,
}

/// Distribution manifest — the output of attribution calculation.
///
/// Records who gets what share of a value event, with provenance
/// linking back to the sweetGrass braid that sourced the attribution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistributionManifest {
    /// Total value being distributed.
    pub total_value: u64,
    /// Total distributed to reachable contributors.
    pub distributed: u64,
    /// Total reverting to gAIa commons (unreachable beneficiaries).
    pub gaia_commons_revert: u64,
    /// Rounding remainder (total - distributed - reverted).
    pub remainder: u64,
    /// Individual shares for reachable contributors.
    pub shares: Vec<ShareEntry>,
    /// Metabolic schedule used for this calculation.
    pub schedule: MetabolicSchedule,
}

impl DistributionManifest {
    /// Percentage of value that was distributed to reachable contributors.
    #[must_use]
    #[expect(clippy::cast_precision_loss, reason = "rates are small enough for f64")]
    pub fn distribution_rate(&self) -> f64 {
        if self.total_value == 0 {
            return 0.0;
        }
        self.distributed as f64 / self.total_value as f64
    }

    /// Percentage reverting to gAIa commons.
    #[must_use]
    #[expect(clippy::cast_precision_loss, reason = "rates are small enough for f64")]
    pub fn gaia_revert_rate(&self) -> f64 {
        if self.total_value == 0 {
            return 0.0;
        }
        self.gaia_commons_revert as f64 / self.total_value as f64
    }
}

impl std::fmt::Display for DistributionManifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "distribution(total={}, distributed={}, gaia_revert={}, shares={})",
            self.total_value,
            self.distributed,
            self.gaia_commons_revert,
            self.shares.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contribution_role_display() {
        assert_eq!(ContributionRole::Infrastructure.to_string(), "infrastructure");
        assert_eq!(ContributionRole::Science.to_string(), "science");
        assert_eq!(ContributionRole::Product.to_string(), "product");
        assert_eq!(ContributionRole::Ecosystem.to_string(), "ecosystem");
    }

    #[test]
    fn distribution_rate_calculation() {
        let manifest = DistributionManifest {
            total_value: 10_000,
            distributed: 500,
            gaia_commons_revert: 100,
            remainder: 9_400,
            shares: vec![],
            schedule: MetabolicSchedule::default(),
        };
        assert!((manifest.distribution_rate() - 0.05).abs() < f64::EPSILON);
        assert!((manifest.gaia_revert_rate() - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn zero_total_no_panic() {
        let manifest = DistributionManifest {
            total_value: 0,
            distributed: 0,
            gaia_commons_revert: 0,
            remainder: 0,
            shares: vec![],
            schedule: MetabolicSchedule::default(),
        };
        assert!((manifest.distribution_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn display_format() {
        let manifest = DistributionManifest {
            total_value: 10_000,
            distributed: 500,
            gaia_commons_revert: 100,
            remainder: 9_400,
            shares: vec![],
            schedule: MetabolicSchedule::default(),
        };
        let display = format!("{manifest}");
        assert!(display.contains("total=10000"));
        assert!(display.contains("distributed=500"));
    }
}
