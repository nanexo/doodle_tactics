mod components;
mod systems;

mod prelude {
 pub use bevy::prelude::*;
 pub use bevy::sprite::{SpriteBundle};
 pub use bevy_asset_loader::{AssetLoader, AssetCollection};
 pub use crate::components::*;
 pub use crate::systems::*;
}

use prelude::*;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    AssetLoading,
    Next,
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "images/arena.png")]
    arena: Handle<Image>,
    #[asset(path = "images/sword.png")]
    sword: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    
    AssetLoader::new(MyStates::AssetLoading)
        .continue_to_state(MyStates::Next)
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app.add_state(MyStates::AssetLoading)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.92, 0.92, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(draw))
        .add_system_set(SystemSet::new()
            .with_system(map_system)
            .with_system(position_unit)
            .with_system(moving_system))
        .run();
}


fn draw(mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    let player_unit = commands.spawn_bundle(SpriteBundle {
        texture: image_assets.sword.clone(),
        transform: Transform::from_scale(Vec3::splat(0.75)),
        ..Default::default()
    })
    .insert(Position::new(0, 1))
    .insert(UnitType::Melee)
    .insert(Party::Player)
    .id();

    let enemy_unit = commands.spawn_bundle(SpriteBundle {
        texture: image_assets.sword.clone(),
        transform: Transform::from_scale(Vec3::splat(0.75)),
        ..Default::default()
    })
    .insert(Position::new(3, 5))
    .insert(UnitType::Melee)
    .insert(Party::Enemy)
    .id();

    
    let enemy_unit_2 = commands.spawn_bundle(SpriteBundle {
        texture: image_assets.sword.clone(),
        transform: Transform::from_scale(Vec3::splat(0.75)),
        ..Default::default()
    })
    .insert(Position::new(1, 5))
    .insert(UnitType::Melee)
    .insert(Party::Enemy)
    .id();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: image_assets.arena.clone(),
        transform: Transform::from_xyz(0., 0., -0.05).with_scale(Vec3::splat(0.8)),
        ..Default::default()
    })
    .add_child(player_unit)
    .add_child(enemy_unit)
    .add_child(enemy_unit_2);
}

fn position_unit(
    mut unit_query: Query<(&Position, &mut Transform), Without<MovingFromCell>>
) 
{
    for (pos, mut transform) in unit_query.iter_mut() {
        transform.translation = pos.to_translation();
    }
}