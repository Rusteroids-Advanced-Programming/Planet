use common_game::components::energy_cell::EnergyCell;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{BasicResource, BasicResourceType, Generator};

pub(crate) fn handle_rocket_creation_result(res: Result<(), String>) {
    if let Err(e) = res {
        println!("Rocket creation error: {}", e);
    }
}

pub(crate ) fn get_charged_cells_num(state: &PlanetState) -> usize {
    let mut count = 0;
    let iter = state.cells_iter();
    for cell in iter {
        if cell.is_charged() {
            count += 1;
        }
    }
    count
}


pub fn gen_basic_resource(generator: &Generator, basic_resource_type: BasicResourceType, cell: &mut EnergyCell) -> Option<BasicResource> {

    let result: Option<BasicResource>;

    match basic_resource_type {
        BasicResourceType::Oxygen => {
            let res = generator.make_oxygen(cell);
            match res {
                Err(_) => {
                    result = None;
                }
                Ok(oxygen_resource) => {
                    result = Some(BasicResource::Oxygen(oxygen_resource));
                }
            }
        }

        BasicResourceType::Carbon => {
            let res = generator.make_carbon(cell);
            match res {
                Err(_e) => {
                    result = None;
                }
                Ok(carbon_resource) => {
                    result = Some(BasicResource::Carbon(carbon_resource));
                }
            }
        }

        BasicResourceType::Hydrogen => {
            let res = generator.make_hydrogen(cell);
            match res {
                Err(_e) => {
                    result = None;
                }
                Ok(hydrogen_resource) => {
                    result = Some(BasicResource::Hydrogen(hydrogen_resource));
                }
            }
        }

        BasicResourceType::Silicon => {

            let res = generator.make_silicon(cell);
            match res {
                Err(_e) => {
                    result = None;
                }
                Ok(silicon_resource) => {
                    result = Some(BasicResource::Silicon(silicon_resource));
                }
            }

        }
    }

    result
}