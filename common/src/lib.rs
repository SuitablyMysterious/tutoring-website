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
