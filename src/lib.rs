use panduza_platform_core::{Producer, Scanner};

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("hantek");

mod dso2c10;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(dso2c10::Package::default().boxed());
    // producers.push(kd3005p::producer::KD3005P::new());
    // producers.push(kd3005p_fake::producer::Kd3005pFake::new());
    return producers;
}

//
//
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let mut scanners: Vec<Box<dyn Scanner>> = vec![];
    // scanners.push(scanner::KoradScanner::default().boxed());
    return scanners;
}
