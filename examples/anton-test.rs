use bevy_app::{App, Startup};
use valence::prelude::*;
use valence_scoreboard::{Objective, ObjectiveBundle, ObjectiveDisplay, ObjectiveScores};

//idea: Tetris
//create terrain
//add button for gamestart
//create scoreboard with points
//create tetris template blocks
//randomly spawn
//rotate on leftclick
//ff on right click
//if block hits ground, add new block
//check for full rows
//if block is at top, end game


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (init_clients))
        .run()
}

pub fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimension_reg: Res<DimensionTypeRegistry>,
    biome_reg: Res<BiomeRegistry>,
) {
    println!("running setup");

    let mut bundle_layer = LayerBundle::new(ident!("overworld"), &dimension_reg, &biome_reg, &server);

    for z in -5..5 {
        for x in -5..5 {
            bundle_layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    create_playing_board(&mut bundle_layer);


    commands.spawn(bundle_layer);
}

pub fn init_clients(mut clients: Query<(&mut Client),Added<Client>>) {
    println!("running init_clients");
    for mut client in &mut clients{
        client.send_chat_message("hello to this test");
    }
}

//either give the function a bundle_layer or get it by query?
pub fn create_playing_board(bundle_layer: &mut LayerBundle){
    for z in -50..50 {
        for x in -50..50 {
            bundle_layer.chunk.set_block([x, 0, z], BlockState::GRASS_BLOCK);
        }
    }

    //create blocks around the grass blocks
    for z in -50..50 {
        bundle_layer.chunk.set_block([-50, 1, z], BlockState::STONE);
        bundle_layer.chunk.set_block([50, 1, z], BlockState::STONE);
    }
    for x in -50..50 {
        bundle_layer.chunk.set_block([x, 1 , -50], BlockState::STONE);
        bundle_layer.chunk.set_block([x, 1 , 50], BlockState::STONE);
    }
}

pub fn create_score_board(commands: &mut Commands){
        commands.spawn(ObjectiveBundle{
            name: Objective::new("name"),
            display: ObjectiveDisplay("Display".bold()),
            render_type: Default::default(),
            scores: Default::default(),
            old_scores: Default::default(),
            position: Default::default(),
            layer: Default::default(),
        });
}

pub fn update_score_board(new_score: i32, running: bool,mut score_boards: Query<(&mut ObjectiveScores)>){
    let mut score_board = score_boards.single_mut();
    score_board.insert("key",1);
}


