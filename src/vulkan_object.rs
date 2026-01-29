// Copyright 2023-2026 The Khronos Group Inc.
//
// SPDX-License-Identifier: Apache-2.0

//! Rust bindings for vulkan-object, mirroring the Python API.
//!
//! This module provides typed access to Vulkan API metadata parsed from the
//! Vulkan XML specification.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

/// Each instance of FeatureRequirement is one part of the AND operation,
/// unless the struct/field are the same, then the depends are AND together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRequirement {
    #[serde(rename = "struct")]
    pub struct_: String,
    /// Can have comma delimiter, which are expressed as OR
    pub field: String,
    /// ex) "VK_EXT_descriptor_indexing", "VK_VERSION_1_2+VkPhysicalDeviceVulkan12Features::descriptorIndexing"
    pub depends: Option<String>,
}

/// `<extension>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    /// ex) VK_KHR_SURFACE
    pub name: String,
    /// Macro with string, ex) VK_KHR_SURFACE_EXTENSION_NAME
    #[serde(rename = "nameString")]
    pub name_string: String,
    /// Macro with string, ex) VK_KHR_SURFACE_SPEC_VERSION
    #[serde(rename = "specVersion")]
    pub spec_version: String,

    /// Only one will be True, the other is False
    pub instance: bool,
    pub device: bool,

    pub depends: Option<String>,
    /// ex) EXT, KHR, etc
    #[serde(rename = "vendorTag")]
    pub vendor_tag: Option<String>,
    /// ex) android
    pub platform: Option<String>,
    /// ex) VK_USE_PLATFORM_ANDROID_KHR
    pub protect: Option<String>,
    pub provisional: bool,
    /// ex) VK_VERSION_1_1
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

    // Reverse lookup fields
    pub handles: Vec<Handle>,
    pub commands: Vec<Command>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub bitmasks: Vec<Bitmask>,
    /// Use the Flags name to see what fields are added
    pub flags: HashMap<String, Vec<Flags>>,
    /// Use the Enum name to see what fields are extended
    #[serde(rename = "enumFields")]
    pub enum_fields: HashMap<String, Vec<EnumField>>,
    /// Use the Bitmask name to see what flag bits are added to it
    #[serde(rename = "flagBits")]
    pub flag_bits: HashMap<String, Vec<Flag>>,
}

/// `<feature>` which represents a version.
/// This will NEVER be Version 1.0, since having 'no version' is same as being 1.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    /// ex) VK_VERSION_1_1
    pub name: String,
    /// ex) "VK_VERSION_1_1" (no macro, so has quotes)
    #[serde(rename = "nameString")]
    pub name_string: String,
    /// ex) VK_API_VERSION_1_1
    #[serde(rename = "nameApi")]
    pub name_api: String,

    #[serde(rename = "featureRequirement")]
    pub feature_requirement: Vec<FeatureRequirement>,
}

/// `<deprecate>`
/// For historical reasons, the XML tag is "deprecate" but we decided in the WG
/// to not use that as the public facing name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legacy {
    /// Spec URL Anchor - ex) legacy-dynamicrendering
    pub link: Option<String>,
    pub version: Option<Box<Version>>,
    pub extensions: Vec<String>,
}

/// `<type>` which represents a dispatch handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    /// ex) VkBuffer
    pub name: String,
    /// ex) ['VkSamplerYcbcrConversionKHR']
    pub aliases: Vec<String>,

    /// ex) VK_OBJECT_TYPE_BUFFER
    #[serde(rename = "type")]
    pub type_: String,
    /// ex) VK_USE_PLATFORM_ANDROID_KHR
    pub protect: Option<String>,

    /// Chain of parent handles, can be None
    pub parent: Option<Box<Handle>>,

    /// Only one will be True, the other is False
    pub instance: bool,
    pub device: bool,

    pub dispatchable: bool,

    /// All extensions that enable the handle
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ExternSync {
    /// no externsync attribute
    None = 1,
    /// externsync="true"
    Always = 2,
    /// externsync="maybe"
    Maybe = 3,
    /// externsync="param->member"
    Subtype = 4,
    /// externsync="maybe:param->member"
    SubtypeMaybe = 5,
}

