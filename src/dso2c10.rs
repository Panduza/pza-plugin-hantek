mod device;

mod scpi_boolean;
pub use scpi_boolean::ScpiBoolean;

use device::Device;
use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Scanner;
use panduza_platform_core::{DriverOperations, Producer};

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
        "Oscilloscope".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        let mut props = panduza_platform_core::Props::default();

        props
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(Device::default()));
    }
}

impl Scanner for Package {
    fn name(&self) -> String {
        "hantek".to_string()
    }

    fn scan(&self) -> Vec<ProductionOrder> {
        let mut orders = Vec::new();

        // if let Ok(devices_list) = nusb::list_devices() {
        //     for dev in devices_list {
        //         let man = dev.manufacturer_string().unwrap_or("?");
        //         let pro = dev.product_string().unwrap_or("?");

        //         let mut po = ProductionOrder::new("std.scpi", format!("{}.{}", man, pro))
        //             .add_u16_setting("usb_vid", dev.vendor_id())
        //             .add_u16_setting("usb_pid", dev.product_id());

        //         if let Some(serial_num) = dev.serial_number() {
        //             po = po.add_string_setting("usb_serial", serial_num);
        //         }

        //         orders.push(po);
        //     }
        // }

        orders
    }
}
