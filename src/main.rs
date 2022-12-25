use futures::prelude::*;
use libp2p::{identity, PeerId, Multiaddr};
use std::error::Error;
use libp2p::ping;
use libp2p::swarm::{Swarm, SwarmEvent};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Constructing network identity.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Constructing transport - The How
    let transport =  libp2p::development_transport(local_key).await?;

    // Constructing Netowork Behaviour - The What
    let behaviour = ping::Behaviour::new(ping::PingConfig::new().with_keep_alive(true));

    //Constructing swarm - The Who
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr);
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr {address, ..} => println!("Listening on {}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }

    Ok(())
}