/// `<command/param>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// ex) pCreateInfo, pAllocator, pBuffer
    pub name: String,
    pub alias: Option<String>,

    /// The "base type" - will not preserve the 'const' or pointer info.
    /// ex) void, uint32_t, VkFormat, VkBuffer, etc
    #[serde(rename = "type")]
    pub type_: String,
    /// The "full type" - will be cDeclaration without the type name.
    /// ex) const void*, uint32_t, const VkFormat, VkBuffer*, etc
    /// For arrays, this will only display the type, fixedSizeArray can be used to get the length.
    #[serde(rename = "fullType")]
    pub full_type: String,

    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,

    /// type contains 'const'
    #[serde(rename = "const")]
    pub const_: bool,
    /// The known length of pointer, will never be 'null-terminated'
    pub length: Option<String>,
    /// If a UTF-8 string, it will be null-terminated
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    /// type contains a pointer (includes 'PFN' function pointers)
    pub pointer: bool,
    /// Used to list how large an array of the type is.
    /// ex) lineWidthRange is ['2']
    /// ex) memoryTypes is ['VK_MAX_MEMORY_TYPES']
    /// ex) VkTransformMatrixKHR:matrix is ['3', '4']
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: Vec<String>,

    pub optional: bool,
    /// If type contains a pointer, is the pointer value optional
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,

    #[serde(rename = "externSync")]
    pub extern_sync: ExternSync,
    /// If type contains a pointer (externSync is SUBTYPE*),
    /// only a specific member is externally synchronized.
    #[serde(rename = "externSyncPointer")]
    pub extern_sync_pointer: Option<String>,

    /// C string of member, example:
    ///   - const void* pNext
    ///   - VkFormat format
    ///   - VkStructureType sType
    #[serde(rename = "cDeclaration")]
    pub c_declaration: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CommandScope {
    None = 1,
    Inside = 2,
    Outside = 3,
    Both = 4,
}

/// `<command>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// ex) vkCmdDraw
    pub name: String,
    /// Because commands are interfaces into layers/drivers, we need all command alias
    pub alias: Option<String>,
    /// ex) 'VK_ENABLE_BETA_EXTENSIONS'
    pub protect: Option<String>,

    /// All extensions that enable the struct
    pub extensions: Vec<String>,
    /// None if Version 1.0
    pub version: Option<Box<Version>>,

    /// ex) void, VkResult, etc
    #[serde(rename = "returnType")]
    pub return_type: String,

    /// Each parameter of the command
    pub params: Vec<Param>,

    /// Only one will be True, the other is False
    pub instance: bool,
    pub device: bool,

    /// ex) [ action, state, synchronization ]
    pub tasks: Vec<String>,
    /// ex) [ VK_QUEUE_GRAPHICS_BIT, VK_QUEUE_COMPUTE_BIT ]
    pub queues: Vec<String>,
    /// VK_KHR_maintenance9 allows some calls to be done with zero queues
    #[serde(rename = "allowNoQueues")]
    pub allow_no_queues: bool,
    /// ex) [ VK_SUCCESS, VK_INCOMPLETE ]
    #[serde(rename = "successCodes")]
    pub success_codes: Vec<String>,
    /// ex) [ VK_ERROR_OUT_OF_HOST_MEMORY ]
    #[serde(rename = "errorCodes")]
    pub error_codes: Vec<String>,

    /// Shows support if command can be in a primary and/or secondary command buffer
    pub primary: bool,
    pub secondary: bool,

    #[serde(rename = "renderPass")]
    pub render_pass: CommandScope,
    #[serde(rename = "videoCoding")]
    pub video_coding: CommandScope,

    #[serde(rename = "implicitExternSyncParams")]
    pub implicit_extern_sync_params: Vec<String>,

    pub legacy: Option<Box<Legacy>>,

    /// C prototype string - ex:
    /// VKAPI_ATTR VkResult VKAPI_CALL vkCreateInstance(
    ///   const VkInstanceCreateInfo* pCreateInfo,
    ///   const VkAllocationCallbacks* pAllocator,
    ///   VkInstance* pInstance);
    #[serde(rename = "cPrototype")]
    pub c_prototype: String,

    /// Function pointer typedef - ex:
    /// typedef VkResult (VKAPI_PTR *PFN_vkCreateInstance)
    ///   (const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance);
    #[serde(rename = "cFunctionPointer")]
    pub c_function_pointer: String,
}

