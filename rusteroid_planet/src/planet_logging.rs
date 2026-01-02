use common_game::components::resource::BasicResourceType;
use common_game::components::rocket::Rocket;
use common_game::logging::{ActorType, LogEvent, EventType, Channel, Payload, Participant};
use common_game::utils::ID;
use crate::ai::planet_ai::RusteroidAI;


pub fn create_orchestrator_participant() -> Participant {
    Participant::new(ActorType::Orchestrator, 0 as ID)
}

pub fn create_planet_participant(id: ID) -> Participant {
    Participant::new(ActorType::Planet, id)
}

pub fn create_explorer_participant(id: ID) -> Participant {
    Participant::new(ActorType::Explorer, id)
}

pub fn set_payload(key_values: Vec<(String, String)>) -> Payload {
    let mut result = Payload::new();
    for (key, value) in key_values {
        result.insert(key, value);
    }
    result
}

pub fn emit_stopped_log(ai: &RusteroidAI, receiver: Option<Participant>, just_set: bool) {
    let mut key = "Planet AI is Stopped".to_string();
    if just_set {
        key = "Planet AI has just been Stopped".to_string();
    }
    let payload = set_payload(vec![(key, format!("Planet {}", ai.id))]);
    let sender = Some(ai.planet_participant.clone());
    let log = LogEvent::new(sender, receiver, EventType::MessagePlanetToOrchestrator, Channel::Info, payload);
    log.emit();
}

pub fn send_stopped_log_to_orchestrator(ai: &RusteroidAI) {
    emit_stopped_log(ai, Some(ai.orchestrator_participant.clone()), false);
}

pub fn send_stopped_log_to_explorer(ai: &RusteroidAI, explorer_id: ID) {
    let explorer_participant = create_explorer_participant(explorer_id);
    emit_stopped_log(ai, Some(explorer_participant), false);
}

pub fn send_something_received_log(ai: &RusteroidAI, object_received: String, sender: Option<Participant>, count: Option<usize>, from_explorer: Option<ID>) {
    let key = format!("{} Received", object_received);
    let mut value = format!("Planet {} received {}", ai.id, object_received);
    let mut event_type = EventType::MessageOrchestratorToPlanet;
    match count {
        Some(count) => {
            value = format!("{} {}", value, count);
        }
        _ => {}
    }

    match from_explorer {
        Some(from_explorer) => {
            value = format!("{} from Explorer {}", value, from_explorer);
            event_type = EventType::MessageExplorerToPlanet;
        }
        _ => {}
    }

    let payload = set_payload(vec![(key, value)]);
    let log = LogEvent::new(sender, Some(ai.planet_participant.clone()), event_type, Channel::Trace, payload);
    log.emit();
}


pub fn sending_response_log(ai: &RusteroidAI, response_name: String, receiver: Option<Participant>, to_explorer: Option<ID>) {
    let key = format!("Sending {}", response_name);
    let mut value = format!("Planet {} is sending {}", ai.id, response_name);
    let mut event_type = EventType::MessagePlanetToOrchestrator;

    match to_explorer {
        Some(to_explorer) => {
            value = format!("{} to Explorer {}", value, to_explorer);
            event_type = EventType::MessagePlanetToExplorer;
        }
        None => {}
    }

    let payload = set_payload(vec![(key, value)]);
    let log = LogEvent::new(Some(ai.planet_participant.clone()), receiver, event_type, Channel::Trace, payload);
    log.emit();
}


pub fn rocket_built_log(ai: &RusteroidAI) {
    let payload = set_payload(vec![("Rocket Built".to_string(), format!("Planet {} built a rocket", ai.id))]);
    let log = LogEvent::new(Some(ai.planet_participant.clone()), None, EventType::InternalPlanetAction, Channel::Info, payload);
    log.emit();
}

pub fn sunray_storage_log(ai: &RusteroidAI, stored: bool, charged_cells: usize) {
    let key:String;
    let mut value:String;
    if stored {
        key = "Sunray Stored".to_string();
        value = format!("Planet {} stored a sunray.", ai.id);
    }
    else {
        key = "Sunray Discarded".to_string();
        value = format!("Planet {} discarded a sunray.", ai.id);
    }
    value = format!("{} There are {} charged energy cells", value, charged_cells);

    let payload = set_payload(vec![(key, value)]);
    let log = LogEvent::new(Some(ai.planet_participant.clone()), None, EventType::InternalPlanetAction, Channel::Trace, payload);
    log.emit();
}

pub fn asteroid_defense_log(ai: &RusteroidAI, defended: &Option<Rocket>, asteroid_num: usize) {
    let key = "Asteroid Defense".to_string();
    let value: String;

    match defended {
        Some(_) => {
            value = format!("Planet {} defended from asteroid #{} with a rocket", ai.id, asteroid_num);
        }
        None => {
            value = format!("Planet {} could not defend from asteroid #{}", ai.id, asteroid_num);
        }
    }

    let payload = set_payload(vec![(key, value)]);
    let log = LogEvent::new(Some(ai.planet_participant.clone()), None, EventType::InternalPlanetAction, Channel::Info, payload);
    log.emit();
}

pub fn resource_generated_log(ai: &RusteroidAI, basic_resource_type: BasicResourceType, for_explorer: ID) {
    let key = "Resource Generated".to_string();
    let value = format!("Planet {} generated a {:?} for Explorer #{}", ai.id, basic_resource_type, for_explorer);
    let payload = set_payload(vec![(key, value)]);
    let log = LogEvent::new(Some(ai.planet_participant.clone()), None, EventType::InternalPlanetAction, Channel::Info, payload);
    log.emit();
}