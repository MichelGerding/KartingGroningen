pub mod heat_api;
pub mod redis;

pub mod traits {
    pub mod as_map;
    pub mod has_id;
    pub mod diff_between_vecs;
    pub mod primitive_to_naive_date;
}

pub mod database {
    pub mod query;
    pub mod models {
        pub mod driver;
        pub mod session;
        pub mod vehicle;
        pub mod lap;

        pub mod general;
    }
}

pub mod helpers {
    pub mod heat;

    pub mod math;
    pub mod logging;

    pub mod rocket_fairings {
        pub mod cors;
    }
}
