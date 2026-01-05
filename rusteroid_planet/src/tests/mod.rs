use common_game::components::forge::Forge;
use std::sync::OnceLock;
use std::time::Duration;

mod explorer;
mod handle_planet;
mod orchestrator;
pub mod test_unit;
mod utils;

const TIMEOUT: Duration = Duration::from_millis(500);

static FORGE: OnceLock<Forge> = OnceLock::new();
