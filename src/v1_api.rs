use crate::raw_api::{self, Object};

pub struct Fan {
    pub name: String,
    pub active_cooling: bool,
    pub desired_speed: u64,
}

pub trait Queryable {
    fn object_name() -> &'static str;
    fn from(value: Object) -> Self;
}

pub fn query<T: Queryable>() -> Vec<T> {
    let name = T::object_name();
    let mut res = vec![];

    for obj in raw_api::query(&format!("SELECT * FROM Win32_{name}")) {
        res.push(T::from(obj))
    }

    res
}

impl Queryable for Fan {
    fn object_name() -> &'static str {
        "Fan"
    }

    fn from(obj: Object) -> Self {
        let active_cooling =
            if let raw_api::Value::Bool(active_cooling) = obj.get_attr("ActiveCooling") {
                active_cooling
            } else {
                panic!();
            };

        let name = if let raw_api::Value::String(name) = obj.get_attr("Name") {
            name
        } else {
            panic!()
        };

        let desired_speed = if let raw_api::Value::UI8(speed) = obj.get_attr("DesiredSpeed") {
            speed
        } else {
            panic!()
        };

        Fan {
            active_cooling,
            name,
            desired_speed,
        }
    }
}
