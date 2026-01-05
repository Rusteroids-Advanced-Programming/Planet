use crate::tests::utils::send_message_and_wait_response;
use common_game::components::asteroid::Asteroid;
use common_game::components::forge::Forge;
use common_game::components::sunray::Sunray;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::PlanetToExplorer;
use common_game::utils::ID;
use crossbeam_channel::{Receiver, Sender};

pub fn send_sunray(
    sender_orch_to_planet: &Sender<OrchestratorToPlanet>,
    receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>,
    sunray: Sunray,
) -> Result<(), String> {
    let resp = send_message_and_wait_response(
        sender_orch_to_planet,
        receiver_planet_to_orch,
        OrchestratorToPlanet::Sunray(sunray),
    )?;
    match resp {
        PlanetToOrchestrator::SunrayAck { planet_id: _ } => Ok(()),
        _ => Err(format!(
            "Received another msg {:?} instead of the sunray ack",
            resp
        )),
    }
}

pub fn send_multiple_sunrays(
    sender_orch_to_planet: &Sender<OrchestratorToPlanet>,
    receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>,
    forge: &Forge,
    num_sunrays: usize,
) {
    for _ in 0..num_sunrays {
        let sunray = forge.generate_sunray();
        send_sunray(sender_orch_to_planet, receiver_planet_to_orch, sunray).unwrap()
    }
}

pub fn send_asteroid(
    sender_orch_to_planet: &Sender<OrchestratorToPlanet>,
    receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>,
    asteroid: Asteroid,
) -> Result<bool, String> {
    let resp = send_message_and_wait_response(
        sender_orch_to_planet,
        receiver_planet_to_orch,
        OrchestratorToPlanet::Asteroid(asteroid),
    )?;
    match resp {
        PlanetToOrchestrator::AsteroidAck {
            planet_id: _,
            rocket,
        } => match rocket {
            Some(_) => Ok(false),
            None => Ok(true),
        },
        _ => Err("Received another msg instead of the asteroid ack".to_string()),
    }
}

pub fn incoming_explorer(
    sender_orch_to_planet: &Sender<OrchestratorToPlanet>,
    receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>,
    expl_id: ID,
    new_sender: Sender<PlanetToExplorer>,
) {
    send_message_and_wait_response(
        sender_orch_to_planet,
        receiver_planet_to_orch,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: expl_id,
            new_sender,
        },
    )
    .unwrap();
}
