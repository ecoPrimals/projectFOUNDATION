use super::manifest::ContributionRole;

/// A rate range for metabolic distribution (min..max percentage).
///
/// Per whitePaper `economics/SUNCLOUD_ECONOMIC_MODEL.md`, the metabolic
/// mandate uses the square-cube law: network cost grows ~squared (surface
/// area), network value grows ~cubed (volume). sunCloud regulates:
/// value nourishes cost; treasury is a heart, not a hoard.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateRange {
    /// Minimum rate (0.0 to 1.0).
    pub min: f64,
    /// Maximum rate (0.0 to 1.0).
    pub max: f64,
}

impl RateRange {
    /// Create a new rate range.
    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Midpoint of the range — used as the default operating rate.
    #[must_use]
    pub const fn midpoint(&self) -> f64 {
        f64::midpoint(self.min, self.max)
    }
}

/// Metabolic rate schedule — how value is distributed by contribution category.
///
/// Default rates from whitePaper `gen4/economics/SUNCLOUD_ACTIVATION.md`:
/// - Infrastructure: 3-7% (keeps the lights on)
/// - Science: 2-5% (rewards discovery)
/// - Product: 0% (products earn from users, not from braids)
/// - Ecosystem: 1-3% (commons maintenance, gAIa Loan)
///
/// The anti-platform principle: 5-15% metabolic vs platform 30% rent.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetabolicSchedule {
    /// Infrastructure providers (compute nodes, storage, networking).
    pub infrastructure: RateRange,
    /// Science contributors (experimenters, data curators, cited authors).
    pub science: RateRange,
    /// Product teams (0% — products earn from users, not braids).
    pub product: RateRange,
    /// Ecosystem maintenance (commons, bounties, education).
    pub ecosystem: RateRange,
}

impl MetabolicSchedule {
    /// Return the operating rate for a contribution role.
    ///
    /// Uses the midpoint of the rate range as the default operating point.
    #[must_use]
    pub const fn rate_for_role(&self, role: &ContributionRole) -> f64 {
        match role {
            ContributionRole::Infrastructure => self.infrastructure.midpoint(),
            ContributionRole::Science => self.science.midpoint(),
            ContributionRole::Product => self.product.midpoint(),
            ContributionRole::Ecosystem => self.ecosystem.midpoint(),
        }
    }
}

impl Default for MetabolicSchedule {
    fn default() -> Self {
        Self {
            infrastructure: RateRange::new(0.03, 0.07),
            science: RateRange::new(0.02, 0.05),
            product: RateRange::new(0.0, 0.0),
            ecosystem: RateRange::new(0.01, 0.03),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_schedule_values() {
        let s = MetabolicSchedule::default();
        assert!((s.infrastructure.midpoint() - 0.05).abs() < f64::EPSILON);
        assert!((s.science.midpoint() - 0.035).abs() < f64::EPSILON);
        assert!((s.product.midpoint() - 0.0).abs() < f64::EPSILON);
        assert!((s.ecosystem.midpoint() - 0.02).abs() < f64::EPSILON);
    }

    #[test]
    fn rate_for_role_maps_correctly() {
        let s = MetabolicSchedule::default();
        assert!((s.rate_for_role(&ContributionRole::Infrastructure) - 0.05).abs() < f64::EPSILON);
        assert!((s.rate_for_role(&ContributionRole::Science) - 0.035).abs() < f64::EPSILON);
        assert!((s.rate_for_role(&ContributionRole::Product) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn custom_schedule() {
        let s = MetabolicSchedule {
            infrastructure: RateRange::new(0.10, 0.20),
            science: RateRange::new(0.05, 0.15),
            product: RateRange::new(0.01, 0.02),
            ecosystem: RateRange::new(0.02, 0.04),
        };
        assert!((s.infrastructure.midpoint() - 0.15).abs() < f64::EPSILON);
        assert!((s.science.midpoint() - 0.10).abs() < f64::EPSILON);
    }

    #[test]
    fn anti_platform_principle() {
        let s = MetabolicSchedule::default();
        let total_max = s.infrastructure.max + s.science.max + s.product.max + s.ecosystem.max;
        assert!(
            total_max < 0.30,
            "total metabolic max ({total_max}) must be < 30% (anti-platform)"
        );
    }

    #[test]
    fn schedule_json_roundtrip() {
        let s = MetabolicSchedule::default();
        let json = serde_json::to_string(&s).unwrap();
        let parsed: MetabolicSchedule = serde_json::from_str(&json).unwrap();
        assert!(
            (parsed.infrastructure.midpoint() - s.infrastructure.midpoint()).abs() < f64::EPSILON
        );
    }
}
