#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use b3_utils::{ledger::Metadata, memory::types::Storable, name_to_slug, wasm::WasmHash};

    use crate::app::{
        app::App,
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
            id: "Test App".to_string(),
            size: 0,
            version: "0.0.1".to_string(),
            features: "".to_string(),
            wasm_hash: WasmHash::from([0; 32]),
        }
    }

    #[test]
    fn test_new_app() {
        let app_args = create_test_app_args();
        let app = App::new(app_args.clone());

        let expected_id = name_to_slug(&app_args.name);

        assert_eq!(app.id(), expected_id);
        assert_eq!(app.release_hashes().len(), 0);
    }

    #[test]
    fn test_add_and_remove_release() {
        let app_args = create_test_app_args();
        let mut app = App::new(app_args);
        let hash = release_mock();
        app.add_release(hash.clone());

        assert_eq!(app.release_hashes().len(), 1);

        let _ = app.deprecate_release(hash.wasm_hash.clone());
        assert_eq!(app.release_hashes().len(), 0);
    }

    #[test]
    fn test_update_release() {
        let app_args = create_test_app_args();
        let mut app = App::new(app_args);

        let release = release_mock();
        app.add_release(release.clone());

        let _ = app.deprecate_release(release.wasm_hash.clone());

        assert!(app.release_hash(&release.wasm_hash).is_none());

        let new_release = release_mock();
        app.add_release(new_release.clone());

        assert!(app.release_hash(&new_release.wasm_hash).is_some());
    }

    #[test]
    fn test_get_release() {
        let app_args = create_test_app_args();
        let mut app = App::new(app_args);
        let release = release_mock();
        app.add_release(release.clone());

        // Mock implementation of `with_releases` needed here
        let release = app.release_hash(&release.wasm_hash);
        assert!(release.is_some());
    }

    #[test]
    fn test_serialization_deserialization() {
        let app_args = create_test_app_args();
        let app = App::new(app_args);
        let serialized = app.to_bytes();
        let deserialized = App::from_bytes(Cow::Owned(serialized.into_owned()));

        let app = app.view();
        let deserialized = deserialized.view();

        assert_eq!(app.created_by, deserialized.created_by);
        assert_eq!(app.created_at, deserialized.created_at);
        assert_eq!(app.updated_at, deserialized.updated_at);
        assert_eq!(app.name, deserialized.name);
        assert_eq!(app.description, deserialized.description);
        assert_eq!(app.latest_release, deserialized.latest_release);
        assert_eq!(app.metadata, deserialized.metadata);
    }
}
