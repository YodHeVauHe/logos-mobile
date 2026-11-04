//! Role assessment against caller supplied, provenance-bearing requirements.

use crate::budget::ResourceBudget;

/// Deployment profile proposed by the engineering specification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NodeRole {
    BlockchainFullNode,
    StorageProvider,
    DeliveryProvider,
    NetworkParticipant,
    MultiServiceNode,
    ConsensusEligible,
}

/// Provenance for an external module or protocol requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Evidence {
    Unverified,
    Verified { reference: String },
}

impl Evidence {
    fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { reference } if !reference.trim().is_empty())
    }
}

/// Resource minima supplied by a verified module contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleRequirement {
    pub role: NodeRole,
    pub minimum_cpu_cores: u32,
    pub minimum_memory_bytes: u64,
    pub minimum_storage_bytes: u64,
    pub minimum_bandwidth_bps: u64,
    pub module_evidence: Evidence,
    /// Independent protocol evidence; required only for consensus eligibility.
    pub protocol_eligibility_evidence: Option<Evidence>,
}

/// Requirements for the complete requested role combination.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadRequirements {
    pub roles: Vec<RoleRequirement>,
}

/// Explainable outcome for one requested role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleAssessment {
    pub role: NodeRole,
    pub status: RoleStatus,
    pub reasons: Vec<RoleConstraint>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleStatus {
    Recommended,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoleConstraint {
    UnverifiedModuleRequirements,
    MissingProtocolEligibilityEvidence,
    UnverifiedProtocolEligibilityEvidence,
    CpuBudgetUnavailable,
    MemoryBudgetUnavailable,
    StorageBudgetUnavailable,
    BandwidthBudgetUnavailable,
    CpuBudgetInsufficient,
    MemoryBudgetInsufficient,
    StorageBudgetInsufficient,
    BandwidthBudgetInsufficient,
    CombinedWorkloadExceedsBudget,
}

/// Assess roles in enum order using verified requirements and the supplied safe budget.
///
/// Requirements for multiple roles are treated as additive. The function makes no
/// assumptions about module defaults, network behavior, or protocol thresholds.
#[must_use]
pub fn assess_roles(
    budget: &ResourceBudget,
    requirements: &[RoleRequirement],
) -> Vec<RoleAssessment> {
    let mut ordered = requirements.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|requirement| requirement.role);
    let mut used_cpu = 0_u32;
    let mut used_memory = 0_u64;
    let mut used_storage = 0_u64;
    let mut used_bandwidth = 0_u64;
    let mut assessments = Vec::with_capacity(ordered.len());

    for requirement in ordered {
        let mut reasons = Vec::new();
        if !requirement.module_evidence.is_verified() {
            reasons.push(RoleConstraint::UnverifiedModuleRequirements);
        }
        if requirement.role == NodeRole::ConsensusEligible {
            match requirement.protocol_eligibility_evidence.as_ref() {
                None => reasons.push(RoleConstraint::MissingProtocolEligibilityEvidence),
                Some(evidence) if !evidence.is_verified() => {
                    reasons.push(RoleConstraint::UnverifiedProtocolEligibilityEvidence);
                }
                Some(_) => {}
            }
        }

        let cpu = budget.cpu_cores.amount;
        let memory = budget.memory_bytes.amount;
        let storage = budget.storage_bytes.amount;
        let bandwidth = budget.bandwidth_bps.amount;
        if cpu.is_none() {
            reasons.push(RoleConstraint::CpuBudgetUnavailable);
        }
        if memory.is_none() {
            reasons.push(RoleConstraint::MemoryBudgetUnavailable);
        }
        if storage.is_none() {
            reasons.push(RoleConstraint::StorageBudgetUnavailable);
        }
        if bandwidth.is_none() {
            reasons.push(RoleConstraint::BandwidthBudgetUnavailable);
        }

        let requested_cpu = used_cpu.saturating_add(requirement.minimum_cpu_cores);
        let requested_memory = used_memory.saturating_add(requirement.minimum_memory_bytes);
        let requested_storage = used_storage.saturating_add(requirement.minimum_storage_bytes);
        let requested_bandwidth = used_bandwidth.saturating_add(requirement.minimum_bandwidth_bps);
        let before_count = reasons.len();
        if cpu.is_some_and(|available| requested_cpu > available) {
            reasons.push(RoleConstraint::CpuBudgetInsufficient);
        }
        if memory.is_some_and(|available| requested_memory > available) {
            reasons.push(RoleConstraint::MemoryBudgetInsufficient);
        }
        if storage.is_some_and(|available| requested_storage > available) {
            reasons.push(RoleConstraint::StorageBudgetInsufficient);
        }
        if bandwidth.is_some_and(|available| requested_bandwidth > available) {
            reasons.push(RoleConstraint::BandwidthBudgetInsufficient);
        }
        if reasons.len() > before_count {
            reasons.push(RoleConstraint::CombinedWorkloadExceedsBudget);
        } else if reasons.is_empty() {
            used_cpu = requested_cpu;
            used_memory = requested_memory;
            used_storage = requested_storage;
            used_bandwidth = requested_bandwidth;
        }
        let status = if reasons.is_empty() {
            RoleStatus::Recommended
        } else {
            RoleStatus::Unavailable
        };
        assessments.push(RoleAssessment {
            role: requirement.role,
            status,
            reasons,
        });
    }
    assessments
}

