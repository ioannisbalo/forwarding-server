#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Activation {
    pub id: String,
    pub assetType: String,
    pub createdDateTime: u64,
    pub flexibilityServiceProviderId: String,
    pub buyerOrganizationId: String,
    pub referenceOfferId: String,
    pub measurementUnit: String,
    pub processType: String,
    pub status: Status,
    pub activationPeriod: DateTimePeriod,
    pub observations: Vec<ActivationObservation>
}

#[derive(Deserialize)]
pub enum Status {
    Received,
    Sent
}

#[derive(Deserialize)]
pub struct ActivationObservation {
    pub quantity: u64,
    pub position: u64,
    pub activationPrice: u64
}

#[derive(Deserialize)]
pub struct DateTimePeriod {
    pub startDateTime: u64,
    pub endDateTime: u64
}
