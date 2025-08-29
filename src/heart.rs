use prometheus::{Counter, Opts, Registry};
use std::io::{self, Write};

pub struct Heart {
    heartbeats: Counter,
}

impl Heart {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let heartbeats = Counter::with_opts(
            Opts::new("heartbeats", "heart-beats since start")
        )?;
        
        registry.register(Box::new(heartbeats.clone()))?;
        
        Ok(Heart { heartbeats })
    }
    
    pub fn beat(&self) {
        print!(".");
        io::stdout().flush().unwrap();
        self.heartbeats.inc();
    }
}
