use bevy_app::{App, Startup};
use valence::prelude::*;

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
    server.

    let bundle_layer = LayerBundle::new(ident!("nether"), &dimension_reg, &biome_reg, &server);

    commands.spawn(bundle_layer)
}

pub fn init_clients(mut clients: Query<(&mut Client),Added<Client>>) {
    println!("running init_clients");
    for mut client in &mut clients{
        client.send_chat_message("hello to this test");
    }
}
