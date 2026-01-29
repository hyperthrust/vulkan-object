#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`Bitmask`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Bitmask\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"extensions\","]
#[doc = "    \"flagExtensions\","]
#[doc = "    \"flagName\","]
#[doc = "    \"flags\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagExtensions\": {"]
#[doc = "      \"title\": \"Flagextensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagName\": {"]
#[doc = "      \"title\": \"Flagname\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Flag\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Bitmask {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "flagExtensions")]
    pub flag_extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "flagName")]
    pub flag_name: ::std::string::String,
    pub flags: ::std::vec::Vec<Flag>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
}
impl ::std::convert::From<&Bitmask> for Bitmask {
    fn from(value: &Bitmask) -> Self {
        value.clone()
    }
}
#[doc = "`Command`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Command\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alias\","]
#[doc = "    \"allowNoQueues\","]
#[doc = "    \"cFunctionPointer\","]
#[doc = "    \"cPrototype\","]
#[doc = "    \"device\","]
#[doc = "    \"errorCodes\","]
#[doc = "    \"extensions\","]
#[doc = "    \"implicitExternSyncParams\","]
#[doc = "    \"instance\","]
#[doc = "    \"legacy\","]
#[doc = "    \"name\","]
#[doc = "    \"params\","]
#[doc = "    \"primary\","]
#[doc = "    \"protect\","]
#[doc = "    \"queues\","]
#[doc = "    \"renderPass\","]
#[doc = "    \"returnType\","]
#[doc = "    \"secondary\","]
#[doc = "    \"successCodes\","]
#[doc = "    \"tasks\","]
#[doc = "    \"version\","]
#[doc = "    \"videoCoding\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alias\": {"]
#[doc = "      \"title\": \"Alias\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"allowNoQueues\": {"]
#[doc = "      \"title\": \"Allownoqueues\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"cFunctionPointer\": {"]
#[doc = "      \"title\": \"Cfunctionpointer\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"cPrototype\": {"]
#[doc = "      \"title\": \"Cprototype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"errorCodes\": {"]
#[doc = "      \"title\": \"Errorcodes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"implicitExternSyncParams\": {"]
#[doc = "      \"title\": \"Implicitexternsyncparams\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"legacy\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Legacy\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"title\": \"Params\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Param\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"primary\": {"]
#[doc = "      \"title\": \"Primary\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"queues\": {"]
#[doc = "      \"title\": \"Queues\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"renderPass\": {"]
#[doc = "      \"title\": \"Renderpass\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"returnType\": {"]
#[doc = "      \"title\": \"Returntype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"secondary\": {"]
#[doc = "      \"title\": \"Secondary\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"successCodes\": {"]
#[doc = "      \"title\": \"Successcodes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tasks\": {"]
#[doc = "      \"title\": \"Tasks\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"videoCoding\": {"]
#[doc = "      \"title\": \"Videocoding\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Command {
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allowNoQueues")]
    pub allow_no_queues: bool,
    #[serde(rename = "cFunctionPointer")]
    pub c_function_pointer: ::std::string::String,
    #[serde(rename = "cPrototype")]
    pub c_prototype: ::std::string::String,
    pub device: bool,
    #[serde(rename = "errorCodes")]
    pub error_codes: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "implicitExternSyncParams")]
    pub implicit_extern_sync_params: ::std::vec::Vec<::std::string::String>,
    pub instance: bool,
    pub legacy: ::std::option::Option<Legacy>,
    pub name: ::std::string::String,
    pub params: ::std::vec::Vec<Param>,
    pub primary: bool,
    pub protect: ::std::option::Option<::std::string::String>,
    pub queues: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "renderPass")]
    pub render_pass: i64,
    #[serde(rename = "returnType")]
    pub return_type: ::std::string::String,
    pub secondary: bool,
    #[serde(rename = "successCodes")]
    pub success_codes: ::std::vec::Vec<::std::string::String>,
    pub tasks: ::std::vec::Vec<::std::string::String>,
    pub version: ::std::option::Option<Version>,
    #[serde(rename = "videoCoding")]
    pub video_coding: i64,
}
impl ::std::convert::From<&Command> for Command {
    fn from(value: &Command) -> Self {
        value.clone()
    }
}
#[doc = "`Constant`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Constant\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Constant {
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    pub value: Value,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Constant> for Constant {
    fn from(value: &Constant) -> Self {
        value.clone()
    }
}
#[doc = "`Enum`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Enum\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"extensions\","]
#[doc = "    \"fieldExtensions\","]
#[doc = "    \"fields\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fieldExtensions\": {"]
#[doc = "      \"title\": \"Fieldextensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"Fields\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/EnumField\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Enum {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fieldExtensions")]
    pub field_extensions: ::std::vec::Vec<::std::string::String>,
    pub fields: ::std::vec::Vec<EnumField>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Enum> for Enum {
    fn from(value: &Enum) -> Self {
        value.clone()
    }
}
#[doc = "`EnumField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"EnumField\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"extensions\","]
#[doc = "    \"name\","]
#[doc = "    \"negative\","]
#[doc = "    \"protect\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"negative\": {"]
#[doc = "      \"title\": \"Negative\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EnumField {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub negative: bool,
    pub protect: ::std::option::Option<::std::string::String>,
    pub value: i64,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
}
impl ::std::convert::From<&EnumField> for EnumField {
    fn from(value: &EnumField) -> Self {
        value.clone()
    }
}
#[doc = "`Extension`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Extension\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"deprecatedBy\","]
#[doc = "    \"device\","]
#[doc = "    \"featureRequirement\","]
#[doc = "    \"instance\","]
#[doc = "    \"name\","]
#[doc = "    \"nameString\","]
#[doc = "    \"obsoletedBy\","]
#[doc = "    \"platform\","]
#[doc = "    \"promotedTo\","]
#[doc = "    \"protect\","]
#[doc = "    \"provisional\","]
#[doc = "    \"ratified\","]
#[doc = "    \"specVersion\","]
#[doc = "    \"specialUse\","]
#[doc = "    \"vendorTag\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitmasks\": {"]
#[doc = "      \"title\": \"Bitmasks\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Bitmask\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"commands\": {"]
#[doc = "      \"title\": \"Commands\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Command\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deprecatedBy\": {"]
#[doc = "      \"title\": \"Deprecatedby\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"enumFields\": {"]
#[doc = "      \"title\": \"Enumfields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/EnumField\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"featureRequirement\": {"]
#[doc = "      \"title\": \"Featurerequirement\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FeatureRequirement\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flagBits\": {"]
#[doc = "      \"title\": \"Flagbits\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/Flag\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/$defs/Flags\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"handles\": {"]
#[doc = "      \"title\": \"Handles\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Handle\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameString\": {"]
#[doc = "      \"title\": \"Namestring\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"obsoletedBy\": {"]
#[doc = "      \"title\": \"Obsoletedby\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"platform\": {"]
#[doc = "      \"title\": \"Platform\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"promotedTo\": {"]
#[doc = "      \"title\": \"Promotedto\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"provisional\": {"]
#[doc = "      \"title\": \"Provisional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"ratified\": {"]
#[doc = "      \"title\": \"Ratified\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"specVersion\": {"]
#[doc = "      \"title\": \"Specversion\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"specialUse\": {"]
#[doc = "      \"title\": \"Specialuse\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vendorTag\": {"]
#[doc = "      \"title\": \"Vendortag\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Extension {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub bitmasks: ::std::vec::Vec<Bitmask>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub commands: ::std::vec::Vec<Command>,
    pub depends: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deprecatedBy")]
    pub deprecated_by: ::std::option::Option<::std::string::String>,
    pub device: bool,
    #[serde(
        rename = "enumFields",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enum_fields: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<EnumField>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub enums: ::std::vec::Vec<Enum>,
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: ::std::vec::Vec<FeatureRequirement>,
    #[serde(
        rename = "flagBits",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flag_bits: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<Flag>>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flags: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<Flags>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub handles: ::std::vec::Vec<Handle>,
    pub instance: bool,
    pub name: ::std::string::String,
    #[serde(rename = "nameString")]
    pub name_string: ::std::string::String,
    #[serde(rename = "obsoletedBy")]
    pub obsoleted_by: ::std::option::Option<::std::string::String>,
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotedTo")]
    pub promoted_to: ::std::option::Option<::std::string::String>,
    pub protect: ::std::option::Option<::std::string::String>,
    pub provisional: bool,
    pub ratified: bool,
    #[serde(rename = "specVersion")]
    pub spec_version: ::std::string::String,
    #[serde(rename = "specialUse")]
    pub special_use: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub structs: ::std::vec::Vec<Struct>,
    #[serde(rename = "vendorTag")]
    pub vendor_tag: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Extension> for Extension {
    fn from(value: &Extension) -> Self {
        value.clone()
    }
}
#[doc = "`FeatureRequirement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FeatureRequirement\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"field\","]
#[doc = "    \"struct\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"field\": {"]
#[doc = "      \"title\": \"Field\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FeatureRequirement {
    pub depends: ::std::option::Option<::std::string::String>,
    pub field: ::std::string::String,
    #[serde(rename = "struct")]
    pub struct_: ::std::string::String,
}
impl ::std::convert::From<&FeatureRequirement> for FeatureRequirement {
    fn from(value: &FeatureRequirement) -> Self {
        value.clone()
    }
}
#[doc = "`Flag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Flag\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"extensions\","]
#[doc = "    \"multiBit\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"value\","]
#[doc = "    \"valueStr\","]
#[doc = "    \"zero\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"multiBit\": {"]
#[doc = "      \"title\": \"Multibit\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"valueStr\": {"]
#[doc = "      \"title\": \"Valuestr\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"zero\": {"]
#[doc = "      \"title\": \"Zero\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Flag {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "multiBit")]
    pub multi_bit: bool,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    pub value: i64,
    #[serde(rename = "valueStr")]
    pub value_str: ::std::string::String,
    pub zero: bool,
}
impl ::std::convert::From<&Flag> for Flag {
    fn from(value: &Flag) -> Self {
        value.clone()
    }
}
#[doc = "`Flags`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Flags\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"baseFlagsType\","]
#[doc = "    \"bitWidth\","]
#[doc = "    \"bitmaskName\","]
#[doc = "    \"extensions\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"baseFlagsType\": {"]
#[doc = "      \"title\": \"Baseflagstype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"bitWidth\": {"]
#[doc = "      \"title\": \"Bitwidth\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"bitmaskName\": {"]
#[doc = "      \"title\": \"Bitmaskname\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Flags {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "baseFlagsType")]
    pub base_flags_type: ::std::string::String,
    #[serde(rename = "bitWidth")]
    pub bit_width: i64,
    #[serde(rename = "bitmaskName")]
    pub bitmask_name: ::std::option::Option<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
}
impl ::std::convert::From<&Flags> for Flags {
    fn from(value: &Flags) -> Self {
        value.clone()
    }
}
#[doc = "`Format`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Format\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"blockExtent\","]
#[doc = "    \"blockSize\","]
#[doc = "    \"chroma\","]
#[doc = "    \"className\","]
#[doc = "    \"components\","]
#[doc = "    \"compressed\","]
#[doc = "    \"name\","]
#[doc = "    \"packed\","]
#[doc = "    \"planes\","]
#[doc = "    \"spirvImageFormat\","]
#[doc = "    \"texelsPerBlock\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"blockExtent\": {"]
#[doc = "      \"title\": \"Blockextent\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"blockSize\": {"]
#[doc = "      \"title\": \"Blocksize\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"chroma\": {"]
#[doc = "      \"title\": \"Chroma\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"className\": {"]
#[doc = "      \"title\": \"Classname\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"Components\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FormatComponent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"compressed\": {"]
#[doc = "      \"title\": \"Compressed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"packed\": {"]
#[doc = "      \"title\": \"Packed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"planes\": {"]
#[doc = "      \"title\": \"Planes\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FormatPlane\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"spirvImageFormat\": {"]
#[doc = "      \"title\": \"Spirvimageformat\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"texelsPerBlock\": {"]
#[doc = "      \"title\": \"Texelsperblock\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Format {
    #[serde(rename = "blockExtent")]
    pub block_extent: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "blockSize")]
    pub block_size: i64,
    pub chroma: ::std::option::Option<::std::string::String>,
    #[serde(rename = "className")]
    pub class_name: ::std::string::String,
    pub components: ::std::vec::Vec<FormatComponent>,
    pub compressed: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub packed: ::std::option::Option<i64>,
    pub planes: ::std::vec::Vec<FormatPlane>,
    #[serde(rename = "spirvImageFormat")]
    pub spirv_image_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "texelsPerBlock")]
    pub texels_per_block: i64,
}
impl ::std::convert::From<&Format> for Format {
    fn from(value: &Format) -> Self {
        value.clone()
    }
}
#[doc = "`FormatComponent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FormatComponent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bits\","]
#[doc = "    \"numericFormat\","]
#[doc = "    \"planeIndex\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bits\": {"]
#[doc = "      \"title\": \"Bits\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"numericFormat\": {"]
#[doc = "      \"title\": \"Numericformat\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"planeIndex\": {"]
#[doc = "      \"title\": \"Planeindex\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FormatComponent {
    pub bits: ::std::string::String,
    #[serde(rename = "numericFormat")]
    pub numeric_format: ::std::string::String,
    #[serde(rename = "planeIndex")]
    pub plane_index: ::std::option::Option<i64>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&FormatComponent> for FormatComponent {
    fn from(value: &FormatComponent) -> Self {
        value.clone()
    }
}
#[doc = "`FormatPlane`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"FormatPlane\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"compatible\","]
#[doc = "    \"heightDivisor\","]
#[doc = "    \"index\","]
#[doc = "    \"widthDivisor\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"compatible\": {"]
#[doc = "      \"title\": \"Compatible\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"heightDivisor\": {"]
#[doc = "      \"title\": \"Heightdivisor\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"index\": {"]
#[doc = "      \"title\": \"Index\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"widthDivisor\": {"]
#[doc = "      \"title\": \"Widthdivisor\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FormatPlane {
    pub compatible: ::std::string::String,
    #[serde(rename = "heightDivisor")]
    pub height_divisor: i64,
    pub index: i64,
    #[serde(rename = "widthDivisor")]
    pub width_divisor: i64,
}
impl ::std::convert::From<&FormatPlane> for FormatPlane {
    fn from(value: &FormatPlane) -> Self {
        value.clone()
    }
}
#[doc = "`Handle`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Handle\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"device\","]
#[doc = "    \"dispatchable\","]
#[doc = "    \"extensions\","]
#[doc = "    \"instance\","]
#[doc = "    \"name\","]
#[doc = "    \"parent\","]
#[doc = "    \"protect\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"title\": \"Device\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"dispatchable\": {"]
#[doc = "      \"title\": \"Dispatchable\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"instance\": {"]
#[doc = "      \"title\": \"Instance\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Handle\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Handle {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub device: bool,
    pub dispatchable: bool,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub instance: bool,
    pub name: ::std::string::String,
    pub parent: ::std::option::Option<::std::boxed::Box<Handle>>,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Handle> for Handle {
    fn from(value: &Handle) -> Self {
        value.clone()
    }
}
#[doc = "`Legacy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Legacy\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"extensions\","]
#[doc = "    \"link\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"link\": {"]
#[doc = "      \"title\": \"Link\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Legacy {
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub link: ::std::option::Option<::std::string::String>,
    pub version: ::std::option::Option<Version>,
}
impl ::std::convert::From<&Legacy> for Legacy {
    fn from(value: &Legacy) -> Self {
        value.clone()
    }
}
#[doc = "`Member`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Member\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bitFieldWidth\","]
#[doc = "    \"cDeclaration\","]
#[doc = "    \"const\","]
#[doc = "    \"externSync\","]
#[doc = "    \"fixedSizeArray\","]
#[doc = "    \"fullType\","]
#[doc = "    \"length\","]
#[doc = "    \"limitType\","]
#[doc = "    \"name\","]
#[doc = "    \"noAutoValidity\","]
#[doc = "    \"nullTerminated\","]
#[doc = "    \"optional\","]
#[doc = "    \"optionalPointer\","]
#[doc = "    \"pointer\","]
#[doc = "    \"selection\","]
#[doc = "    \"selector\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitFieldWidth\": {"]
#[doc = "      \"title\": \"Bitfieldwidth\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cDeclaration\": {"]
#[doc = "      \"title\": \"Cdeclaration\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"const\": {"]
#[doc = "      \"title\": \"Const\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"externSync\": {"]
#[doc = "      \"title\": \"Externsync\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"fixedSizeArray\": {"]
#[doc = "      \"title\": \"Fixedsizearray\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fullType\": {"]
#[doc = "      \"title\": \"Fulltype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"length\": {"]
#[doc = "      \"title\": \"Length\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"limitType\": {"]
#[doc = "      \"title\": \"Limittype\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"noAutoValidity\": {"]
#[doc = "      \"title\": \"Noautovalidity\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nullTerminated\": {"]
#[doc = "      \"title\": \"Nullterminated\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optional\": {"]
#[doc = "      \"title\": \"Optional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optionalPointer\": {"]
#[doc = "      \"title\": \"Optionalpointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pointer\": {"]
#[doc = "      \"title\": \"Pointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"selection\": {"]
#[doc = "      \"title\": \"Selection\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"selector\": {"]
#[doc = "      \"title\": \"Selector\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Member {
    #[serde(rename = "bitFieldWidth")]
    pub bit_field_width: ::std::option::Option<i64>,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: ::std::string::String,
    #[serde(rename = "const")]
    pub const_: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: i64,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fullType")]
    pub full_type: ::std::string::String,
    pub length: ::std::option::Option<::std::string::String>,
    #[serde(rename = "limitType")]
    pub limit_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    pub pointer: bool,
    pub selection: ::std::vec::Vec<::std::string::String>,
    pub selector: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Member> for Member {
    fn from(value: &Member) -> Self {
        value.clone()
    }
}
#[doc = "`Param`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Param\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alias\","]
#[doc = "    \"cDeclaration\","]
#[doc = "    \"const\","]
#[doc = "    \"externSync\","]
#[doc = "    \"externSyncPointer\","]
#[doc = "    \"fixedSizeArray\","]
#[doc = "    \"fullType\","]
#[doc = "    \"length\","]
#[doc = "    \"name\","]
#[doc = "    \"noAutoValidity\","]
#[doc = "    \"nullTerminated\","]
#[doc = "    \"optional\","]
#[doc = "    \"optionalPointer\","]
#[doc = "    \"pointer\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alias\": {"]
#[doc = "      \"title\": \"Alias\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cDeclaration\": {"]
#[doc = "      \"title\": \"Cdeclaration\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"const\": {"]
#[doc = "      \"title\": \"Const\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"externSync\": {"]
#[doc = "      \"title\": \"Externsync\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"externSyncPointer\": {"]
#[doc = "      \"title\": \"Externsyncpointer\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fixedSizeArray\": {"]
#[doc = "      \"title\": \"Fixedsizearray\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fullType\": {"]
#[doc = "      \"title\": \"Fulltype\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"length\": {"]
#[doc = "      \"title\": \"Length\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"noAutoValidity\": {"]
#[doc = "      \"title\": \"Noautovalidity\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"nullTerminated\": {"]
#[doc = "      \"title\": \"Nullterminated\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optional\": {"]
#[doc = "      \"title\": \"Optional\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"optionalPointer\": {"]
#[doc = "      \"title\": \"Optionalpointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"pointer\": {"]
#[doc = "      \"title\": \"Pointer\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Param {
    pub alias: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cDeclaration")]
    pub c_declaration: ::std::string::String,
    #[serde(rename = "const")]
    pub const_: bool,
    #[serde(rename = "externSync")]
    pub extern_sync: i64,
    #[serde(rename = "externSyncPointer")]
    pub extern_sync_pointer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedSizeArray")]
    pub fixed_size_array: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fullType")]
    pub full_type: ::std::string::String,
    pub length: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(rename = "noAutoValidity")]
    pub no_auto_validity: bool,
    #[serde(rename = "nullTerminated")]
    pub null_terminated: bool,
    pub optional: bool,
    #[serde(rename = "optionalPointer")]
    pub optional_pointer: bool,
    pub pointer: bool,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&Param> for Param {
    fn from(value: &Param) -> Self {
        value.clone()
    }
}
#[doc = "`Spirv`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Spirv\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capability\","]
#[doc = "    \"enable\","]
#[doc = "    \"extension\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capability\": {"]
#[doc = "      \"title\": \"Capability\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"enable\": {"]
#[doc = "      \"title\": \"Enable\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SpirvEnables\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extension\": {"]
#[doc = "      \"title\": \"Extension\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Spirv {
    pub capability: bool,
    pub enable: ::std::vec::Vec<SpirvEnables>,
    pub extension: bool,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&Spirv> for Spirv {
    fn from(value: &Spirv) -> Self {
        value.clone()
    }
}
#[doc = "`SpirvEnables`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SpirvEnables\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"extension\","]
#[doc = "    \"feature\","]
#[doc = "    \"member\","]
#[doc = "    \"property\","]
#[doc = "    \"requires\","]
#[doc = "    \"struct\","]
#[doc = "    \"value\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extension\": {"]
#[doc = "      \"title\": \"Extension\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"feature\": {"]
#[doc = "      \"title\": \"Feature\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"member\": {"]
#[doc = "      \"title\": \"Member\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"property\": {"]
#[doc = "      \"title\": \"Property\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"requires\": {"]
#[doc = "      \"title\": \"Requires\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Version\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SpirvEnables {
    pub extension: ::std::option::Option<::std::string::String>,
    pub feature: ::std::option::Option<::std::string::String>,
    pub member: ::std::option::Option<::std::string::String>,
    pub property: ::std::option::Option<::std::string::String>,
    pub requires: ::std::option::Option<::std::string::String>,
    #[serde(rename = "struct")]
    pub struct_: ::std::option::Option<::std::string::String>,
    pub value: ::std::option::Option<::std::string::String>,
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SpirvEnables> for SpirvEnables {
    fn from(value: &SpirvEnables) -> Self {
        value.clone()
    }
}
#[doc = "`Struct`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Struct\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"aliases\","]
#[doc = "    \"allowDuplicate\","]
#[doc = "    \"extendedBy\","]
#[doc = "    \"extends\","]
#[doc = "    \"extensions\","]
#[doc = "    \"members\","]
#[doc = "    \"name\","]
#[doc = "    \"protect\","]
#[doc = "    \"returnedOnly\","]
#[doc = "    \"sType\","]
#[doc = "    \"union\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"aliases\": {"]
#[doc = "      \"title\": \"Aliases\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"allowDuplicate\": {"]
#[doc = "      \"title\": \"Allowduplicate\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"extendedBy\": {"]
#[doc = "      \"title\": \"Extendedby\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extends\": {"]
#[doc = "      \"title\": \"Extends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"members\": {"]
#[doc = "      \"title\": \"Members\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Member\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protect\": {"]
#[doc = "      \"title\": \"Protect\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"returnedOnly\": {"]
#[doc = "      \"title\": \"Returnedonly\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"sType\": {"]
#[doc = "      \"title\": \"Stype\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"union\": {"]
#[doc = "      \"title\": \"Union\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/Version\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"videoStdHeader\": {"]
#[doc = "      \"title\": \"Videostdheader\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Struct {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,
    #[serde(rename = "extendedBy")]
    pub extended_by: ::std::vec::Vec<::std::string::String>,
    pub extends: ::std::vec::Vec<::std::string::String>,
    pub extensions: ::std::vec::Vec<::std::string::String>,
    pub members: ::std::vec::Vec<Member>,
    pub name: ::std::string::String,
    pub protect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnedOnly")]
    pub returned_only: bool,
    #[serde(rename = "sType")]
    pub s_type: ::std::option::Option<::std::string::String>,
    pub union: bool,
    pub version: ::std::option::Option<Version>,
    #[serde(
        rename = "videoStdHeader",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std_header: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Struct> for Struct {
    fn from(value: &Struct) -> Self {
        value.clone()
    }
}
#[doc = "`SyncAccess`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncAccess\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"equivalent\","]
#[doc = "    \"flag\","]
#[doc = "    \"support\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"equivalent\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncEquivalent\""]
#[doc = "    },"]
#[doc = "    \"flag\": {"]
#[doc = "      \"$ref\": \"#/$defs/Flag\""]
#[doc = "    },"]
#[doc = "    \"support\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncSupport\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncAccess {
    pub equivalent: SyncEquivalent,
    pub flag: Flag,
    pub support: SyncSupport,
}
impl ::std::convert::From<&SyncAccess> for SyncAccess {
    fn from(value: &SyncAccess) -> Self {
        value.clone()
    }
}
#[doc = "`SyncEquivalent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncEquivalent\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"accesses\","]
#[doc = "    \"max\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"accesses\": {"]
#[doc = "      \"title\": \"Accesses\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max\": {"]
#[doc = "      \"title\": \"Max\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncEquivalent {
    pub accesses: ::std::option::Option<::std::vec::Vec<Flag>>,
    pub max: bool,
    pub stages: ::std::option::Option<::std::vec::Vec<Flag>>,
}
impl ::std::convert::From<&SyncEquivalent> for SyncEquivalent {
    fn from(value: &SyncEquivalent) -> Self {
        value.clone()
    }
}
#[doc = "`SyncPipeline`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncPipeline\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"name\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncPipelineStage\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncPipeline {
    pub depends: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub stages: ::std::vec::Vec<SyncPipelineStage>,
}
impl ::std::convert::From<&SyncPipeline> for SyncPipeline {
    fn from(value: &SyncPipeline) -> Self {
        value.clone()
    }
}
#[doc = "`SyncPipelineStage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncPipelineStage\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"after\","]
#[doc = "    \"before\","]
#[doc = "    \"order\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"after\": {"]
#[doc = "      \"title\": \"After\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"before\": {"]
#[doc = "      \"title\": \"Before\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"order\": {"]
#[doc = "      \"title\": \"Order\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncPipelineStage {
    pub after: ::std::option::Option<::std::string::String>,
    pub before: ::std::option::Option<::std::string::String>,
    pub order: ::std::option::Option<::std::string::String>,
    pub value: ::std::string::String,
}
impl ::std::convert::From<&SyncPipelineStage> for SyncPipelineStage {
    fn from(value: &SyncPipelineStage) -> Self {
        value.clone()
    }
}
#[doc = "`SyncStage`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncStage\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"equivalent\","]
#[doc = "    \"flag\","]
#[doc = "    \"support\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"equivalent\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncEquivalent\""]
#[doc = "    },"]
#[doc = "    \"flag\": {"]
#[doc = "      \"$ref\": \"#/$defs/Flag\""]
#[doc = "    },"]
#[doc = "    \"support\": {"]
#[doc = "      \"$ref\": \"#/$defs/SyncSupport\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncStage {
    pub equivalent: SyncEquivalent,
    pub flag: Flag,
    pub support: SyncSupport,
}
impl ::std::convert::From<&SyncStage> for SyncStage {
    fn from(value: &SyncStage) -> Self {
        value.clone()
    }
}
#[doc = "`SyncSupport`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"SyncSupport\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"max\","]
#[doc = "    \"queues\","]
#[doc = "    \"stages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"max\": {"]
#[doc = "      \"title\": \"Max\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"queues\": {"]
#[doc = "      \"title\": \"Queues\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stages\": {"]
#[doc = "      \"title\": \"Stages\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/Flag\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SyncSupport {
    pub max: bool,
    pub queues: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub stages: ::std::option::Option<::std::vec::Vec<Flag>>,
}
impl ::std::convert::From<&SyncSupport> for SyncSupport {
    fn from(value: &SyncSupport) -> Self {
        value.clone()
    }
}
#[doc = "`Value`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Value\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    Number(f64),
}
impl ::std::convert::From<&Self> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Value {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for Value {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Value {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`Version`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Version\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"featureRequirement\","]
#[doc = "    \"name\","]
#[doc = "    \"nameApi\","]
#[doc = "    \"nameString\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"featureRequirement\": {"]
#[doc = "      \"title\": \"Featurerequirement\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/FeatureRequirement\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameApi\": {"]
#[doc = "      \"title\": \"Nameapi\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nameString\": {"]
#[doc = "      \"title\": \"Namestring\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Version {
    #[serde(rename = "featureRequirement")]
    pub feature_requirement: ::std::vec::Vec<FeatureRequirement>,
    pub name: ::std::string::String,
    #[serde(rename = "nameApi")]
    pub name_api: ::std::string::String,
    #[serde(rename = "nameString")]
    pub name_string: ::std::string::String,
}
impl ::std::convert::From<&Version> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}
#[doc = "`VideoCodec`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoCodec\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"formats\","]
#[doc = "    \"name\","]
#[doc = "    \"profiles\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"title\": \"Capabilities\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"formats\": {"]
#[doc = "      \"title\": \"Formats\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoFormat\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"profiles\": {"]
#[doc = "      \"title\": \"Profiles\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoProfiles\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoCodec {
    pub capabilities: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub formats: ::std::collections::HashMap<::std::string::String, VideoFormat>,
    pub name: ::std::string::String,
    pub profiles: ::std::collections::HashMap<::std::string::String, VideoProfiles>,
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VideoCodec> for VideoCodec {
    fn from(value: &VideoCodec) -> Self {
        value.clone()
    }
}
#[doc = "`VideoFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoFormat\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"properties\","]
#[doc = "    \"requiredCaps\","]
#[doc = "    \"usage\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"properties\": {"]
#[doc = "      \"title\": \"Properties\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"requiredCaps\": {"]
#[doc = "      \"title\": \"Requiredcaps\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoRequiredCapabilities\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"usage\": {"]
#[doc = "      \"title\": \"Usage\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoFormat {
    pub name: ::std::string::String,
    pub properties: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(rename = "requiredCaps")]
    pub required_caps: ::std::vec::Vec<VideoRequiredCapabilities>,
    pub usage: ::std::string::String,
}
impl ::std::convert::From<&VideoFormat> for VideoFormat {
    fn from(value: &VideoFormat) -> Self {
        value.clone()
    }
}
#[doc = "`VideoProfileMember`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoProfileMember\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"title\": \"Values\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoProfileMember {
    pub name: ::std::string::String,
    pub values: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&VideoProfileMember> for VideoProfileMember {
    fn from(value: &VideoProfileMember) -> Self {
        value.clone()
    }
}
#[doc = "`VideoProfiles`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoProfiles\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"members\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"title\": \"Members\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoProfileMember\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoProfiles {
    pub members: ::std::collections::HashMap<::std::string::String, VideoProfileMember>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&VideoProfiles> for VideoProfiles {
    fn from(value: &VideoProfiles) -> Self {
        value.clone()
    }
}
#[doc = "`VideoRequiredCapabilities`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoRequiredCapabilities\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"member\","]
#[doc = "    \"struct\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"member\": {"]
#[doc = "      \"title\": \"Member\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"title\": \"Struct\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"title\": \"Value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoRequiredCapabilities {
    pub member: ::std::string::String,
    #[serde(rename = "struct")]
    pub struct_: ::std::string::String,
    pub value: ::std::string::String,
}
impl ::std::convert::From<&VideoRequiredCapabilities> for VideoRequiredCapabilities {
    fn from(value: &VideoRequiredCapabilities) -> Self {
        value.clone()
    }
}
#[doc = "`VideoStd`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoStd\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"constants\": {"]
#[doc = "      \"title\": \"Constants\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Constant\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headers\": {"]
#[doc = "      \"title\": \"Headers\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoStdHeader\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoStd {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub constants: ::std::collections::HashMap<::std::string::String, Constant>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enums: ::std::collections::HashMap<::std::string::String, Enum>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub headers: ::std::collections::HashMap<::std::string::String, VideoStdHeader>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub structs: ::std::collections::HashMap<::std::string::String, Struct>,
}
impl ::std::convert::From<&VideoStd> for VideoStd {
    fn from(value: &VideoStd) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VideoStd {
    fn default() -> Self {
        Self {
            constants: Default::default(),
            enums: Default::default(),
            headers: Default::default(),
            structs: Default::default(),
        }
    }
}
#[doc = "`VideoStdHeader`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VideoStdHeader\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"depends\","]
#[doc = "    \"headerFile\","]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"depends\": {"]
#[doc = "      \"title\": \"Depends\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headerFile\": {"]
#[doc = "      \"title\": \"Headerfile\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Version\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VideoStdHeader {
    pub depends: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "headerFile")]
    pub header_file: ::std::string::String,
    pub name: ::std::string::String,
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VideoStdHeader> for VideoStdHeader {
    fn from(value: &VideoStdHeader) -> Self {
        value.clone()
    }
}
#[doc = "`VulkanObject`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"VulkanObject\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bitmasks\": {"]
#[doc = "      \"title\": \"Bitmasks\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Bitmask\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"commands\": {"]
#[doc = "      \"title\": \"Commands\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Command\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"constants\": {"]
#[doc = "      \"title\": \"Constants\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Constant\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"enums\": {"]
#[doc = "      \"title\": \"Enums\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Enum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"title\": \"Extensions\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Extension\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"flags\": {"]
#[doc = "      \"title\": \"Flags\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Flags\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"formats\": {"]
#[doc = "      \"title\": \"Formats\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Format\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"handles\": {"]
#[doc = "      \"title\": \"Handles\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Handle\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"headerVersion\": {"]
#[doc = "      \"title\": \"Headerversion\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"headerVersionComplete\": {"]
#[doc = "      \"title\": \"Headerversioncomplete\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"platforms\": {"]
#[doc = "      \"title\": \"Platforms\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"spirv\": {"]
#[doc = "      \"title\": \"Spirv\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/Spirv\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"structs\": {"]
#[doc = "      \"title\": \"Structs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Struct\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncAccess\": {"]
#[doc = "      \"title\": \"Syncaccess\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncAccess\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncPipeline\": {"]
#[doc = "      \"title\": \"Syncpipeline\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncPipeline\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"syncStage\": {"]
#[doc = "      \"title\": \"Syncstage\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/SyncStage\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vendorTags\": {"]
#[doc = "      \"title\": \"Vendortags\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"versions\": {"]
#[doc = "      \"title\": \"Versions\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/Version\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"videoCodecs\": {"]
#[doc = "      \"title\": \"Videocodecs\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/$defs/VideoCodec\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"videoStd\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/VideoStd\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct VulkanObject {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub bitmasks: ::std::collections::HashMap<::std::string::String, Bitmask>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub commands: ::std::collections::HashMap<::std::string::String, Command>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub constants: ::std::collections::HashMap<::std::string::String, Constant>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub enums: ::std::collections::HashMap<::std::string::String, Enum>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub extensions: ::std::collections::HashMap<::std::string::String, Extension>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub flags: ::std::collections::HashMap<::std::string::String, Flags>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub formats: ::std::collections::HashMap<::std::string::String, Format>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub handles: ::std::collections::HashMap<::std::string::String, Handle>,
    #[serde(rename = "headerVersion", default)]
    pub header_version: ::std::string::String,
    #[serde(rename = "headerVersionComplete", default)]
    pub header_version_complete: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub platforms: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub spirv: ::std::vec::Vec<Spirv>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub structs: ::std::collections::HashMap<::std::string::String, Struct>,
    #[serde(
        rename = "syncAccess",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_access: ::std::vec::Vec<SyncAccess>,
    #[serde(
        rename = "syncPipeline",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_pipeline: ::std::vec::Vec<SyncPipeline>,
    #[serde(
        rename = "syncStage",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub sync_stage: ::std::vec::Vec<SyncStage>,
    #[serde(
        rename = "vendorTags",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub vendor_tags: ::std::vec::Vec<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub versions: ::std::collections::HashMap<::std::string::String, Version>,
    #[serde(
        rename = "videoCodecs",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub video_codecs: ::std::collections::HashMap<::std::string::String, VideoCodec>,
    #[serde(
        rename = "videoStd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub video_std: ::std::option::Option<VideoStd>,
}
impl ::std::convert::From<&VulkanObject> for VulkanObject {
    fn from(value: &VulkanObject) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VulkanObject {
    fn default() -> Self {
        Self {
            bitmasks: Default::default(),
            commands: Default::default(),
            constants: Default::default(),
            enums: Default::default(),
            extensions: Default::default(),
            flags: Default::default(),
            formats: Default::default(),
            handles: Default::default(),
            header_version: Default::default(),
            header_version_complete: Default::default(),
            platforms: Default::default(),
            spirv: Default::default(),
            structs: Default::default(),
            sync_access: Default::default(),
            sync_pipeline: Default::default(),
            sync_stage: Default::default(),
            vendor_tags: Default::default(),
            versions: Default::default(),
            video_codecs: Default::default(),
            video_std: Default::default(),
        }
    }
}
