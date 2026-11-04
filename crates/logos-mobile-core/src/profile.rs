/// Confidence a platform profiler assigns to a single measurement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Confidence {
    /// Best-effort estimate; policy can reject it for consequential decisions.
    Low,
    /// Measurement is useful but may have platform-specific uncertainty.
    Medium,
    /// Direct or otherwise high-reliability observation.
    High,
}

/// A measured value, its sample time, and its provenance quality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Measurement<T> {
    value: Option<T>,
    sampled_at_seconds: Option<u64>,
    confidence: Option<Confidence>,
}

impl<T> Measurement<T> {
    /// Create a measurement with an explicit Unix timestamp and confidence.
    #[must_use]
    pub const fn observed(value: T, sampled_at_seconds: u64, confidence: Confidence) -> Self {
        Self {
            value: Some(value),
            sampled_at_seconds: Some(sampled_at_seconds),
            confidence: Some(confidence),
        }
    }

    /// Create an unavailable measurement. This is distinct from a measured zero.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            value: None,
            sampled_at_seconds: None,
            confidence: None,
        }
    }

    /// Return the value when it has been observed.
    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Return the sample time when it has been observed.
    #[must_use]
    pub const fn sampled_at_seconds(&self) -> Option<u64> {
        self.sampled_at_seconds
    }

    /// Return the assigned confidence when it has been observed.
    #[must_use]
    pub const fn confidence(&self) -> Option<Confidence> {
        self.confidence
    }

    /// Classify availability, age, and confidence against caller-supplied policy.
    #[must_use]
    pub fn status(
        &self,
        now_seconds: u64,
        max_age_seconds: u64,
        minimum_confidence: Confidence,
    ) -> MeasurementStatus {
        let (Some(_), Some(sampled_at), Some(confidence)) =
            (&self.value, self.sampled_at_seconds, self.confidence)
        else {
            return MeasurementStatus::Missing;
        };

        if sampled_at > now_seconds {
            return MeasurementStatus::FutureDated;
        }

        let age_seconds = now_seconds - sampled_at;
        if age_seconds > max_age_seconds {
            return MeasurementStatus::Stale { age_seconds };
        }

        if confidence < minimum_confidence {
            return MeasurementStatus::LowConfidence;
        }

        MeasurementStatus::Fresh { age_seconds }
    }
}

/// Result of checking a measurement against an explicit time and policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeasurementStatus {
    /// No value, timestamp, or confidence was supplied.
    Missing,
    /// The timestamp is later than the caller's current time.
    FutureDated,
    /// The observation exceeds the caller's maximum age.
    Stale { age_seconds: u64 },
    /// The observation does not meet the caller's confidence floor.
    LowConfidence,
    /// The observation meets its age and confidence requirements.
    Fresh { age_seconds: u64 },
}

/// Device architecture reported by a platform adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Architecture {
    Arm64,
    Arm32,
    X86_64,
    X86,
    Other(String),
}

/// Thermal severity reported by a platform adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThermalState {
    Unknown,
    Nominal,
    Fair,
    Serious,
    Critical,
    Shutdown,
}

/// Network class reported by a platform adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkType {
    Unknown,
    None,
    Wifi,
    Cellular,
    Ethernet,
    Other,
}

/// Timestamped device observations consumed by platform-independent policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityProfile {
    pub architecture: Measurement<Architecture>,
    pub cpu_cores: Measurement<u32>,
    pub total_memory_bytes: Measurement<u64>,
    pub available_memory_bytes: Measurement<u64>,
    pub available_storage_bytes: Measurement<u64>,
    pub battery_percent: Measurement<u8>,
    pub is_charging: Measurement<bool>,
    pub thermal_state: Measurement<ThermalState>,
    pub network_type: Measurement<NetworkType>,
    pub bandwidth_estimate_bps: Measurement<u64>,
}

/// Boundary implemented by platform adapters or deterministic test profilers.
pub trait Profiler {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Collect one device snapshot. Adapters timestamp each sample when observed.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the platform cannot collect a snapshot.
    fn collect(&self) -> Result<CapabilityProfile, Self::Error>;
}

#[cfg(test)]
mod tests {
    // All values below are synthetic test fixtures, not platform observations.
    use super::{Confidence, Measurement, MeasurementStatus};

    #[test]
    fn fresh_zero_measurement_is_not_treated_as_missing() {
        let measurement = Measurement::observed(0_u64, 90, Confidence::High);

        assert_eq!(
            measurement.status(100, 30, Confidence::Low),
            MeasurementStatus::Fresh { age_seconds: 10 }
        );
    }

    #[test]
    fn stale_measurement_reports_its_age() {
        let measurement = Measurement::observed(8_u32, 10, Confidence::High);

        assert_eq!(
            measurement.status(50, 20, Confidence::Low),
            MeasurementStatus::Stale { age_seconds: 40 }
        );
    }

    #[test]
    fn unknown_measurement_is_distinct_from_zero() {
        let measurement = Measurement::<u64>::unknown();

        assert_eq!(
            measurement.status(100, 30, Confidence::Low),
            MeasurementStatus::Missing
        );
        assert_eq!(measurement.value(), None);
    }

    #[test]
    fn confidence_below_policy_floor_is_not_fresh() {
        let measurement = Measurement::observed(8_u32, 99, Confidence::Low);

        assert_eq!(
            measurement.status(100, 30, Confidence::Medium),
            MeasurementStatus::LowConfidence
        );
    }

    #[test]
    fn future_dated_measurement_is_not_fresh() {
        let measurement = Measurement::observed(8_u32, 101, Confidence::High);

        assert_eq!(
            measurement.status(100, 30, Confidence::Low),
            MeasurementStatus::FutureDated
        );
    }
}
