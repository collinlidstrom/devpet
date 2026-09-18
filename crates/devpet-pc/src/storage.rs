use devpet_core::PetState;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const SCHEMA: u16 = 1;
const OFFLINE_CAP_MINUTES: u64 = 7 * 24 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveGame {
    pub schema_version: u16,
    pub pet: PetState,
    pub last_saved_at: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn path() -> PathBuf {
    PathBuf::from("devpet-save.json")
}

pub fn load() -> PetState {
    let Ok(raw) = fs::read_to_string(path()) else {
        return PetState::default();
    };
    let Ok(save) = serde_json::from_str::<SaveGame>(&raw) else {
        return PetState::default();
    };
    if save.schema_version != SCHEMA {
        return PetState::default();
    }
    let mut pet = save.pet;
    let mins = (now().saturating_sub(save.last_saved_at) / 60).min(OFFLINE_CAP_MINUTES) as u32;
    pet.advance_minutes(mins);
    pet
}
pub fn save(pet: &PetState) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(&SaveGame {
        schema_version: SCHEMA,
        pet: pet.clone(),
        last_saved_at: now(),
    })
    .map_err(std::io::Error::other)?;
    let tmp = path().with_extension("json.tmp");
    fs::write(&tmp, data)?;
    fs::rename(tmp, path())
}
