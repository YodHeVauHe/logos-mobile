//! Deterministic resource-budget policy over timestamped device observations.

use crate::profile::{CapabilityProfile, Confidence, Measurement, MeasurementStatus, ThermalState};

/// A reason a budget was reduced or could not be calculated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BudgetLimitation {
    /// The measurement was not supplied.
    MissingMeasurement,
    /// The measurement is older than policy permits.
    StaleMeasurement,
    /// The measurement is dated in the future.
    FutureDatedMeasurement,
    /// The measurement confidence is below policy requirements.
    LowConfidence,
    /// A configured safety reserve reduced the result.
    ReserveApplied,
    /// A configured policy fraction reduced the result.
    PolicyCap,
    /// Battery status reduced available CPU budget.
    BatteryDerating,
    /// Thermal status reduced available CPU budget.
    ThermalDerating,
    /// Battery data was unavailable and its conservative factor was applied.
    UnknownBatteryConservative,
    /// Thermal data was unavailable and its conservative factor was applied.
    UnknownThermalConservative,
}

/// An amount and the explicit reasons that constrain it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetedAmount<T> {
    /// `None` means the input evidence was insufficient; `Some(0)` is measured zero.
    pub amount: Option<T>,
    /// Safety or evidence limitations applied to this amount.
    pub limitations: Vec<BudgetLimitation>,
}

impl<T> BudgetedAmount<T> {
    fn unavailable(reason: BudgetLimitation) -> Self {
        Self {
            amount: None,
            limitations: vec![reason],
        }
    }
}

/// CPU factors by reported thermal state, in integer percentages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThermalCpuFactors {
    pub unknown: u8,
    pub nominal: u8,
    pub fair: u8,
    pub serious: u8,
    pub critical: u8,
    pub shutdown: u8,
}

/// Caller-selected caps, reserves, freshness requirements, and derating factors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetPolicy {
    pub max_cpu_fraction_percent: u8,
    pub max_memory_fraction_percent: u8,
    pub max_storage_fraction_percent: u8,
    pub memory_reserve_bytes: u64,
    pub storage_reserve_bytes: u64,
    pub max_measurement_age_seconds: u64,
    pub minimum_confidence: Confidence,
    pub battery_low_threshold_percent: u8,
    pub battery_critical_threshold_percent: u8,
    pub low_battery_cpu_factor_percent: u8,
    pub critical_battery_cpu_factor_percent: u8,
    pub unknown_battery_cpu_factor_percent: u8,
    pub thermal_cpu_factors: ThermalCpuFactors,
}

/// Independent resource limits derived from a single device snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceBudget {
    pub cpu_cores: BudgetedAmount<u32>,
    pub memory_bytes: BudgetedAmount<u64>,
    pub storage_bytes: BudgetedAmount<u64>,
    pub bandwidth_bps: BudgetedAmount<u64>,
}

/// Invalid caller policy or impossible platform measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceBudgetError {
    InvalidPercentage { field: &'static str, value: u8 },
    InvalidBatteryThresholds,
    InvalidBatteryPercentage(u8),
}

impl std::fmt::Display for ResourceBudgetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPercentage { field, value } => {
                write!(formatter, "{field} percentage {value} exceeds 100")
            }
            Self::InvalidBatteryThresholds => formatter.write_str(
                "critical battery threshold must be less than or equal to low threshold",
            ),
            Self::InvalidBatteryPercentage(value) => {
                write!(formatter, "battery percentage {value} exceeds 100")
            }
        }
    }
}

impl std::error::Error for ResourceBudgetError {}

impl BudgetPolicy {
    fn validate(&self) -> Result<(), ResourceBudgetError> {
        for (field, value) in [
            ("max_cpu_fraction", self.max_cpu_fraction_percent),
            ("max_memory_fraction", self.max_memory_fraction_percent),
            ("max_storage_fraction", self.max_storage_fraction_percent),
            (
                "low_battery_cpu_factor",
                self.low_battery_cpu_factor_percent,
            ),
            (
                "critical_battery_cpu_factor",
                self.critical_battery_cpu_factor_percent,
            ),
            (
                "unknown_battery_cpu_factor",
                self.unknown_battery_cpu_factor_percent,
            ),
            ("thermal_unknown", self.thermal_cpu_factors.unknown),
            ("thermal_nominal", self.thermal_cpu_factors.nominal),
            ("thermal_fair", self.thermal_cpu_factors.fair),
            ("thermal_serious", self.thermal_cpu_factors.serious),
            ("thermal_critical", self.thermal_cpu_factors.critical),
            ("thermal_shutdown", self.thermal_cpu_factors.shutdown),
        ] {
            if value > 100 {
                return Err(ResourceBudgetError::InvalidPercentage { field, value });
            }
        }
        if self.battery_critical_threshold_percent > self.battery_low_threshold_percent
            || self.battery_low_threshold_percent > 100
        {
            return Err(ResourceBudgetError::InvalidBatteryThresholds);
        }
        Ok(())
    }
}

