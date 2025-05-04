pub fn serde_example() -> Result<(), serde_json::Error> {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename = "Win32_Fan")]
    #[serde(rename_all = "PascalCase")]
    pub struct Fan {
        name: String,
        active_cooling: bool,
        desired_speed: u64,
    }

    let fan: Fan =
        serde_json::from_str(r#"{ "Name": "CPU1", "ActiveCooling": true, "DesiredSpeed": 1100 }"#)?;

    println!("Fan `{}` is running at {} RPM", fan.name, fan.desired_speed);

    let fan: Fan = serde_json::from_str(r#"["CPU1", true, 1100]"#)?;

    println!(
        "Serde from seq: Fan `{}` is running at {} RPM",
        fan.name, fan.desired_speed
    );

    Ok(())
}

pub fn serde_manual_impl_example() -> Result<(), serde_json::Error> {
    use serde::Deserialize;

    #[derive(Debug)]
    pub struct Fan {
        name: String,
        active_cooling: bool,
        desired_speed: u64,
    }

    // Define a custom visitor that match each named field to a concrete value.
    struct FanVisitor;
    impl<'de> serde::de::Visitor<'de> for FanVisitor {
        type Value = Fan;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            todo!()
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut name: Option<String> = None;
            let mut active_cooling: Option<bool> = None;
            let mut desired_speed: Option<u64> = None;

            // We only support string keys,
            // but notice that `next_value()` will return different types for different fields:
            //  a `String` for name, a `bool` for active_cooling, etc.
            while let Ok(Some(key)) = map.next_key::<&str>() {
                match key {
                    "Name" => name = map.next_value()?,
                    "ActiveCooling" => active_cooling = map.next_value()?,
                    "DesiredSpeed" => desired_speed = map.next_value()?,
                    _ => panic!(),
                }
            }

            Ok(Fan {
                name: name.unwrap(),
                active_cooling: active_cooling.unwrap(),
                desired_speed: desired_speed.unwrap(),
            })
        }
    }

    impl<'de> serde::Deserialize<'de> for Fan {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &'static [&'static str] = &["Name", "ActiveCooling", "DesiredSpeed"];

            deserializer.deserialize_struct("Fan", FIELDS, FanVisitor {})
        }
    }

    let fan: Fan =
        serde_json::from_str(r#"{ "Name": "CPU1", "ActiveCooling": true, "DesiredSpeed": 1100 }"#)?;

    println!(
        "Manual serde: Fan `{}` is running at {} RPM",
        fan.name, fan.desired_speed
    );

    Ok(())
}
