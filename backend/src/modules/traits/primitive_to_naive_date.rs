use chrono::NaiveDateTime;
use time::{format_description, PrimitiveDateTime};

pub trait ChangeDateType {
    fn to_naive_date(self) -> NaiveDateTime;
    fn to_primitive_date(self) -> PrimitiveDateTime;
}

impl ChangeDateType for PrimitiveDateTime {
    fn to_naive_date(self) -> NaiveDateTime {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        NaiveDateTime::parse_from_str(
            &self
                .format(&format)
                .unwrap(), "%Y-%m-%d %H:%M:%S")
            .unwrap()
    }

    fn to_primitive_date(self) -> PrimitiveDateTime {
        self
    }
}


impl ChangeDateType for NaiveDateTime {
    fn to_naive_date(self) -> NaiveDateTime {
        self
    }

    fn to_primitive_date(self) -> PrimitiveDateTime {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        PrimitiveDateTime::parse(
            &self.format("%Y-%m-%d %H:%M:%S").to_string(),
            &format
        ).unwrap()
    }
}


