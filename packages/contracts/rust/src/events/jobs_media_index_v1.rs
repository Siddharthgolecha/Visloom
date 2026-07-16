#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

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
#[doc = "API → worker: please index this media. Stream `jobs.media.index.v1` per docs/conventions/events.md:26."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://visloom/contracts/events/jobs.media.index.v1.json\","]
#[doc = "  \"title\": \"JobsMediaIndexV1\","]
#[doc = "  \"description\": \"API → worker: please index this media. Stream `jobs.media.index.v1` per docs/conventions/events.md:26.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"event_id\","]
#[doc = "    \"occurred_at\","]
#[doc = "    \"trace_id\","]
#[doc = "    \"traceparent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event_id_ref\","]
#[doc = "        \"media_id\","]
#[doc = "        \"media_kind\","]
#[doc = "        \"owner_account_id\","]
#[doc = "        \"source_uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event_id_ref\": {"]
#[doc = "          \"description\": \"ULID of the tenant event this media belongs to (ADR 0008:44-49). Named `_ref` to disambiguate from the envelope's `event_id` (message identity).\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "        },"]
#[doc = "        \"media_id\": {"]
#[doc = "          \"description\": \"ULID primary key of the media row.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "        },"]
#[doc = "        \"media_kind\": {"]
#[doc = "          \"description\": \"Media discriminator per ADR 0007.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"photo\","]
#[doc = "            \"video\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"owner_account_id\": {"]
#[doc = "          \"description\": \"ULID of the account that initiated indexing (RBAC principal per ADR 0005).\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "        },"]
#[doc = "        \"source_uri\": {"]
#[doc = "          \"description\": \"Opaque URI the worker reads media bytes from. Object-store layout is deferred to slice 5 per ADR 0007:53-57.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"event_id\": {"]
#[doc = "      \"description\": \"ULID identifying this specific message.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    },"]
#[doc = "    \"occurred_at\": {"]
#[doc = "      \"description\": \"ISO-8601 / RFC 3339 timestamp in UTC.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"trace_id\": {"]
#[doc = "      \"description\": \"The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-f]{32}$\""]
#[doc = "    },"]
#[doc = "    \"traceparent\": {"]
#[doc = "      \"description\": \"W3C Trace Context traceparent header value, propagation vehicle per ADR 0015.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-f]{2}-[0-9a-f]{32}-[0-9a-f]{16}-[0-9a-f]{2}$\""]
#[doc = "    },"]
#[doc = "    \"tracestate\": {"]
#[doc = "      \"description\": \"Optional W3C tracestate value, present when the originating request had one.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct JobsMediaIndexV1 {
    pub data: JobsMediaIndexV1Data,
    #[doc = "ULID identifying this specific message."]
    pub event_id: JobsMediaIndexV1EventId,
    #[doc = "ISO-8601 / RFC 3339 timestamp in UTC."]
    pub occurred_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation."]
    pub trace_id: JobsMediaIndexV1TraceId,
    #[doc = "W3C Trace Context traceparent header value, propagation vehicle per ADR 0015."]
    pub traceparent: JobsMediaIndexV1Traceparent,
    #[doc = "Optional W3C tracestate value, present when the originating request had one."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tracestate: ::std::option::Option<::std::string::String>,
}
impl JobsMediaIndexV1 {
    pub fn builder() -> builder::JobsMediaIndexV1 {
        Default::default()
    }
}
#[doc = "`JobsMediaIndexV1Data`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event_id_ref\","]
#[doc = "    \"media_id\","]
#[doc = "    \"media_kind\","]
#[doc = "    \"owner_account_id\","]
#[doc = "    \"source_uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"event_id_ref\": {"]
#[doc = "      \"description\": \"ULID of the tenant event this media belongs to (ADR 0008:44-49). Named `_ref` to disambiguate from the envelope's `event_id` (message identity).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    },"]
#[doc = "    \"media_id\": {"]
#[doc = "      \"description\": \"ULID primary key of the media row.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    },"]
#[doc = "    \"media_kind\": {"]
#[doc = "      \"description\": \"Media discriminator per ADR 0007.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"photo\","]
#[doc = "        \"video\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"owner_account_id\": {"]
#[doc = "      \"description\": \"ULID of the account that initiated indexing (RBAC principal per ADR 0005).\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    },"]
#[doc = "    \"source_uri\": {"]
#[doc = "      \"description\": \"Opaque URI the worker reads media bytes from. Object-store layout is deferred to slice 5 per ADR 0007:53-57.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct JobsMediaIndexV1Data {
    #[doc = "ULID of the tenant event this media belongs to (ADR 0008:44-49). Named `_ref` to disambiguate from the envelope's `event_id` (message identity)."]
    pub event_id_ref: JobsMediaIndexV1DataEventIdRef,
    #[doc = "ULID primary key of the media row."]
    pub media_id: JobsMediaIndexV1DataMediaId,
    #[doc = "Media discriminator per ADR 0007."]
    pub media_kind: JobsMediaIndexV1DataMediaKind,
    #[doc = "ULID of the account that initiated indexing (RBAC principal per ADR 0005)."]
    pub owner_account_id: JobsMediaIndexV1DataOwnerAccountId,
    #[doc = "Opaque URI the worker reads media bytes from. Object-store layout is deferred to slice 5 per ADR 0007:53-57."]
    pub source_uri: ::std::string::String,
}
impl JobsMediaIndexV1Data {
    pub fn builder() -> builder::JobsMediaIndexV1Data {
        Default::default()
    }
}
#[doc = "ULID of the tenant event this media belongs to (ADR 0008:44-49). Named `_ref` to disambiguate from the envelope's `event_id` (message identity)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ULID of the tenant event this media belongs to (ADR 0008:44-49). Named `_ref` to disambiguate from the envelope's `event_id` (message identity).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1DataEventIdRef(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1DataEventIdRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1DataEventIdRef> for ::std::string::String {
    fn from(value: JobsMediaIndexV1DataEventIdRef) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1DataEventIdRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9A-HJKMNP-TV-Z]{26}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-HJKMNP-TV-Z]{26}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1DataEventIdRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1DataEventIdRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1DataEventIdRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1DataEventIdRef {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "ULID primary key of the media row."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ULID primary key of the media row.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1DataMediaId(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1DataMediaId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1DataMediaId> for ::std::string::String {
    fn from(value: JobsMediaIndexV1DataMediaId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1DataMediaId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9A-HJKMNP-TV-Z]{26}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-HJKMNP-TV-Z]{26}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1DataMediaId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Media discriminator per ADR 0007."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Media discriminator per ADR 0007.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"photo\","]
#[doc = "    \"video\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JobsMediaIndexV1DataMediaKind {
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "video")]
    Video,
}
impl ::std::fmt::Display for JobsMediaIndexV1DataMediaKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Photo => f.write_str("photo"),
            Self::Video => f.write_str("video"),
        }
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1DataMediaKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "photo" => Ok(Self::Photo),
            "video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1DataMediaKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1DataMediaKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1DataMediaKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ULID of the account that initiated indexing (RBAC principal per ADR 0005)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ULID of the account that initiated indexing (RBAC principal per ADR 0005).\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1DataOwnerAccountId(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1DataOwnerAccountId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1DataOwnerAccountId> for ::std::string::String {
    fn from(value: JobsMediaIndexV1DataOwnerAccountId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1DataOwnerAccountId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9A-HJKMNP-TV-Z]{26}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-HJKMNP-TV-Z]{26}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1DataOwnerAccountId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1DataOwnerAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1DataOwnerAccountId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1DataOwnerAccountId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "ULID identifying this specific message."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ULID identifying this specific message.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1EventId(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1EventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1EventId> for ::std::string::String {
    fn from(value: JobsMediaIndexV1EventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1EventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9A-HJKMNP-TV-Z]{26}$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9A-HJKMNP-TV-Z]{26}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1EventId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-f]{32}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1TraceId(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1TraceId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1TraceId> for ::std::string::String {
    fn from(value: JobsMediaIndexV1TraceId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1TraceId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-f]{32}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{32}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1TraceId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "W3C Trace Context traceparent header value, propagation vehicle per ADR 0015."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"W3C Trace Context traceparent header value, propagation vehicle per ADR 0015.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-f]{2}-[0-9a-f]{32}-[0-9a-f]{16}-[0-9a-f]{2}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct JobsMediaIndexV1Traceparent(::std::string::String);
impl ::std::ops::Deref for JobsMediaIndexV1Traceparent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JobsMediaIndexV1Traceparent> for ::std::string::String {
    fn from(value: JobsMediaIndexV1Traceparent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for JobsMediaIndexV1Traceparent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[0-9a-f]{2}-[0-9a-f]{32}-[0-9a-f]{16}-[0-9a-f]{2}$")
                    .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{2}-[0-9a-f]{32}-[0-9a-f]{16}-[0-9a-f]{2}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for JobsMediaIndexV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JobsMediaIndexV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JobsMediaIndexV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for JobsMediaIndexV1Traceparent {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct JobsMediaIndexV1 {
        data: ::std::result::Result<super::JobsMediaIndexV1Data, ::std::string::String>,
        event_id: ::std::result::Result<super::JobsMediaIndexV1EventId, ::std::string::String>,
        occurred_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        trace_id: ::std::result::Result<super::JobsMediaIndexV1TraceId, ::std::string::String>,
        traceparent:
            ::std::result::Result<super::JobsMediaIndexV1Traceparent, ::std::string::String>,
        tracestate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JobsMediaIndexV1 {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                occurred_at: Err("no value supplied for occurred_at".to_string()),
                trace_id: Err("no value supplied for trace_id".to_string()),
                traceparent: Err("no value supplied for traceparent".to_string()),
                tracestate: Ok(Default::default()),
            }
        }
    }
    impl JobsMediaIndexV1 {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1Data>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1EventId>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {e}"));
            self
        }
        pub fn occurred_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.occurred_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for occurred_at: {e}"));
            self
        }
        pub fn trace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1TraceId>,
            T::Error: ::std::fmt::Display,
        {
            self.trace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace_id: {e}"));
            self
        }
        pub fn traceparent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1Traceparent>,
            T::Error: ::std::fmt::Display,
        {
            self.traceparent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traceparent: {e}"));
            self
        }
        pub fn tracestate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tracestate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tracestate: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JobsMediaIndexV1> for super::JobsMediaIndexV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JobsMediaIndexV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                event_id: value.event_id?,
                occurred_at: value.occurred_at?,
                trace_id: value.trace_id?,
                traceparent: value.traceparent?,
                tracestate: value.tracestate?,
            })
        }
    }
    impl ::std::convert::From<super::JobsMediaIndexV1> for JobsMediaIndexV1 {
        fn from(value: super::JobsMediaIndexV1) -> Self {
            Self {
                data: Ok(value.data),
                event_id: Ok(value.event_id),
                occurred_at: Ok(value.occurred_at),
                trace_id: Ok(value.trace_id),
                traceparent: Ok(value.traceparent),
                tracestate: Ok(value.tracestate),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JobsMediaIndexV1Data {
        event_id_ref:
            ::std::result::Result<super::JobsMediaIndexV1DataEventIdRef, ::std::string::String>,
        media_id: ::std::result::Result<super::JobsMediaIndexV1DataMediaId, ::std::string::String>,
        media_kind:
            ::std::result::Result<super::JobsMediaIndexV1DataMediaKind, ::std::string::String>,
        owner_account_id:
            ::std::result::Result<super::JobsMediaIndexV1DataOwnerAccountId, ::std::string::String>,
        source_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JobsMediaIndexV1Data {
        fn default() -> Self {
            Self {
                event_id_ref: Err("no value supplied for event_id_ref".to_string()),
                media_id: Err("no value supplied for media_id".to_string()),
                media_kind: Err("no value supplied for media_kind".to_string()),
                owner_account_id: Err("no value supplied for owner_account_id".to_string()),
                source_uri: Err("no value supplied for source_uri".to_string()),
            }
        }
    }
    impl JobsMediaIndexV1Data {
        pub fn event_id_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1DataEventIdRef>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id_ref: {e}"));
            self
        }
        pub fn media_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1DataMediaId>,
            T::Error: ::std::fmt::Display,
        {
            self.media_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_id: {e}"));
            self
        }
        pub fn media_kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1DataMediaKind>,
            T::Error: ::std::fmt::Display,
        {
            self.media_kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_kind: {e}"));
            self
        }
        pub fn owner_account_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JobsMediaIndexV1DataOwnerAccountId>,
            T::Error: ::std::fmt::Display,
        {
            self.owner_account_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner_account_id: {e}"));
            self
        }
        pub fn source_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_uri: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<JobsMediaIndexV1Data> for super::JobsMediaIndexV1Data {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JobsMediaIndexV1Data,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id_ref: value.event_id_ref?,
                media_id: value.media_id?,
                media_kind: value.media_kind?,
                owner_account_id: value.owner_account_id?,
                source_uri: value.source_uri?,
            })
        }
    }
    impl ::std::convert::From<super::JobsMediaIndexV1Data> for JobsMediaIndexV1Data {
        fn from(value: super::JobsMediaIndexV1Data) -> Self {
            Self {
                event_id_ref: Ok(value.event_id_ref),
                media_id: Ok(value.media_id),
                media_kind: Ok(value.media_kind),
                owner_account_id: Ok(value.owner_account_id),
                source_uri: Ok(value.source_uri),
            }
        }
    }
}
