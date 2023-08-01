/**
 * Data model for server responses
 */

use std::os::fd::RawFd;

#[derive(Clone, serde::Serialize)]
pub struct InstancePayload {
    pub args: Vec<String>,
    pub cwd: String,
}

#[derive(Clone, serde::Serialize)]
pub struct PtyPayload {
    pub res: String,
    pub fd: RawFd,
    pub status: u32
}