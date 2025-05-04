use crate::raw_api;

#[derive(Default)]
pub struct Fan {
    pub name: String,
    pub active_cooling: bool,
    pub desired_speed: u64,
}

pub fn query_fans() -> Vec<Fan> {
    let struct_name = "Fan";
    let mut res = vec![];

    for obj in raw_api::query(&format!("SELECT * FROM Win32_{struct_name}")) {
        let active_cooling =
            if let raw_api::Value::Bool(active_cooling) = obj.get_attr("ActiveCooling") {
                active_cooling
            } else {
                panic!();
            };

        // ..

        res.push(Fan {
            active_cooling,
            ..Default::default()
        });
    }

    res
}
