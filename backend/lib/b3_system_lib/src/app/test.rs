#[cfg(test)]
mod tests {
    use b3_utils::{ledger::Metadata, memory::types::Storable, name_to_slug, wasm::WasmHash};

    use crate::app::{app::App, types::CreateAppArgs};

    use std::borrow::Cow;

    fn create_test_app_args() -> CreateAppArgs {
        CreateAppArgs {
            name: "Test App".to_string(),
            description: "A test application".to_string(),
            metadata: Metadata::new(),
        }
    }

    fn wasm_hash_mock() -> WasmHash {
        WasmHash::from([0; 32])
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
        let hash = wasm_hash_mock();
        app.add_release_hash(hash.clone());

        assert_eq!(app.release_hashes().len(), 1);

        app.remove_release_hash(hash.clone());
        assert_eq!(app.release_hashes().len(), 0);
    }

    #[test]
    fn test_update_release() {
        let app_args = create_test_app_args();
        let mut app = App::new(app_args);

        let hash = wasm_hash_mock();
        app.add_release_hash(hash.clone());

        app.remove_release_hash(hash);

        assert!(app.release_hash(&hash).is_none());

        let new_hash = wasm_hash_mock();
        app.add_release_hash(new_hash.clone());

        assert!(app.release_hash(&new_hash).is_some());
    }

    #[test]
    fn test_get_release() {
        let app_args = create_test_app_args();
        let mut app = App::new(app_args);
        let hash = wasm_hash_mock();
        app.add_release_hash(hash.clone());

        // Mock implementation of `with_releases` needed here
        let release = app.release_hash(&hash);
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
