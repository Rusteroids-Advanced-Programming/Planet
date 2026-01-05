use crate::ai::planet_ai::RusteroidAI;
use crate::ai::utils::{get_charged_cells_num, handle_rocket_creation_result};
use crate::planet_logging::{
    rocket_built_log, send_something_received_log, send_stopped_log_to_orchestrator,
    sending_response_log, sunray_storage_log,
};
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::sunray::Sunray;

impl RusteroidAI {
    pub(crate) fn handle_sunray_impl(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        sunray: Sunray,
    ) {
        send_something_received_log(
            self,
            "Sunray".to_string(),
            Some(self.orchestrator_participant.clone()),
            Some(self.sunray_count),
            None,
        );

        if self.stopped {
            send_stopped_log_to_orchestrator(self);
            return;
        }

        self.sunray_count += 1;
        // println!("Received Sunray {}", self.sunray_count);
        let opt_sunray = state.charge_cell(sunray);

        match opt_sunray {
            Some(_) => {
                // println!("Could not store sunray. All energy cells are charged");
                sunray_storage_log(self, false, get_charged_cells_num(state));
            }
            None => {
                // println!("Stored sunray in an energy cell, charged cells: {}", get_charged_cells_num(state));
                sunray_storage_log(self, true, get_charged_cells_num(state));
            }
        }

        // USING SUNRAY WITHOUT DISCARDING IT
        // IF ALL CELLS ARE CHARGED AND EXPLORER IS ON PLANET

        // if let Some(sunray) = opt_sunray {
        //     if self.is_there_explorer {
        //
        //     }
        // }

        if !state.has_rocket() {
            let (_, i) = state.full_cell().unwrap();
            let res = state.build_rocket(i);
            handle_rocket_creation_result(res);

            rocket_built_log(self);
            // println!("Rocket built with sunray {}", self.sunray_count);
        }

        sending_response_log(
            self,
            "Sunray ACK".to_string(),
            Some(self.orchestrator_participant.clone()),
            None,
        );
    }
}
