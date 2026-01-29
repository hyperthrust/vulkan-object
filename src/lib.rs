pub mod vulkan_object;

const VK_JSON: &str = include_str!("vk.json");

pub fn load_vulkan_object_from_json_str(
    s: &str,
) -> serde_json::Result<vulkan_object::VulkanObject> {
    serde_json::from_str(s)
}

pub fn load_vulkan_object() -> vulkan_object::VulkanObject {
    load_vulkan_object_from_json_str(VK_JSON).expect("Failed to parse embedded vk.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_serialization() {
        // Do one round to have a predictable JSON string.
        let expected =
            serde_json::to_string(&serde_json::from_str::<serde_json::Value>(VK_JSON).unwrap())
                .unwrap();
        // Deserialize and then serialize to JSON string.
        let actual = serde_json::to_string(&load_vulkan_object()).unwrap();

        // Compare both JSON strings, they have to match.
        assert!(expected == actual);
    }
}