fn limitation_for(status: MeasurementStatus) -> BudgetLimitation {
    match status {
        MeasurementStatus::Missing => BudgetLimitation::MissingMeasurement,
        MeasurementStatus::FutureDated => BudgetLimitation::FutureDatedMeasurement,
        MeasurementStatus::Stale { .. } => BudgetLimitation::StaleMeasurement,
        MeasurementStatus::LowConfidence => BudgetLimitation::LowConfidence,
        MeasurementStatus::Fresh { .. } => unreachable!("fresh observations have no limitation"),
    }
}

fn read<T: Copy>(
    measurement: &Measurement<T>,
    policy: &BudgetPolicy,
    now: u64,
) -> Result<T, BudgetLimitation> {
    let status = measurement.status(
        now,
        policy.max_measurement_age_seconds,
        policy.minimum_confidence,
    );
    if !matches!(status, MeasurementStatus::Fresh { .. }) {
        return Err(limitation_for(status));
    }
    measurement
        .value()
        .copied()
        .ok_or(BudgetLimitation::MissingMeasurement)
}

fn push_once(reasons: &mut Vec<BudgetLimitation>, reason: BudgetLimitation) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn ratio_floor(value: u128, factor: u8) -> u128 {
    value * u128::from(factor) / 100
}

fn calculate_cpu_budget(
    profile: &CapabilityProfile,
    policy: &BudgetPolicy,
    now: u64,
) -> Result<BudgetedAmount<u32>, ResourceBudgetError> {
    let cores = match read(&profile.cpu_cores, policy, now) {
        Ok(value) => value,
        Err(reason) => return Ok(BudgetedAmount::unavailable(reason)),
    };
    let mut limitations = Vec::new();
    let mut amount = ratio_floor(u128::from(cores), policy.max_cpu_fraction_percent);
    if policy.max_cpu_fraction_percent < 100 {
        push_once(&mut limitations, BudgetLimitation::PolicyCap);
    }
    let battery = read(&profile.battery_percent, policy, now);
    let charging = read(&profile.is_charging, policy, now);
    let battery_factor = match (battery, charging) {
        (Ok(value), _) if value > 100 => {
            return Err(ResourceBudgetError::InvalidBatteryPercentage(value));
        }
        (Ok(_), Ok(true)) => 100,
        (Ok(value), _) if value <= policy.battery_critical_threshold_percent => {
            policy.critical_battery_cpu_factor_percent
        }
        (Ok(value), _) if value <= policy.battery_low_threshold_percent => {
            policy.low_battery_cpu_factor_percent
        }
        (Ok(_), _) => 100,
        _ => {
            push_once(
                &mut limitations,
                BudgetLimitation::UnknownBatteryConservative,
            );
            policy.unknown_battery_cpu_factor_percent
        }
    };
    if battery_factor < 100 {
        amount = ratio_floor(amount, battery_factor);
        push_once(&mut limitations, BudgetLimitation::BatteryDerating);
    }
    let thermal = read(&profile.thermal_state, policy, now);
    let thermal_factor = match thermal {
        Ok(ThermalState::Unknown) | Err(_) => {
            push_once(
                &mut limitations,
                BudgetLimitation::UnknownThermalConservative,
            );
            policy.thermal_cpu_factors.unknown
        }
        Ok(ThermalState::Nominal) => policy.thermal_cpu_factors.nominal,
        Ok(ThermalState::Fair) => policy.thermal_cpu_factors.fair,
        Ok(ThermalState::Serious) => policy.thermal_cpu_factors.serious,
        Ok(ThermalState::Critical) => policy.thermal_cpu_factors.critical,
        Ok(ThermalState::Shutdown) => policy.thermal_cpu_factors.shutdown,
    };
    if thermal_factor < 100 {
        amount = ratio_floor(amount, thermal_factor);
        push_once(&mut limitations, BudgetLimitation::ThermalDerating);
    }
    Ok(BudgetedAmount {
        amount: Some(u32::try_from(amount).unwrap_or(u32::MAX)),
        limitations,
    })
}

