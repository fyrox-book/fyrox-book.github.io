
// ANCHOR: player_imports

use crate::{player::Player};
use fyrox::{
    core::pool::Handle,
    core::{reflect::prelude::*, visitor::prelude::*},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
    event::Event,
};
use fyrox_ui::message::UiMessage;
use std::path::Path;
// ANCHOR_END: player_imports
// ANCHOR: player_mod_reg
// Add this line
pub mod player;

// ANCHOR_END: player_mod_reg