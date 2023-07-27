/**
 * Data model for server responses
 */
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub res: String,
    pub status: u32
}