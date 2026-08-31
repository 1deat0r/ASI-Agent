use serde::{Deserialize, Serialize};

use crate::domain::EffectClass;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub policy_version: String,
    pub reason_code: String,
    pub explanation: String,
    pub requested_effect: EffectClass,
    pub maximum_effect: EffectClass,
}

/// Immutable v0.1 policy boundary.
#[derive(Clone, Debug)]
pub struct PolicyEngine {
    version: &'static str,
    maximum_effect: EffectClass,
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self {
            version: "asi.policy.v0.1",
            maximum_effect: EffectClass::ReadOnly,
        }
    }
}

impl PolicyEngine {
    #[must_use]
    pub fn evaluate(&self, effect: EffectClass) -> PolicyDecision {
        if effect <= self.maximum_effect {
            PolicyDecision {
                allowed: true,
                policy_version: self.version.to_owned(),
                reason_code: "within-v0.1-authority".to_owned(),
                explanation: format!(
                    "{effect} is within the v0.1 maximum authority of {}",
                    self.maximum_effect
                ),
                requested_effect: effect,
                maximum_effect: self.maximum_effect,
            }
        } else {
            PolicyDecision {
                allowed: false,
                policy_version: self.version.to_owned(),
                reason_code: "effect-class-denied".to_owned(),
                explanation: format!(
                    "{effect} exceeds the v0.1 maximum authority of {}",
                    self.maximum_effect
                ),
                requested_effect: effect,
                maximum_effect: self.maximum_effect,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v01_fails_closed_above_read_only() {
        let policy = PolicyEngine::default();
        assert!(policy.evaluate(EffectClass::None).allowed);
        assert!(policy.evaluate(EffectClass::ReadOnly).allowed);
        assert!(!policy.evaluate(EffectClass::WorkspaceWrite).allowed);
        assert!(!policy.evaluate(EffectClass::External).allowed);
    }
}
