///
use panduza_platform_core::Producer;

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("hantek", "0.1.0");

//
// Import modules
mod DSO2C10;


//
// Export the producers of the plugin
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(DSO2C10::Package::default().boxed());

    return producers;
}
    