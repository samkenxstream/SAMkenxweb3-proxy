//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: rename to StatLevel? AccountingLevel? What?
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "log_level")]
pub enum TrackingLevel {
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "aggregated")]
    Aggregated,
    #[sea_orm(string_value = "detailed")]
    Detailed,
}

impl Default for TrackingLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "method")]
pub enum Method {
    #[sea_orm(string_value = "eth_call")]
    EthCall,
    #[sea_orm(string_value = "eth_estimateGas")]
    EthEstimateGas,
    #[sea_orm(string_value = "eth_sendRawTransaction")]
    EthSendRawTransaction,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "owner")]
    Owner,
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "collaborator")]
    Collaborator,
}
