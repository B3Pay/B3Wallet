#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use b3_utils::{ledger::Metadata, memory::types::Storable, name_to_slug, wasm::WasmHash};

    use crate::app::{
        app::AppData,
        types::{CreateAppArgs, CreateReleaseArgs},
    };

    fn create_test_app_args() -> CreateAppArgs {
        CreateAppArgs {
            name: "Test App".to_string(),
            description: "A test application".to_string(),
            metadata: Metadata::new(),
        }
    }

    fn release_mock() -> CreateReleaseArgs {
        CreateReleaseArgs {
            app_id: "Test App".to_string(),
            size: 2,
            version: "0.0.1".to_string(),
            features: "".to_string(),
            wasm_hash: WasmHash::from([0; 32]),
        }
    }

    #[test]
    fn test_new_app() {
        let app_args = create_test_app_args();
        let app = AppData::new(app_args.clone());

        let expected_id = name_to_slug(&app_args.name);

        assert_eq!(app.id(), expected_id);
        assert_eq!(app.release_hashes().len(), 0);
    }

    #[test]
    fn test_add_and_remove_release() {
        let app_args = create_test_app_args();
        let mut app_data = AppData::new(app_args);
        let release_args = release_mock();
        let mut release = app_data
            .add_release(release_args.clone())
            .unwrap_or_else(|_| panic!("Failed to deprecate release"));

        assert_eq!(release.is_loaded(), false);

        let size = release
            .load_wasm_chunk(&vec![0])
            .unwrap_or_else(|_| panic!("Failed to load wasm chunk"));

        assert_eq!(size, 1usize);
        assert_eq!(release.is_loaded(), false);

        assert_eq!(app_data.release_hashes().len(), 1);

        let _ = app_data.deprecate_release(release_args.wasm_hash.clone());
        assert_eq!(app_data.release_hashes().len(), 1);
    }

    #[test]
    fn test_update_release() {
        let app_args = create_test_app_args();
        let mut app_data = AppData::new(app_args);

        let release = release_mock();
        let _ = app_data.add_release(release.clone());

        app_data
            .deprecate_release(release.wasm_hash.clone())
            .unwrap_or_else(|_| panic!("Failed to deprecate release"));

        let release = app_data
            .release(&release.wasm_hash)
            .unwrap_or_else(|_| panic!("Failed to deprecate release"));

        assert!(release.is_deprecated());

        let new_release = release_mock();
        let _ = app_data.add_release(new_release.clone());

        assert!(app_data.release_hash(&new_release.wasm_hash).is_some());
    }

    #[test]
    fn test_get_release() {
        let app_args = create_test_app_args();
        let mut app = AppData::new(app_args);
        let release = release_mock();
        let _ = app.add_release(release.clone());

        // Mock implementation of `with_releases` needed here
        let release = app.release_hash(&release.wasm_hash);
        assert!(release.is_some());
    }

    #[test]
    fn test_serialization_deserialization() {
        let app_args = create_test_app_args();
        let app = AppData::new(app_args);
        let serialized = app.to_bytes();
        let deserialized = AppData::from_bytes(Cow::Owned(serialized.into_owned()));

        let app = app.view();
        let deserialized = deserialized.view();

        assert_eq!(app.created_by, deserialized.created_by);
        assert_eq!(app.created_at, deserialized.created_at);
        assert_eq!(app.updated_at, deserialized.updated_at);
        assert_eq!(app.name, deserialized.name);
        assert_eq!(app.description, deserialized.description);
        assert_eq!(app.releases, deserialized.releases);
        assert_eq!(app.metadata, deserialized.metadata);
    }
}
