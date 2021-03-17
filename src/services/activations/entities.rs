#![allow(non_snake_case)]

use serde::Serialize;
use std::convert::From;
use crate::controllers::activations::dto;

#[derive(Serialize)]
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

#[derive(Serialize)]
pub enum Status {
    Received,
    Sent
}

impl From<dto::Status> for Status {
    fn from(status: dto::Status) -> Self {
        match status {
            dto::Status::Received => Self::Received,
            dto::Status::Sent => Self::Sent
        }
    }
}

#[derive(Serialize)]
pub struct ActivationObservation {
    pub quantity: u64,
    pub position: u64,
    pub activationPrice: u64
}

#[derive(Serialize)]
pub struct DateTimePeriod {
    pub startDateTime: u64,
    pub endDateTime: u64
}

impl From<dto::Activation> for Activation {
  fn from(activation_dto: dto::Activation) -> Self {
    Activation {
      id: activation_dto.id,
      assetType: activation_dto.assetType,
      createdDateTime: activation_dto.createdDateTime,
      flexibilityServiceProviderId: activation_dto.flexibilityServiceProviderId,
      buyerOrganizationId: activation_dto.buyerOrganizationId,
      referenceOfferId: activation_dto.referenceOfferId,
      measurementUnit: activation_dto.measurementUnit,
      processType: activation_dto.processType,
      status: Status::from(activation_dto.status),
      activationPeriod: DateTimePeriod {
        startDateTime: activation_dto.activationPeriod.startDateTime,
        endDateTime: activation_dto.activationPeriod.endDateTime
      },
      observations: activation_dto.observations.iter().map(
        |observation| {
          ActivationObservation {
            quantity: observation.quantity,
            position: observation.position,
            activationPrice: observation.activationPrice
          }
        }
      ).collect()
    }
  }
}
