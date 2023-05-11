use crate::types::{CanisterStatus, Signer, Version};

impl Signer {
    pub async fn version(&self) -> Result<Version, String> {
        if let Some(signer_id) = self.signer_id {
            let (version,): (Version,) = ic_cdk::call(signer_id, "version", ())
                .await
                .map_err(|(_, message)| format!("Failed to get version: {}!", message))?;

            Ok(version)
        } else {
            Err("Signer ID is not set!".to_string())
        }
    }

    pub async fn status(&self) -> Result<CanisterStatus, String> {
        if let Some(signer_id) = self.signer_id {
            let (canister_status,): (CanisterStatus,) = ic_cdk::call(signer_id, "status", ())
                .await
                .map_err(|(_, message)| format!("Failed to get status: {}!", message))?;

            Ok(canister_status)
        } else {
            Err("Signer ID is not set!".to_string())
        }
    }
}
