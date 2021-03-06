use crate::handlers::Handler;
use io::connection::Connection;
use protocol::packets::status as status_packets;
use status_packets::serverbound::Ping;
use status_packets::clientbound::{Packet as ClientboundPacket, Pong};
use failure::Fallible;

impl Handler for Ping {
    type Context = Connection;

    fn handle(&mut self, connection: &mut Self::Context) -> Fallible<()> {
        let pong = ClientboundPacket::Pong(Pong {
            payload: self.payload
        });

        connection.send(&pong)
    }
}

