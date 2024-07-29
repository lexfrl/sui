// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use move_core_types::{ident_str, identifier::IdentStr};
use serde::{Deserialize, Serialize};
use sui_types::base_types::{ObjectID, SuiAddress};

// Config / constants of the service.
pub(crate) const MOVE_REGISTRY_MODULE: &IdentStr = ident_str!("name");
pub(crate) const MOVE_REGISTRY_TYPE: &IdentStr = ident_str!("Name");
// TODO(manos): Replace with actual package id on mainnet.
const MOVE_REGISTRY_PACKAGE: &str =
    "0x1a841abe817c38221596856bc975b3b84f2f68692191e9247e185213d3d02fd8";
// TODO(manos): Replace with actual registry table id on mainnet.
const MOVE_REGISTRY_TABLE_ID: &str =
    "0x250b60446b8e7b8d9d7251600a7228dbfda84ccb4b23a56a700d833e221fae4f";
const DEFAULT_PAGE_LIMIT: u16 = 50;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct MoveRegistryConfig {
    pub(crate) external_api_url: Option<String>,
    #[serde(default = "default_resolution_type")]
    pub(crate) resolution_type: ResolutionType,
    #[serde(default = "default_page_limit")]
    pub(crate) page_limit: u16,
    #[serde(default = "default_package_address")]
    pub(crate) package_address: SuiAddress,
    #[serde(default = "default_registry_id")]
    pub(crate) registry_id: ObjectID,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum ResolutionType {
    Internal,
    External,
}

impl MoveRegistryConfig {
    pub(crate) fn new(
        resolution_type: ResolutionType,
        external_api_url: Option<String>,
        page_limit: u16,
        package_address: SuiAddress,
        registry_id: ObjectID,
    ) -> Self {
        Self {
            resolution_type,
            external_api_url,
            page_limit,
            package_address,
            registry_id,
        }
    }
}

// TODO: Keeping the values as is, because we'll remove the default getters
// when we refactor to use `[GraphqlConfig]` macro.
impl Default for MoveRegistryConfig {
    fn default() -> Self {
        Self::new(
            ResolutionType::Internal,
            None,
            DEFAULT_PAGE_LIMIT,
            SuiAddress::from_str(MOVE_REGISTRY_PACKAGE).unwrap(),
            ObjectID::from_str(MOVE_REGISTRY_TABLE_ID).unwrap(),
        )
    }
}

fn default_resolution_type() -> ResolutionType {
    ResolutionType::Internal
}

fn default_package_address() -> SuiAddress {
    SuiAddress::from_str(MOVE_REGISTRY_PACKAGE).unwrap()
}

fn default_registry_id() -> ObjectID {
    ObjectID::from_str(MOVE_REGISTRY_TABLE_ID).unwrap()
}

fn default_page_limit() -> u16 {
    DEFAULT_PAGE_LIMIT
}
