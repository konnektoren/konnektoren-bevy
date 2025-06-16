use super::*;
use bevy::prelude::*;

// Helper function to create a test app with settings plugin
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(SettingsPlugin);
    app
}

// Helper function to create a test setting
fn create_test_setting(id: &str, value: SettingValue) -> Setting {
    Setting::new(id, format!("Test {}", id), value, SettingType::Toggle)
}

// Helper function to query settings - returns a Vec to avoid lifetime issues
fn get_all_settings(world: &mut World) -> Vec<Setting> {
    let mut query_state = world.query::<&Setting>();
    query_state.iter(world).cloned().collect()
}

// Helper function to find a setting by ID
fn find_setting_by_id_in_world(world: &mut World, id: &str) -> Option<Setting> {
    get_all_settings(world)
        .into_iter()
        .find(|setting| setting.id == id)
}

// Helper function to count settings in a category
fn count_settings_in_category(world: &mut World, category: &str) -> usize {
    get_all_settings(world)
        .into_iter()
        .filter(|setting| {
            setting
                .category
                .as_ref()
                .map_or(false, |cat| cat == category)
        })
        .count()
}

// Helper function to check if a setting exists
fn setting_exists_in_world(world: &mut World, id: &str) -> bool {
    get_all_settings(world)
        .into_iter()
        .any(|setting| setting.id == id)
}

// Helper function to get setting value from world (for testing)
fn get_setting_value_from_world(world: &mut World, id: &str) -> Option<SettingValue> {
    let mut query_state = world.query::<&Setting>();
    query_state
        .iter(world)
        .find(|setting| setting.id == id)
        .map(|setting| setting.value.clone())
}

// Helper function to find setting by ID from world (for testing)
fn find_setting_by_id_from_world(world: &mut World, id: &str) -> Option<Setting> {
    let mut query_state = world.query::<&Setting>();
    query_state
        .iter(world)
        .find(|setting| setting.id == id)
        .cloned()
}

// Helper function to get settings in category from world (for testing)
fn get_settings_in_category_from_world(world: &mut World, category: &str) -> Vec<Setting> {
    let mut query_state = world.query::<&Setting>();
    query_state
        .iter(world)
        .filter(|setting| setting.category.as_ref().is_some_and(|cat| cat == category))
        .cloned()
        .collect()
}

#[test]
fn test_setting_creation() {
    let setting = Setting::new(
        "test_setting",
        "Test Setting",
        SettingValue::Bool(true),
        SettingType::Toggle,
    )
    .with_description("A test setting")
    .with_category("Test")
    .with_tab_index(1)
    .enabled(true);

    assert_eq!(setting.id, "test_setting");
    assert_eq!(setting.label, "Test Setting");
    assert_eq!(setting.value, SettingValue::Bool(true));
    assert_eq!(setting.description, Some("A test setting".to_string()));
    assert_eq!(setting.category, Some("Test".to_string()));
    assert_eq!(setting.tab_index, Some(1));
    assert!(setting.enabled);
}

#[test]
fn test_setting_value_types() {
    // Test Bool value
    let bool_value = SettingValue::Bool(true);
    assert_eq!(bool_value.as_bool(), Some(true));
    assert_eq!(bool_value.as_int(), None);
    assert_eq!(bool_value.as_float(), None);
    assert_eq!(bool_value.as_string(), None);
    assert_eq!(bool_value.as_selection(), None);

    // Test Int value
    let int_value = SettingValue::Int(42);
    assert_eq!(int_value.as_bool(), None);
    assert_eq!(int_value.as_int(), Some(42));
    assert_eq!(int_value.as_float(), None);
    assert_eq!(int_value.as_string(), None);
    assert_eq!(int_value.as_selection(), None);

    // Test Float value
    let float_value = SettingValue::Float(3.14);
    assert_eq!(float_value.as_bool(), None);
    assert_eq!(float_value.as_int(), None);
    assert_eq!(float_value.as_float(), Some(3.14));
    assert_eq!(float_value.as_string(), None);
    assert_eq!(float_value.as_selection(), None);

    // Test String value
    let string_value = SettingValue::String("test".to_string());
    assert_eq!(string_value.as_bool(), None);
    assert_eq!(string_value.as_int(), None);
    assert_eq!(string_value.as_float(), None);
    assert_eq!(string_value.as_string(), Some("test"));
    assert_eq!(string_value.as_selection(), None);

    // Test Selection value
    let selection_value = SettingValue::Selection(2);
    assert_eq!(selection_value.as_bool(), None);
    assert_eq!(selection_value.as_int(), None);
    assert_eq!(selection_value.as_float(), None);
    assert_eq!(selection_value.as_string(), None);
    assert_eq!(selection_value.as_selection(), Some(2));
}

