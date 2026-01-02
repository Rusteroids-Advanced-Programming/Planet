use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use crate::ai::planet_ai::RusteroidAI;
use crate::planet_logging::{emit_stopped_log, send_something_received_log, sending_response_log};

impl RusteroidAI {
    pub(crate) fn on_start_impl(&mut self, _state: &PlanetState, _generator: &Generator, _combinator: &Combinator) {
        send_something_received_log(self, "Start AI message".to_string(), Some(self.orchestrator_participant.clone()), None, None);
        // println!("Starting planet :) ");
        self.stopped = false;
        sending_response_log(self, "StartAIResponse".to_string(), Some(self.orchestrator_participant.clone()), None);
    }
    
    pub(crate) fn on_stop_impl(&mut self, _state: &PlanetState, _generator: &Generator, _combinator: &Combinator) {
        self.stopped = true;
        emit_stopped_log(self, Some(self.orchestrator_participant.clone()), true);
        // println!("Planet stopped :( ");
        sending_response_log(self, "StopAIResponse".to_string(), Some(self.orchestrator_participant.clone()), None);
    }
}