fn calculate_memory_budget(
    profile: &CapabilityProfile,
    policy: &BudgetPolicy,
    now: u64,
) -> BudgetedAmount<u64> {
    match (
        read(&profile.total_memory_bytes, policy, now),
        read(&profile.available_memory_bytes, policy, now),
    ) {
        (Ok(total), Ok(available)) => {
            let mut limitations = Vec::new();
            let after_reserve = available.saturating_sub(policy.memory_reserve_bytes);
            if policy.memory_reserve_bytes > 0 {
                push_once(&mut limitations, BudgetLimitation::ReserveApplied);
            }
            let cap = u64::try_from(ratio_floor(
                u128::from(total),
                policy.max_memory_fraction_percent,
            ))
            .unwrap_or(u64::MAX);
            let amount = after_reserve.min(cap);
            if cap < after_reserve {
                push_once(&mut limitations, BudgetLimitation::PolicyCap);
            }
            BudgetedAmount {
                amount: Some(amount),
                limitations,
            }
        }
        (Err(reason), _) | (_, Err(reason)) => BudgetedAmount::unavailable(reason),
    }
}

fn calculate_storage_budget(
    profile: &CapabilityProfile,
    policy: &BudgetPolicy,
    now: u64,
) -> BudgetedAmount<u64> {
    match read(&profile.available_storage_bytes, policy, now) {
        Err(reason) => BudgetedAmount::unavailable(reason),
        Ok(available) => {
            let after_reserve = available.saturating_sub(policy.storage_reserve_bytes);
            let cap = u64::try_from(ratio_floor(
                u128::from(available),
                policy.max_storage_fraction_percent,
            ))
            .unwrap_or(u64::MAX);
            let mut limitations = Vec::new();
            if policy.storage_reserve_bytes > 0 {
                push_once(&mut limitations, BudgetLimitation::ReserveApplied);
            }
            if cap < after_reserve {
                push_once(&mut limitations, BudgetLimitation::PolicyCap);
            }
            BudgetedAmount {
                amount: Some(after_reserve.min(cap)),
                limitations,
            }
        }
    }
}

/// Calculate deterministic CPU, memory, storage, and bandwidth budgets.
///
/// # Errors
///
/// Returns an error when a percentage is outside 0–100 or a battery observation
/// reports a value above 100.
pub fn calculate_budget(
    profile: &CapabilityProfile,
    policy: &BudgetPolicy,
    now: u64,
) -> Result<ResourceBudget, ResourceBudgetError> {
    policy.validate()?;

    let cpu_cores = calculate_cpu_budget(profile, policy, now)?;
    let memory_bytes = calculate_memory_budget(profile, policy, now);
    let storage_bytes = calculate_storage_budget(profile, policy, now);
    let bandwidth_bps = match read(&profile.bandwidth_estimate_bps, policy, now) {
        Ok(amount) => BudgetedAmount {
            amount: Some(amount),
            limitations: Vec::new(),
        },
        Err(reason) => BudgetedAmount::unavailable(reason),
    };

    Ok(ResourceBudget {
        cpu_cores,
        memory_bytes,
        storage_bytes,
        bandwidth_bps,
    })
}

#[cfg(test)]
mod tests {
    // All profiles below are synthetic test fixtures; no device adapter is run.
    use super::{
        BudgetLimitation, BudgetPolicy, BudgetedAmount, ResourceBudgetError, ThermalCpuFactors,
        calculate_budget,
    };
    use crate::profile::{
        Architecture, CapabilityProfile, Confidence, Measurement, NetworkType, ThermalState,
    };

    const NOW: u64 = 100;

    fn observed<T>(value: T) -> Measurement<T> {
        Measurement::observed(value, NOW, Confidence::High)
    }

    fn profile() -> CapabilityProfile {
        CapabilityProfile {
            architecture: observed(Architecture::Arm64),
            cpu_cores: observed(8_u32),
            total_memory_bytes: observed(16_000_000_000_u64),
            available_memory_bytes: observed(12_000_000_000_u64),
            available_storage_bytes: observed(100_000_000_000_u64),
            battery_percent: observed(80_u8),
            is_charging: observed(true),
            thermal_state: observed(ThermalState::Nominal),
            network_type: observed(NetworkType::Wifi),
            bandwidth_estimate_bps: observed(50_000_000_u64),
        }
    }

