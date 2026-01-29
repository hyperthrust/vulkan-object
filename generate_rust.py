import ast
import re
from pathlib import Path

# Type overrides for known inconsistencies.
FIELD_TYPE_OVERRIDES = {
    ("Flag", "aliases"): "Option<Vec<String>>",
    ("SyncSupport", "queues"): "Option<Vec<String>>",
    ("SyncSupport", "stages"): "Option<Vec<Flag>>",
    ("SyncEquivalent", "stages"): "Option<Vec<Flag>>",
    ("SyncEquivalent", "accesses"): "Option<Vec<Flag>>",
    ("VulkanObject", "headerVersion"): "String",
    ("Param", "alias"): "Option<String>",
    ("Constant", "value"): "ConstantValue",
}

# Fields that need Box<> for recursive types.
BOXED_FIELDS = {("Handle", "parent")}

# Fields where int maps to specific Rust types.
INT_TYPE_OVERRIDES = {
    ("EnumField", "value"): "i64",
    ("Flag", "value"): "u64",
}

RESERVED_KEYWORDS = {"type", "struct", "const"}


def to_snake_case(name: str) -> str:
    return re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()


def parse_type(node, class_name: str, field_name: str) -> str:
    override = FIELD_TYPE_OVERRIDES.get((class_name, field_name))
    if override:
        return override
    if isinstance(node, ast.Constant) and node.value is None:
        return "None"
    if isinstance(node, ast.Name):
        name = node.id
        if name == "str":
            return "String"
        if name == "bool":
            return "bool"
        if name == "int":
            return INT_TYPE_OVERRIDES.get((class_name, field_name), "u32")
        if name == "float":
            return "f64"
        return name
    if isinstance(node, ast.Subscript):
        base = node.value.id if isinstance(node.value, ast.Name) else str(node.value)
        if base == "list":
            inner = parse_type(node.slice, class_name, field_name)
            return f"Vec<{inner}>"
        if base == "dict":
            if isinstance(node.slice, ast.Tuple):
                key = parse_type(node.slice.elts[0], class_name, field_name)
                val = parse_type(node.slice.elts[1], class_name, field_name)
                return f"IndexMap<{key}, {val}>"
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
        left = parse_type(node.left, class_name, field_name)
        right = parse_type(node.right, class_name, field_name)
        if right == "None":
            if (class_name, field_name) in BOXED_FIELDS:
                return f"Option<Box<{left}>>"
            return f"Option<{left}>"
        if left == "int" and right == "float":
            return "ConstantValue"
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        name = node.value
        if (class_name, field_name) in BOXED_FIELDS:
            return f"Option<Box<{name}>>"
        return name
    return "UNKNOWN"


def extract_classes(source: str) -> tuple[dict, dict]:
    tree = ast.parse(source)
    dataclasses = {}
    enums = {}
    for node in ast.walk(tree):
        if isinstance(node, ast.ClassDef):
            is_dataclass = any(
                (isinstance(d, ast.Name) and d.id == "dataclass")
                or (
                    isinstance(d, ast.Call)
                    and isinstance(d.func, ast.Name)
                    and d.func.id == "dataclass"
                )
                for d in node.decorator_list
            )
            is_enum = any(
                isinstance(b, ast.Name) and b.id == "Enum" for b in node.bases
            )
            if is_enum:
                values = []
                for item in node.body:
                    if isinstance(item, ast.Assign) and isinstance(
                        item.targets[0], ast.Name
                    ):
                        values.append(item.targets[0].id)
                enums[node.name] = values
            elif is_dataclass:
                fields = []
                for item in node.body:
                    if isinstance(item, ast.AnnAssign) and isinstance(
                        item.target, ast.Name
                    ):
                        field_name = item.target.id
                        rust_type = parse_type(item.annotation, node.name, field_name)
                        fields.append((field_name, rust_type))
                dataclasses[node.name] = fields
    return dataclasses, enums


def generate_rust(dataclasses: dict, enums: dict) -> str:
    lines = [
        "use indexmap::IndexMap;",
        "use serde::{Deserialize, Serialize};",
        "use serde_repr::{Deserialize_repr, Serialize_repr};",
        "",
    ]
    # f64 doesn't impl Eq, so types containing ConstantValue can't derive Eq
    no_eq = {"ConstantValue", "Constant", "VideoStd", "VulkanObject"}
    all_names = sorted(set(dataclasses.keys()) | set(enums.keys()) | {"ConstantValue"})
    for name in all_names:
        if name == "ConstantValue":
            lines += [
                "#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]",
                "#[serde(untagged)]",
                "pub enum ConstantValue {",
                "    Int(u64),",
                "    Float(f64),",
                "}",
                "",
            ]
        elif name in enums:
            lines.append(
                "#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]"
            )
            lines.append("#[repr(u8)]")
            lines.append(f"pub enum {name} {{")
            for i, v in enumerate(enums[name], 1):
                rust_name = v.replace("_", " ").title().replace(" ", "")
                lines.append(f"    {rust_name} = {i},")
            lines.append("}")
            lines.append("")
        elif name in dataclasses:
            derives = ["Clone", "Debug"]
            if name == "VideoStd":
                derives.append("Default")
            derives.append("Deserialize")
            if name not in no_eq:
                derives.append("Eq")
            derives.append("PartialEq")
            derives.append("Serialize")
            lines.append(f"#[derive({', '.join(derives)})]")
            lines.append(f"pub struct {name} {{")
            for field_name, rust_type in dataclasses[name]:
                snake_name = to_snake_case(field_name)
                needs_rename = (
                    snake_name != field_name or field_name in RESERVED_KEYWORDS
                )
                if field_name in RESERVED_KEYWORDS:
                    snake_name = field_name + "_"
                if needs_rename:
                    lines.append(f'    #[serde(rename = "{field_name}")]')
                lines.append(f"    pub {snake_name}: {rust_type},")
            lines.append("}")
            lines.append("")
    return "\n".join(lines)


def main():
    script_dir = Path(__file__).parent
    source = (script_dir / "_vulkan_object.py").read_text(encoding="utf-8")
    dataclasses, enums = extract_classes(source)
    rust_code = generate_rust(dataclasses, enums)
    (script_dir / "src" / "vulkan_object.rs").write_text(
        rust_code, encoding="utf-8", newline="\n"
    )
    print("Generated src/vulkan_object.rs")


if __name__ == "__main__":
    main()
