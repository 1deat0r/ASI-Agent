use anyhow::{Result, bail};

use crate::domain::EffectClass;
use crate::harness::{AdapterKind, DetectedHarness};

#[derive(Clone, Debug, Default)]
pub struct HarnessRegistry;

impl HarnessRegistry {
    #[must_use]
    pub fn discover(&self) -> Vec<DetectedHarness> {
        AdapterKind::ALL
            .into_iter()
            .map(AdapterKind::detect)
            .collect()
    }

    pub fn resolve(&self, requested: &str, effect: EffectClass) -> Result<AdapterKind> {
        if requested == "auto" {
            return self.select(effect);
        }

        let adapter = AdapterKind::ALL
            .into_iter()
            .find(|candidate| candidate.id() == requested)
            .ok_or_else(|| anyhow::anyhow!("unknown harness {requested}"))?;
        let detected = adapter.detect();
        if !detected.installed {
            bail!("harness {requested} is not installed");
        }
        Ok(adapter)
    }

    fn select(&self, effect: EffectClass) -> Result<AdapterKind> {
        let priority: &[AdapterKind] = match effect {
            EffectClass::None => &[
                AdapterKind::Pi,
                AdapterKind::OhMyPi,
                AdapterKind::Claude,
                AdapterKind::ConstructFixture,
            ],
            EffectClass::ReadOnly => &[
                AdapterKind::Codex,
                AdapterKind::Pi,
                AdapterKind::OhMyPi,
                AdapterKind::Claude,
            ],
            EffectClass::WorkspaceWrite | EffectClass::External => &[],
        };

        priority
            .iter()
            .copied()
            .find(|adapter| adapter.detect().installed)
            .ok_or_else(|| anyhow::anyhow!("no installed harness supports effect {effect}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_is_always_discoverable_but_never_hidden_authority() {
        let registry = HarnessRegistry;
        let fixture = registry
            .discover()
            .into_iter()
            .find(|harness| harness.descriptor.id == "construct-fixture")
            .expect("fixture should be present");
        assert!(fixture.installed);
        assert_eq!(fixture.descriptor.authority_owner, "asi-agent");
    }
}
