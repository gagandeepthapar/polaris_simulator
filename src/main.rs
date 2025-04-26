pub mod shims;

use altai_rs::quatlib::qangle;
use ndarray::{concatenate, Axis};
use polaris_fsw as fsw;
use polaris_fsw::actuators::types::ActuatorBus;
use polaris_plant::{self as plant};

use serde_yaml;
use std::{f64::consts::PI, fs, time};

use env_logger;
use log;
// use polaris_log::PlotLog;

// const CONFIG: &str = "./configs/default.yaml";
const CONFIG: &str = "./configs/spacecraft.yaml";
const SIM_FIN: f64 = 103. * 60. / 4.;
const SC_Ts: f64 = 0.1;
const Ts: f64 = 0.1;
const SMALL: f64 = 1e-8;

fn main() {
    // Initialize SIM
    env_logger::init();

    // Read YAML Inputs
    let yaml_str = fs::read_to_string(CONFIG).unwrap();

    let fsw_inits: shims::fsw_shim::FSW_ParamShim = serde_yaml::from_str(&yaml_str).unwrap();
    let plant_inits: shims::sc_shim::Spacecraft_ParamShim =
        serde_yaml::from_str(&yaml_str).unwrap();

    // Initialize GNC FCSW
    let mut gnc_fcsw = fsw::FlightSoftware::initialize(fsw_inits.to_fsw());
    log::info!("FSW: {:?}", gnc_fcsw);

    // Initialize GNC Plant
    let mut gnc_plant = plant::Spacecraft::initialize(SC_Ts, plant_inits.to_spacecraft());
    log::info!("PLANT: {:?}", gnc_plant);

    // Allocate stack data for raw sensor/actuator commands
    let mut raw_sensor_bus = gnc_plant.initial_state();
    let mut actuator_bus: ActuatorBus = ActuatorBus::default();

    let r0 = gnc_plant
        .curr_sc_state
        .truth_ephemeris
        .signal
        .r_sc_eci
        .clone();
    let q0 = gnc_plant
        .curr_sc_state
        .truth_attitude
        .signal
        .q_sc_eci
        .clone();

    // Loop
    let mut ts_diff: f64;

    let start = time::Instant::now();
    for _ in 0..((SIM_FIN / SC_Ts) as usize) {
        // Run GNC
        // Handle mismatched SC/Ts rates
        ts_diff = (gnc_plant.sim_time / Ts).round() * Ts;
        if (ts_diff - gnc_plant.sim_time).abs() <= SMALL {
            actuator_bus = gnc_fcsw.gnc_loop(&mut raw_sensor_bus); // TODO: REPLACE STRUCT w/ PROTOBUF?
        }

        // Run Plant (Runs at each step)
        raw_sensor_bus = gnc_plant.simulate_plant(&mut actuator_bus); // TODO: REPLACE STRUCT w/ PROTOBUF?
                                                                      // panic!();

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
