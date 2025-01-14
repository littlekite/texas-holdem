use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    renet::{RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
    RenetServerPlugin,
};
use network::{handle_create_room, handle_enter_room};
use room::{Player, Room};
use texas_holdem_common::PROTOCOL_ID;

use crate::{
    network::{handle_events_system, handle_get_rooms},
    room::RoomList,
};

mod network;
mod room;

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .insert_resource(RoomList(Vec::new()))
        .add_systems((
            handle_get_rooms,
            handle_create_room,
            handle_enter_room,
            handle_events_system,
        ))
        .run();
}
