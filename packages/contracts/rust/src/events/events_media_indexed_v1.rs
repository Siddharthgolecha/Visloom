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
#[doc = "Worker → API: media indexing succeeded, embedding stored. Stream `events.media.indexed.v1` per docs/conventions/events.md:27."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://visloom/contracts/events/events.media.indexed.v1.json\","]
#[doc = "  \"title\": \"EventsMediaIndexedV1\","]
#[doc = "  \"description\": \"Worker → API: media indexing succeeded, embedding stored. Stream `events.media.indexed.v1` per docs/conventions/events.md:27.\","]
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
#[doc = "        \"embedder_model_id\","]
#[doc = "        \"embedder_version\","]
#[doc = "        \"embedding_ref\","]
#[doc = "        \"frames\","]
#[doc = "        \"media_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"embedder_model_id\": {"]
#[doc = "          \"description\": \"Same field name as `/healthz` `embedder.model_id` (api.md:19-20) to avoid naming drift between control-plane and data-plane.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"embedder_version\": {"]
#[doc = "          \"description\": \"Same field name as `/healthz` `embedder.version`.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"embedding_ref\": {"]
#[doc = "          \"description\": \"Opaque pointer into the pgvector column populated per ADR 0009. Encoding decided by slice 6/9.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"frames\": {"]
#[doc = "          \"description\": \"Number of embedded frames — 1 for photo, N for video-keyframe per ADR 0007:48-54.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"media_id\": {"]
#[doc = "          \"description\": \"ULID matching the `media_id` on the originating `jobs.media.index.v1` message.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
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
pub struct EventsMediaIndexedV1 {
    pub data: EventsMediaIndexedV1Data,
    #[doc = "ULID identifying this specific message."]
    pub event_id: EventsMediaIndexedV1EventId,
    #[doc = "ISO-8601 / RFC 3339 timestamp in UTC."]
    pub occurred_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation."]
    pub trace_id: EventsMediaIndexedV1TraceId,
    #[doc = "W3C Trace Context traceparent header value, propagation vehicle per ADR 0015."]
    pub traceparent: EventsMediaIndexedV1Traceparent,
    #[doc = "Optional W3C tracestate value, present when the originating request had one."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tracestate: ::std::option::Option<::std::string::String>,
}
impl EventsMediaIndexedV1 {
    pub fn builder() -> builder::EventsMediaIndexedV1 {
        Default::default()
    }
}
#[doc = "`EventsMediaIndexedV1Data`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"embedder_model_id\","]
#[doc = "    \"embedder_version\","]
#[doc = "    \"embedding_ref\","]
#[doc = "    \"frames\","]
#[doc = "    \"media_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"embedder_model_id\": {"]
#[doc = "      \"description\": \"Same field name as `/healthz` `embedder.model_id` (api.md:19-20) to avoid naming drift between control-plane and data-plane.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"embedder_version\": {"]
#[doc = "      \"description\": \"Same field name as `/healthz` `embedder.version`.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"embedding_ref\": {"]
#[doc = "      \"description\": \"Opaque pointer into the pgvector column populated per ADR 0009. Encoding decided by slice 6/9.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"frames\": {"]
#[doc = "      \"description\": \"Number of embedded frames — 1 for photo, N for video-keyframe per ADR 0007:48-54.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"media_id\": {"]
#[doc = "      \"description\": \"ULID matching the `media_id` on the originating `jobs.media.index.v1` message.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventsMediaIndexedV1Data {
    #[doc = "Same field name as `/healthz` `embedder.model_id` (api.md:19-20) to avoid naming drift between control-plane and data-plane."]
    pub embedder_model_id: EventsMediaIndexedV1DataEmbedderModelId,
    #[doc = "Same field name as `/healthz` `embedder.version`."]
    pub embedder_version: EventsMediaIndexedV1DataEmbedderVersion,
    #[doc = "Opaque pointer into the pgvector column populated per ADR 0009. Encoding decided by slice 6/9."]
    pub embedding_ref: EventsMediaIndexedV1DataEmbeddingRef,
    #[doc = "Number of embedded frames — 1 for photo, N for video-keyframe per ADR 0007:48-54."]
    pub frames: ::std::num::NonZeroU64,
    #[doc = "ULID matching the `media_id` on the originating `jobs.media.index.v1` message."]
    pub media_id: EventsMediaIndexedV1DataMediaId,
}
impl EventsMediaIndexedV1Data {
    pub fn builder() -> builder::EventsMediaIndexedV1Data {
        Default::default()
    }
}
#[doc = "Same field name as `/healthz` `embedder.model_id` (api.md:19-20) to avoid naming drift between control-plane and data-plane."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Same field name as `/healthz` `embedder.model_id` (api.md:19-20) to avoid naming drift between control-plane and data-plane.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventsMediaIndexedV1DataEmbedderModelId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1DataEmbedderModelId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1DataEmbedderModelId> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1DataEmbedderModelId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1DataEmbedderModelId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1DataEmbedderModelId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1DataEmbedderModelId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1DataEmbedderModelId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1DataEmbedderModelId {
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
#[doc = "Same field name as `/healthz` `embedder.version`."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Same field name as `/healthz` `embedder.version`.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventsMediaIndexedV1DataEmbedderVersion(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1DataEmbedderVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1DataEmbedderVersion> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1DataEmbedderVersion) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1DataEmbedderVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1DataEmbedderVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1DataEmbedderVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1DataEmbedderVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1DataEmbedderVersion {
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
#[doc = "Opaque pointer into the pgvector column populated per ADR 0009. Encoding decided by slice 6/9."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Opaque pointer into the pgvector column populated per ADR 0009. Encoding decided by slice 6/9.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventsMediaIndexedV1DataEmbeddingRef(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1DataEmbeddingRef {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1DataEmbeddingRef> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1DataEmbeddingRef) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1DataEmbeddingRef {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1DataEmbeddingRef {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1DataEmbeddingRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1DataEmbeddingRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1DataEmbeddingRef {
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
#[doc = "ULID matching the `media_id` on the originating `jobs.media.index.v1` message."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ULID matching the `media_id` on the originating `jobs.media.index.v1` message.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventsMediaIndexedV1DataMediaId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1DataMediaId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1DataMediaId> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1DataMediaId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1DataMediaId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1DataMediaId {
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
pub struct EventsMediaIndexedV1EventId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1EventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1EventId> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1EventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1EventId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1EventId {
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
pub struct EventsMediaIndexedV1TraceId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1TraceId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1TraceId> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1TraceId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1TraceId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1TraceId {
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
pub struct EventsMediaIndexedV1Traceparent(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexedV1Traceparent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexedV1Traceparent> for ::std::string::String {
    fn from(value: EventsMediaIndexedV1Traceparent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexedV1Traceparent {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexedV1Traceparent {
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
    pub struct EventsMediaIndexedV1 {
        data: ::std::result::Result<super::EventsMediaIndexedV1Data, ::std::string::String>,
        event_id: ::std::result::Result<super::EventsMediaIndexedV1EventId, ::std::string::String>,
        occurred_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        trace_id: ::std::result::Result<super::EventsMediaIndexedV1TraceId, ::std::string::String>,
        traceparent:
            ::std::result::Result<super::EventsMediaIndexedV1Traceparent, ::std::string::String>,
        tracestate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventsMediaIndexedV1 {
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
    impl EventsMediaIndexedV1 {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1Data>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1EventId>,
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
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1TraceId>,
            T::Error: ::std::fmt::Display,
        {
            self.trace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace_id: {e}"));
            self
        }
        pub fn traceparent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1Traceparent>,
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
    impl ::std::convert::TryFrom<EventsMediaIndexedV1> for super::EventsMediaIndexedV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexedV1,
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
    impl ::std::convert::From<super::EventsMediaIndexedV1> for EventsMediaIndexedV1 {
        fn from(value: super::EventsMediaIndexedV1) -> Self {
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
    pub struct EventsMediaIndexedV1Data {
        embedder_model_id: ::std::result::Result<
            super::EventsMediaIndexedV1DataEmbedderModelId,
            ::std::string::String,
        >,
        embedder_version: ::std::result::Result<
            super::EventsMediaIndexedV1DataEmbedderVersion,
            ::std::string::String,
        >,
        embedding_ref: ::std::result::Result<
            super::EventsMediaIndexedV1DataEmbeddingRef,
            ::std::string::String,
        >,
        frames: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        media_id:
            ::std::result::Result<super::EventsMediaIndexedV1DataMediaId, ::std::string::String>,
    }
    impl ::std::default::Default for EventsMediaIndexedV1Data {
        fn default() -> Self {
            Self {
                embedder_model_id: Err("no value supplied for embedder_model_id".to_string()),
                embedder_version: Err("no value supplied for embedder_version".to_string()),
                embedding_ref: Err("no value supplied for embedding_ref".to_string()),
                frames: Err("no value supplied for frames".to_string()),
                media_id: Err("no value supplied for media_id".to_string()),
            }
        }
    }
    impl EventsMediaIndexedV1Data {
        pub fn embedder_model_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1DataEmbedderModelId>,
            T::Error: ::std::fmt::Display,
        {
            self.embedder_model_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for embedder_model_id: {e}"));
            self
        }
        pub fn embedder_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1DataEmbedderVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.embedder_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for embedder_version: {e}"));
            self
        }
        pub fn embedding_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1DataEmbeddingRef>,
            T::Error: ::std::fmt::Display,
        {
            self.embedding_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for embedding_ref: {e}"));
            self
        }
        pub fn frames<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.frames = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for frames: {e}"));
            self
        }
        pub fn media_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexedV1DataMediaId>,
            T::Error: ::std::fmt::Display,
        {
            self.media_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsMediaIndexedV1Data> for super::EventsMediaIndexedV1Data {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexedV1Data,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                embedder_model_id: value.embedder_model_id?,
                embedder_version: value.embedder_version?,
                embedding_ref: value.embedding_ref?,
                frames: value.frames?,
                media_id: value.media_id?,
            })
        }
    }
    impl ::std::convert::From<super::EventsMediaIndexedV1Data> for EventsMediaIndexedV1Data {
        fn from(value: super::EventsMediaIndexedV1Data) -> Self {
            Self {
                embedder_model_id: Ok(value.embedder_model_id),
                embedder_version: Ok(value.embedder_version),
                embedding_ref: Ok(value.embedding_ref),
                frames: Ok(value.frames),
                media_id: Ok(value.media_id),
            }
        }
    }
}
