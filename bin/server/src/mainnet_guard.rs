use anyhow::{bail, Result};
use octo_wallet_core::StellarNetwork;

pub fn ensure_webhook_override_is_safe(
    network: StellarNetwork,
    allow_local_webhooks: bool,
) -> Result<()> {
    if network == StellarNetwork::Public && allow_local_webhooks {
        bail!("OCTO_ALLOW_LOCAL_WEBHOOKS must not be set when NETWORK=mainnet");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_local_webhook_override_on_mainnet() {
        assert!(ensure_webhook_override_is_safe(StellarNetwork::Public, true).is_err());
    }

    #[test]
    fn allows_local_webhook_override_on_testnet() {
        assert!(ensure_webhook_override_is_safe(StellarNetwork::Testnet, true).is_ok());
    }
}
