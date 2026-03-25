#![no_std]
use soroban_sdk::{Env, String, Vec, contracterror};

#[allow(missing_docs)]

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StateSizeError {
    ContributorLimitExceeded = 1,
    RoadmapLimitExceeded = 2,
    StretchGoalLimitExceeded = 3,
    StringTooLong = 4,
}

// ── Limits ───────────────────────────────────────────────────────────────────

pub const MAX_CONTRIBUTORS: u32 = 128;
pub const MAX_PLEDGERS: u32 = 128;
pub const MAX_ROADMAP_ITEMS: u32 = 32;
pub const MAX_STRETCH_GOALS: u32 = 32;
pub const MAX_TITLE_LENGTH: u32 = 128;
pub const MAX_DESCRIPTION_LENGTH: u32 = 2048;
pub const MAX_SOCIAL_LINKS_LENGTH: u32 = 512;
pub const MAX_BONUS_GOAL_DESCRIPTION_LENGTH: u32 = 280;
pub const MAX_ROADMAP_DESCRIPTION_LENGTH: u32 = 280;
pub const MAX_METADATA_TOTAL_LENGTH: u32 = 2304;
pub const MAX_STRING_LEN: u32 = 256;

// ── Validation helpers ────────────────────────────────────────────────────────

pub fn validate_title(title: &String) -> Result<(), &'static str> {
    if title.len() > MAX_TITLE_LENGTH {
        return Err("title exceeds MAX_TITLE_LENGTH bytes");
    }
    Ok(())
}

pub fn validate_description(description: &String) -> Result<(), &'static str> {
    if description.len() > MAX_DESCRIPTION_LENGTH {
        return Err("description exceeds MAX_DESCRIPTION_LENGTH bytes");
    }
    Ok(())
}

pub fn validate_metadata_total_length(
    title_len: u32,
    description_len: u32,
    socials_len: u32,
) -> Result<(), &'static str> {
    let sum = title_len
        .checked_add(description_len)
        .and_then(|v| v.checked_add(socials_len))
        .ok_or("metadata exceeds MAX_METADATA_TOTAL_LENGTH bytes")?;
    if sum > MAX_METADATA_TOTAL_LENGTH {
        return Err("metadata exceeds MAX_METADATA_TOTAL_LENGTH bytes");
    }
    Ok(())
}

pub fn validate_contributor_capacity(len: u32) -> Result<(), &'static str> {
    if len >= MAX_CONTRIBUTORS {
        return Err("contributors exceed MAX_CONTRIBUTORS");
    }
    Ok(())
}

pub fn validate_pledger_capacity(len: u32) -> Result<(), &'static str> {
    if len >= MAX_PLEDGERS {
        return Err("pledgers exceed MAX_PLEDGERS");
    }
    Ok(())
}

pub fn check_contributor_limit(env: &Env) -> Result<(), StateSizeError> {
    let contributors: Vec<soroban_sdk::Address> = env
        .storage()
        .persistent()
        .get(&crate::DataKey::Contributors)
        .unwrap_or_else(|| Vec::new(env));

    if contributors.len() >= MAX_CONTRIBUTORS {
        return Err(StateSizeError::ContributorLimitExceeded);
    }
    Ok(())
}

pub fn check_pledger_limit(env: &Env) -> Result<(), StateSizeError> {
    let pledgers: Vec<soroban_sdk::Address> = env
        .storage()
        .persistent()
        .get(&crate::DataKey::Pledgers)
        .unwrap_or_else(|| Vec::new(env));

    if pledgers.len() >= MAX_PLEDGERS {
        return Err(StateSizeError::ContributorLimitExceeded);
    }
    Ok(())
}
