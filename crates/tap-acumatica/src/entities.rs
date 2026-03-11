use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Acumatica field wrapper ─────────────────────────────────────────────
// Acumatica REST responses wrap every field value in `{ "value": ... }`.

/// A wrapper for an Acumatica field, e.g. `{ "value": "ABCCOMP" }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field<T> {
    pub value: T,
}

impl<T> Field<T> {
    pub fn into_inner(self) -> T {
        self.value
    }
}

// ── BusinessAccount (BAccount) ──────────────────────────────────────────

/// Represents the Acumatica `BusinessAccount` entity from the Default endpoint.
///
/// Only the most common fields are typed; everything else is captured by
/// `extra`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessAccount {
    #[serde(default)]
    pub business_account_i_d: Option<Field<String>>,

    /// This is the "CompanyName" in some endpoint versions.
    #[serde(alias = "CompanyName")]
    #[serde(default)]
    pub business_account_name: Option<Field<String>>,

    #[serde(default)]
    pub status: Option<Field<String>>,

    /// Account type — "Customer", "Vendor", "Combined", "Prospect", etc.
    #[serde(rename = "Type")]
    #[serde(default)]
    pub account_type: Option<Field<String>>,

    #[serde(default)]
    pub main_contact: Option<MainContact>,

    #[serde(default)]
    pub last_modified_date_time: Option<Field<String>>,

    #[serde(default)]
    pub created_date_time: Option<Field<String>>,

    /// Catch-all for extra/custom fields.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Nested `MainContact` detail object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainContact {
    #[serde(default)]
    pub display_name: Option<Field<String>>,

    #[serde(default)]
    pub email: Option<Field<String>>,

    #[serde(default)]
    pub phone1: Option<Field<String>>,

    #[serde(default)]
    pub address: Option<Address>,

    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// Nested `Address` within a contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    #[serde(default)]
    pub address_line1: Option<Field<String>>,

    #[serde(default)]
    pub city: Option<Field<String>>,

    #[serde(default)]
    pub state: Option<Field<String>>,

    #[serde(default)]
    pub postal_code: Option<Field<String>>,

    #[serde(default)]
    pub country: Option<Field<String>>,

    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ── Stream registry ─────────────────────────────────────────────────────

/// Metadata about a supported Acumatica entity / stream.
#[derive(Debug, Clone)]
pub struct StreamDef {
    /// The DStream stream name (lowercase, snake_case)
    pub stream_name: &'static str,
    /// The Acumatica entity name as it appears in the REST URL
    pub entity_name: &'static str,
    /// Primary key field(s)
    pub key_properties: &'static [&'static str],
    /// Replication key for incremental sync (if any)
    pub replication_key: Option<&'static str>,
}

/// All streams currently supported by this tap.
pub fn supported_streams() -> Vec<StreamDef> {
    vec![
        StreamDef {
            stream_name: "business_accounts",
            entity_name: "BusinessAccount",
            key_properties: &["BusinessAccountID"],
            replication_key: Some("LastModifiedDateTime"),
        },
    ]
}
