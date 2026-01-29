pub mod schema;

const VK_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/vk.json"));

pub fn load_vulkan_object_from_json_str(s: &str) -> serde_json::Result<schema::VulkanObject> {
    serde_json::from_str(s)
}

pub fn load_vulkan_object() -> schema::VulkanObject {
    load_vulkan_object_from_json_str(VK_JSON).expect("Failed to parse embedded vk.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vulkan_object() {
        let vo = load_vulkan_object();
        assert!(!vo.commands.is_empty());
    }
}
