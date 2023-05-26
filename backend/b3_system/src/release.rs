use b3_helper_lib::{
    revert,
    types::{Blob, Version},
};
use b3_system_lib::{
    error::SystemError,
    store::{
        with_latest_release, with_release, with_release_mut, with_releases, with_releases_mut,
        with_version_release, with_version_release_mut,
    },
    types::{LoadRelease, Release, ReleaseArgs, Releases},
};
use ic_cdk::{export::candid::candid_method, query, update};

use crate::guard::caller_is_controller;

// QUERY CALLS

#[candid_method(query)]
#[query]
fn releases() -> Releases {
    with_releases(|r| r.clone())
}

#[candid_method(query)]
#[query]
fn latest_release() -> Release {
    with_latest_release(|r| r.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
pub fn get_release(version: Version) -> Result<Release, SystemError> {
    with_version_release(version, |r| r.clone())
}

#[candid_method(query)]
#[query]
pub fn get_release_by_index(index: usize) -> Result<Release, SystemError> {
    with_release(index, |r| r.clone())
}

// UPDATE CALLS

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn update_release(release_args: ReleaseArgs) -> Result<(), SystemError> {
    let version = release_args.version.clone();

    with_version_release_mut(version, |vrs| {
        vrs.update(release_args);
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn load_release(blob: Blob, release_args: ReleaseArgs) -> Result<LoadRelease, SystemError> {
    let version = release_args.version.clone();

    let release_index =
        with_releases_mut(|rs| match rs.iter().position(|r| r.version == version) {
            Some(index) => index,
            None => {
                let release = Release::new(release_args);
                rs.push(release);

                rs.len() - 1
            }
        });

    let total = with_release_mut(release_index, |r| r.load_wasm(&blob))
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);

    let chunks = blob.len();

    Ok(LoadRelease {
        version,
        chunks,
        total,
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
pub fn remove_release(version: Version) -> Result<Release, SystemError> {
    with_releases_mut(|rs| match rs.iter().position(|r| r.version == version) {
        Some(index) => Ok(rs.remove(index)),
        None => Err(SystemError::ReleaseNotFound),
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_latest_release() {
    with_releases_mut(|rs| {
        rs.pop();
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn deprecate_release(version: Version) -> Result<(), SystemError> {
    with_version_release_mut(version, |vrs| {
        vrs.deprecate();
    })
}
