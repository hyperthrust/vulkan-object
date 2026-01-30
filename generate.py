import inspect
import re
from dataclasses import asdict, fields, is_dataclass
from enum import Enum
from json import JSONEncoder, dump
from pathlib import Path
from types import NoneType, UnionType
from typing import get_args, get_origin, get_type_hints

from vulkan_object import get_vulkan_object
from vulkan_object import vulkan_object as vulkan_object_module

# --------------- #
# Regular consts. #
# --------------- #

RESERVED_KEYWORDS = {"type", "struct", "const"}
BASIC_TYPES = {str: "String", bool: "bool", float: "f64", int: "u32", NoneType: "None"}

# ---------------------------- #
# Regular Rust code templates. #
# ---------------------------- #

RUST_HEADER = """\
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};"""
ENUM_TEMPLATE = """\
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum {name} {{
{fields}
}}"""
STRUCT_TEMPLATE = """\
#[derive({derives})]
pub struct {name} {{
{fields}
}}"""


# ---------------------------------------------------- #
# Adaption code for current shape of vulkan_object.py. #
# This could change in the future.                     #
# ---------------------------------------------------- #

TYPE_OVERRIDES = {
    ("Constant", "value"): "ConstantValue",
    ("EnumField", "value"): "i64",
    ("Flag", "aliases"): "Option<Vec<String>>",
    ("Flag", "value"): "u64",
    ("Handle", "parent"): "Option<Box<Handle>>",  # Recursive type needs Box
    ("Param", "alias"): "Option<String>",
    ("SyncEquivalent", "accesses"): "Option<Vec<Flag>>",
    ("SyncEquivalent", "stages"): "Option<Vec<Flag>>",
    ("SyncSupport", "queues"): "Option<Vec<String>>",
    ("SyncSupport", "stages"): "Option<Vec<Flag>>",
    ("VulkanObject", "headerVersion"): "String",
}
NO_EQ_OVERRIDES = {"ConstantValue", "Constant", "VideoStd", "VulkanObject"}
TEMPLATE_OVERRIDES = {
    "ConstantValue": """\
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ConstantValue {
    Int(u64),
    Float(f64),
}"""
}


def to_rust_enum_variant_name(name: str) -> str:
    """Convert screaming snake case to pascal case."""

    return name.replace("_", " ").title().replace(" ", "")


def to_rust_field_name(name: str) -> tuple[str, bool]:
    """Convert pascal case to snake case, returning (rust_name, needs_serde_rename)."""

    snake_case = re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()
    if name in RESERVED_KEYWORDS:
        return name + "_", True
    return snake_case, snake_case != name


def python_type_to_rust(py_type, class_name: str, field_name: str) -> str:
    """Convert a Python type to a Rust type string."""

    if rust_type := TYPE_OVERRIDES.get((class_name, field_name)):
        return rust_type
    if rust_type := BASIC_TYPES.get(py_type):
        return rust_type

    # Class references (dataclasses, enums).
    if isinstance(py_type, type):
        return py_type.__name__

    origin = get_origin(py_type)
    args = get_args(py_type)

    # Union types (T | None) → Option<T>.
    if origin is UnionType:
        non_none = [a for a in args if a is not NoneType]
        if len(non_none) == 1:
            return f"Option<{python_type_to_rust(non_none[0], class_name, field_name)}>"

    # Lists.
    if origin is list:
        return f"Vec<{python_type_to_rust(args[0], class_name, field_name)}>"

    # Dicts.
    if origin is dict:
        k = python_type_to_rust(args[0], class_name, field_name)
        v = python_type_to_rust(args[1], class_name, field_name)
        return f"IndexMap<{k}, {v}>"

    raise TypeError(f"Unsupported type {py_type} for {class_name}.{field_name}")


def extract_classes(module) -> tuple[dict, dict]:
    """Extract dataclasses and enums from a module using runtime introspection."""

    dataclasses_dict = {}
    enums_dict = {}

    for name, cls in inspect.getmembers(module, inspect.isclass):
        # Skip imported classes (only process classes defined in this module).
        if cls.__module__ != module.__name__:
            continue

        if is_dataclass(cls):
            hints = get_type_hints(cls)
            field_list = [
                (f.name, python_type_to_rust(hints[f.name], name, f.name))
                for f in fields(cls)
            ]
            dataclasses_dict[name] = field_list
        elif issubclass(cls, Enum):
            enums_dict[name] = [member.name for member in cls]

    return dataclasses_dict, enums_dict


def generate_enum(name: str, const_variants: list[str]) -> str:
    """Generates a Rust enum."""

    return ENUM_TEMPLATE.format(
        name=name,
        fields="\n".join(
            [
                f"    {to_rust_enum_variant_name(v)} = {i},"
                for i, v in enumerate(const_variants, 1)
            ]
        ),
    )


def generate_struct(name: str, struct_fields: list[tuple[str, str]]) -> str:
    """Generates a Rust struct."""

    derives = (
        ["Clone", "Debug", "Deserialize"]
        + (["Eq"] if name not in NO_EQ_OVERRIDES else [])
        + ["PartialEq", "Serialize"]
    )

    field_lines = []
    for field_name, rust_type in struct_fields:
        rust_name, needs_rename = to_rust_field_name(field_name)
        if needs_rename:
            field_lines.append(f'    #[serde(rename = "{field_name}")]')
        field_lines.append(f"    pub {rust_name}: {rust_type},")

    return STRUCT_TEMPLATE.format(
        derives=", ".join(derives), name=name, fields="\n".join(field_lines)
    )


def generate_rust(dataclasses_dict: dict, enums_dict: dict) -> str:
    """Generate complete Rust code from extracted classes."""

    sections = [RUST_HEADER]
    for name in sorted(
        dataclasses_dict.keys() | enums_dict.keys() | TEMPLATE_OVERRIDES.keys()
    ):
        if template := TEMPLATE_OVERRIDES.get(name):
            sections.append(template)
        elif name in enums_dict:
            sections.append(generate_enum(name, enums_dict[name]))
        elif name in dataclasses_dict:
            sections.append(generate_struct(name, dataclasses_dict[name]))
    return "\n\n".join(sections) + "\n"


class EnumEncoder(JSONEncoder):
    """Converts enum to integer.."""

    def default(self, obj):
        if isinstance(obj, Enum):
            return obj.value
        return super().default(obj)


if __name__ == "__main__":
    # Generate vk.json.
    vulkan_object = get_vulkan_object(video=True)
    with open("src/vk.json", "w", encoding="utf-8", newline="\n") as f:
        dump(asdict(vulkan_object), f, indent=2, cls=EnumEncoder)
    print("Generated src/vk.json")

    # Generate vulkan_object.rs.
    dataclasses, enums = extract_classes(vulkan_object_module)
    rust_code = generate_rust(dataclasses, enums)
    (Path(__file__).parent / "src" / "vulkan_object.rs").write_text(
        rust_code, encoding="utf-8", newline="\n"
    )
    print("Generated src/vulkan_object.rs")
