use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Scanner;

#[derive(Default)]
pub struct KoradScanner {}

impl KoradScanner {
    ///
    ///
    ///
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Scanner for KoradScanner {
    fn name(&self) -> String {
        "Korad".to_string()
    }

    fn scan(&self) -> Vec<ProductionOrder> {
        let orders = Vec::new();
        println!("pok");

        orders
    }
}
