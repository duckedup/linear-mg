#[cynic::schema("linear")]
pub mod schema {}

macro_rules! string_scalar {
    ($name:ident, $schema_name:ident) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        cynic::impl_scalar!($name, schema::$schema_name);
    };
}

string_scalar!(DateTime, DateTime);
string_scalar!(TimelessDate, TimelessDate);
string_scalar!(Uuid, UUID);
string_scalar!(DateTimeOrDuration, DateTimeOrDuration);
string_scalar!(TimelessDateOrDuration, TimelessDateOrDuration);
string_scalar!(Duration, Duration);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Json(pub serde_json::Value);
cynic::impl_scalar!(Json, schema::JSON);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct JsonObject(pub serde_json::Value);
cynic::impl_scalar!(JsonObject, schema::JSONObject);
