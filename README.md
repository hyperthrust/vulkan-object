# vulkan-object 1.4.341

Rust bindings for [vulkan-object](https://github.com/KhronosGroup/vulkan-object), providing typed access to Vulkan API metadata.

## Usage

```rust
use vulkan_object::load_vulkan_object;

fn main() {
    let vk = load_vulkan_object();
    for cmd in &vk.commands {
        println!("{}", cmd.name);
    }
}
```

## API

- `load_vulkan_object() -> VulkanObject` - Load the embedded Vulkan metadata (recommended).
- `load_vulkan_object_from_json_str(&str) -> Result<VulkanObject>` - Parse from JSON string.

## Update Procedure

To update to a new Vulkan version:

1. Update `vulkan-object` version in `requirements.txt`
2. Regenerate the schema files:
   ```sh
   python -m venv venv
   source venv/bin/activate  # or venv\Scripts\activate on Windows
   pip install -r requirements.txt
   python generate_schema.py
   ```
3. Update the crate version in `Cargo.toml` to match.
4. Build to regenerate Rust types:
   ```sh
   cargo build
   ```

### How It Works

1. `generate_schema.py` extracts data from the Python `vulkan-object` package into `vk.json` and generates a JSON schema (`vk-schema.json`).
2. `build.rs` uses [typify](https://github.com/oxidecomputer/typify) to generate Rust types from the JSON schema.
3. The crate embeds `vk.json` and provides typed access via serde deserialization.

### Known issues

- The enums ExternSync and CommandScope are obfuscated (int instead of variant).
  - The VulkanObject already obfuscates this compared to vk.xml.

- The VulkanObject needs to be patched because:
  - Some type hints are wrong (optional vs non-optional) and the JSON schema would be wrong as well.
  - The enum fields serialize to integers and cannot be loaded back into enums
  - Remove all init=False so that we can create a patched VulkanObject instance from the original VulkanObject.
    - This becomes unnecessary once we do not need a patched VulkanObject anymore.

- If the original VulkanObject definition changes, the patched VulkanObject in patched_vulkan_object.py needs to be updated as well.
  - This is not difficult. Check the diff and see what was added/changed. Then, do these changes in the patched VulkanObject.
  - Hopefully, the original VulkanObject gets fixed at some day so that the patched version is not necessary anymore.
