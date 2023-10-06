use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::helper_structs::CounterpartyType;
use super::res_template::ResponseTemplate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Counterparty {
    r#ref: Uuid,
    description: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    counterparty: Option<String>,
    ownership_form: Option<String>,
    ownership_form_ref: Option<String>,
    ownership_form_description: Option<String>,
    EDRPOU: Option<String>,
    counterparty_type: CounterpartyType,
    contact_person: Option<ResponseTemplate<ContactPerson>>,
    city: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContactPerson {
    r#ref: Uuid,
    description: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    phones: Option<String>,
    email: Option<String>,
}
