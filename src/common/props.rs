use panduza_platform_core::InstanceSettings;

///
///
///
#[derive(Debug)]
struct PowerSupplySettings {
    min_voltage: f64,
    max_voltage: f64,
    min_current: f64,
    max_current: f64,
}

impl PowerSupplySettings {
    ///
    ///
    ///
    pub fn new(min_voltage: f64, max_voltage: f64, min_current: f64, max_current: f64) -> Self {
        Self {
            min_voltage,
            max_voltage,
            min_current,
            max_current,
        }
    }

    ///
    ///
    ///
    pub fn override_from_instance_settings(&mut self, settings: Option<InstanceSettings>) {
        if let Some(data) = settings {
            if data.is_object() {
                if let Some(map) = data.as_object() {
                    //
                    //
                    let min_voltage = map
                        .get("min_voltage")
                        .and_then(|v| v.as_f64().and_then(|v| Some(v)));
                    if let Some(v) = min_voltage {
                        if v > self.min_voltage {
                            self.min_voltage = v;
                        }
                    }
                    //
                    //
                    let max_voltage = map
                        .get("max_voltage")
                        .and_then(|v| v.as_f64().and_then(|v| Some(v)));
                    if let Some(v) = max_voltage {
                        if v < self.max_voltage {
                            self.max_voltage = v;
                        }
                    }
                    //
                    //
                    let min_current = map
                        .get("min_current")
                        .and_then(|v| v.as_f64().and_then(|v| Some(v)));
                    if let Some(v) = min_current {
                        if v > self.min_current {
                            self.min_current = v;
                        }
                    }
                    //
                    //
                    let max_current = map
                        .get("max_current")
                        .and_then(|v| v.as_f64().and_then(|v| Some(v)));
                    if let Some(v) = max_current {
                        if v < self.max_current {
                            self.max_current = v;
                        }
                    }
                }
            }
        }
    }
}
