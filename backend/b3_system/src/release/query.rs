use b3_helper::{b3_revert, types::Version};
use b3_system_lib::{
    error::SystemError,
    store::{with_latest_release, with_release, with_releases, with_version_release},
    types::{Release, Releases},
};
use ic_cdk::{export::candid::candid_method, query};

#[candid_method(query)]
#[query]
fn releases() -> Releases {
    with_releases(|r| r.clone())
}

#[candid_method(query)]
#[query]
fn latest_release() -> Release {
    with_latest_release(|r| r.clone()).unwrap_or_else(|err| b3_revert(err))
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
