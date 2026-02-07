use bevy::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::prelude::*;

// COMMON RESOURCES
pub struct Colors;
impl Colors {
    pub const PRESSED: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const DARK_GRAY: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const LIGHT_GRAY: Color = Color::srgb(0.25, 0.25, 0.25);
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Title,
    Loading,
    Playing,
    Lobby,
    InGame,
}

// COMMMON COMPONENTS
#[derive(Component)]
pub struct ID(pub u16);

#[derive(Component)]
pub struct Name(pub String);

// COMMON RESOURCES
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct GameSeed(pub u64);

#[derive(Resource)]
pub struct GlobalRng(pub ChaCha8Rng);

// GLOBAL EVENTS
#[derive(Event)]
struct GenerateCard {
    who: bool
}

// COMMMON SYSTEMS
pub fn initialize_rng(mut commands: Commands) {
    let random_seed = rand::random::<u64>();
    commands.insert_resource(GameSeed(random_seed));
    commands.insert_resource(GlobalRng(ChaCha8Rng::seed_from_u64(random_seed)));
}

pub fn sync_rng_with_seed(
    game_seed: Res<GameSeed>,
    mut global_rng: ResMut<GlobalRng>
) {
    if game_seed.is_changed() {
        println!("Seed alterada para: {}. Reiniciando o RNG.", game_seed.0);
        *global_rng = GlobalRng(ChaCha8Rng::seed_from_u64(game_seed.0));
    }
}

pub fn get_host_card_id(mut rng: &mut ChaCha8Rng, cards: &[u16]) -> u16 {
    cards.choose(rng).copied().unwrap_or_default()
}

