use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        info,
        net::{NetListener, NetStream},
        reflect::prelude::*,
        visitor::prelude::*,
    },
    plugin::{Plugin, PluginContext},
    scene::base::SceneNodeId,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter},
    path::PathBuf,
};

// ANCHOR: messages
// Server messages are meant to be sent to clients.
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    LoadLevel { path: PathBuf },
}

#[derive(Serialize, Deserialize, Debug)]
// Client messages are meant to be sent to a server.
pub enum ClientMessage {
    PlayerInput { left: bool, right: bool },
}
// ANCHOR_END: messages

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeState {
    pub node: SceneNodeId,
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
}

// ANCHOR: client_server
// Implements listen server.
#[derive(Default, Reflect, Debug)]
pub struct Game {
    server: Option<Server>,
    client: Option<Client>,
}

impl Plugin for Game {
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        self.server = Some(Server::new());
        self.client = Some(Client::connect(Server::ADDRESS));
    }

    // ANCHOR: update_loop
    fn update(&mut self, context: &mut PluginContext) {
        if let Some(server) = self.server.as_mut() {
            server.accept_connections();
            server.read_messages();
        }
        if let Some(client) = self.client.as_mut() {
            client.read_messages();
        }
    }
    // ANCHOR_END: update_loop
}

#[derive(Reflect)]
pub struct Server {
    #[reflect(hidden)]
    listener: NetListener,
    #[reflect(hidden)]
    connections: Vec<NetStream>,
}

impl Server {
    const ADDRESS: &'static str = "127.0.0.1:10000";

    pub fn new() -> Self {
        Self {
            listener: NetListener::bind(Self::ADDRESS).unwrap(),
            connections: Default::default(),
        }
    }

    pub fn accept_connections(&mut self) {
        self.connections.extend(self.listener.accept_connections())
    }

    pub fn read_messages(&mut self) {
        for connection in self.connections.iter_mut() {
            connection
                .process_input::<ClientMessage>(|msg| info!("Received client message: {msg:?}"));
        }
    }
}

#[derive(Reflect)]
pub struct Client {
    #[reflect(hidden)]
    connection: NetStream,
}

impl Client {
    pub fn connect(address: &str) -> Self {
        Self {
            connection: NetStream::connect(address).unwrap(),
        }
    }

    pub fn read_messages(&mut self) {
        self.connection
            .process_input::<ServerMessage>(|msg| info!("Received server message: {msg:?}"));
    }
}

impl Visit for Game {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        // This must be implemented only for hot-reloading support. It is up to you to maintain
        // client-server connections during hot-reloading, and it is out of the scope of this example.
        Ok(())
    }
}
// ANCHOR_END: client_server

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Server")
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client")
    }
}