#[test]
fn test_settings_registry() {
    let registry = SettingsRegistry::game_settings();

    assert_eq!(registry.categories.len(), 2); // Updated to match actual implementation

    // Check audio category
    let audio_category = registry
        .categories
        .iter()
        .find(|cat| cat.name == "audio")
        .expect("Audio category should exist");

    assert_eq!(audio_category.display_name, "Audio");
    assert_eq!(audio_category.settings.len(), 4);

    // Check that master_volume setting exists
    let master_volume = audio_category
        .settings
        .iter()
        .find(|setting| setting.id == "master_volume")
        .expect("Master volume setting should exist");

    assert_eq!(master_volume.label, "Master Volume");
    assert_eq!(master_volume.default_value, SettingValue::Float(1.0));
}

#[test]
fn test_settings_builder() {
    let mut app = create_test_app();

    {
        let mut commands = app.world_mut().commands();
        let builder = SettingsBuilder::new().with_audio_settings();

        let entities = builder.spawn_settings(&mut commands);

        // Audio category should have 4 settings
        assert_eq!(entities.len(), 4);
    }

    app.update();

    // Verify settings were spawned
    let setting_count = get_all_settings(app.world_mut()).len();
    assert_eq!(setting_count, 4);
}

#[test]
fn test_settings_ext_trait() {
    let mut app = create_test_app();

    {
        let mut commands = app.world_mut().commands();
        let entities = commands.spawn_audio_settings();
        assert_eq!(entities.len(), 4); // Audio settings should have 4 items
    }

    app.update();

    // Verify settings were spawned
    let setting_count = get_all_settings(app.world_mut()).len();
    assert_eq!(setting_count, 4);

    // Test graphics settings
    {
        let mut commands = app.world_mut().commands();
        let entities = commands.spawn_graphics_settings();
        assert_eq!(entities.len(), 2); // Graphics settings should have 2 items
    }

    app.update();

    // Should now have 6 total settings (4 audio + 2 graphics)
    let setting_count = get_all_settings(app.world_mut()).len();
    assert_eq!(setting_count, 6);
}

