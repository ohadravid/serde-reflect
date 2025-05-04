use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    // .. inner workings ..
    inner: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,

    String(String),

    I1(i8),
    I2(i16),
    I4(i32),
    I8(i64),

    UI1(u8),
    UI2(u16),
    UI4(u32),
    UI8(u64),

    R4(f32),
    R8(f64),

    Bool(bool),

    Array(Vec<Value>),

    Object(Object),
}

impl Object {
    pub fn get_attr(&self, name: &str) -> Value {
        self.inner.get(name).unwrap().clone()
    }
}

pub fn query(query: &str) -> Vec<Object> {
    vec![
        Object {
            inner: HashMap::from_iter([
                ("Name".to_string(), Value::String("CPU0".to_string())),
                ("ActiveCooling".to_string(), Value::Bool(true)),
                ("DesiredSpeed".to_string(), Value::UI8(100)),
            ]),
        },
        Object {
            inner: HashMap::from_iter([
                ("Name".to_string(), Value::String("CPU1".to_string())),
                ("ActiveCooling".to_string(), Value::Bool(true)),
                ("DesiredSpeed".to_string(), Value::UI8(150)),
            ]),
        },
        Object {
            inner: HashMap::from_iter([
                (
                    "Name".to_string(),
                    Value::String("North Bridge Sink".to_string()),
                ),
                ("ActiveCooling".to_string(), Value::Bool(false)),
                ("DesiredSpeed".to_string(), Value::UI8(0)),
            ]),
        },
    ]
}
