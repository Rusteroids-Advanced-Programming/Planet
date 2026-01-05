use crate::ai::planet_ai::RusteroidAI;
use common_game::components::planet::{Planet, PlanetType};
use common_game::components::resource::BasicResourceType::Carbon;
use common_game::components::resource::ComplexResourceType;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::ExplorerToPlanet;
use common_game::utils::ID;
use crossbeam_channel::{Receiver, Sender};

pub struct Rusteroids {
    pub planet: Planet,
}

impl Rusteroids {
    pub fn new(
        id: ID,
        cells_reserved_for_rockets: usize,
        rx: Receiver<OrchestratorToPlanet>,
        sx: Sender<PlanetToOrchestrator>,
        explorer_rx: Receiver<ExplorerToPlanet>,
    ) -> Result<Self, String> {
        let planet_type = PlanetType::A;
        let ai = RusteroidAI::new(id, cells_reserved_for_rockets);
        let gen_rules = vec![Carbon];
        let comb_rules: Vec<ComplexResourceType> = Vec::new();

        let planet = Planet::new(
            id,
            planet_type,
            Box::new(ai),
            gen_rules,
            comb_rules,
            (rx, sx),
            explorer_rx,
        );
        match planet {
            Err(e) => Err(e),
            Ok(planet) => Ok(Self { planet }),
        }
    }
}
