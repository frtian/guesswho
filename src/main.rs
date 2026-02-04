// main.rs
mod implementation;
use implementation::grid::*;
use implementation::common::*;
use implementation::chars::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CELL_SIZE: f32 = 80.0;
const GRID_PADDING: f32 = 5.0;
const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 4;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Guess Who - Bevy".to_string(),
            resolution: (680, 400).into(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(CharacterDataPlugin);
    app.init_state::<GameState>();
    app.add_systems(Startup, (setup_camera, initialize_game_seed));
    app.add_systems(Startup, start_loading);
    app.add_systems(Update, check_loading_status.run_if(in_state(GameState::Loading)));
    app.add_systems(Update, (handle_mouse_hover, handle_keyboard_input));
    app.add_systems(OnEnter(GameState::Playing), spawn_grid);
    app.run();
}

fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut char_handle: ResMut<CharacterHandle>,
) {
    // Inicia o carregamento assíncrono. O Bevy vai ler o arquivo em background.
    char_handle.0 = asset_server.load("data/characters.json");
}

fn check_loading_status(
    char_handle: Res<CharacterHandle>,
    character_assets: Res<Assets<CharacterCollectionAsset>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Verifica se o asset já existe no banco de assets carregados
    if character_assets.get(&char_handle.0).is_some() {
        println!("JSON Carregado com Sucesso!");
        next_state.set(GameState::Playing);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_grid(mut commands: Commands) {
    let total_width = (GRID_COLS as f32) * (CELL_SIZE + GRID_PADDING) - GRID_PADDING;
    let total_height = (GRID_ROWS as f32) * (CELL_SIZE + GRID_PADDING) - GRID_PADDING;

    // Ponto inicial (Topo-Esquerda relativo ao centro do mundo 0,0)
    let start_x = -total_width / 2.0 + (CELL_SIZE / 2.0);
    let start_y = total_height / 2.0 - (CELL_SIZE / 2.0);

    let mut id = 0;

    // Iteração Lógica
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let x = start_x + (col as f32 * (CELL_SIZE + GRID_PADDING));
            let y = start_y - (row as f32 * (CELL_SIZE + GRID_PADDING)); // Y cresce para cima no Bevy, então subtraímos para ir para baixo

            let position = Vec3::new(x, y, 0.0);
            render_cell(&mut commands, position, CELL_SIZE, col, row, ID(id));
            id += 1;
        }
    }
}

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_seed: Option<ResMut<GameSeed>>,
    commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("R pressed - regenerating game seed");
        if let Some(ref seed) = game_seed {
            println!("Game Seed: {}", seed.seed);
        }
        let seed: u64 = gen_game_seed();
        update_game_seed(commands, game_seed, seed);
    }
}

fn handle_mouse_hover(
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_cells: Query<(&Transform, &mut Sprite, &GridCell)>, 
) {
    let (camera, cam_transform): (&Camera, &GlobalTransform) = q_camera.single().unwrap();

    let window: &Window = q_window.single().unwrap();

    // 2. Obter posição do cursor
    if let Some(cursor_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(cam_transform, cursor).ok()) 
    {
        // 3. Checar colisão
        for (transform, mut sprite, _cell) in q_cells.iter_mut() {
            let position = transform.translation.truncate(); 
            let half_size = CELL_SIZE / 2.0;

            let is_hovering = cursor_position.x > position.x - half_size
                && cursor_position.x < position.x + half_size
                && cursor_position.y > position.y - half_size
                && cursor_position.y < position.y + half_size;

            if is_hovering {
                if sprite.color != Color::WHITE {
                    sprite.color = Color::WHITE;
                }
            } else {
                let default_color = Color::srgb(0.2, 0.2, 0.2);
                if sprite.color != default_color {
                    sprite.color = default_color;
                }
            }
        }
    }
}