/// `<member>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// ex) sType, pNext, flags, size, usage
    pub name: String,

    /// The "base type" - will not preserve the 'const' or pointer info.
    /// ex) void, uint32_t, VkFormat, VkBuffer, etc
    #[serde(rename = "type")]
    pub type_: String,
    /// The "full type" - will be cDeclaration without the type name.
    /// ex) const void*, uint32_t, const VkFormat, VkBuffer*, etc
    /// For arrays, this will only display the type, fixedSizeArray can be used to get the length.
    #[serde(rename = "fullType")]
    pub full_type: String,

    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    /// ex) 'max', 'bitmask', 'bits', 'min,mul'
    #[serde(rename = "limitType")]
    pub limit_type: Option<String>,

    /// type contains 'const'
    #[serde(rename = "const")]
    pub const_: bool,
    /// The known length of pointer, will never be 'null-terminated'
    pub length: Option<String>,
    /// If a UTF-8 string, it will be null-terminated
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    /// type contains a pointer (includes 'PFN' function pointers)
    pub pointer: bool,
    /// Used to list how large an array of the type is.
    /// ex) lineWidthRange is ['2']
    /// ex) memoryTypes is ['VK_MAX_MEMORY_TYPES']
    /// ex) VkTransformMatrixKHR:matrix is ['3', '4']
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: Vec<String>,

    pub optional: bool,
    /// If type contains a pointer, is the pointer value optional
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,

    #[serde(rename = "externSync")]
    pub extern_sync: ExternSync,

    /// C string of member, example:
    ///   - const void* pNext
    ///   - VkFormat format
    ///   - VkStructureType sType
    #[serde(rename = "cDeclaration")]
    pub c_declaration: String,

    /// Bit width (only for bit field struct members)
    #[serde(rename = "bitFieldWidth")]
    pub bit_field_width: Option<i32>,

    /// Selector for the union, this type determines the used data type in the union
    pub selector: Option<String>,
    /// Valid selections for the union member
    pub selection: Vec<String>,
}

/// `<type category="struct">` or `<type category="union">`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    /// ex) VkImageSubresource2
    pub name: String,
    /// ex) ['VkImageSubresource2KHR', 'VkImageSubresource2EXT']
    pub aliases: Vec<String>,

    /// All extensions that enable the struct
    pub extensions: Vec<String>,
    /// None if Version 1.0
    pub version: Option<Box<Version>>,
    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    pub members: Vec<Member>,

    /// Unions are just a subset of a Structs
    pub union: bool,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,

    /// ex) VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO
    #[serde(rename = "sType")]
    pub s_type: Option<String>,
    /// Can have a pNext point to itself
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,

    /// Struct names that this struct extends
    pub extends: Vec<String>,
    /// Struct names that can be extended by this struct
    #[serde(rename = "extendedBy")]
    pub extended_by: Vec<String>,

    /// This field is only set for enum definitions coming from Video Std headers
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

/// `<enum>` of type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumField {
    /// ex) VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT
    pub name: String,
    /// ex) ['VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT']
    pub aliases: Vec<String>,

    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    /// True if negative values are allowed (ex. VkResult)
    pub negative: bool,
    pub value: i64,
    /// Value as shown in spec (ex. "0", "2", "1000267000", "0x00000004")
    #[serde(rename = "valueStr")]
    pub value_str: String,

    /// Some fields are enabled from 2 extensions (ex) VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR)
    /// None if part of 1.0 core
    pub extensions: Vec<String>,
}

/// `<enums>` of type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    /// ex) VkLineRasterizationMode
    pub name: String,
    /// ex) ['VkLineRasterizationModeKHR', 'VkLineRasterizationModeEXT']
    pub aliases: Vec<String>,

    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    /// 32 or 64 (currently all are 32, but field is to match with Bitmask)
    #[serde(rename = "bitWidth")]
    pub bit_width: i32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,

    pub fields: Vec<EnumField>,

    /// None if part of 1.0 core
    pub extensions: Vec<String>,
    /// Unique list of all extensions that are involved in 'fields' (superset of 'extensions')
    #[serde(rename = "fieldExtensions")]
    pub field_extensions: Vec<String>,

    /// This field is only set for enum definitions coming from Video Std headers
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

/// `<enum>` of type bitmask
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    /// ex) VK_ACCESS_2_SHADER_READ_BIT
    pub name: String,
    /// ex) ['VK_ACCESS_2_SHADER_READ_BIT_KHR']
    pub aliases: Option<Vec<String>>,

    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    pub value: u64,
    /// Value as shown in spec (ex. "0x00000000", "0x00000004", "0x0000000F", "0x800000000ULL")
    #[serde(rename = "valueStr")]
    pub value_str: String,
    /// If true, more than one bit is set (ex) VK_SHADER_STAGE_ALL_GRAPHICS)
    #[serde(rename = "multiBit")]
    pub multi_bit: bool,
    /// If true, the value is zero (ex) VK_PIPELINE_STAGE_NONE)
    pub zero: bool,

    /// Some fields are enabled from 2 extensions (ex) VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT)
    /// None if part of 1.0 core
    pub extensions: Vec<String>,
}

