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
#[doc = "Worker → API: media indexing failed. Stream `events.media.index_failed.v1` per docs/conventions/events.md:29."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://visloom/contracts/events/events.media.index_failed.v1.json\","]
#[doc = "  \"title\": \"EventsMediaIndexFailedV1\","]
#[doc = "  \"description\": \"Worker → API: media indexing failed. Stream `events.media.index_failed.v1` per docs/conventions/events.md:29.\","]
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
#[doc = "        \"failure\","]
#[doc = "        \"media_id\","]
#[doc = "        \"retry\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"failure\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"code\","]
#[doc = "            \"message\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"code\": {"]
#[doc = "              \"description\": \"Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"unreadable_source\","]
#[doc = "                \"unsupported_kind\","]
#[doc = "                \"embedder_error\","]
#[doc = "                \"internal\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"message\": {"]
#[doc = "              \"description\": \"Human-readable failure message. Non-PII per errors.md:36-37.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"media_id\": {"]
#[doc = "          \"description\": \"ULID matching the `media_id` on the originating `jobs.media.index.v1` message.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "        },"]
#[doc = "        \"retry\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"attempt\","]
#[doc = "            \"next_at\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"attempt\": {"]
#[doc = "              \"description\": \"1-indexed retry counter for consumer log correlation.\","]
#[doc = "              \"type\": \"integer\","]
#[doc = "              \"minimum\": 1.0"]
#[doc = "            },"]
#[doc = "            \"next_at\": {"]
#[doc = "              \"description\": \"RFC 3339 timestamp of the next scheduled retry, or null when the worker gives up.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"format\": \"date-time\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
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
pub struct EventsMediaIndexFailedV1 {
    pub data: EventsMediaIndexFailedV1Data,
    #[doc = "ULID identifying this specific message."]
    pub event_id: EventsMediaIndexFailedV1EventId,
    #[doc = "ISO-8601 / RFC 3339 timestamp in UTC."]
    pub occurred_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "The 32-hex trace-id segment of `traceparent`. Log-only convenience per events.md:57-59; never used for propagation."]
    pub trace_id: EventsMediaIndexFailedV1TraceId,
    #[doc = "W3C Trace Context traceparent header value, propagation vehicle per ADR 0015."]
    pub traceparent: EventsMediaIndexFailedV1Traceparent,
    #[doc = "Optional W3C tracestate value, present when the originating request had one."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tracestate: ::std::option::Option<::std::string::String>,
}
impl EventsMediaIndexFailedV1 {
    pub fn builder() -> builder::EventsMediaIndexFailedV1 {
        Default::default()
    }
}
#[doc = "`EventsMediaIndexFailedV1Data`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"failure\","]
#[doc = "    \"media_id\","]
#[doc = "    \"retry\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"failure\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"code\","]
#[doc = "        \"message\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"code\": {"]
#[doc = "          \"description\": \"Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"unreadable_source\","]
#[doc = "            \"unsupported_kind\","]
#[doc = "            \"embedder_error\","]
#[doc = "            \"internal\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"Human-readable failure message. Non-PII per errors.md:36-37.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"media_id\": {"]
#[doc = "      \"description\": \"ULID matching the `media_id` on the originating `jobs.media.index.v1` message.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9A-HJKMNP-TV-Z]{26}$\""]
#[doc = "    },"]
#[doc = "    \"retry\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"attempt\","]
#[doc = "        \"next_at\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"attempt\": {"]
#[doc = "          \"description\": \"1-indexed retry counter for consumer log correlation.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"next_at\": {"]
#[doc = "          \"description\": \"RFC 3339 timestamp of the next scheduled retry, or null when the worker gives up.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"format\": \"date-time\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventsMediaIndexFailedV1Data {
    pub failure: EventsMediaIndexFailedV1DataFailure,
    #[doc = "ULID matching the `media_id` on the originating `jobs.media.index.v1` message."]
    pub media_id: EventsMediaIndexFailedV1DataMediaId,
    pub retry: EventsMediaIndexFailedV1DataRetry,
}
impl EventsMediaIndexFailedV1Data {
    pub fn builder() -> builder::EventsMediaIndexFailedV1Data {
        Default::default()
    }
}
#[doc = "`EventsMediaIndexFailedV1DataFailure`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"unreadable_source\","]
#[doc = "        \"unsupported_kind\","]
#[doc = "        \"embedder_error\","]
#[doc = "        \"internal\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"Human-readable failure message. Non-PII per errors.md:36-37.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventsMediaIndexFailedV1DataFailure {
    #[doc = "Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer."]
    pub code: EventsMediaIndexFailedV1DataFailureCode,
    #[doc = "Human-readable failure message. Non-PII per errors.md:36-37."]
    pub message: ::std::string::String,
}
impl EventsMediaIndexFailedV1DataFailure {
    pub fn builder() -> builder::EventsMediaIndexFailedV1DataFailure {
        Default::default()
    }
}
#[doc = "Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stable, machine-readable failure code (posture per errors.md:16-19). New codes are additive per §SemVer.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"unreadable_source\","]
#[doc = "    \"unsupported_kind\","]
#[doc = "    \"embedder_error\","]
#[doc = "    \"internal\""]
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
pub enum EventsMediaIndexFailedV1DataFailureCode {
    #[serde(rename = "unreadable_source")]
    UnreadableSource,
    #[serde(rename = "unsupported_kind")]
    UnsupportedKind,
    #[serde(rename = "embedder_error")]
    EmbedderError,
    #[serde(rename = "internal")]
    Internal,
}
impl ::std::fmt::Display for EventsMediaIndexFailedV1DataFailureCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UnreadableSource => f.write_str("unreadable_source"),
            Self::UnsupportedKind => f.write_str("unsupported_kind"),
            Self::EmbedderError => f.write_str("embedder_error"),
            Self::Internal => f.write_str("internal"),
        }
    }
}
impl ::std::str::FromStr for EventsMediaIndexFailedV1DataFailureCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "unreadable_source" => Ok(Self::UnreadableSource),
            "unsupported_kind" => Ok(Self::UnsupportedKind),
            "embedder_error" => Ok(Self::EmbedderError),
            "internal" => Ok(Self::Internal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventsMediaIndexFailedV1DataFailureCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexFailedV1DataFailureCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexFailedV1DataFailureCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
pub struct EventsMediaIndexFailedV1DataMediaId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexFailedV1DataMediaId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexFailedV1DataMediaId> for ::std::string::String {
    fn from(value: EventsMediaIndexFailedV1DataMediaId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexFailedV1DataMediaId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexFailedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexFailedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexFailedV1DataMediaId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexFailedV1DataMediaId {
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
#[doc = "`EventsMediaIndexFailedV1DataRetry`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"attempt\","]
#[doc = "    \"next_at\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"attempt\": {"]
#[doc = "      \"description\": \"1-indexed retry counter for consumer log correlation.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"next_at\": {"]
#[doc = "      \"description\": \"RFC 3339 timestamp of the next scheduled retry, or null when the worker gives up.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventsMediaIndexFailedV1DataRetry {
    #[doc = "1-indexed retry counter for consumer log correlation."]
    pub attempt: ::std::num::NonZeroU64,
    #[doc = "RFC 3339 timestamp of the next scheduled retry, or null when the worker gives up."]
    pub next_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
impl EventsMediaIndexFailedV1DataRetry {
    pub fn builder() -> builder::EventsMediaIndexFailedV1DataRetry {
        Default::default()
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
pub struct EventsMediaIndexFailedV1EventId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexFailedV1EventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexFailedV1EventId> for ::std::string::String {
    fn from(value: EventsMediaIndexFailedV1EventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexFailedV1EventId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexFailedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexFailedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexFailedV1EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexFailedV1EventId {
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
pub struct EventsMediaIndexFailedV1TraceId(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexFailedV1TraceId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexFailedV1TraceId> for ::std::string::String {
    fn from(value: EventsMediaIndexFailedV1TraceId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexFailedV1TraceId {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexFailedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexFailedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexFailedV1TraceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexFailedV1TraceId {
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
pub struct EventsMediaIndexFailedV1Traceparent(::std::string::String);
impl ::std::ops::Deref for EventsMediaIndexFailedV1Traceparent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventsMediaIndexFailedV1Traceparent> for ::std::string::String {
    fn from(value: EventsMediaIndexFailedV1Traceparent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventsMediaIndexFailedV1Traceparent {
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
impl ::std::convert::TryFrom<&str> for EventsMediaIndexFailedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventsMediaIndexFailedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventsMediaIndexFailedV1Traceparent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventsMediaIndexFailedV1Traceparent {
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
    pub struct EventsMediaIndexFailedV1 {
        data: ::std::result::Result<super::EventsMediaIndexFailedV1Data, ::std::string::String>,
        event_id:
            ::std::result::Result<super::EventsMediaIndexFailedV1EventId, ::std::string::String>,
        occurred_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        trace_id:
            ::std::result::Result<super::EventsMediaIndexFailedV1TraceId, ::std::string::String>,
        traceparent: ::std::result::Result<
            super::EventsMediaIndexFailedV1Traceparent,
            ::std::string::String,
        >,
        tracestate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventsMediaIndexFailedV1 {
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
    impl EventsMediaIndexFailedV1 {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1Data>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1EventId>,
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
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1TraceId>,
            T::Error: ::std::fmt::Display,
        {
            self.trace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace_id: {e}"));
            self
        }
        pub fn traceparent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1Traceparent>,
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
    impl ::std::convert::TryFrom<EventsMediaIndexFailedV1> for super::EventsMediaIndexFailedV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexFailedV1,
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
    impl ::std::convert::From<super::EventsMediaIndexFailedV1> for EventsMediaIndexFailedV1 {
        fn from(value: super::EventsMediaIndexFailedV1) -> Self {
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
    pub struct EventsMediaIndexFailedV1Data {
        failure: ::std::result::Result<
            super::EventsMediaIndexFailedV1DataFailure,
            ::std::string::String,
        >,
        media_id: ::std::result::Result<
            super::EventsMediaIndexFailedV1DataMediaId,
            ::std::string::String,
        >,
        retry:
            ::std::result::Result<super::EventsMediaIndexFailedV1DataRetry, ::std::string::String>,
    }
    impl ::std::default::Default for EventsMediaIndexFailedV1Data {
        fn default() -> Self {
            Self {
                failure: Err("no value supplied for failure".to_string()),
                media_id: Err("no value supplied for media_id".to_string()),
                retry: Err("no value supplied for retry".to_string()),
            }
        }
    }
    impl EventsMediaIndexFailedV1Data {
        pub fn failure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1DataFailure>,
            T::Error: ::std::fmt::Display,
        {
            self.failure = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for failure: {e}"));
            self
        }
        pub fn media_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1DataMediaId>,
            T::Error: ::std::fmt::Display,
        {
            self.media_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_id: {e}"));
            self
        }
        pub fn retry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1DataRetry>,
            T::Error: ::std::fmt::Display,
        {
            self.retry = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for retry: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsMediaIndexFailedV1Data> for super::EventsMediaIndexFailedV1Data {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexFailedV1Data,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                failure: value.failure?,
                media_id: value.media_id?,
                retry: value.retry?,
            })
        }
    }
    impl ::std::convert::From<super::EventsMediaIndexFailedV1Data> for EventsMediaIndexFailedV1Data {
        fn from(value: super::EventsMediaIndexFailedV1Data) -> Self {
            Self {
                failure: Ok(value.failure),
                media_id: Ok(value.media_id),
                retry: Ok(value.retry),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventsMediaIndexFailedV1DataFailure {
        code: ::std::result::Result<
            super::EventsMediaIndexFailedV1DataFailureCode,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for EventsMediaIndexFailedV1DataFailure {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl EventsMediaIndexFailedV1DataFailure {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventsMediaIndexFailedV1DataFailureCode>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsMediaIndexFailedV1DataFailure>
        for super::EventsMediaIndexFailedV1DataFailure
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexFailedV1DataFailure,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::EventsMediaIndexFailedV1DataFailure>
        for EventsMediaIndexFailedV1DataFailure
    {
        fn from(value: super::EventsMediaIndexFailedV1DataFailure) -> Self {
            Self {
                code: Ok(value.code),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EventsMediaIndexFailedV1DataRetry {
        attempt: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        next_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EventsMediaIndexFailedV1DataRetry {
        fn default() -> Self {
            Self {
                attempt: Err("no value supplied for attempt".to_string()),
                next_at: Err("no value supplied for next_at".to_string()),
            }
        }
    }
    impl EventsMediaIndexFailedV1DataRetry {
        pub fn attempt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.attempt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attempt: {e}"));
            self
        }
        pub fn next_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.next_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_at: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EventsMediaIndexFailedV1DataRetry>
        for super::EventsMediaIndexFailedV1DataRetry
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EventsMediaIndexFailedV1DataRetry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                attempt: value.attempt?,
                next_at: value.next_at?,
            })
        }
    }
    impl ::std::convert::From<super::EventsMediaIndexFailedV1DataRetry>
        for EventsMediaIndexFailedV1DataRetry
    {
        fn from(value: super::EventsMediaIndexFailedV1DataRetry) -> Self {
            Self {
                attempt: Ok(value.attempt),
                next_at: Ok(value.next_at),
            }
        }
    }
}
