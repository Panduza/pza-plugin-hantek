//
pub mod device;
use panduza_platform_core::Actions;
use panduza_platform_core::Producer;

mod specialized_interface;
pub use specialized_interface::SpecializedInterface;
pub use specialized_interface::BooleanAccessorIndex;
pub use specialized_interface::StringAccessorIndex;
pub use specialized_interface::NumberAccessorIndex;
pub use specialized_interface::TriggerAccessorIndex;
pub use specialized_interface::VectorF32AccessorIndex;

#[derive(Default)]
pub struct Package {}

impl Package {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Producer for Package {
    fn manufacturer(&self) -> String {
        "hantek".to_string()
    }

    fn model(&self) -> String {
        "DSO2C10".to_string()
    }

    fn description(&self) -> String {
        "---".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn Actions>, panduza_platform_core::Error> {
        return Ok(Box::new(device::Device::new()));
    }
}
    