use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetConnectionConfig},
    RenetClientPlugin,
};
use lobby::{
    lobby_create_room_ui, lobby_room_list_ui, lobby_set_player_name_ui, CreateRoomEvent,
    EnterRoomEvent, NewRoomSettings, PlayerName, RoomList,
};
use network::{create_room, enter_room};
use texas_holdem_common::{util::timestamp, PROTOCOL_ID};

use crate::{
    network::get_rooms,
    table::{setup_one_card, setup_table},
};

mod lobby;
mod network;
mod room;
mod table;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppState {
    #[default]
    Lobby,
    Gaming,
}

fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin::default())
        .add_plugin(EguiPlugin)
        .add_state::<AppState>()
        .add_event::<CreateRoomEvent>()
        .add_event::<EnterRoomEvent>()
        .insert_resource(new_renet_client())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RoomList(Vec::new()))
        .insert_resource(PlayerName(format!("Player{}", timestamp())))
        .insert_resource(NewRoomSettings::default())
        .add_startup_systems((setup_camera,))
        .add_systems(
            (
                get_rooms,
                create_room,
                enter_room,
                lobby_room_list_ui,
                lobby_create_room_ui,
                lobby_set_player_name_ui,
            )
                .in_set(OnUpdate(AppState::Lobby)),
        )
        .add_systems((setup_table, setup_one_card).in_schedule(OnEnter(AppState::Gaming)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.projection.scale = 2.5;
    commands.spawn(camera2d_bundle);
}
