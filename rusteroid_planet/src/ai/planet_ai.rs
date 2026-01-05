use crate::planet_logging::{create_orchestrator_participant, create_planet_participant};
use common_game::components::planet::{DummyPlanetState, PlanetAI, PlanetState};
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::logging::Participant;
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use common_game::utils::ID;

pub struct RusteroidAI {
    pub(crate) id: ID,
    pub(crate) destroyed: bool,
    pub(crate) stopped: bool,
    pub(crate) sunray_count: usize,
    pub(crate) asteroid_count: usize,
    pub(crate) is_there_explorer: bool,
    pub(crate) cells_reserved: usize,
    pub(crate) orchestrator_participant: Participant,
    pub(crate) planet_participant: Participant,
}

impl RusteroidAI {
    pub fn new(id: ID, cells_reserved_for_rocket: usize) -> Self {
        Self {
            id,
            destroyed: false,
            stopped: false,
            cells_reserved: cells_reserved_for_rocket,
            sunray_count: 0,
            asteroid_count: 0,
            is_there_explorer: false,
            orchestrator_participant: create_orchestrator_participant(),
            planet_participant: create_planet_participant(id),
        }
    }
}

impl PlanetAI for RusteroidAI {
    fn handle_sunray(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        sunray: Sunray,
    ) {
        self.handle_sunray_impl(state, generator, combinator, sunray);
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        self.handle_asteroid_impl(state, generator, combinator)
    }

    fn handle_internal_state_req(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> DummyPlanetState {
        self.handle_internal_state_req_impl(state, generator, combinator)
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        self.handle_explorer_msg_impl(state, generator, combinator, msg)
    }

    fn on_explorer_arrival(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        explorer_id: ID,
    ) {
        self.on_explorer_arrival_impl(state, generator, combinator, explorer_id);
    }

    fn on_explorer_departure(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        explorer_id: ID,
    ) {
        self.on_explorer_departure_impl(state, generator, combinator, explorer_id);
    }

    fn on_start(&mut self, state: &PlanetState, generator: &Generator, combinator: &Combinator) {
        self.on_start_impl(state, generator, combinator);
    }

    fn on_stop(&mut self, state: &PlanetState, generator: &Generator, combinator: &Combinator) {
        self.on_stop_impl(state, generator, combinator);
    }
}
