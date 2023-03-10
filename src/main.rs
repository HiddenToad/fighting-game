use bevy::prelude::*;

mod player;
use player::Player;

mod enemy;
use enemy::Enemy;

mod utilities;
use utilities::*;

mod consts;
use consts::*;

#[derive(Bundle)]
struct EntityBundle {
    pos: Transform,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,

    #[bundle]
    entity: EntityBundle,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,

    #[bundle]
    entity: EntityBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let player_texture: Handle<Image> = asset_server.load("milk.png"); //add path
    //let enemy_texture: Handle<Image> = asset_server.load("path/to/enemy/todo"); //add path

    //commands.insert_resource(ImageResources::new("player", player_texture));
    // commands.insert_resource(ImageResources::new("enemy", enemy_texture));

    commands
        .spawn(PlayerBundle {
            player: Player {
                up: KeyCode::W,
                left: KeyCode::A,
                down: KeyCode::S,
                right: KeyCode::D,
                health: BASE_HEALTH,
                max_health: BASE_HEALTH,
                hp_regen: BASE_REGEN,
                percent_damage: BASE_PERCENT_DAMAGE,
                flat_damage: BASE_FLAT_DAMAGE,
                attack_speed: BASE_ATTACK_SPEED,
                range: BASE_RANGE,
                armor: BASE_ARMOR,
                dodge: BASE_DODGE,
                speed: BASE_PERCENT_SPEED,
            },
            entity: EntityBundle {
                pos: Transform::from_translation(Vec3::ZERO),
            },
        })
        .insert(SpriteBundle {
            texture: asset_server.load("milk.png"),
            transform: Transform {
                scale: Vec3::splat(PLAYER_SCALE),
                ..default()
            },
            ..default()
        });

    commands.spawn(Camera2dBundle::default());
}

fn control(mut query: Query<(&mut Player, &mut Transform)>, input: Res<Input<KeyCode>>) {
    for (player, mut transform) in query.iter_mut() {
        if input.pressed(player.up) {
            transform.translation.y += player.move_dist();
        }
        if input.pressed(player.down) {
            transform.translation.y -= player.move_dist();
        }
        if input.pressed(player.left) {
            transform.translation.x -= player.move_dist();
        }
        if input.pressed(player.right) {
            transform.translation.x += player.move_dist();
        }
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(control)
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .run();
}
