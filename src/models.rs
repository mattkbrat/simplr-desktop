// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


#[derive(Queryable, Debug)]
pub struct Account {
    pub id: String,
    pub contact: String,
    pub cosigner: Option<String>,
    pub date_of_birth: Option<String>,
    pub license_number: String,
    pub license_expiration: Option<String>,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub current_standing: Option<String>,
    pub notes: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Charge {
    pub id: String,
    pub name: String,
    pub amount: String,
    pub date_effective: String,
}

#[derive(Queryable, Debug)]
pub struct Creditor {
    pub id: String,
    pub business_name: String,
    pub contact: String,
    pub filing_fees: String,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub apr: String,
}

#[derive(Queryable, Debug)]
pub struct Deal {
    pub id: String,
    pub state: i32,
    pub date: String,
    pub account: String,
    pub inventory: String,
    pub creditor: Option<String>,
    pub cash: String,
    pub down: Option<String>,
    pub apr: String,
    pub finance: Option<String>,
    pub lien: Option<String>,
    pub pmt: Option<String>,
    pub term: String,
    pub tax_city: Option<String>,
    pub tax_state: Option<String>,
    pub tax_county: Option<String>,
    pub tax_rtd: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct DealCharge {
    pub deal: Option<String>,
    pub charge: Option<String>,
    pub date: Option<String>,
    pub note: Option<String>,
    pub id: String,
}

#[derive(Queryable, Debug)]
pub struct DealSalesman {
    pub id: String,
    pub deal: String,
    pub salesman: String,
}

#[derive(Queryable, Debug)]
pub struct DealTrade {
    pub id: String,
    pub deal: String,
    pub vin: String,
    pub value: String,
}

#[derive(Queryable, Debug)]
pub struct DefaultCharge {
    pub id: String,
    pub creditor: String,
    pub charge: String,
}

#[derive(Queryable, Debug)]
pub struct Inventory {
    pub id: String,
    pub vin: String,
    pub year: String,
    pub make: String,
    pub model: Option<String>,
    pub body: Option<String>,
    pub color: Option<String>,
    pub fuel: Option<String>,
    pub cwt: Option<String>,
    pub mileage: Option<String>,
    pub date_added: Option<String>,
    pub date_modified: Option<String>,
    pub picture: Option<String>,
    pub cash: Option<String>,
    pub credit: Option<String>,
    pub down: Option<String>,
    pub state: i32,
}

#[derive(Queryable, Debug)]
pub struct Key {
    pub id: String,
    pub hashed_password: Option<String>,
    pub user_id: String,
}

#[derive(Queryable, Debug)]
pub struct Payment {
    pub id: String,
    pub deal: String,
    pub date: String,
    pub amount: String,
}

#[derive(Queryable, Debug)]
pub struct Person {
    pub id: String,
    pub name_prefix: Option<String>,
    pub first_name: String,
    pub middle_initial: Option<String>,
    pub last_name: String,
    pub name_suffix: Option<String>,
    pub address_1: String,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: String,
    pub state_province: String,
    pub zip_postal: String,
    pub zip_4: Option<String>,
    pub country: String,
    pub phone_primary: String,
    pub phone_secondary: Option<String>,
    pub phone_tertiary: Option<String>,
    pub email_primary: Option<String>,
    pub email_secondary: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Salesman {
    pub id: String,
    pub person: String,
}

#[derive(Queryable, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub active_expires: i64,
    pub idle_expires: i64,
    pub iv: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

