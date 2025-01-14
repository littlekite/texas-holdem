use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use texas_holdem_common::{
    channel::{
        CreateRoomMessage, EnterRoomMessage, GetRoomsMessage, CREATE_ROOM_CHANNEL_ID,
        ENTER_ROOT_CHANNEL_ID, GET_ROOMS_CHANNEL_ID,
    },
    util::timestamp,
};

use crate::{
    lobby::{CreateRoomEvent, EnterRoomEvent, NewRoomSettings, PlayerName, RoomList},
    AppState,
};

pub fn get_rooms(
    mut client: ResMut<RenetClient>,
    mut room_list: ResMut<RoomList>,
    mut refresh_cd: Local<f32>,
    time: Res<Time>,
) {
    *refresh_cd -= time.delta_seconds();

    if *refresh_cd < 0.0 {
        let message = GetRoomsMessage {
            timestamp: timestamp(),
            rooms: Vec::new(),
        };
        client.send_message(GET_ROOMS_CHANNEL_ID, serde_json::to_vec(&message).unwrap());
        *refresh_cd = 5.0;
    }

    while let Some(message) = client.receive_message(GET_ROOMS_CHANNEL_ID) {
        if let Ok(message) = serde_json::from_slice::<GetRoomsMessage>(&message) {
            info!("Received get rooms message: {:?}", message);
            room_list.0 = message.rooms;
        }
    }
}

pub fn create_room(
    mut create_room_er: EventReader<CreateRoomEvent>,
    mut client: ResMut<RenetClient>,
    new_room_settings: Res<NewRoomSettings>,
    player_name: Res<PlayerName>,
    mut last_timestamp: Local<u64>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // TODO 防止重复创建房间
    for _ in create_room_er.iter() {
        let timestamp = timestamp();
        let message = CreateRoomMessage {
            timestamp,
            room_name: new_room_settings.room_name.clone(),
            room_password: new_room_settings.room_password.clone(),
            player_name: player_name.0.clone(),
            room_id: 0,
        };
        client.send_message(
            CREATE_ROOM_CHANNEL_ID,
            serde_json::to_vec(&message).unwrap(),
        );
        *last_timestamp = timestamp;
    }

    while let Some(message) = client.receive_message(CREATE_ROOM_CHANNEL_ID) {
        if let Ok(message) = serde_json::from_slice::<CreateRoomMessage>(&message) {
            if message.timestamp == *last_timestamp {
                info!("Received create room message: {:?}", message);
                app_state.set(AppState::Gaming);
            }
        }
    }
}

pub fn enter_room(
    mut enter_room_er: EventReader<EnterRoomEvent>,
    mut client: ResMut<RenetClient>,
    player_name: Res<PlayerName>,
    mut last_timestamp: Local<u64>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in enter_room_er.iter() {
        let timestamp = timestamp();
        let message = EnterRoomMessage {
            timestamp,
            room_id: event.room_id,
            room_password: event.room_password.clone(),
            player_name: player_name.0.clone(),
            success: false,
        };
        client.send_message(ENTER_ROOT_CHANNEL_ID, serde_json::to_vec(&message).unwrap());
        *last_timestamp = timestamp;
    }

    while let Some(message) = client.receive_message(ENTER_ROOT_CHANNEL_ID) {
        if let Ok(message) = serde_json::from_slice::<EnterRoomMessage>(&message) {
            if message.timestamp == *last_timestamp && message.success {
                info!("Received enter room message: {:?}", message);
                app_state.set(AppState::Gaming);
            }
        }
    }
}
