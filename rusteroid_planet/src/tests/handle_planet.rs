use common_game::components::planet::DummyPlanetState;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use crossbeam_channel::{Receiver, Sender};
use crate::tests::utils::send_message_and_wait_response;

pub fn start_planet_ai(sender_orch_to_planet: &Sender<OrchestratorToPlanet>, receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>) -> Result<(), String> {
    let resp = send_message_and_wait_response(sender_orch_to_planet, receiver_planet_to_orch, OrchestratorToPlanet::StartPlanetAI);
    match resp {
        Ok(response) => {
            match response {
                PlanetToOrchestrator::StartPlanetAIResult { planet_id: _ } => { Ok(()) }
                _ => { Err("Received another msg instead of the start planet ai result".to_string())
                }
            }
        }
        Err(e) => { Err(e) }
    }
}

pub fn stop_planet_ai(sender_orch_to_planet: &Sender<OrchestratorToPlanet>, receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>) -> Result<(), String> {
    let resp = send_message_and_wait_response(sender_orch_to_planet, receiver_planet_to_orch, OrchestratorToPlanet::StopPlanetAI)?;
    match resp {
        PlanetToOrchestrator::StopPlanetAIResult {planet_id: _} => { Ok(()) }
        _ => { Err(format!("Received another msg {:?} instead of the stop planet", resp))}
    }
}

pub fn kill_planet(sender_orch_to_planet: &Sender<OrchestratorToPlanet>, receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>) -> Result<(), String> {
    let resp = send_message_and_wait_response(sender_orch_to_planet, receiver_planet_to_orch, OrchestratorToPlanet::KillPlanet)?;
    match resp {
        PlanetToOrchestrator::KillPlanetResult {planet_id: _} => {
            Ok(())
        }
        _ => {Err("Received another msg instead of the kill planet result".to_string())}
    }
}

pub fn get_internal_state(sender_orch_to_planet: &Sender<OrchestratorToPlanet>, receiver_planet_to_orch: &Receiver<PlanetToOrchestrator>) -> Result<DummyPlanetState, String> {
    let resp = send_message_and_wait_response(sender_orch_to_planet, receiver_planet_to_orch, OrchestratorToPlanet::InternalStateRequest)?;
    match resp {
        PlanetToOrchestrator::InternalStateResponse {planet_state, planet_id} => {
            println!("Planet {} State: {:?}", planet_id, planet_state);
            Ok(planet_state)
        }
        _ => {
            Err(format!("Received another msg {:?} instead of the internal state ack", resp))
        }
    }
}