use crate::ai::planet_ai::RusteroidAI;
use crate::ai::utils::handle_rocket_creation_result;
use crate::planet_logging::{
    asteroid_defense_log, rocket_built_log, send_something_received_log,
    send_stopped_log_to_orchestrator, sending_response_log,
};
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;

impl RusteroidAI {
    pub(crate) fn handle_asteroid_impl(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> Option<Rocket> {
        send_something_received_log(
            self,
            "Asteroid".to_string(),
            Some(self.orchestrator_participant.clone()),
            Some(self.asteroid_count),
            None,
        );

        if self.stopped {
            send_stopped_log_to_orchestrator(self);
            self.destroyed = true;
            return None;
        }
        self.asteroid_count += 1;

        // println!("Received Asteroid {}", self.asteroid_count);

        let rocket = state.take_rocket();
        let result: Option<Rocket>;
        match rocket {
            Some(rocket) => {
                result = Some(rocket);

                // println!("Rocket used for asteroid {}", self.asteroid_count);
                let res = state.full_cell();
                match res {
                    Some((_, i)) => {
                        let res = state.build_rocket(i);
                        handle_rocket_creation_result(res);

                        rocket_built_log(self);
                        // println!("Rocket rebuilt to guarantee safety");
                    }
                    None => {}
                }
            }

            None => {
                let res = state.full_cell();
                match res {
                    Some((_, i)) => {
                        let res = state.build_rocket(i);
                        handle_rocket_creation_result(res);
                        result = state.take_rocket();

                        rocket_built_log(self);
                        // println!("Rocket built for asteroid {}", self.asteroid_count);
                    }
                    None => {
                        result = None;
                        self.destroyed = true;
                        // println!("Could not find a Rocket! Destroyed by the asteroid {}", self.asteroid_count);
                    }
                }
            }
        }

        asteroid_defense_log(self, &result, self.asteroid_count);
        sending_response_log(
            self,
            "Asteroid Ack".to_string(),
            Some(self.orchestrator_participant.clone()),
            None,
        );

        result
    }
}
