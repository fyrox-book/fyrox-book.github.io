use fyrox::core::algebra::{UnitQuaternion, Vector3};
use fyrox::core::net::{NetListener, NetStream};
use fyrox::scene::base::SceneNodeId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    LoadLevel { path: PathBuf },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    PlayerInput { left: bool, right: bool },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeState {
    pub node: SceneNodeId,
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
}

// ANCHOR: client_server
pub struct Server {
    listener: NetListener,
    connections: Vec<NetStream>,
}

pub struct Client {
    connection: NetStream,
}
// ANCHOR_END: client_server
