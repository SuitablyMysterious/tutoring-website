//! Shared DTOs used by both the frontend islands and the backend API.

use serde::{Deserialize, Serialize};

/// Payload for `POST /api/enquiry`.
///
/// `website` is a honeypot field: rendered invisibly in the form and left
/// empty by humans. Bots that fill it get a fake-success response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnquiryRequest {
    pub name: String,
    pub email: String,
    pub message: String,
    #[serde(default)]
    pub website: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn website_defaults_to_empty_when_missing() {
        let req: EnquiryRequest =
            serde_json::from_str(r#"{"name":"a","email":"a@b.example","message":"hi"}"#).unwrap();
        assert_eq!(req.website, "");
    }

    #[test]
    fn round_trips_through_json() {
        let req = EnquiryRequest {
            name: "Jane".into(),
            email: "jane@example.com".into(),
            message: "Hello".into(),
            website: "".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let back: EnquiryRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, req.name);
        assert_eq!(back.email, req.email);
        assert_eq!(back.message, req.message);
        assert_eq!(back.website, req.website);
    }
}
