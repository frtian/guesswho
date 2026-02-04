use bevy::prelude::*;
use super::common::{ID, Name};

// RESOURCE: Guarda o que é comum a TODAS as cartas (o verso)
// Carregue isso no seu startup system principal
#[derive(Resource)]
pub struct GlobalCardAssets {
    pub back_image: Handle<Image>,
}

// COMPONENT: Guarda apenas o estado lógico da carta
#[derive(Component, Debug)]
pub struct Card {
    pub is_flipped: bool, // true = vendo a frente, false = vendo o verso
    pub is_eliminated: bool, // para a lógica do jogo
}

// COMPONENT: Guarda a textura ÚNICA daquela pessoa
#[derive(Component)]
pub struct CardFace(pub Handle<Image>);

// BUNDLE: Facilita a criação da entidade completa
#[derive(Bundle)]
pub struct CardBundle {
    pub sprite: Sprite,         // O componente visual do Bevy
    pub transform: Transform,    // Posição no grid
    pub card: Card,              // Lógica de estado
    pub face: CardFace,          // Handle da frente (guardado para quando virar)
    pub id: ID,                  // Seu componente ID
    pub name: Name,              // Seu componente Name
}

// SYSTEM: Sincroniza o visual com o estado lógico
// Roda no Update. Se o estado mudar, troca a textura.
pub fn update_card_visuals(
    assets: Res<GlobalCardAssets>,
    mut query: Query<(&Card, &CardFace, &mut Sprite), Changed<Card>>,
) {
    for (card, face, mut sprite) in query.iter_mut() {
        if card.is_flipped {
            sprite.image = face.0.clone(); // Mostra a pessoa
             sprite.color = Color::WHITE;
        } else {
            sprite.image = assets.back_image.clone(); // Mostra o verso
            
            // Opcional: Escurecer se estiver eliminada
            if card.is_eliminated {
                 sprite.color = Color::srgb(0.5, 0.5, 0.5); 
            }
        }
    }
}