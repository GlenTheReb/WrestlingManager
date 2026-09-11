//! Validated domain values and the Rust-owned IPC contract.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use ts_rs::TS;

pub mod discovery;
pub mod game;
pub mod identity;
pub mod match_rules;
pub mod ratings;
pub mod relationships;
pub mod traits;

pub const CURRENT_ENGINE_VERSION: &str = "0.6.0";
pub const CURRENT_SCHEMA_VERSION: u32 = 7;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaveId(String);

impl SaveId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let bytes = value.as_bytes();

        if bytes.is_empty()
            || bytes.len() > 48
            || !bytes[0].is_ascii_alphanumeric()
            || bytes
                .iter()
                .any(|byte| !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-'))
        {
            return Err(ValidationError::InvalidSaveId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for SaveId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for SaveId {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seed(u64);

impl Seed {
    pub fn parse(value: &str) -> Result<Self, ValidationError> {
        if value.is_empty()
            || value.bytes().any(|byte| !byte.is_ascii_digit())
            || (value.len() > 1 && value.starts_with('0'))
        {
            return Err(ValidationError::InvalidSeed);
        }

        value
            .parse::<u64>()
            .map(Self)
            .map_err(|_| ValidationError::InvalidSeed)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub fn canonical(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for Seed {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    #[error(
        "save ID must be 1-48 lower-case ASCII letters, digits, or hyphens and start with a letter or digit"
    )]
    InvalidSaveId,
    #[error("seed must be the canonical decimal representation of an unsigned 64-bit integer")]
    InvalidSeed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewGameSpec {
    pub save_id: SaveId,
    pub seed: Seed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameRequest {
    pub save_id: String,
    pub seed: String,
}

impl CreateGameRequest {
    pub fn validate(self) -> Result<NewGameSpec, ValidationError> {
        Ok(NewGameSpec {
            save_id: SaveId::parse(self.save_id)?,
            seed: Seed::parse(&self.seed)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct PromotionOverview {
    pub save_id: String,
    pub promotion_id: String,
    pub name: String,
    pub initials: String,
    pub region: String,
    pub founded_on: String,
    pub current_date: String,
    #[ts(type = "number")]
    pub cash_pence: i64,
    pub seed: String,
    pub engine_version: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveSummary {
    pub save_id: String,
    pub promotion_name: String,
    pub current_date: String,
    pub seed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct IpcError {
    pub code: String,
    pub message: String,
}
