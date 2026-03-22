pub mod domain {
    pub mod models;
    pub mod ports;
    pub mod services;
}

pub mod application {
    pub mod use_cases;
}

pub mod adapters {
    pub mod input;
    pub mod output;
}

pub mod infrastructure {
    pub mod bootstrap;
}