    fn policy() -> BudgetPolicy {
        BudgetPolicy {
            max_cpu_fraction_percent: 80,
            max_memory_fraction_percent: 50,
            max_storage_fraction_percent: 20,
            memory_reserve_bytes: 2_000_000_000,
            storage_reserve_bytes: 10_000_000_000,
            max_measurement_age_seconds: 30,
            minimum_confidence: Confidence::Medium,
            battery_low_threshold_percent: 30,
            battery_critical_threshold_percent: 15,
            low_battery_cpu_factor_percent: 70,
            critical_battery_cpu_factor_percent: 40,
            unknown_battery_cpu_factor_percent: 50,
            thermal_cpu_factors: ThermalCpuFactors {
                unknown: 50,
                nominal: 100,
                fair: 90,
                serious: 60,
                critical: 25,
                shutdown: 0,
            },
        }
    }

    #[test]
    fn reserves_and_policy_caps_bound_the_budget() {
        let result = calculate_budget(&profile(), &policy(), NOW).unwrap();

        assert_eq!(result.cpu_cores.amount, Some(6));
        assert_eq!(result.memory_bytes.amount, Some(8_000_000_000));
        assert_eq!(result.storage_bytes.amount, Some(20_000_000_000));
        assert_eq!(result.bandwidth_bps.amount, Some(50_000_000));
        assert!(
            result
                .memory_bytes
                .limitations
                .contains(&BudgetLimitation::ReserveApplied)
        );
        assert!(
            result
                .storage_bytes
                .limitations
                .contains(&BudgetLimitation::PolicyCap)
        );
    }

    #[test]
    fn stale_cpu_measurement_makes_only_cpu_budget_unavailable() {
        let mut device = profile();
        device.cpu_cores = Measurement::observed(8, NOW - 31, Confidence::High);

        let result = calculate_budget(&device, &policy(), NOW).unwrap();

        assert_eq!(
            result.cpu_cores,
            BudgetedAmount::unavailable(BudgetLimitation::StaleMeasurement)
        );
        assert_eq!(result.memory_bytes.amount, Some(8_000_000_000));
    }

    #[test]
    fn low_confidence_memory_is_not_used_as_a_budget() {
        let mut device = profile();
        device.available_memory_bytes =
            Measurement::observed(12_000_000_000_u64, NOW, Confidence::Low);

        let result = calculate_budget(&device, &policy(), NOW).unwrap();

        assert_eq!(
            result.memory_bytes,
            BudgetedAmount::unavailable(BudgetLimitation::LowConfidence)
        );
    }

    #[test]
    fn thermal_and_low_battery_factors_derate_cpu_deterministically() {
        let mut device = profile();
        device.thermal_state = Measurement::observed(ThermalState::Serious, NOW, Confidence::High);
        device.is_charging = Measurement::observed(false, NOW, Confidence::High);
        device.battery_percent = Measurement::observed(10, NOW, Confidence::High);

        let result = calculate_budget(&device, &policy(), NOW).unwrap();

        // 8 cores × 80% policy cap × 40% critical-battery factor × 60% serious-thermal factor.
        assert_eq!(result.cpu_cores.amount, Some(1));
        assert!(
            result
                .cpu_cores
                .limitations
                .contains(&BudgetLimitation::BatteryDerating)
        );
        assert!(
            result
                .cpu_cores
                .limitations
                .contains(&BudgetLimitation::ThermalDerating)
        );
    }

    #[test]
    fn measured_zero_is_available_zero_not_unknown() {
        let mut device = profile();
        device.cpu_cores = Measurement::observed(0, NOW, Confidence::High);
        device.available_memory_bytes = Measurement::observed(0, NOW, Confidence::High);

        let result = calculate_budget(&device, &policy(), NOW).unwrap();

        assert_eq!(result.cpu_cores.amount, Some(0));
        assert_eq!(result.memory_bytes.amount, Some(0));
    }

    #[test]
    fn arithmetic_saturates_instead_of_overflowing() {
        let mut device = profile();
        device.total_memory_bytes = Measurement::observed(u64::MAX, NOW, Confidence::High);
        device.available_memory_bytes = Measurement::observed(u64::MAX, NOW, Confidence::High);
        let mut limits = policy();
        limits.memory_reserve_bytes = u64::MAX - 1;

        let result = calculate_budget(&device, &limits, NOW).unwrap();

        assert_eq!(result.memory_bytes.amount, Some(1));
    }

    #[test]
    fn invalid_battery_percentage_is_rejected() {
        let mut device = profile();
        device.battery_percent = Measurement::observed(101, NOW, Confidence::High);

        assert_eq!(
            calculate_budget(&device, &policy(), NOW),
            Err(ResourceBudgetError::InvalidBatteryPercentage(101))
        );
    }
}
