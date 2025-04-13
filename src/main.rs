use polaris_fsw as fsw;
use polaris_fsw::actuators::types::ActuatorBus;
use polaris_log::{self as log, types::LogLevel};
use polaris_plant::{self as plant};

fn main() {
    // Setup
    // // Get Logger
    let (fsw_logger, plant_logger, sim_logger) =
        log::create_loggers("FSW", 10, "PLANT", 10, "SIM", Some(LogLevel::WARN));

    // Initialize SIM
    let mut gnc_sim = sim_logger;

    // Initialize GNC FCSW
    println!("Initializing FSW");
    let mut gnc_fcsw = fsw::FlightSoftware::initialize(fsw_logger);

    // Initialize GNC Plant
    println!("Initializing PLANT");
    let mut gnc_plant = plant::Spacecraft::initialize(plant_logger);

    // Allocate stack data for raw sensor/actuator commands
    let mut raw_sensor_bus = gnc_plant.initial_state();
    let mut actuator_bus: ActuatorBus;

    // Loop
    loop {
        // Run GNC
        println!("READING RAW");
        actuator_bus = gnc_fcsw.gnc_loop(&mut raw_sensor_bus);

        // Run Plant
        println!("COMMANDING ACTUATORS");
        raw_sensor_bus = gnc_plant.simulate_plant(&mut actuator_bus);

        // Log
    }
}
