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

    // Remember: `derive(Deserialize)` automatically generates this code
    // (both new struct and the impl) at compile time, based on the `Fan` struct definition.

    // A struct with no fields, only needed so we can attach the impl to something.
    struct FanVisitor;
    impl<'de> serde::de::Visitor<'de> for FanVisitor {
        // The visitor specify what type it is going to produce
        // (as indicated by the return type of `visit_map`).
        type Value = Fan;

        // The `map` is where the data from the deserializer is coming from.
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            // Match each named field to a concrete value.
            let (mut name, mut active_cooling, mut desired_speed) = (None, None, None);

            // Use the `map` we got from the deserializer.
            // `key`'s type is `&str`, which means we call `map.next_key::<&str>()`.
            while let Ok(Some(key)) = map.next_key() {
                // But, `next_value()` returns different types for different fields.
                // We'll explain how later, when we implement our own `Deserializer` and `MapAccess`.
                match key {
                    "Name" => {
                        let val: String = map.next_value()?;
                        name = Some(val);
                    }
                    "ActiveCooling" => {
                        let val: bool = map.next_value()?;
                        active_cooling = Some(val);
                    }
                    "DesiredSpeed" => {
                        let val: u64 = map.next_value()?;
                        desired_speed = Some(val);
                    }
                    _ => panic!(),
                }
            }

            Ok(Fan {
                name: name.unwrap(),
                active_cooling: active_cooling.unwrap(),
                desired_speed: desired_speed.unwrap(),
            })
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            todo!()
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