#[cfg(test)]
mod tests {
    use super::{
        Evidence, NodeRole, RoleRequirement, RoleStatus, WorkloadRequirements, assess_roles,
    };
    use crate::budget::{BudgetedAmount, ResourceBudget};

    fn budget() -> ResourceBudget {
        ResourceBudget {
            cpu_cores: BudgetedAmount {
                amount: Some(8),
                limitations: vec![],
            },
            memory_bytes: BudgetedAmount {
                amount: Some(8_000),
                limitations: vec![],
            },
            storage_bytes: BudgetedAmount {
                amount: Some(8_000),
                limitations: vec![],
            },
            bandwidth_bps: BudgetedAmount {
                amount: Some(8_000),
                limitations: vec![],
            },
        }
    }

    fn requirement(role: NodeRole, evidence: Evidence) -> RoleRequirement {
        RoleRequirement {
            role,
            minimum_cpu_cores: 2,
            minimum_memory_bytes: 1_000,
            minimum_storage_bytes: 1_000,
            minimum_bandwidth_bps: 1_000,
            module_evidence: evidence,
            protocol_eligibility_evidence: None,
        }
    }

    fn verified() -> Evidence {
        Evidence::Verified {
            reference: "module-docs:v1".into(),
        }
    }

    #[test]
    fn verified_requirements_fit_budget_and_preserve_deterministic_order() {
        let requirements = vec![
            requirement(NodeRole::StorageProvider, verified()),
            requirement(NodeRole::BlockchainFullNode, verified()),
        ];
        let assessment = assess_roles(&budget(), &requirements);
        assert_eq!(
            assessment.iter().map(|item| item.role).collect::<Vec<_>>(),
            vec![NodeRole::BlockchainFullNode, NodeRole::StorageProvider]
        );
        assert!(
            assessment
                .iter()
                .all(|item| item.status == RoleStatus::Recommended)
        );
    }

    #[test]
    fn unverified_requirements_never_become_recommendations() {
        let assessment = assess_roles(
            &budget(),
            &[requirement(
                NodeRole::DeliveryProvider,
                Evidence::Unverified,
            )],
        );
        assert_eq!(assessment[0].status, RoleStatus::Unavailable);
    }

    #[test]
    fn missing_budget_and_shortfalls_are_explained() {
        let mut constrained = budget();
        constrained.cpu_cores.amount = None;
        constrained.memory_bytes.amount = Some(0);
        let assessment = assess_roles(
            &constrained,
            &[requirement(NodeRole::NetworkParticipant, verified())],
        );
        assert_eq!(assessment[0].status, RoleStatus::Unavailable);
        assert!(
            assessment[0]
                .reasons
                .contains(&super::RoleConstraint::CpuBudgetUnavailable)
        );
        assert!(
            assessment[0]
                .reasons
                .contains(&super::RoleConstraint::MemoryBudgetInsufficient)
        );
    }

    #[test]
    fn consensus_requires_independent_protocol_eligibility_evidence() {
        let requirement = requirement(NodeRole::ConsensusEligible, verified());
        let assessment = assess_roles(&budget(), &[requirement]);
        assert_eq!(assessment[0].status, RoleStatus::Unavailable);
    }

    #[test]
    fn consensus_can_only_be_recommended_with_protocol_evidence() {
        let mut requirement = requirement(NodeRole::ConsensusEligible, verified());
        requirement.protocol_eligibility_evidence = Some(Evidence::Verified {
            reference: "protocol-rule:eligibility-v2".into(),
        });
        let assessment = assess_roles(&budget(), &[requirement]);
        assert_eq!(assessment[0].status, RoleStatus::Recommended);
    }

    #[test]
    fn multiple_roles_are_assessed_against_the_combined_workload() {
        let requirements = WorkloadRequirements {
            roles: vec![
                requirement(NodeRole::StorageProvider, verified()),
                requirement(NodeRole::DeliveryProvider, verified()),
            ],
        };
        let assessment = assess_roles(&budget(), &requirements.roles);
        assert_eq!(
            assessment
                .iter()
                .filter(|item| item.status == RoleStatus::Recommended)
                .count(),
            2
        );
    }
}
