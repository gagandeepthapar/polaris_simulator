pub mod shims;

use polaris_fsw as fsw;
use polaris_fsw::actuators::types::ActuatorBus;
use polaris_plant::{self as plant};

use serde_yaml;
use std::{fs, time};

use env_logger;
use log;

const CONFIG: &str = "./configs/default.yaml";
const SIM_FIN: f64 = 100.;
const SC_Ts: f64 = 0.1;
const Ts: f64 = 0.1;
const SMALL: f64 = 1e-12;

fn main() {
    // Initialize SIM
    env_logger::init();

    // Read YAML Inputs
    let yaml_str = fs::read_to_string(CONFIG).unwrap();
    let data: shims::fsw_shim::FSW_ParamShim = serde_yaml::from_str(&yaml_str).unwrap();
    log::trace!("FSW: {:?}", data);

    let data2: shims::sc_shim::Spacecraft_ParamShim = serde_yaml::from_str(&yaml_str).unwrap();
    log::trace!("PLANT: {:?}", data2);

    // Initialize GNC FCSW
    let mut gnc_fcsw = fsw::FlightSoftware::initialize(data.to_fsw());

    // Initialize GNC Plant
    let mut gnc_plant = plant::Spacecraft::initialize(SC_Ts, data2.to_spacecraft());

    // Allocate stack data for raw sensor/actuator commands
    let mut raw_sensor_bus = gnc_plant.initial_state();
    let mut actuator_bus: ActuatorBus = ActuatorBus::default();

    // Loop
    let start = time::Instant::now();
    for _ in 0..((SIM_FIN / SC_Ts) as usize) {
        // Run GNC
        if gnc_plant.sim_time % Ts <= SMALL {
            // Check if running at FSW time
            actuator_bus = gnc_fcsw.gnc_loop(&mut raw_sensor_bus);
        }

        // Run Plant (Runs at each step)
        raw_sensor_bus = gnc_plant.simulate_plant(&mut actuator_bus);

        // Log
    }
    let end = time::Instant::now();

    // Post Process
    let dur = end.duration_since(start).as_nanos() as f64 / 1000.;
    log::error!(
        "{} sec simulation took {} usec to run; {:.4}x FTRT",
        SIM_FIN,
        dur,
        SIM_FIN * 1e6 / dur
    );
}
