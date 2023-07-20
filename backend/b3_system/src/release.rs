use std::str::FromStr;

use b3_system_lib::{
    error::SystemError,
    store::{
        with_hash_release, with_latest_release, with_release, with_release_map, with_release_mut,
        with_releases, with_releases_mut, with_version_release, with_version_release_mut,
    },
    types::{LoadRelease, Release, ReleaseArgs, ReleaseMap, Releases},
};
use b3_utils::{
    release::ReleaseTypes,
    revert,
    types::WalletVersion,
    wasm::{Blob, WasmHash},
};
use candid::candid_method;
use ic_cdk::{query, update};

use crate::guard::caller_is_controller;

// QUERY CALLS

#[candid_method(query)]
#[query]
fn release_map() -> ReleaseMap {
    with_release_map(|r| r.clone())
}

#[candid_method(query)]
#[query]
fn releases(name: String) -> Releases {
    with_releases(&name, |r| r.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
fn latest_release(name: String) -> Release {
    with_latest_release(&name, |r| r.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
pub fn get_release(name: String, version: WalletVersion) -> Release {
    with_version_release(&name, version, |r| r.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
pub fn get_release_by_index(name: String, index: usize) -> Release {
    with_release(&name, index, |r| r.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
pub fn get_release_by_hash_string(name: String, hash: WasmHash) -> Release {
    with_hash_release(&name, hash, |r| r.clone()).unwrap_or_else(revert)
}

// UPDATE CALLS

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn update_release(name: String, release_args: ReleaseArgs) {
    let version = release_args.version.clone();
    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    with_version_release_mut(release_name, version, |vrs| {
        vrs.update(release_args);
    })
    .unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn load_release(name: String, blob: Blob, release_args: ReleaseArgs) -> LoadRelease {
    let version = release_args.version.clone();
    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    let release_index = with_releases_mut(release_name, |rs| {
        match rs.iter().position(|r| r.version == version) {
            Some(index) => index,
            None => {
                let release = Release::new(release_args);
                rs.push(release);

                rs.len() - 1
            }
        }
    });

    let total = with_release_mut(&name, release_index, |r| {
        r.load_wasm(&blob).unwrap_or_else(revert)
    })
    .unwrap_or_else(revert);

    let chunks = blob.len();

    LoadRelease {
        version,
        chunks,
        total,
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
pub fn remove_release(name: String, version: WalletVersion) -> Release {
    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    with_releases_mut(release_name, |rs| {
        match rs.iter().position(|r| r.version == version) {
            Some(index) => Ok(rs.remove(index)),
            None => Err(SystemError::ReleaseNotFound),
        }
    })
    .unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_latest_release(name: String) {
    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    with_releases_mut(release_name, |rs| {
        rs.pop();
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn deprecate_release(name: String, version: WalletVersion) {
    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    with_version_release_mut(release_name, version, |vrs| {
        vrs.deprecate();
    })
    .unwrap_or_else(revert)
}
