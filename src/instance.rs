use vulkano::instance::debug::DebugCallback;
use vulkano::instance::debug::MessageSeverity;
use vulkano::instance::debug::MessageType;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::{app_info_from_cargo_toml, instance::layers_list};

use std::sync::Arc;

#[cfg(all(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_KHRONOS_validation",
    "VK_LAYER_LUNARG_standard_validation",
];

pub fn create_instance() -> Arc<Instance> {
    let required_extensions = InstanceExtensions {
        ext_debug_utils: ENABLE_VALIDATION_LAYERS,
        ..vulkano_win::required_extensions()
    };
    let app_info = app_info_from_cargo_toml!();
    if ENABLE_VALIDATION_LAYERS {
        Instance::new(
            Some(&app_info),
            &required_extensions,
            check_validation_layer_support().into_iter(),
        )
        .expect("failed to create Vulkan instance")
    } else {
        Instance::new(Some(&app_info), &required_extensions, None)
            .expect("failed to create Vulkan instance")
    }
}

fn check_validation_layer_support() -> Vec<&'static str> {
    let layers: Vec<_> = layers_list()
        .unwrap()
        .map(|l| l.name().to_owned())
        .collect();
    dbg!(&layers);
    let validation_layers: Vec<_> = VALIDATION_LAYERS
        .iter()
        .filter(|layer_name| layers.contains(&layer_name.to_string()))
        .cloned()
        .collect();
    if validation_layers.is_empty() {
        panic!("no validation extension found!");
    }
    validation_layers
}

pub fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
    if !ENABLE_VALIDATION_LAYERS {
        return None;
    }
    let mut serverity = MessageSeverity::errors_and_warnings();
    serverity.verbose = false;
    DebugCallback::new(instance, serverity, MessageType::all(), |msg| {
        println!("validation layer: {:?}", msg.description);
    })
    .ok()
}
