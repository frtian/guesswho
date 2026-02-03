// grid.rs
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GridCell {
    pub logical_x: usize, // Índice da coluna (0..cols)
    pub logical_y: usize, // Índice da linha (0..rows)
    pub id: u16,
}

// Marker para facilitar query
#[derive(Component)]
pub struct GridRoot; 

pub fn render_cell(
    commands: &mut Commands,
    position: Vec3,
    size: f32,
    logical_x: usize,
    logical_y: usize,
    id: u16,
) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2), // Cinza escuro base
            custom_size: Some(Vec2::splat(size)),
            ..Default::default()
        },
        Transform::from_translation(position),
        GridCell {
            logical_x,
            logical_y,
            id,
        },
    ));
}