/// `<enums>` of type bitmask
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bitmask {
    /// ex) VkAccessFlagBits2
    pub name: String,
    /// ex) ['VkAccessFlagBits2KHR']
    pub aliases: Vec<String>,

    /// ex) VkAccessFlags2
    #[serde(rename = "flagName")]
    pub flag_name: String,
    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    /// 32 or 64
    #[serde(rename = "bitWidth")]
    pub bit_width: i32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,

    pub flags: Vec<Flag>,

    /// None if part of 1.0 core
    pub extensions: Vec<String>,
    /// Unique list of all extensions that are involved in 'flag' (superset of 'extensions')
    #[serde(rename = "flagExtensions")]
    pub flag_extensions: Vec<String>,
}

/// `<type>` defining flags types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    /// ex) VkAccessFlags2
    pub name: String,
    /// ex) ['VkAccessFlags2KHR']
    pub aliases: Vec<String>,

    /// ex) VkAccessFlagBits2
    #[serde(rename = "bitmaskName")]
    pub bitmask_name: Option<String>,
    /// ex) VK_ENABLE_BETA_EXTENSIONS
    pub protect: Option<String>,

    /// ex) VkFlags
    #[serde(rename = "baseFlagsType")]
    pub base_flags_type: String,
    /// 32 or 64
    #[serde(rename = "bitWidth")]
    pub bit_width: i32,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,

    /// None if part of 1.0 core
    pub extensions: Vec<String>,
}

/// Value for a constant (can be integer or float)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstantValue {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    /// ex) VK_UUID_SIZE
    pub name: String,
    /// ex) uint32_t, float
    #[serde(rename = "type")]
    pub type_: String,
    pub value: ConstantValue,
    /// Value as shown in spec (ex. "(~0U)", "256U", etc)
    #[serde(rename = "valueStr")]
    pub value_str: String,

    /// This field is only set for enum definitions coming from Video Std headers
    #[serde(rename = "videoStdHeader")]
    pub video_std_header: Option<String>,
}

/// `<format/component>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatComponent {
    /// ex) R, G, B, A, D, S, etc
    #[serde(rename = "type")]
    pub type_: String,
    /// Will be an INT or 'compressed'
    pub bits: String,
    /// ex) UNORM, SINT, etc
    #[serde(rename = "numericFormat")]
    pub numeric_format: String,
    /// None if no planeIndex in format
    #[serde(rename = "planeIndex")]
    pub plane_index: Option<i32>,
}

/// `<format/plane>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatPlane {
    pub index: i32,
    #[serde(rename = "widthDivisor")]
    pub width_divisor: i32,
    #[serde(rename = "heightDivisor")]
    pub height_divisor: i32,
    pub compatible: String,
}

/// `<format>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub name: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "blockSize")]
    pub block_size: i32,
    #[serde(rename = "texelsPerBlock")]
    pub texels_per_block: i32,
    #[serde(rename = "blockExtent")]
    pub block_extent: Vec<String>,
    /// None == not-packed
    pub packed: Option<i32>,
    pub chroma: Option<String>,
    pub compressed: Option<String>,
    /// `<format/component>`
    pub components: Vec<FormatComponent>,
    /// `<format/plane>`
    pub planes: Vec<FormatPlane>,
    #[serde(rename = "spirvImageFormat")]
    pub spirv_image_format: Option<String>,
}

/// `<syncsupport>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSupport {
    /// ex) [ VK_QUEUE_GRAPHICS_BIT, VK_QUEUE_COMPUTE_BIT ]
    pub queues: Option<Vec<String>>,
    /// VkPipelineStageFlagBits2
    pub stages: Option<Vec<Flag>>,
    /// If this supports max values
    pub max: bool,
}

/// `<syncequivalent>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEquivalent {
    /// VkPipelineStageFlagBits2
    pub stages: Option<Vec<Flag>>,
    /// VkAccessFlagBits2
    pub accesses: Option<Vec<Flag>>,
    /// If this equivalent to everything
    pub max: bool,
}

/// `<syncstage>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStage {
    /// VkPipelineStageFlagBits2
    pub flag: Flag,
    pub support: SyncSupport,
    pub equivalent: SyncEquivalent,
}

/// `<syncaccess>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAccess {
    /// VkAccessFlagBits2
    pub flag: Flag,
    pub support: SyncSupport,
    pub equivalent: SyncEquivalent,
}

/// `<syncpipelinestage>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPipelineStage {
    pub order: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub value: String,
}

