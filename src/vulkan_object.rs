use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Bitmask {
    pub name: String,
    pub aliases: Vec<String>,
    #[serde(rename = "flagName")]
    pub flag_name: String,
    pub protect: Option<String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: u32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    pub flags: Vec<Flag>,
    pub extensions: Vec<String>,
    #[serde(rename = "flagExtensions")]
    pub flag_extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Command {
    pub name: String,
    pub alias: Option<String>,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
    pub version: Option<Version>,
    #[serde(rename = "returnType")]
    pub return_type: String,
    pub params: Vec<Param>,
    pub instance: bool,
    pub device: bool,
    pub tasks: Vec<String>,
    pub queues: Vec<String>,
    #[serde(rename = "allowNoQueues")]
    pub allow_no_queues: bool,
    #[serde(rename = "successCodes")]
    pub success_codes: Vec<String>,
    #[serde(rename = "errorCodes")]
    pub error_codes: Vec<String>,
    pub primary: bool,
    pub secondary: bool,
    #[serde(rename = "renderPass")]
    pub render_pass: CommandScope,
    #[serde(rename = "videoCoding")]
    pub video_coding: CommandScope,
    #[serde(rename = "implicitExternSyncParams")]
    pub implicit_extern_sync_params: Vec<String>,
    pub legacy: Option<Legacy>,
    #[serde(rename = "cPrototype")]
    pub c_prototype: String,
    #[serde(rename = "cFunctionPointer")]
    pub c_function_pointer: String,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum CommandScope {
    None = 1,
    Inside = 2,
    Outside = 3,
    Both = 4,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Constant {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: ConstantValue,
    #[serde(rename = "valueStr")]
    pub value_str: String,
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ConstantValue {
    Int(u64),
    Float(f64),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Enum {
    pub name: String,
    pub aliases: Vec<String>,
    pub protect: Option<String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: u32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    pub fields: Vec<EnumField>,
    pub extensions: Vec<String>,
    #[serde(rename = "fieldExtensions")]
    pub field_extensions: Vec<String>,
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnumField {
    pub name: String,
    pub aliases: Vec<String>,
    pub protect: Option<String>,
    pub negative: bool,
    pub value: i64,
    #[serde(rename = "valueStr")]
    pub value_str: String,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Extension {
    pub name: String,
    #[serde(rename = "nameString")]
    pub name_string: String,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    #[serde(rename = "specVersionValue")]
    pub spec_version_value: u32,
    pub instance: bool,
    pub device: bool,
    pub depends: Option<String>,
    #[serde(rename = "vendorTag")]
    pub vendor_tag: Option<String>,
    pub platform: Option<String>,
    pub protect: Option<String>,
    pub provisional: bool,
    #[serde(rename = "promotedTo")]
    pub promoted_to: Option<String>,
    #[serde(rename = "deprecatedBy")]
    pub deprecated_by: Option<String>,
    #[serde(rename = "obsoletedBy")]
    pub obsoleted_by: Option<String>,
    #[serde(rename = "specialUse")]
    pub special_use: Vec<String>,
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: Vec<FeatureRequirement>,
    pub ratified: bool,
    pub handles: Vec<Handle>,
    pub commands: Vec<Command>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub bitmasks: Vec<Bitmask>,
    pub flags: IndexMap<String, Vec<Flags>>,
    #[serde(rename = "enumFields")]
    pub enum_fields: IndexMap<String, Vec<EnumField>>,
    #[serde(rename = "flagBits")]
    pub flag_bits: IndexMap<String, Vec<Flag>>,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum ExternSync {
    None = 1,
    Always = 2,
    Maybe = 3,
    Subtype = 4,
    SubtypeMaybe = 5,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeatureRequirement {
    #[serde(rename = "struct")]
    pub struct_: String,
    pub field: String,
    pub depends: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Flag {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub protect: Option<String>,
    pub value: u64,
    #[serde(rename = "valueStr")]
    pub value_str: String,
    pub bitpos: Option<u32>,
    #[serde(rename = "multiBit")]
    pub multi_bit: bool,
    pub zero: bool,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Flags {
    pub name: String,
    pub aliases: Vec<String>,
    #[serde(rename = "bitmaskName")]
    pub bitmask_name: Option<String>,
    pub protect: Option<String>,
    #[serde(rename = "baseFlagsType")]
    pub base_flags_type: String,
    #[serde(rename = "bitWidth")]
    pub bit_width: u32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Format {
    pub name: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "blockSize")]
    pub block_size: u32,
    #[serde(rename = "texelsPerBlock")]
    pub texels_per_block: u32,
    #[serde(rename = "blockExtent")]
    pub block_extent: Vec<String>,
    pub packed: Option<u32>,
    pub chroma: Option<String>,
    pub compressed: Option<String>,
    pub components: Vec<FormatComponent>,
    pub planes: Vec<FormatPlane>,
    #[serde(rename = "spirvImageFormat")]
    pub spirv_image_format: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormatComponent {
    #[serde(rename = "type")]
    pub type_: String,
    pub bits: String,
    #[serde(rename = "numericFormat")]
    pub numeric_format: String,
    #[serde(rename = "planeIndex")]
    pub plane_index: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormatPlane {
    pub index: u32,
    #[serde(rename = "widthDivisor")]
    pub width_divisor: u32,
    #[serde(rename = "heightDivisor")]
    pub height_divisor: u32,
    pub compatible: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FuncPointer {
    pub name: String,
    pub protect: Option<String>,
    #[serde(rename = "returnType")]
    pub return_type: String,
    pub requires: Option<String>,
    pub params: Vec<FuncPointerParam>,
    #[serde(rename = "cFunctionPointer")]
    pub c_function_pointer: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FuncPointerParam {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fullType")]
    pub full_type: String,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Handle {
    pub name: String,
    pub aliases: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub protect: Option<String>,
    pub parent: Option<Box<Handle>>,
    pub instance: bool,
    pub device: bool,
    pub dispatchable: bool,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Legacy {
    pub link: Option<String>,
    pub version: Option<Version>,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Member {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fullType")]
    pub full_type: String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "limitType")]
    pub limit_type: Option<String>,
    #[serde(rename = "const")]
    pub const_: bool,
    pub length: Option<String>,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub pointer: bool,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: Vec<String>,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: ExternSync,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: String,
    #[serde(rename = "bitFieldWidth")]
    pub bit_field_width: Option<u32>,
    pub selector: Option<String>,
    pub selection: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Param {
    pub name: String,
    pub alias: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fullType")]
    pub full_type: String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "const")]
    pub const_: bool,
    pub length: Option<String>,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub pointer: bool,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: Vec<String>,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: ExternSync,
    #[serde(rename = "externSyncPointer")]
    pub extern_sync_pointer: Option<String>,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Spirv {
    pub name: String,
    pub extension: bool,
    pub capability: bool,
    pub enable: Vec<SpirvEnables>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SpirvEnables {
    pub version: Option<String>,
    pub extension: Option<String>,
    #[serde(rename = "struct")]
    pub struct_: Option<String>,
    pub feature: Option<String>,
    pub requires: Option<String>,
    pub property: Option<String>,
    pub member: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Struct {
    pub name: String,
    pub aliases: Vec<String>,
    pub extensions: Vec<String>,
    pub version: Option<Version>,
    pub protect: Option<String>,
    pub members: Vec<Member>,
    pub union: bool,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    #[serde(rename = "sType")]
    pub s_type: Option<String>,
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,
    pub extends: Vec<String>,
    #[serde(rename = "extendedBy")]
    pub extended_by: Vec<String>,
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncAccess {
    pub flag: Flag,
    pub support: SyncSupport,
    pub equivalent: SyncEquivalent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncEquivalent {
    pub stages: Option<Vec<Flag>>,
    pub accesses: Option<Vec<Flag>>,
    pub max: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncPipeline {
    pub name: String,
    pub depends: Vec<String>,
    pub stages: Vec<SyncPipelineStage>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncPipelineStage {
    pub order: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncStage {
    pub flag: Flag,
    pub support: SyncSupport,
    pub equivalent: SyncEquivalent,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SyncSupport {
    pub queues: Option<Vec<String>>,
    pub stages: Option<Vec<Flag>>,
    pub max: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Version {
    pub name: String,
    #[serde(rename = "nameString")]
    pub name_string: String,
    #[serde(rename = "nameApi")]
    pub name_api: String,
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: Vec<FeatureRequirement>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoCodec {
    pub name: String,
    pub value: Option<String>,
    pub profiles: IndexMap<String, VideoProfiles>,
    pub capabilities: IndexMap<String, String>,
    pub formats: IndexMap<String, VideoFormat>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoFormat {
    pub name: String,
    pub usage: String,
    #[serde(rename = "requiredCaps")]
    pub required_caps: Vec<VideoRequiredCapabilities>,
    pub properties: IndexMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoProfileMember {
    pub name: String,
    pub values: IndexMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoProfiles {
    pub name: String,
    pub members: IndexMap<String, VideoProfileMember>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoRequiredCapabilities {
    #[serde(rename = "struct")]
    pub struct_: String,
    pub member: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VideoStd {
    pub headers: IndexMap<String, VideoStdHeader>,
    pub enums: IndexMap<String, Enum>,
    pub structs: IndexMap<String, Struct>,
    pub constants: IndexMap<String, Constant>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoStdHeader {
    pub name: String,
    pub version: Option<String>,
    #[serde(rename = "headerFile")]
    pub header_file: String,
    pub depends: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VulkanObject {
    #[serde(rename = "headerVersion")]
    pub header_version: String,
    #[serde(rename = "headerVersionComplete")]
    pub header_version_complete: String,
    pub extensions: IndexMap<String, Extension>,
    pub versions: IndexMap<String, Version>,
    pub handles: IndexMap<String, Handle>,
    pub commands: IndexMap<String, Command>,
    pub structs: IndexMap<String, Struct>,
    pub enums: IndexMap<String, Enum>,
    pub bitmasks: IndexMap<String, Bitmask>,
    pub flags: IndexMap<String, Flags>,
    pub constants: IndexMap<String, Constant>,
    pub formats: IndexMap<String, Format>,
    #[serde(rename = "funcPointers")]
    pub func_pointers: IndexMap<String, FuncPointer>,
    #[serde(rename = "syncStage")]
    pub sync_stage: Vec<SyncStage>,
    #[serde(rename = "syncAccess")]
    pub sync_access: Vec<SyncAccess>,
    #[serde(rename = "syncPipeline")]
    pub sync_pipeline: Vec<SyncPipeline>,
    pub spirv: Vec<Spirv>,
    pub platforms: IndexMap<String, String>,
    #[serde(rename = "vendorTags")]
    pub vendor_tags: Vec<String>,
    #[serde(rename = "videoCodecs")]
    pub video_codecs: IndexMap<String, VideoCodec>,
    #[serde(rename = "videoStd")]
    pub video_std: Option<VideoStd>,
}
