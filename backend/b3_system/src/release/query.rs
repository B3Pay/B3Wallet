use crate::store::{with_latest_release, with_release, with_releases, with_version_release};
use crate::types::{Release, Releases, Version};
use ic_cdk::{export::candid::candid_method, query, trap};

#[candid_method(query)]
#[query]
fn releases() -> Releases {
    with_releases(|r| r.clone())
}

#[candid_method(query)]
#[query]
fn latest_release() -> Release {
    with_latest_release(|r| r.clone()).unwrap_or_else(|e| trap(&e))
}

#[candid_method(query)]
#[query]
pub fn get_release(version: Version) -> Result<Release, String> {
    with_version_release(version, |r| r.clone())
}

#[candid_method(query)]
#[query]
pub fn get_release_by_index(index: usize) -> Result<Release, String> {
    with_release(index, |r| r.clone())
}
