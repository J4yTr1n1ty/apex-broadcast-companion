use std::any::Any;
use std::collections::HashMap;

use prost::Message;

use crate::events::Init;

// Registry mapping type URLs to deserialization functions
pub fn get_registry(
) -> HashMap<String, fn(&[u8]) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>> {
    let mut registry: HashMap<
        String,
        fn(&[u8]) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
    > = HashMap::new();
    registry.insert(
        "type.googleapis.com/rtech.liveapi.Init".to_string(),
        deserialize_init,
    );
    registry
}

fn deserialize_init(bytes: &[u8]) -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
    Ok(Box::new(Init::decode(bytes)?))
}
