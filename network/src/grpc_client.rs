use grpcio::{ChannelBuilder, EnvBuilder};
use proto::proto::mirbft::Proposal;
use proto::proto::{ab_grpc::AtomicBroadcastClient, mirbft::Message};
use std::sync::Arc;

pub struct GrpcClient {
    client: AtomicBroadcastClient,
}

impl GrpcClient {
    pub fn new(host: &str, port: u16) -> Self {
        let conn_addr = format!("{}:{}", host, port);

        // Create a GRPC client
        let env = Arc::new(EnvBuilder::new().name_prefix("grpc-client-").build());
        let ch = ChannelBuilder::new(env).connect(&conn_addr);
        let client = AtomicBroadcastClient::new(ch);

        GrpcClient { client }
    }

    pub fn broadcast(&mut self, msg: &Message) {
        self.client.broadcast(msg).unwrap();
    }
}

#[test]
fn test_client() {
    use proto::proto::mirbft::Prepare;
    let mut client = GrpcClient::new("127.0.0.1", 8081);
    let mut msg = Message::new();
    let mut proposal = Proposal::new();
    proposal.set_payload(vec![1, 2, 3, 4]);
    msg.set_proposal(proposal);
    client.broadcast(&msg);
}
