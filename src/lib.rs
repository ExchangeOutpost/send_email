mod exchange_outpost;
use crate::exchange_outpost::{FinData, schedule_email};
use extism_pdk::{FnResult, Json, ToBytes, encoding, plugin_fn};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    email: String,
    body: String,
}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    let email_target = fin_data.get_call_argument::<String>("email")?;
    let body = fin_data.get_call_argument::<String>("body")?;
    schedule_email(&email_target, &body)?;
    Ok(Output {
        email: email_target,
        body,
    })
}
