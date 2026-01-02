use std::sync::OnceLock;
use std::time::Duration;
use common_game::components::forge::Forge;

mod utils;
mod handle_planet;
mod explorer;
mod orchestrator;
pub mod test_unit;

const TIMEOUT: Duration = Duration::from_millis(500);

static FORGE: OnceLock<Forge> = OnceLock::new();