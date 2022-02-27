use bevy::{
    prelude::*, sprite::{SpriteBundle, MaterialMesh2dBundle}, render::render_resource::SamplerDescriptor,
};
use bevy_asset_loader::{AssetLoader, AssetCollection};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    AssetLoading,
    Next,
}

struct RepeatedBackgroundImage {
    repeated: Handle<Image>,
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "background.png")]
    background: Handle<Image>,
    #[asset(path = "arena.png")]
    arena: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    
    AssetLoader::new(MyStates::AssetLoading)
        .continue_to_state(MyStates::Next)
        .with_collection::<ImageAssets>()
        .init_resource::<RepeatedBackgroundImage>()
        .build(&mut app);

    app.add_state(MyStates::AssetLoading)
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(draw))
        .run();
}

impl FromWorld for RepeatedBackgroundImage {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut images = cell
            .get_resource_mut::<Assets<Image>>()
            .expect("Failed to get Assets<Image>");
        let image_assets = cell
            .get_resource::<ImageAssets>()
            .expect("Failed to get ImageAssets");

        let background_image = images.get(image_assets.background.clone()).unwrap();
        let mut repeated = background_image.clone();
        repeated.sampler_descriptor = SamplerDescriptor {
            address_mode_u: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_v: bevy::render::render_resource::AddressMode::Repeat,
            ..Default::default()
        };
        
        RepeatedBackgroundImage {
            repeated: images.add(repeated)
        }
    }
}

fn draw(mut commands: Commands, 
    repeated_background: Res<RepeatedBackgroundImage>, 
    image_assets: Res<ImageAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: image_assets.arena.clone(),
        transform: Transform::from_xyz(0., 0., -0.05),
        ..Default::default()
    });

    let material = ColorMaterial {
        texture: Some(repeated_background.repeated.clone()),
        color: Color::WHITE,
    };

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(material),
        ..Default::default()
    });

    // commands.spawn_bundle(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::splat(1.)),

    //         ..Default::default()
    //     },
    //     texture: repeated_background.repeated.clone(),
    //     transform: Transform::default().with_scale(Vec3::splat(20.0)),
    //     ..Default::default()
    // });
}