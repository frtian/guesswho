use bevy::prelude::*;
use serde::Deserialize;
use bevy_common_assets::json::JsonAssetPlugin;

// 1. Defina a estrutura de um personagem individual
#[derive(Deserialize, Debug, Clone)]
pub struct CharacterData {
    pub id: u16,
    pub name: String,
    pub texture_path: String,
}

// 2. Defina o Asset que representa o ARQUIVO JSON (uma lista de personagens)
#[derive(Deserialize, Asset, TypePath, Debug, Clone)]
pub struct CharacterCollectionAsset {
    pub characters: Vec<CharacterData>,
}

// 3. Plugin para registrar esse tipo
pub struct CharacterDataPlugin;

impl Plugin for CharacterDataPlugin {
    fn build(&self, app: &mut App) {
        app
            // Registra o loader para arquivos .json convertendo para CharacterCollectionAsset
            .add_plugins(JsonAssetPlugin::<CharacterCollectionAsset>::new(&["json"]))
            .init_resource::<CharacterHandle>(); // Recurso para guardar o handle do arquivo
    }
}

// Resource para segurar o handle enquanto carrega
#[derive(Resource, Default)]
pub struct CharacterHandle(pub Handle<CharacterCollectionAsset>);