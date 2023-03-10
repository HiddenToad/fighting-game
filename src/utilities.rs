use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ImageResources(HashMap<&'static str, Handle<Image>>);

impl ImageResources {
    pub fn new(label: &'static str, image_handle: Handle<Image>) -> Self {
        let mut map = HashMap::new();
        map.insert(label, image_handle);
        Self(map)
    }
}
