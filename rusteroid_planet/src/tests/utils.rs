use crate::rusteroids::Rusteroids;
use crate::tests::{FORGE, TIMEOUT};
use common_game::components::forge::Forge;
use common_game::components::planet::Planet;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use crossbeam_channel::{Receiver, Sender};
use std::thread;

pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn get_forge() -> &'static Forge {
    FORGE.get_or_init(|| Forge::new().expect("Forge init failed"))
}

pub fn start_planet_thread(mut planet: Planet) {
    let _handle = thread::spawn(move || {
        let _ = planet.run();
    });
}

pub fn create_channels_for_planet() -> (
    (Sender<OrchestratorToPlanet>, Receiver<OrchestratorToPlanet>),
    (Sender<PlanetToOrchestrator>, Receiver<PlanetToOrchestrator>),
    (Sender<ExplorerToPlanet>, Receiver<ExplorerToPlanet>),
    (Sender<PlanetToExplorer>, Receiver<PlanetToExplorer>),
) {
    let orch_to_planet = crossbeam_channel::unbounded::<OrchestratorToPlanet>();

    // Planet -> Orchestrator
    let planet_to_orch = crossbeam_channel::unbounded::<PlanetToOrchestrator>();

    // Explorer -> Planet
    let expl_to_planet = crossbeam_channel::unbounded::<ExplorerToPlanet>();

    // Planet -> Explorer
    let planet_to_expl = crossbeam_channel::unbounded::<PlanetToExplorer>();

    (
        orch_to_planet,
        planet_to_orch,
        expl_to_planet,
        planet_to_expl,
    )
}

pub fn init_test_planet() -> (
    Planet,
    Sender<OrchestratorToPlanet>,
    Receiver<PlanetToOrchestrator>,
    Sender<ExplorerToPlanet>,
    Sender<PlanetToExplorer>,
    Receiver<PlanetToExplorer>,
) {
    // channel creation

    // Orchestrator -> Planet

    let channels = create_channels_for_planet();
    let num_cells_for_rocket = 2;
    let planet = Rusteroids::new(
        1,
        num_cells_for_rocket,
        channels.0.1,
        channels.1.0,
        channels.2.1,
    )
    .unwrap();

    (
        planet.planet,
        channels.0.0,
        channels.1.1,
        channels.2.0,
        channels.3.0,
        channels.3.1,
    )
}

pub fn send_message_and_wait_response<T, B>(
    sender_channel: &Sender<T>,
    receiver_channel: &Receiver<B>,
    msg: T,
) -> Result<B, String> {
    sender_channel
        .send(msg)
        .map_err(|_| "Channel disconnected while sending".to_string())?;

    let response = receiver_channel.recv_timeout(TIMEOUT);
    match response {
        Ok(msg) => Ok(msg),
        Err(e) => {
            let error_msg = format!("Error while waiting for response: {}", e);
            Err(error_msg)
        }
    }
}
