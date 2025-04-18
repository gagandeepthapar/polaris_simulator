use crate::fsw::fsw_types;
use ndarray::array;
use polaris_fsw::fsw_types::{MultibodyArchitecture, ParamBus};

use serde::Deserialize;

pub trait FSW_Shim {
    type FSW_Analog: fsw_types::Param;
    fn to_fsw(&self) -> Self::FSW_Analog;
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct FSW_ParamShim {
    pub Sensors: SensorShim,
    pub Estimation: EstimationShim,
    pub Reference: ReferenceShim,
    pub Control: ControlShim,
    pub Actuators: ActuatorShim,
    pub Multibody: MultibodyShim,
}

impl FSW_ParamShim {
    pub fn to_fsw(&self) -> fsw_types::ParamBus {
        ParamBus::initialize(
            self.Sensors.to_fsw(),
            self.Estimation.to_fsw(),
            self.Reference.to_fsw(),
            self.Control.to_fsw(),
            self.Actuators.to_fsw(),
            self.Multibody.to_fsw(),
        )
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct ActuatorShim {}
impl FSW_Shim for ActuatorShim {
    type FSW_Analog = fsw_types::ActuatorArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        fsw_types::ActuatorArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct ControlShim {}
impl FSW_Shim for ControlShim {
    type FSW_Analog = fsw_types::ControlArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        fsw_types::ControlArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct EstimationShim {}
impl FSW_Shim for EstimationShim {
    type FSW_Analog = fsw_types::EstimationArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        fsw_types::EstimationArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct ReferenceShim {}
impl FSW_Shim for ReferenceShim {
    type FSW_Analog = fsw_types::ReferenceArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        fsw_types::ReferenceArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct SensorShim {}
impl FSW_Shim for SensorShim {
    type FSW_Analog = fsw_types::SensorArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        fsw_types::SensorArchitecture::default()
    }
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct MultibodyShim {
    pub j_multibody: [[f64; 3]; 3],
}

impl FSW_Shim for MultibodyShim {
    type FSW_Analog = fsw_types::MultibodyArchitecture;
    fn to_fsw(&self) -> Self::FSW_Analog {
        MultibodyArchitecture::initialize(array![[
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
