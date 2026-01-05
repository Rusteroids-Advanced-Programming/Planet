use crate::tests::explorer::{
    ask_for_resource, combine_resources, get_available_energy_cells, get_combination_list,
    get_supported_resources,
};
use crate::tests::handle_planet::{
    get_internal_state, kill_planet, start_planet_ai, stop_planet_ai,
};
use crate::tests::orchestrator::{
    incoming_explorer, send_asteroid, send_multiple_sunrays, send_sunray,
};
use crate::tests::utils::{get_forge, init_logger, init_test_planet, start_planet_thread};
use common_game::components::resource::BasicResourceType;
use common_game::utils::ID;

#[test]
fn test_sunray() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        _sx_expl_to_planet,
        _sx_planet_to_expl,
        _rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);

    let forge = get_forge();
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();

    let num_sunrays = 8;

    for _ in 0..num_sunrays {
        let sunray = forge.generate_sunray();
        send_sunray(&sx_orch_to_planet, &rx_planet_to_orch, sunray).unwrap();
    }
}

#[test]
fn test_asteroid() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        _sx_expl_to_planet,
        _sx_planet_to_expl,
        _rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);

    let forge = get_forge();
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let num_sunrays = 2;
    let num_asteroids = 3;
    for _ in 0..num_sunrays {
        let sunray = forge.generate_sunray();
        send_sunray(&sx_orch_to_planet, &rx_planet_to_orch, sunray).unwrap();
    }

    for _ in 0..num_asteroids {
        let asteroid = forge.generate_asteroid();
        let destroyed = send_asteroid(&sx_orch_to_planet, &rx_planet_to_orch, asteroid).unwrap();
        if destroyed {
            kill_planet(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
        }
    }

    let sunray = forge.generate_sunray();
    let res = send_sunray(&sx_orch_to_planet, &rx_planet_to_orch, sunray);
    match res {
        Ok(_) => {
            panic!("Sunray handled by a dead planet");
        }
        Err(_) => {
            println!("Sunray not handled because planet is destroyed");
        }
    }
}

#[test]
fn test_extraction() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        sx_expl_to_planet,
        sx_planet_to_expl,
        rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);

    let forge = get_forge();
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();

    send_multiple_sunrays(&sx_orch_to_planet, &rx_planet_to_orch, &forge, 4);
    let explorer_id: ID = 5;
    let mut asked_resource = BasicResourceType::Carbon;
    let num_resource = 3;

    incoming_explorer(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        explorer_id,
        sx_planet_to_expl,
    );

    for _ in 0..num_resource {
        ask_for_resource(
            &sx_expl_to_planet,
            &rx_planet_to_expl,
            asked_resource,
            explorer_id,
        )
        .unwrap();
        asked_resource = BasicResourceType::Hydrogen;
    }

    asked_resource = BasicResourceType::Carbon;

    for _ in 0..2 {
        ask_for_resource(
            &sx_expl_to_planet,
            &rx_planet_to_expl,
            asked_resource,
            explorer_id,
        )
        .unwrap();
    }
}

#[test]
fn test_combination_list() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        sx_expl_to_planet,
        sx_planet_to_expl,
        rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let explorer_id: ID = 5;
    incoming_explorer(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        explorer_id,
        sx_planet_to_expl,
    );
    get_combination_list(&sx_expl_to_planet, &rx_planet_to_expl, explorer_id).unwrap();
}

#[test]
fn test_supported_resources() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        sx_expl_to_planet,
        sx_planet_to_expl,
        rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let explorer_id: ID = 5;
    incoming_explorer(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        explorer_id,
        sx_planet_to_expl,
    );
    get_supported_resources(&sx_expl_to_planet, &rx_planet_to_expl, explorer_id).unwrap();
}

#[test]
fn test_available_energy_cell() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        sx_expl_to_planet,
        sx_planet_to_expl,
        rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let explorer_id: ID = 5;
    incoming_explorer(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        explorer_id,
        sx_planet_to_expl,
    );

    let forge = get_forge();
    send_multiple_sunrays(&sx_orch_to_planet, &rx_planet_to_orch, &forge, 7);

    get_available_energy_cells(&sx_expl_to_planet, &rx_planet_to_expl, explorer_id).unwrap();
}

#[test]
fn test_combine_resources() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        sx_expl_to_planet,
        sx_planet_to_expl,
        rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let explorer_id: ID = 5;
    incoming_explorer(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        explorer_id,
        sx_planet_to_expl,
    );
    let forge = get_forge();
    send_multiple_sunrays(&sx_orch_to_planet, &rx_planet_to_orch, &forge, 6);

    let c1 = ask_for_resource(
        &sx_expl_to_planet,
        &rx_planet_to_expl,
        BasicResourceType::Carbon,
        explorer_id,
    )
    .unwrap()
    .unwrap()
    .to_carbon()
    .unwrap();
    let c2 = ask_for_resource(
        &sx_expl_to_planet,
        &rx_planet_to_expl,
        BasicResourceType::Carbon,
        explorer_id,
    )
    .unwrap()
    .unwrap()
    .to_carbon()
    .unwrap();
    combine_resources(&sx_expl_to_planet, &rx_planet_to_expl, explorer_id, c1, c2).unwrap();
}

#[test]
fn test_internal_state_req() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        _sx_expl_to_planet,
        _sx_planet_to_expl,
        _rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    let forge = get_forge();
    send_multiple_sunrays(&sx_orch_to_planet, &rx_planet_to_orch, &forge, 5);
    get_internal_state(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
}

#[test]
fn test_planet_stop() {
    init_logger();
    let (
        rusteroids,
        sx_orch_to_planet,
        rx_planet_to_orch,
        _sx_expl_to_planet,
        _sx_planet_to_expl,
        _rx_planet_to_expl,
    ) = init_test_planet();
    start_planet_thread(rusteroids);
    start_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();
    stop_planet_ai(&sx_orch_to_planet, &rx_planet_to_orch).unwrap();

    let forge = get_forge();
    let res = send_sunray(
        &sx_orch_to_planet,
        &rx_planet_to_orch,
        forge.generate_sunray(),
    );
    match res {
        Err(e) => {
            println!("Error (Expected Stop Ack) : {}", e);
        }
        Ok(_) => {}
    }
}