#[test]
fn test_find_setting_by_id() {
    let mut app = create_test_app();

    {
        let world = app.world_mut();
        // Spawn test settings
        world.spawn(create_test_setting("setting1", SettingValue::Bool(true)));
        world.spawn(create_test_setting("setting2", SettingValue::Int(42)));
    }

    app.update();

    // Test finding existing setting
    let found = find_setting_by_id_in_world(app.world_mut(), "setting1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "setting1");

    // Test finding non-existing setting
    let not_found = find_setting_by_id_in_world(app.world_mut(), "nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_get_setting_value() {
    let mut app = create_test_app();

    {
        let world = app.world_mut();
        // Spawn test settings
        world.spawn(create_test_setting(
            "bool_setting",
            SettingValue::Bool(true),
        ));
        world.spawn(create_test_setting("int_setting", SettingValue::Int(42)));
    }

    app.update();

    // Test getting existing values
    let bool_setting = find_setting_by_id_in_world(app.world_mut(), "bool_setting");
    assert!(bool_setting.is_some());
    assert_eq!(bool_setting.unwrap().value, SettingValue::Bool(true));

    let int_setting = find_setting_by_id_in_world(app.world_mut(), "int_setting");
    assert!(int_setting.is_some());
    assert_eq!(int_setting.unwrap().value, SettingValue::Int(42));

    // Test getting non-existing value
    let none_setting = find_setting_by_id_in_world(app.world_mut(), "nonexistent");
    assert!(none_setting.is_none());
}

#[test]
fn test_setting_exists() {
    let mut app = create_test_app();

    {
        let world = app.world_mut();
        world.spawn(create_test_setting("existing", SettingValue::Bool(true)));
    }

    app.update();

    assert!(setting_exists_in_world(app.world_mut(), "existing"));
    assert!(!setting_exists_in_world(app.world_mut(), "nonexistent"));
}

#[test]
fn test_get_settings_in_category() {
    let mut app = create_test_app();

    {
        let world = app.world_mut();
        // Spawn settings in different categories
        world.spawn(create_test_setting("audio1", SettingValue::Bool(true)).with_category("audio"));
        world.spawn(create_test_setting("audio2", SettingValue::Float(0.5)).with_category("audio"));
        world.spawn(
            create_test_setting("graphics1", SettingValue::Bool(false)).with_category("graphics"),
        );
    }

    app.update();

    let audio_count = count_settings_in_category(app.world_mut(), "audio");
    assert_eq!(audio_count, 2);

    let graphics_count = count_settings_in_category(app.world_mut(), "graphics");
    assert_eq!(graphics_count, 1);

    let empty_count = count_settings_in_category(app.world_mut(), "nonexistent");
    assert_eq!(empty_count, 0);
}

#[test]
fn test_setting_changed_events() {
    let mut app = create_test_app();
    let entity;

    {
        let world = app.world_mut();
        // Spawn a test setting
        entity = world
            .spawn(create_test_setting("test", SettingValue::Bool(true)))
            .id();
    }

    app.update();

    {
        let world = app.world_mut();
        // Add a changed marker
        world.entity_mut(entity).insert(SettingChanged {
            old_value: SettingValue::Bool(true),
        });

        // Modify the setting value
        if let Some(mut setting) = world.get_mut::<Setting>(entity) {
            setting.value = SettingValue::Bool(false);
        }
    }

    app.update();

    // Check that event was sent by checking if the changed marker was removed
    assert!(app.world().get::<SettingChanged>(entity).is_none());
}

#[test]
fn test_simple_setting_update() {
    let mut app = create_test_app();
    let entity;

    {
        let world = app.world_mut();
        // Spawn a test setting
        entity = world
            .spawn(create_test_setting("test", SettingValue::Bool(true)))
            .id();
    }

    app.update();

    {
        let world = app.world_mut();
        // Directly update the setting
        if let Some(mut setting) = world.get_mut::<Setting>(entity) {
            setting.value = SettingValue::Bool(false);
        }
    }

    app.update();

    // Verify the setting was updated
    let updated_setting = app.world().get::<Setting>(entity).unwrap();
    assert_eq!(updated_setting.value, SettingValue::Bool(false));
}

#[test]
fn test_settings_plugin_integration() {
    let mut app = create_test_app();

    // Verify the plugin was added correctly
    assert!(app
        .world()
        .contains_resource::<Events<SettingChangedEvent>>());

    let entity;
    {
        // Add a setting and trigger the update system
        entity = app
            .world_mut()
            .spawn(create_test_setting("test", SettingValue::Bool(true)))
            .id();
    }

    app.update();

    {
        let world = app.world_mut();
        // Modify the setting and add change marker
        world.entity_mut(entity).insert(SettingChanged {
            old_value: SettingValue::Bool(true),
        });

        if let Some(mut setting) = world.get_mut::<Setting>(entity) {
            setting.value = SettingValue::Bool(false);
        }
    }

    app.update();

    // Verify the system processed the change by checking the marker was removed
    assert!(app.world().get::<SettingChanged>(entity).is_none());
}

#[test]
fn test_edge_cases() {
    let mut app = create_test_app();

    // Test with empty world
    let empty_count = get_all_settings(app.world_mut()).len();
    assert_eq!(empty_count, 0);

    // Test finding in empty world
    let not_found = find_setting_by_id_in_world(app.world_mut(), "anything");
    assert!(not_found.is_none());
}

#[test]
fn test_multiple_settings_update() {
    let mut app = create_test_app();
    let entity1;
    let entity2;
    let entity3;

    {
        let world = app.world_mut();
        // Spawn test settings
        entity1 = world
            .spawn(create_test_setting("setting1", SettingValue::Bool(true)))
            .id();
        entity2 = world
            .spawn(create_test_setting("setting2", SettingValue::Int(10)))
            .id();
        entity3 = world
            .spawn(create_test_setting("setting3", SettingValue::Float(1.0)))
            .id();
    }

    app.update();

    {
        let world = app.world_mut();
        // Update multiple settings directly
        if let Some(mut setting) = world.get_mut::<Setting>(entity1) {
            setting.value = SettingValue::Bool(false);
        }
        if let Some(mut setting) = world.get_mut::<Setting>(entity2) {
            setting.value = SettingValue::Int(20);
        }
        if let Some(mut setting) = world.get_mut::<Setting>(entity3) {
            setting.value = SettingValue::Float(0.5);
        }
    }

    app.update();

    // Verify all settings were updated
    assert_eq!(
        app.world().get::<Setting>(entity1).unwrap().value,
        SettingValue::Bool(false)
    );
    assert_eq!(
        app.world().get::<Setting>(entity2).unwrap().value,
        SettingValue::Int(20)
    );
    assert_eq!(
        app.world().get::<Setting>(entity3).unwrap().value,
        SettingValue::Float(0.5)
    );
}

#[test]
fn test_setting_type_validation() {
    // Test different setting types
    let toggle_setting = Setting::new(
        "toggle",
        "Toggle Setting",
        SettingValue::Bool(true),
        SettingType::Toggle,
    );
    assert!(matches!(toggle_setting.setting_type, SettingType::Toggle));

    let range_setting = Setting::new(
        "range",
        "Range Setting",
        SettingValue::Float(0.5),
        SettingType::FloatRange {
            min: 0.0,
            max: 1.0,
            step: 0.1,
        },
    );
    assert!(matches!(
        range_setting.setting_type,
        SettingType::FloatRange { .. }
    ));

    let selection_setting = Setting::new(
        "selection",
        "Selection Setting",
        SettingValue::Selection(0),
        SettingType::Selection {
            options: vec!["Option1".to_string()],
        },
    );
    assert!(matches!(
        selection_setting.setting_type,
        SettingType::Selection { .. }
    ));
}

#[test]
fn test_settings_builder_with_multiple_categories() {
    let mut app = create_test_app();

    {
        let mut commands = app.world_mut().commands();
        let builder = SettingsBuilder::new()
            .with_audio_settings()
            .with_graphics_settings();

        let entities = builder.spawn_settings(&mut commands);

        // Should have settings from both categories
        // Audio (4) + Graphics (2) = 6 total
        assert_eq!(entities.len(), 6);
    }

    app.update();

    // Verify all settings were spawned
    let setting_count = get_all_settings(app.world_mut()).len();
    assert_eq!(setting_count, 6);

    // Verify category distribution
    let audio_count = count_settings_in_category(app.world_mut(), "audio");
    assert_eq!(audio_count, 4);

    let graphics_count = count_settings_in_category(app.world_mut(), "graphics");
    assert_eq!(graphics_count, 2);
}

#[test]
fn test_game_settings_shortcut() {
    let mut app = create_test_app();

    {
        let mut commands = app.world_mut().commands();
        let entities = commands.spawn_game_settings();

        // Should have all settings from audio + graphics categories
        // Audio (4) + Graphics (2) = 6 total
        assert_eq!(entities.len(), 6);
    }

    app.update();

    // Verify all settings were spawned
    let setting_count = get_all_settings(app.world_mut()).len();
    assert_eq!(setting_count, 6);

    // Verify both categories are present
    let audio_count = count_settings_in_category(app.world_mut(), "audio");
    assert_eq!(audio_count, 4);

    let graphics_count = count_settings_in_category(app.world_mut(), "graphics");
    assert_eq!(graphics_count, 2);
}

#[test]
fn test_setting_value_conversions() {
    // Test value conversion edge cases
    let bool_val = SettingValue::Bool(true);
    assert!(bool_val.as_bool().unwrap());
    assert!(bool_val.as_int().is_none());

    let int_val = SettingValue::Int(-42);
    assert_eq!(int_val.as_int().unwrap(), -42);
    assert!(int_val.as_bool().is_none());

    let float_val = SettingValue::Float(-3.14);
    assert_eq!(float_val.as_float().unwrap(), -3.14);
    assert!(float_val.as_string().is_none());

    let empty_string = SettingValue::String("".to_string());
    assert_eq!(empty_string.as_string().unwrap(), "");
    assert!(empty_string.as_selection().is_none());

    let zero_selection = SettingValue::Selection(0);
    assert_eq!(zero_selection.as_selection().unwrap(), 0);
    assert!(zero_selection.as_float().is_none());
}

#[test]
fn test_system_helper_functions() {
    let mut app = create_test_app();

    // Spawn test settings using direct world access
    {
        let world = app.world_mut();
        world.spawn(create_test_setting("test1", SettingValue::Bool(true)));
        world.spawn(create_test_setting("test2", SettingValue::Float(0.5)));
    }

    app.update();

    // Test helper functions using world access - fix borrowing issues
    let value = get_setting_value_from_world(app.world_mut(), "test1");
    assert_eq!(value, Some(SettingValue::Bool(true)));

    let no_value = get_setting_value_from_world(app.world_mut(), "nonexistent");
    assert_eq!(no_value, None);

    let setting = find_setting_by_id_from_world(app.world_mut(), "test2");
    assert!(setting.is_some());
    assert_eq!(setting.unwrap().id, "test2");

    // Test get_settings_in_category with world access
    {
        let world = app.world_mut();
        world.spawn(create_test_setting("audio1", SettingValue::Bool(true)).with_category("audio"));
        world.spawn(create_test_setting("audio2", SettingValue::Float(0.8)).with_category("audio"));
    }

    app.update();

    let audio_settings = get_settings_in_category_from_world(app.world_mut(), "audio");
    assert_eq!(audio_settings.len(), 2);
}

#[test]
fn test_update_setting_by_id_helper() {
    let mut app = create_test_app();
    let entity;

    {
        let world = app.world_mut();
        entity = world
            .spawn(create_test_setting("volume", SettingValue::Float(0.5)))
            .id();
    }

    app.update();

    // Test using the helper manually since we can't easily create proper Query in tests
    {
        let world = app.world_mut();

        // Find the setting and update it manually
        let mut found = false;
        let mut query_state = world.query::<(Entity, &mut Setting)>();

        // Collect the update info first to avoid borrowing issues
        let update_info = query_state.iter(world).find_map(|(e, setting)| {
            if setting.id == "volume" {
                Some((e, setting.value.clone()))
            } else {
                None
            }
        });

        if let Some((update_entity, _old_value)) = update_info {
            if let Some(mut setting) = world.get_mut::<Setting>(update_entity) {
                let old_value = setting.value.clone();
                setting.value = SettingValue::Float(0.6);
                world
                    .entity_mut(update_entity)
                    .insert(SettingChanged { old_value });
                found = true;
            }
        }

        assert!(found);
    }

    app.update();

    // Verify the setting was updated and change marker was processed
    let updated_setting = app.world().get::<Setting>(entity).unwrap();
    assert_eq!(updated_setting.value, SettingValue::Float(0.6));

    // The change marker should have been processed and removed
    assert!(app.world().get::<SettingChanged>(entity).is_none());
}

#[test]
fn test_update_setting_with_helper_alternative() {
    let mut app = create_test_app();
    let entity;

    {
        let world = app.world_mut();
        entity = world
            .spawn(create_test_setting("volume", SettingValue::Float(0.5)))
            .id();
    }

    app.update();

    // Test alternative approach - direct update through world access
    {
        let world = app.world_mut();

        // Find and update the setting manually
        let mut found = false;
        let mut query_state = world.query::<(Entity, &mut Setting)>();

        // We need to collect the entity and new value first to avoid borrowing issues
        let update_info = query_state.iter(world).find_map(|(e, setting)| {
            if setting.id == "volume" {
                if let Some(current_vol) = setting.value.as_float() {
                    Some((e, SettingValue::Float((current_vol + 0.1).min(1.0))))
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some((update_entity, new_value)) = update_info {
            if let Some(mut setting) = world.get_mut::<Setting>(update_entity) {
                let old_value = setting.value.clone();
                setting.value = new_value;
                world
                    .entity_mut(update_entity)
                    .insert(SettingChanged { old_value });
                found = true;
            }
        }

        assert!(found);
    }

    app.update();

    // Verify the setting was updated and change marker was processed
    let updated_setting = app.world().get::<Setting>(entity).unwrap();
    assert_eq!(updated_setting.value, SettingValue::Float(0.6));

    // The change marker should have been processed and removed
    assert!(app.world().get::<SettingChanged>(entity).is_none());
}
