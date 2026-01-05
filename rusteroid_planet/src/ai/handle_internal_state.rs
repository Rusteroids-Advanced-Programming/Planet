use crate::ai::planet_ai::RusteroidAI;
use crate::planet_logging::send_something_received_log;
use common_game::components::planet::{DummyPlanetState, PlanetState};
use common_game::components::resource::{Combinator, Generator};

impl RusteroidAI {
    pub(crate) fn handle_internal_state_req_impl(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> DummyPlanetState {
        send_something_received_log(
            self,
            "Internal State Req".to_string(),
            Some(self.orchestrator_participant.clone()),
            None,
            None,
        );
        state.to_dummy()
    }
}
