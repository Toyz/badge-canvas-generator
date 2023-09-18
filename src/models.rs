use serde::{Serialize, Deserialize};
use std::fmt;

// Define the BadgeInfo struct
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeInfo {
    pub name: String,
    pub creator_id: i64,
    pub image_url: String,
    pub xloc: i64,
    pub yloc: i64,
    pub image_width: i64,
    pub image_height: i64,
    pub creator_badge_index: i64,
}

impl fmt::Display for BadgeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_id_string())
    }
}

impl BadgeInfo {
    pub fn to_id_string(&self) -> String {
        format!("badge-{}-{}", self.creator_id, self.creator_badge_index)
    }

    pub fn to_offset_location(&self) -> (i64, i64) {
        (self.xloc, self.yloc % 100)
    }
}

// Define the AvatarProfileCard struct
#[derive(Debug, Serialize)]
pub struct AvatarProfileCard {
    pub avname: String,
    pub cid: i64,  // The user ID
    pub badges: Vec<(String, BadgeInfo)>,
}
