from dataclasses import asdict
from enum import Enum
from json import JSONEncoder, dumps, loads

import dacite
import vulkan_object as original
from pydantic import TypeAdapter

import patched_vulkan_object as patched


# Required to convert normal enum to integer.
class EnumEncoder(JSONEncoder):
    def default(self, obj):
        if isinstance(obj, Enum):
            return obj.value
        return super().default(obj)


# Loads the VulkanObject and converts it into a patched VulkanObject.
def load_patched_vulkan_object() -> patched.VulkanObject:
    return patched.VulkanObject(**asdict(original.get_vulkan_object()))


# Dump VulkanObject to JSON.
def dump_vulkan_object_to_json(obj: original.VulkanObject) -> str:
    return dumps(asdict(obj), indent=2, cls=EnumEncoder)


# Load patched VulkanObject from JSON.
def load_patched_vulkan_object_from_json(
    json_str: str,
) -> patched.VulkanObject:
    return dacite.from_dict(patched.VulkanObject, loads(json_str))


# Dump patched VulkanObject to JSON.
def dump_patched_vulkan_object_to_json(obj: patched.VulkanObject) -> str:
    return dumps(asdict(obj), indent=2, cls=EnumEncoder)


# Create a JSON schema of the patched VulkanObject.
def patched_vulkan_object_json_schema() -> str:
    adapter = TypeAdapter(patched.VulkanObject)
    schema = adapter.json_schema()
    return dumps(schema)


if __name__ == "__main__":
    vulkan_object = original.get_vulkan_object()
    vulkan_object_json = dump_vulkan_object_to_json(vulkan_object)
    patched_vulkan_object = load_patched_vulkan_object_from_json(vulkan_object_json)
    patched_vulkan_object_json = dump_patched_vulkan_object_to_json(
        patched_vulkan_object
    )
    assert vulkan_object_json == patched_vulkan_object_json
    json_schema = patched_vulkan_object_json_schema()
    with open("vk.json", "w") as f:
        f.write(patched_vulkan_object_json)
    with open("vk-schema.json", "w") as f:
        f.write(json_schema)
