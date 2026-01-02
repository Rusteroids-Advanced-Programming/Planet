use std::collections::HashSet;
use common_game::components::resource::{BasicResource, BasicResourceType, Carbon, ComplexResourceRequest, ComplexResourceType, GenericResource};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use common_game::utils::ID;
use crossbeam_channel::{Receiver, Sender};
use crate::tests::TIMEOUT;

pub fn ask_for_resource(sender_expl_to_planet: &Sender<ExplorerToPlanet>, receiver_planet_to_expl: &Receiver<PlanetToExplorer>, asked_resource: BasicResourceType, expl_id: ID) -> Result<Option<BasicResource>, String> {
    sender_expl_to_planet.send(ExplorerToPlanet::GenerateResourceRequest {explorer_id: expl_id, resource: asked_resource}).unwrap();
    let resp = receiver_planet_to_expl.recv_timeout(TIMEOUT).unwrap();

    match resp {
        PlanetToExplorer::GenerateResourceResponse {resource} => {
            match resource {
                Some(resource) => {
                    if resource.get_type() == asked_resource {
                        println!("Explorer {} received requested resource {:?}", expl_id, asked_resource);
                        Ok(Some(resource))
                    }
                    else {
                        Err(format!("Received a different resource from requested type: {:?}, received: {:?}", asked_resource, resource.get_type()))
                    }
                }
                None => {
                    println!("Explorer {} DID NOT receive requested resource {:?}", expl_id, resource);
                    Ok(None)
                }
            }
        }
        _ => {Err(format!("Received unexpected response: {:?}", resp))}
    }
}

pub fn get_combination_list(sender_expl_to_planet: &Sender<ExplorerToPlanet>, receiver_planet_to_expl: &Receiver<PlanetToExplorer>, expl_id: ID) -> Result<HashSet<ComplexResourceType>, String> {
    sender_expl_to_planet.send(ExplorerToPlanet::SupportedCombinationRequest {explorer_id: expl_id}).unwrap();
    let resp = receiver_planet_to_expl.recv_timeout(TIMEOUT).unwrap();

    match resp {
        PlanetToExplorer::SupportedCombinationResponse {combination_list} => {
            println!("Combination list: {:?}", combination_list);
            Ok(combination_list)
        }
        _ => {Err(format!("Received unexpected response: {:?}", resp))}
    }
}

pub fn get_supported_resources(sender_expl_to_planet: &Sender<ExplorerToPlanet>, receiver_planet_to_expl: &Receiver<PlanetToExplorer>, expl_id: ID) -> Result<HashSet<BasicResourceType>, String> {
    sender_expl_to_planet.send(ExplorerToPlanet::SupportedResourceRequest {explorer_id: expl_id}).unwrap();
    let resp = receiver_planet_to_expl.recv_timeout(TIMEOUT).unwrap();

    match resp {
        PlanetToExplorer::SupportedResourceResponse {resource_list} => {
            println!("Resource list: {:?}", resource_list);
            Ok(resource_list)
        }
        _ => {Err(format!("Received unexpected response: {:?}", resp))}
    }
}

pub fn get_available_energy_cells(sender_expl_to_planet: &Sender<ExplorerToPlanet>, receiver_planet_to_expl: &Receiver<PlanetToExplorer>, expl_id: ID) -> Result<ID, String> {
    sender_expl_to_planet.send(ExplorerToPlanet::AvailableEnergyCellRequest {explorer_id: expl_id}).unwrap();
    let resp = receiver_planet_to_expl.recv_timeout(TIMEOUT).unwrap();

    match resp {
        PlanetToExplorer::AvailableEnergyCellResponse {available_cells} => {
            println!("Available energy cells: {:?}", available_cells);
            Ok(available_cells)
        }
        _ => {Err(format!("Received unexpected response: {:?}", resp))}
    }
}

pub fn combine_resources(
    sender_expl_to_planet: &Sender<ExplorerToPlanet>,
    receiver_planet_to_expl: &Receiver<PlanetToExplorer>,
    expl_id: ID,
    c1: Carbon,
    c2: Carbon,
) -> Result<(String, GenericResource, GenericResource), String> {

    let complex_request = ComplexResourceRequest::Diamond(c1, c2);

    sender_expl_to_planet.send(ExplorerToPlanet::CombineResourceRequest { explorer_id: expl_id, msg: complex_request }).unwrap();

    let resp = receiver_planet_to_expl.recv_timeout(TIMEOUT).unwrap();

    match resp {
        PlanetToExplorer::CombineResourceResponse { complex_response } => {
            match complex_response {
                Ok(_) => Err(
                    "Successful combination of resources on a planet that doesn't support combination"
                        .to_string(),
                ),
                Err(e) => {
                    println!(
                        "Error (as expected) while combining resources: {:?} + {:?}, Reason: {}",
                        e.1, e.2, e.0
                    );
                    Ok(e)
                }
            }
        }
        _ => Err(format!("Unexpected response received {:?}", resp)),
    }
}