/// `<syncpipeline>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPipeline {
    pub name: String,
    pub depends: Vec<String>,
    pub stages: Vec<SyncPipelineStage>,
}

/// What is needed to enable the SPIR-V element
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// `<spirvextension>` and `<spirvcapability>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spirv {
    pub name: String,
    /// Only one will be True, the other is False
    pub extension: bool,
    pub capability: bool,
    pub enable: Vec<SpirvEnables>,
}

/// `<videorequirecapabilities>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRequiredCapabilities {
    /// ex) VkVideoEncodeCapabilitiesKHR
    #[serde(rename = "struct")]
    pub struct_: String,
    /// ex) flags
    pub member: String,
    /// ex) VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR
    /// May contain XML boolean expressions ("+" means AND, "," means OR)
    pub value: String,
}

/// `<videoformat>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    /// ex) Decode Output
    pub name: String,
    /// ex) VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR
    /// May contain XML boolean expressions ("+" means AND, "," means OR)
    pub usage: String,

    #[serde(rename = "requiredCaps")]
    pub required_caps: Vec<VideoRequiredCapabilities>,
    pub properties: HashMap<String, String>,
}

/// `<videoprofilemember>` and `<videoprofile>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoProfileMember {
    pub name: String,
    /// Video profile struct member (value attribute of `<videoprofile>`) value as key,
    /// profile name substring (name attribute of `<videoprofile>`) as value
    pub values: HashMap<String, String>,
}

/// `<videoprofiles>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoProfiles {
    pub name: String,
    pub members: HashMap<String, VideoProfileMember>,
}

/// `<videocodec>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCodec {
    /// ex) H.264 Decode
    pub name: String,
    /// If no video codec operation flag bit is associated with the codec
    /// then it is a codec category (e.g. decode, encode), not a specific codec
    pub value: Option<String>,

    pub profiles: HashMap<String, VideoProfiles>,
    pub capabilities: HashMap<String, String>,
    pub formats: HashMap<String, VideoFormat>,
}

/// `<extension>` in video.xml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStdHeader {
    /// ex) vulkan_video_codec_h264std_decode
    pub name: String,
    /// ex) VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0
    /// None if it is a shared common Video Std header
    pub version: Option<String>,

    /// ex) vk_video/vulkan_video_codec_h264std_decode.h
    #[serde(rename = "headerFile")]
    pub header_file: String,

    /// Other Video Std headers that this one depends on
    pub depends: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoStd {
    pub headers: HashMap<String, VideoStdHeader>,
    pub enums: HashMap<String, Enum>,
    pub structs: HashMap<String, Struct>,
    pub constants: HashMap<String, Constant>,
}

/// This is the global Vulkan Object that holds all the information from parsing the XML.
/// This struct is designed so all generator scripts can use this to obtain data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulkanObject {
    /// Value of VK_HEADER_VERSION (ex. '345')
    #[serde(rename = "headerVersion")]
    pub header_version: String,
    /// Value of VK_HEADER_VERSION_COMPLETE (ex. '1.2.345')
    #[serde(rename = "headerVersionComplete")]
    pub header_version_complete: String,

    pub extensions: HashMap<String, Extension>,
    pub versions: HashMap<String, Version>,

    pub handles: HashMap<String, Handle>,
    pub commands: HashMap<String, Command>,
    pub structs: HashMap<String, Struct>,
    pub enums: HashMap<String, Enum>,
    pub bitmasks: HashMap<String, Bitmask>,
    pub flags: HashMap<String, Flags>,
    pub constants: HashMap<String, Constant>,
    pub formats: HashMap<String, Format>,

    #[serde(rename = "syncStage")]
    pub sync_stage: Vec<SyncStage>,
    #[serde(rename = "syncAccess")]
    pub sync_access: Vec<SyncAccess>,
    #[serde(rename = "syncPipeline")]
    pub sync_pipeline: Vec<SyncPipeline>,

    pub spirv: Vec<Spirv>,

    /// ex) { xlib: VK_USE_PLATFORM_XLIB_KHR }
    pub platforms: HashMap<String, String>,
    /// List of all vendor suffix names (KHR, EXT, etc.)
    #[serde(rename = "vendorTags")]
    pub vendor_tags: Vec<String>,

    /// Video codec information from the vk.xml
    #[serde(rename = "videoCodecs")]
    pub video_codecs: HashMap<String, VideoCodec>,

    /// Video Std header information from the video.xml
    #[serde(rename = "videoStd")]
    pub video_std: Option<VideoStd>,
}
