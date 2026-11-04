//! Platform-independent policy and lifecycle primitives for Logos Mobile.

pub mod profile;

pub use profile::{
    Architecture, CapabilityProfile, Confidence, Measurement, MeasurementStatus, NetworkType,
    Profiler, ThermalState,
};
