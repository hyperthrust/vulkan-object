from dataclasses import asdict
from enum import Enum
from json import JSONEncoder, dump

from vulkan_object import get_vulkan_object


# Required to convert normal enum to integer.
class EnumEncoder(JSONEncoder):
    def default(self, obj):
        if isinstance(obj, Enum):
            return obj.value
        return super().default(obj)


if __name__ == "__main__":
    vulkan_object = get_vulkan_object()
    with open("src/vk.json", "w") as f:
        dump(asdict(vulkan_object), f, indent=2, cls=EnumEncoder)
