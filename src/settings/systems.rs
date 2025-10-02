use super::components::*;
use bevy::prelude::*;

/// System to detect and handle setting changes
pub fn update_settings_from_components(
    mut changed_settings: Query<(Entity, &mut Setting, Option<&SettingChanged>), Changed<Setting>>,
    mut commands: Commands,
    mut setting_events: MessageWriter<SettingChangedEvent>,
) {
    for (entity, setting, changed_marker) in changed_settings.iter_mut() {
        if let Some(changed) = changed_marker {
            // Send event about the change
            setting_events.write(SettingChangedEvent {
                setting_id: setting.id.clone(),
                old_value: changed.old_value.clone(),
                new_value: setting.value.clone(),
            });

            // Remove the changed marker
            commands.entity(entity).remove::<SettingChanged>();
        }
    }
}

/// Helper function to update a setting value by ID
pub fn update_setting_by_id(
    setting_id: &str,
    new_value: SettingValue,
    mut settings_query: Query<(Entity, &mut Setting)>,
    mut commands: Commands,
) -> bool {
    for (entity, mut setting) in settings_query.iter_mut() {
        if setting.id == setting_id {
            let old_value = setting.value.clone();
            setting.value = new_value;

            commands.entity(entity).insert(SettingChanged { old_value });
            return true;
        }
    }
    false
}

/// Query helper to find a setting by ID
pub fn find_setting_by_id<'a>(
    settings_query: &'a Query<&Setting>,
    setting_id: &str,
) -> Option<&'a Setting> {
    settings_query
        .iter()
        .find(|setting| setting.id == setting_id)
}

/// Query helper to get setting value by ID
pub fn get_setting_value(
    settings_query: &Query<&Setting>,
    setting_id: &str,
) -> Option<SettingValue> {
    find_setting_by_id(settings_query, setting_id).map(|setting| setting.value.clone())
}

/// Helper function to update a setting value using a closure
pub fn update_setting_with<F>(
    setting_id: &str,
    mut settings_query: Query<(Entity, &mut Setting)>,
    mut commands: Commands,
    updater: F,
) -> bool
where
    F: FnOnce(&SettingValue) -> Option<SettingValue>,
{
    let setting_info = settings_query.iter().find_map(|(entity, setting)| {
        if setting.id == setting_id {
            Some((entity, setting.value.clone()))
        } else {
            None
        }
    });

    if let Some((entity, current_value)) = setting_info {
        if let Some(new_value) = updater(&current_value) {
            if let Ok((_, mut setting)) = settings_query.get_mut(entity) {
                let old_value = setting.value.clone();
                setting.value = new_value;
                commands.entity(entity).insert(SettingChanged { old_value });
                return true;
            }
        }
    }
    false
}

/// Helper function to get all settings in a category
pub fn get_settings_in_category<'a>(
    settings_query: &'a Query<&Setting>,
    category: &str,
) -> Vec<&'a Setting> {
    settings_query
        .iter()
        .filter(|setting| setting.category.as_ref().is_some_and(|cat| cat == category))
        .collect()
}
