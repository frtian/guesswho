use bevy::prelude::*;
use rand::{Rng, RngCore};

// COMMON COLORS
pub struct Colors;
impl Colors {
    pub const PRESSED: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const DARK_GRAY: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const LIGHT_GRAY: Color = Color::srgb(0.8, 0.8, 0.8);
}

// COMMMON COMPONENTS
#[derive(Component)]
pub struct ID(pub u16);

#[derive(Component)]
pub struct Name(pub String);

// COMMON RESOURCES
#[derive(Resource)]
pub struct GameSeed {
    pub seed: u64,
}

// COMMMON SYSTEMS
pub fn initialize_game_seed(mut commands: Commands) {
    let mut rng = rand::rng();
    let seed = rng.next_u64();
    commands.insert_resource(GameSeed { seed });
}


pub fn gen_game_seed() -> u64 {
    let mut rng = rand::rng();
    rng.next_u64()
}

pub fn update_game_seed(mut commands: Commands, game_seed: Option<ResMut<GameSeed>>, seed: u64) {
    if let Some(mut gs) = game_seed {
        gs.seed = seed;
    } else {
        commands.insert_resource(GameSeed { seed });
    }
}