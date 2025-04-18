use crate::plant::sc_types::*;
use ndarray::array;

use serde::Deserialize;

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
pub struct SpacecraftEphemerisShim {}
impl Spacecraft_Shim for SpacecraftEphemerisShim {
    type Spacecraft_Analog = SpacecraftEphemerisArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftEphemerisArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SpacecraftAttitudeShim {}
impl Spacecraft_Shim for SpacecraftAttitudeShim {
    type Spacecraft_Analog = SpacecraftAttitudeArchitecture;
    fn to_spacecraft(&self) -> Self::Spacecraft_Analog {
        SpacecraftAttitudeArchitecture::default()
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
        SpacecraftMultibodyArchitecture::initialize(array![[
            self.j_multibody[0][0],
            self.j_multibody[0][1],
            self.j_multibody[0][2],
            self.j_multibody[1][0],
            self.j_multibody[1][1],
            self.j_multibody[1][2],
            self.j_multibody[2][0],
            self.j_multibody[2][1],
            self.j_multibody[2][2]
        ]])
    }
}
