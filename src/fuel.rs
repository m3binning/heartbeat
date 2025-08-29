use prometheus::{Gauge, Opts, Registry};
use std::io::{self, Write};

const FUEL_MAX: u32 = 3600;
const FUEL_DELTA_1HZ: u32 = 2;

pub struct Fuel {
    fuel_remaining: u32,
    fuel_gauge: Gauge,
}

impl Fuel {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let fuel_gauge = Gauge::with_opts(
            Opts::new("fuel_gauge", "gauge of fuel of some resource")
        )?;
        
        registry.register(Box::new(fuel_gauge.clone()))?;
        fuel_gauge.set(FUEL_MAX as f64);
        
        Ok(Fuel {
            fuel_remaining: FUEL_MAX,
            fuel_gauge,
        })
    }
    
    pub fn burn(&mut self) {
        if self.fuel_remaining >= FUEL_DELTA_1HZ {
            self.fuel_remaining -= FUEL_DELTA_1HZ;
            self.fuel_gauge.sub(FUEL_DELTA_1HZ as f64);
            print!("**");
            
            // Purely for display purposes, if the amount of fuel burned is a
            // multiple of 40, go to the next line.
            if (FUEL_MAX - self.fuel_remaining) % 40 == 0 {
                println!();
            }
            io::stdout().flush().unwrap();
        }
    }
}
