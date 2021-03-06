use uuid::Uuid;
use std::fmt;
use super::schema::{payments, payment_items, threeds_datas, cards, payment_tokens};
use chrono::prelude::*;
use diesel::data_types::PgMoney as Pence;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, DbEnum, PartialEq)]
pub enum PaymentState {
    OPEN,
    PAID,
    COMPLETE
}


#[derive(Copy, Clone, Debug, Deserialize, Serialize, DbEnum, PartialEq)]
pub enum PaymentEnvironment {
    TEST,
    LIVE
}

impl fmt::Display for PaymentEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentEnvironment::TEST => write!(f, "Test"),
            PaymentEnvironment::LIVE => write!(f, "Live")
        }
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug, PartialEq)]
pub struct Payment {
    pub id: Uuid,
    pub time: NaiveDateTime,
    pub state: PaymentState,
    pub customer_id: Uuid,
    pub environment: PaymentEnvironment,
    pub payment_method: Option<String>,
}

#[derive(Clone, Debug, Insertable)]
#[table_name="payments"]
pub struct NewPayment<'a> {
    pub id: &'a Uuid,
    pub time: &'a NaiveDateTime,
    pub state: PaymentState,
    pub customer_id: &'a Uuid,
    pub environment:PaymentEnvironment
}

#[derive(Queryable, Identifiable, Associations, AsChangeset, Clone, Debug, PartialEq)]
#[belongs_to(Payment)]
pub struct PaymentItem {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub item_type: String,
    pub item_data: serde_json::Value,
    pub title: String,
    pub quantity: i32,
    pub price: Pence
}

#[derive(Clone, Debug, Insertable)]
#[table_name="payment_items"]
pub struct NewPaymentItem<'a> {
    pub id: &'a Uuid,
    pub payment_id: &'a Uuid,
    pub item_type: &'a str,
    pub item_data: &'a serde_json::Value,
    pub title: &'a str,
    pub quantity: i32,
    pub price: &'a Pence
}

#[derive(Queryable, Identifiable, Associations, AsChangeset, Clone, Debug, PartialEq)]
#[belongs_to(Payment)]
pub struct ThreedsData {
    pub id: i64,
    pub payment_id: Uuid,
    pub one_time_3ds_token: String,
    pub redirect_url: String,
    pub order_id: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Debug, Insertable)]
#[table_name="threeds_datas"]
pub struct NewThreedsData<'a> {
    pub payment_id: &'a Uuid,
    pub one_time_3ds_token: &'a str,
    pub redirect_url: &'a str,
    pub order_id: &'a str,
}

#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug, PartialEq)]
pub struct Card {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub pan: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub name_on_card: String,
}

#[derive(Clone, Debug, Insertable)]
#[table_name="cards"]
pub struct NewCard<'a> {
    pub id: &'a Uuid,
    pub customer_id: &'a Uuid,
    pub pan: &'a str,
    pub exp_month: i32,
    pub exp_year: i32,
    pub name_on_card: &'a str,
}

#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug, PartialEq)]
pub struct PaymentToken {
    pub id: i64,
    pub name: String,
    pub token: Vec<u8>,
}
