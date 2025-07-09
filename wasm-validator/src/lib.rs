use wasm_bindgen::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Profile {
    name: String,
    bio: String,
    picture: String,
}

#[wasm_bindgen]
pub fn validate_profile(json: &str) -> bool {
    let parsed: Result<Profile, _> = serde_json::from_str(json);
    match parsed {
        Ok(profile) => profile.picture.starts_with("bzz://"),
        Err(_) => false,
    }
}
