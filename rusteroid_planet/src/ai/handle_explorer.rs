use crate::ai::planet_ai::RusteroidAI;
use crate::ai::utils::{gen_basic_resource, get_charged_cells_num};
use crate::planet_logging::{
    create_explorer_participant, resource_generated_log, send_something_received_log,
    send_stopped_log_to_explorer, sending_response_log,
};
use common_game::components::planet::PlanetState;
use common_game::components::resource::{
    BasicResource, Combinator, ComplexResourceRequest, Generator, GenericResource,
};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use common_game::utils::ID;

impl RusteroidAI {
    pub(crate) fn on_explorer_arrival_impl(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        explorer_id: ID,
    ) {
        send_something_received_log(
            self,
            format!("Explorer {} arrival", explorer_id),
            Some(self.orchestrator_participant.clone()),
            None,
            None,
        );
        self.is_there_explorer = true;
        sending_response_log(
            self,
            format!("Explorer {} arrival Response", explorer_id),
            Some(self.orchestrator_participant.clone()),
            None,
        );
    }
    pub(crate) fn on_explorer_departure_impl(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        explorer_id: ID,
    ) {
        send_something_received_log(
            self,
            format!("Explorer {} departure", explorer_id),
            Some(self.orchestrator_participant.clone()),
            None,
            None,
        );
        self.is_there_explorer = false;
        sending_response_log(
            self,
            format!("Explorer {} departure Response", explorer_id),
            Some(self.orchestrator_participant.clone()),
            None,
        );
    }
    pub(crate) fn handle_explorer_msg_impl(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        if self.stopped {
            send_stopped_log_to_explorer(self, msg.explorer_id());
            return Some(PlanetToExplorer::Stopped);
        }

        let explorer_participant = create_explorer_participant(msg.explorer_id());

        match msg {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id } => {
                send_something_received_log(
                    self,
                    "Supported Resources Request".to_string(),
                    Some(explorer_participant.clone()),
                    None,
                    Some(explorer_id),
                );
                sending_response_log(
                    self,
                    "SupportedResourceResponse".to_string(),
                    Some(explorer_participant),
                    Some(explorer_id),
                );
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: generator.all_available_recipes(),
                })
            }
            ExplorerToPlanet::SupportedCombinationRequest { explorer_id } => {
                send_something_received_log(
                    self,
                    "Supported Combination Request".to_string(),
                    Some(explorer_participant.clone()),
                    None,
                    Some(explorer_id),
                );
                sending_response_log(
                    self,
                    "SupportedCombinationResponse".to_string(),
                    Some(explorer_participant),
                    Some(explorer_id),
                );
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: combinator.all_available_recipes(),
                })
            }

            ExplorerToPlanet::GenerateResourceRequest {
                explorer_id,
                resource,
            } => {
                send_something_received_log(
                    self,
                    "Generate Resource Request".to_string(),
                    Some(explorer_participant.clone()),
                    None,
                    Some(explorer_id),
                );
                // println!("Received generate resource request for {:?}", resource);

                let resource_list = generator.all_available_recipes();
                let result: Option<BasicResource>;

                match resource_list.get(&resource) {
                    None => {
                        result = None;
                    }
                    Some(resource) => {
                        if get_charged_cells_num(state) >= self.cells_reserved {
                            //Preserve n cells to build a rockets
                            let (c, _i) = state.full_cell().unwrap();
                            result = gen_basic_resource(generator, *resource, c);
                            resource_generated_log(self, *resource, explorer_id);
                        } else {
                            result = None;
                        }
                    }
                }
                sending_response_log(
                    self,
                    "GenerateResourceResponse".to_string(),
                    Some(explorer_participant),
                    Some(explorer_id),
                );
                Some(PlanetToExplorer::GenerateResourceResponse { resource: result })
            }

            ExplorerToPlanet::CombineResourceRequest {
                explorer_id,
                msg: request,
            } => {
                send_something_received_log(
                    self,
                    "Combine Resource Request".to_string(),
                    Some(explorer_participant.clone()),
                    None,
                    Some(explorer_id),
                );

                let lhs: GenericResource;
                let rhs: GenericResource;

                match request {
                    ComplexResourceRequest::Diamond(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }

                    ComplexResourceRequest::AIPartner(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }

                    ComplexResourceRequest::Life(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }

                    ComplexResourceRequest::Dolphin(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }

                    ComplexResourceRequest::Water(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }

                    ComplexResourceRequest::Robot(tmp1, tmp2) => {
                        lhs = tmp1.to_generic();
                        rhs = tmp2.to_generic();
                    }
                }
                sending_response_log(
                    self,
                    "CombineResourceResponse".to_string(),
                    Some(explorer_participant),
                    Some(explorer_id),
                );
                Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response: Err((
                        "Cannot combine resources on this planet".to_string(),
                        lhs,
                        rhs,
                    )),
                })
            }

            ExplorerToPlanet::AvailableEnergyCellRequest { explorer_id } => {
                send_something_received_log(
                    self,
                    "Available Energy Cell Request".to_string(),
                    Some(explorer_participant.clone()),
                    None,
                    Some(explorer_id),
                );
                let num_cells = get_charged_cells_num(state) as ID;
                sending_response_log(
                    self,
                    "AvailableEnergyCellResponse".to_string(),
                    Some(explorer_participant),
                    Some(explorer_id),
                );
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells: num_cells,
                })
            }
        }
    }
}
