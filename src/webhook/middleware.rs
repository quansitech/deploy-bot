//! Webhook validation middleware

use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Validate GitHub webhook signature
pub fn validate_github_signature(
    payload: &[u8],
    signature: &str,
    secret: &str,
) -> Result<(), crate::error::AppError> {
    type HmacSha256 = Hmac<Sha256>;

    let expected_signature = format!("sha256={}", {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|_| crate::error::AppError::WebhookValidation("Invalid secret".to_string()))?;
        mac.update(payload);
        hex::encode(mac.finalize().into_bytes())
    });

    if signature != expected_signature {
        return Err(crate::error::AppError::WebhookValidation("Invalid signature".to_string()));
    }

    Ok(())
}

/// Validate GitLab webhook token
pub fn validate_gitlab_token(token: &str, secret: &str) -> Result<(), crate::error::AppError> {
    if token != secret {
        return Err(crate::error::AppError::WebhookValidation("Invalid token".to_string()));
    }
    Ok(())
}

/// Validate Codeup webhook token
pub fn validate_codeup_token(token: &str, secret: &str) -> Result<(), crate::error::AppError> {
    if token != secret {
        return Err(crate::error::AppError::WebhookValidation("Invalid token".to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_github_signature_valid() {
        let payload = b"{\"action\":\"push\"}";
        let secret = "test-secret";

        // Generate valid signature
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        let result = validate_github_signature(payload, &signature, secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_github_signature_invalid() {
        let payload = b"{\"action\":\"push\"}";
        let secret = "test-secret";
        let invalid_signature = "sha256=invalidsignature";

        let result = validate_github_signature(payload, invalid_signature, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_github_signature_empty_secret() {
        let payload = b"{\"action\":\"push\"}";

        let result = validate_github_signature(payload, "any", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_gitlab_token_valid() {
        let result = validate_gitlab_token("valid-token", "valid-token");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_gitlab_token_invalid() {
        let result = validate_gitlab_token("wrong-token", "correct-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_gitlab_token_empty() {
        let result = validate_gitlab_token("", "");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_codeup_token_valid() {
        let result = validate_codeup_token("valid-token", "valid-token");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_codeup_token_invalid() {
        let result = validate_codeup_token("wrong-token", "correct-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_codeup_token_empty() {
        let result = validate_codeup_token("", "");
        assert!(result.is_ok());
    }
}
