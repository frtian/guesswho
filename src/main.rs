// main.rs
mod implementation; // Assumindo que seu grid está aqui
use implementation::grid::*;
use bevy::{camera, prelude::*};
use bevy::window::PrimaryWindow;

// Constantes Ajustadas
const CELL_SIZE: f32 = 80.0;
const GRID_PADDING: f32 = 5.0;
const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Guess Who - Bevy".to_string(),
                resolution: (680, 400).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_camera, spawn_grid))
        .add_systems(Update, handle_mouse_hover) // Sistema unificado
        .run();
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
            render_cell(&mut commands, position, CELL_SIZE, col, row, id);
            id += 1;
        }
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