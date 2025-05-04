This repo contains the sources for all the code from the "A Rust API Inspired by Python, Powered by Serde" article [link](https://ohadravid.github.io/posts/).

It shows how to use Serde's traits to turn this "raw" API:

```rust
let res = raw_api::query("SELECT * FROM Win32_Fan");

for obj in res {
    if obj.get_attr("ActiveCooling") == Value::Bool(true) {
        if let Value::String(name) = obj.get_attr("Name") {
            if let Value::UI8(speed) = obj.get_attr("DesiredSpeed") {
                println!("Fan `{name}` is running at {speed} RPM");
            }
        }
    }
}
```

Into this API:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fan { .. }

let res: Vec<Fan> = query();

for fan in res {
    if fan.active_cooling {
        println!(
            "Fan `{}` is running at {} RPM",
            fan.name, fan.desired_speed
        );
    }
}
```

It is based on work done for the [`wmi-rs` crate](https://github.com/ohadravid/wmi-rs).