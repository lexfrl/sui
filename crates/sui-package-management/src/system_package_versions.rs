// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::{collections::BTreeMap, sync::LazyLock};

use anyhow::Context;
use sui_protocol_config::ProtocolVersion;

/// Static mapping from protocol versions to the metadata for the system packages
// Generated by [generate_system_packages_version_table] in build.rs
static VERSION_TABLE: LazyLock<BTreeMap<ProtocolVersion, SystemPackagesVersion>> =
    LazyLock::new(|| {
        BTreeMap::from(include!(concat!(
            env!("OUT_DIR"),
            "/system_packages_version_table.rs"
        )))
    });

#[derive(Debug)]
pub struct SystemPackagesVersion {
    pub git_revision: String,
    pub packages: Vec<SystemPackage>,
}

#[derive(Debug)]
pub struct SystemPackage {
    /// The name of the package, e.g. "Sui"
    pub package_name: String,

    /// The path to the package in the sui monorepo
    /// e.g. "crates/sui-framework/packages/sui-framework"
    pub repo_path: String,
}

impl PartialEq for SystemPackagesVersion {
    fn eq(&self, other: &Self) -> bool {
        self.git_revision == other.git_revision
    }
}

/// Return the system packages snapshot for the latest known protocol version
pub fn latest_system_packages() -> &'static SystemPackagesVersion {
    VERSION_TABLE
        .last_key_value()
        .expect("known system package version table should be nonempty")
        .1
}

/// Return the latest protocol version that is not newer than the requested `version`
/// (or `Err` if there is no such version).
///
/// The returned [ProtocolVersion] is the protocol version that introduced the returned
/// [SystemPackagesVersion]; this may be older than the requested `version` if either:
/// 1. the system packages did not change when `version` was released, or
/// 2. this binary is older than the requested version and therefore doesn't know about the latest
///    version of the system packages
///
/// You can distinguish these cases by comparing `version` with [ProtocolVersion::MAX].
pub fn system_packages_for_protocol(
    version: ProtocolVersion,
) -> anyhow::Result<(&'static SystemPackagesVersion, ProtocolVersion)> {
    let (protocol, system_packages) = VERSION_TABLE
        .range(..=version)
        .next_back()
        .context(format!("Unrecognized protocol version {version:?}"))?;
    Ok((system_packages, *protocol))
}

#[test]
/// There is at least one known version of the system packages
fn test_nonempty_version_table() {
    assert!(!VERSION_TABLE.is_empty());
}

#[test]
/// the hash for a specific version that we have one for is correctly returned
fn test_exact_version() {
    let (system_packages, protocol) = system_packages_for_protocol(4.into()).unwrap();
    assert_eq!(
        system_packages.git_revision,
        "f5d26f1b3ae89f68cb66f3a007e90065e5286905"
    );
    assert_eq!(protocol, 4.into());
    assert!(system_packages
        .packages
        .iter()
        .any(|p| p.package_name == "Sui"));
}

#[test]
/// we get the right hash for a version that we don't have an exact entry for
fn test_gap_version() {
    // versions 56 and 57 are missing in the manifest; version 55 should be returned
    assert_eq!(
        system_packages_for_protocol(56.into()).unwrap(),
        system_packages_for_protocol(55.into()).unwrap(),
    );
    assert_eq!(
        system_packages_for_protocol(57.into()).unwrap(),
        system_packages_for_protocol(55.into()).unwrap(),
    );
    // version 58 is present though!
    assert_ne!(
        system_packages_for_protocol(58.into()).unwrap(),
        system_packages_for_protocol(55.into()).unwrap(),
    );
}

#[test]
/// we get the correct hash for the latest known protocol version
fn test_version_latest() {
    assert_eq!(
        system_packages_for_protocol(ProtocolVersion::MAX)
            .unwrap()
            .0,
        latest_system_packages()
    );
    assert_eq!(
        system_packages_for_protocol(ProtocolVersion::MAX + 1)
            .unwrap()
            .0,
        latest_system_packages()
    );
}

#[test]
/// we get an error if the protocol version is too small or too large
fn test_version_errors() {
    assert!(system_packages_for_protocol(0.into()).is_err());
}
