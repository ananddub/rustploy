use auto_di::singleton;
use socketioxide::{SocketIo, layer::SocketIoLayer};

pub struct Socket {
    pub io: SocketIo,
    pub layer: SocketIoLayer,
}

#[singleton]
pub async fn socket_init() -> Socket {
    let (layer, io) = SocketIo::new_layer();
    auto_socket::register_global(&io)
        .await
        .expect("failed to register socket handlers");
    Socket { io, layer }
}
