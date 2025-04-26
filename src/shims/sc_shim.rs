use crate::plant::sc_types::*;
use altai_rs::meta::types::{Quaternion4, Vector3};
use serde::Deserialize;

/*

SHIMS are used to convert from INPUT file into POLARIS TYPES
Common use case is for initialization

*/

pub trait Spacecraft_Shim {
    type Spacecraft_Analog: SpacecraftParam;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog;
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Spacecraft_ParamShim {
    pub SC_Actuators: SpacecraftActuatorShim,
    pub SC_Ephemeris: SpacecraftEphemerisShim,
    pub SC_Attitude: SpacecraftAttitudeShim,
    pub SC_Multibody: SpacecraftMultibodyShim,
    pub SC_Sensors: SpacecraftSensorShim,
}

impl Spacecraft_ParamShim {
    pub fn to_spacecraft(&self) -> SpacecraftParamBus {
        SpacecraftParamBus::initialize(
            self.SC_Actuators.to_spacecraft(),
            self.SC_Ephemeris.to_spacecraft(),
            self.SC_Attitude.to_spacecraft(),
            self.SC_Multibody.to_spacecraft(),
            self.SC_Sensors.to_spacecraft(),
        )
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftActuatorShim {}
impl Spacecraft_Shim for SpacecraftActuatorShim {
    type Spacecraft_Analog = SpacecraftActuatorArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftActuatorArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftEphemerisShim {
    pub r_sc: [[f64; 3]; 1],
    pub v_sc: [[f64; 3]; 1],
}
impl Spacecraft_Shim for SpacecraftEphemerisShim {
    type Spacecraft_Analog = SpacecraftEphemerisArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftEphemerisArchitecture::initialize(
            Vector3::from_shape_vec((3, 1), self.r_sc[0].to_vec()).unwrap(),
            Vector3::from_shape_vec((3, 1), self.v_sc[0].to_vec()).unwrap(),
        )
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftAttitudeShim {
    pub q_sc_eci: [[f64; 4]; 1],
    pub omega_sc: [[f64; 3]; 1],
}
impl Spacecraft_Shim for SpacecraftAttitudeShim {
    type Spacecraft_Analog = SpacecraftAttitudeArchitecture;

    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftAttitudeArchitecture::initialize(
            Quaternion4::from_shape_vec((4, 1), self.q_sc_eci[0].to_vec()).unwrap(),
            Vector3::from_shape_vec((3, 1), self.omega_sc[0].to_vec()).unwrap(),
        )
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftSensorShim {}
impl Spacecraft_Shim for SpacecraftSensorShim {
    type Spacecraft_Analog = SpacecraftSensorArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftSensorArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftMultibodyShim {
    pub j_multibody: [[f64; 3]; 3],
}

impl Spacecraft_Shim for SpacecraftMultibodyShim {
    type Spacecraft_Analog = SpacecraftMultibodyArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftMultibodyArchitecture::initialize(
            Vector3::from_shape_vec((3, 3), self.j_multibody.as_flattened().to_vec()).unwrap(),
        )
    }
}
