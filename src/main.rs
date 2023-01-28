#![doc = include_str!("../README.md")]
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};

mod board;
use board::{BoardPlugin, TILE_SIZE};
use piece::PiecePlugin;

mod piece;

const WIDTH: f32 = 512.0;
const HEIGHT: f32 = 512.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Resource)]
pub struct GameAssets {
    white_pawn: Handle<Image>,
    white_rock: Handle<Image>,
    white_bishop: Handle<Image>,
    white_knight: Handle<Image>,
    white_queen: Handle<Image>,
    white_king: Handle<Image>,
    black_pawn: Handle<Image>,
    black_rock: Handle<Image>,
    black_knight: Handle<Image>,
    black_bishop: Handle<Image>,
    black_queen: Handle<Image>,
    black_king: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!(
                    "{} - v{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ),
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_plugins(DefaultPickingPlugins)
        // Systems
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loader)
        .add_startup_system(spawn_camera)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecePlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(TILE_SIZE * 4.0, TILE_SIZE * 4.0, 999.9),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}

fn asset_loader(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        white_pawn: asset_server.load("white_pawn.png"),
        white_rock: asset_server.load("white_rock.png"),
        white_knight: asset_server.load("white_knight.png"),
        white_bishop: asset_server.load("white_bishop.png"),
        white_queen: asset_server.load("white_queen.png"),
        white_king: asset_server.load("white_king.png"),
        black_pawn: asset_server.load("black_pawn.png"),
        black_rock: asset_server.load("black_rock.png"),
        black_knight: asset_server.load("black_knight.png"),
        black_bishop: asset_server.load("black_bishop.png"),
        black_queen: asset_server.load("black_queen.png"),
        black_king: asset_server.load("black_king.png"),
    });
}