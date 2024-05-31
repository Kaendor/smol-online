use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Event, Debug, Clone, Default, Serialize, Deserialize)]
pub struct TileSelection(pub UVec2);
