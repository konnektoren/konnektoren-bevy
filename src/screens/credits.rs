use crate::{
    theme::KonnektorenTheme,
    ui::{
        responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
        widgets::{ResponsiveText, ThemedButton},
    },
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Widget},
    EguiContextPass, EguiContexts,
};

/// Plugin for reusable credits screen functionality
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreditsDismissed>()
            .add_systems(Update, (check_credits_config, handle_credits_completion))
            .add_systems(EguiContextPass, render_credits_ui);
    }
}

/// Configuration for the credits screen
#[derive(Component, Clone)]
pub struct CreditsConfig {
    /// Main title of the application/game
    pub app_title: String,
    /// Subtitle describing the credits
    pub subtitle: String,
    /// List of team members/contributors with their roles
    pub team_members: Vec<(String, String)>, // (name, role/description)
    /// List of assets and their attributions
    pub assets: Vec<(String, String)>, // (asset name, attribution)
    /// List of special thanks
    pub special_thanks: Vec<(String, String)>, // (name, reason)
    /// List of technologies used
    pub technologies: Vec<(String, String)>, // (tech name, description)
    /// Copyright information
    pub copyright_info: Option<String>,
    /// Allow manual dismissal (back button/escape)
    pub manual_dismissal: bool,
    /// Custom extension widget renderer
    pub extension_widget: Option<fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo)>,
    /// Additional custom sections
    pub custom_sections: Vec<CustomCreditsSection>,
    /// Button text for dismissal
    pub dismiss_button_text: String,
}

/// Custom section for extending the credits screen
#[derive(Clone)]
pub struct CustomCreditsSection {
    pub title: String,
    pub renderer: fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo),
}

impl Default for CreditsConfig {
    fn default() -> Self {
        Self {
            app_title: "Konnektoren".to_string(),
            subtitle: "Credits".to_string(),
            team_members: vec![(
                "Development Team".to_string(),
                "Built with passion and dedication".to_string(),
            )],
            assets: vec![
                (
                    "Icons".to_string(),
                    "Various sources under Creative Commons".to_string(),
                ),
                ("Fonts".to_string(), "Open source typography".to_string()),
            ],
            special_thanks: vec![
                (
                    "Bevy Community".to_string(),
                    "For the amazing game engine".to_string(),
                ),
                (
                    "Rust Community".to_string(),
                    "For the incredible programming language".to_string(),
                ),
            ],
            technologies: vec![
                (
                    "Rust".to_string(),
                    "Safe, fast, and modern programming language".to_string(),
                ),
                ("Bevy".to_string(), "Data-driven game engine".to_string()),
                ("egui".to_string(), "Immediate mode GUI library".to_string()),
                (
                    "WebAssembly".to_string(),
                    "For web browser support".to_string(),
                ),
            ],
            copyright_info: Some("All rights reserved.".to_string()),
            manual_dismissal: true,
            extension_widget: None,
            custom_sections: vec![],
            dismiss_button_text: "← Back".to_string(),
        }
    }
}

impl CreditsConfig {
    pub fn new(app_title: impl Into<String>) -> Self {
        Self {
            app_title: app_title.into(),
            ..Default::default()
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn with_team_members(mut self, team_members: Vec<(String, String)>) -> Self {
        self.team_members = team_members;
        self
    }

    pub fn add_team_member(mut self, name: impl Into<String>, role: impl Into<String>) -> Self {
        self.team_members.push((name.into(), role.into()));
        self
    }

    pub fn with_assets(mut self, assets: Vec<(String, String)>) -> Self {
        self.assets = assets;
        self
    }

    pub fn add_asset(mut self, name: impl Into<String>, attribution: impl Into<String>) -> Self {
        self.assets.push((name.into(), attribution.into()));
        self
    }

    pub fn with_special_thanks(mut self, special_thanks: Vec<(String, String)>) -> Self {
        self.special_thanks = special_thanks;
        self
    }

    pub fn add_special_thanks(
        mut self,
        name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        self.special_thanks.push((name.into(), reason.into()));
        self
    }

    pub fn with_technologies(mut self, technologies: Vec<(String, String)>) -> Self {
        self.technologies = technologies;
        self
    }

    pub fn add_technology(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.technologies.push((name.into(), description.into()));
        self
    }

    pub fn with_copyright_info(mut self, info: impl Into<String>) -> Self {
        self.copyright_info = Some(info.into());
        self
    }

    pub fn with_extension_widget(
        mut self,
        widget: fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo),
    ) -> Self {
        self.extension_widget = Some(widget);
        self
    }

    pub fn add_custom_section(
        mut self,
        title: impl Into<String>,
        renderer: fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo),
    ) -> Self {
        self.custom_sections.push(CustomCreditsSection {
            title: title.into(),
            renderer,
        });
        self
    }

    pub fn with_dismiss_button_text(mut self, text: impl Into<String>) -> Self {
        self.dismiss_button_text = text.into();
        self
    }

    /// Create a default game-focused credits config
    pub fn for_game(title: impl Into<String>) -> Self {
        Self {
            app_title: title.into(),
            subtitle: "Game Credits".to_string(),
            team_members: vec![
                (
                    "Game Designer".to_string(),
                    "Concept and gameplay design".to_string(),
                ),
                (
                    "Developer".to_string(),
                    "Programming and implementation".to_string(),
                ),
                ("Artist".to_string(), "Visual design and assets".to_string()),
            ],
            assets: vec![
                (
                    "Game Art".to_string(),
                    "Original artwork and sprites".to_string(),
                ),
                (
                    "Sound Effects".to_string(),
                    "Audio design and implementation".to_string(),
                ),
                (
                    "Music".to_string(),
                    "Background music and themes".to_string(),
                ),
            ],
            ..Default::default()
        }
    }
}

/// Component marking an active credits screen
#[derive(Component)]
pub struct ActiveCredits {
    config: CreditsConfig,
    navigation_state: CreditsNavigationState,
}

/// Navigation state for keyboard/gamepad support
#[derive(Clone)]
pub struct CreditsNavigationState {
    pub current_index: usize,
    pub max_index: usize,
    pub enabled: bool,
}

impl Default for CreditsNavigationState {
    fn default() -> Self {
        Self {
            current_index: 0,
            max_index: 0,
            enabled: true,
        }
    }
}

/// Event sent when credits screen should be dismissed
#[derive(Event)]
pub struct CreditsDismissed {
    pub entity: Entity,
}

/// System to check for new credits configurations and set them up
#[allow(clippy::type_complexity)]
fn check_credits_config(
    mut commands: Commands,
    query: Query<(Entity, &CreditsConfig), (Without<ActiveCredits>, Changed<CreditsConfig>)>,
    existing_credits: Query<Entity, With<ActiveCredits>>,
) {
    for (entity, config) in query.iter() {
        info!("Setting up credits screen for entity {:?}", entity);

        // Clean up any existing credits screens first
        for existing_entity in existing_credits.iter() {
            info!("Cleaning up existing credits screen: {:?}", existing_entity);
            commands.entity(existing_entity).remove::<ActiveCredits>();
        }

        // Calculate navigation indices
        let mut nav_state = CreditsNavigationState {
            max_index: 0,
            ..Default::default()
        };
        if config.manual_dismissal {
            nav_state.max_index += 1;
        }

        commands.entity(entity).insert(ActiveCredits {
            config: config.clone(),
            navigation_state: nav_state,
        });
    }
}

/// System to render credits UI
fn render_credits_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    mut query: Query<(Entity, &mut ActiveCredits)>,
    mut dismiss_events: EventWriter<CreditsDismissed>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    let ctx = contexts.ctx_mut();

    // Only render the first (most recent) credits screen to avoid widget ID conflicts
    if let Some((entity, mut credits)) = query.iter_mut().next() {
        // Check dismissal first with separate borrow
        let should_dismiss = credits.config.manual_dismissal && input.just_pressed(KeyCode::Escape);
        if should_dismiss {
            dismiss_events.write(CreditsDismissed { entity });
            return;
        }

        // Destructure to get separate borrows
        let ActiveCredits {
            config,
            navigation_state,
        } = &mut *credits;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.base_100))
            .show(ctx, |ui| {
                render_credits_content(
                    ui,
                    config,
                    navigation_state,
                    &theme,
                    &responsive,
                    entity,
                    &mut dismiss_events,
                );
            });
    }
}

/// Render credits screen content
fn render_credits_content(
    ui: &mut egui::Ui,
    config: &CreditsConfig,
    nav_state: &mut CreditsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    dismiss_events: &mut EventWriter<CreditsDismissed>,
) {
    ui.vertical_centered(|ui| {
        let max_width = if responsive.is_mobile() {
            ui.available_width() * 0.95
        } else {
            800.0_f32.min(ui.available_width() * 0.9)
        };

        ui.set_max_width(max_width);

        let top_spacing = responsive.spacing(ResponsiveSpacing::Large);
        ui.add_space(top_spacing);

        // Header section
        render_credits_header(ui, config, theme, responsive);

        // Main scrollable content
        let scroll_height = ui.available_height() - 80.0;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                render_credits_sections(ui, config, nav_state, theme, responsive);
            });

        // Back button at bottom
        if config.manual_dismissal {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
            render_credits_dismiss_button(
                ui,
                config,
                theme,
                responsive,
                nav_state,
                entity,
                dismiss_events,
            );
        }

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    });
}

fn render_credits_header(
    ui: &mut egui::Ui,
    config: &CreditsConfig,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    ui.vertical_centered(|ui| {
        // Title
        ResponsiveText::new(&config.subtitle, ResponsiveFontSize::Title, theme.primary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

        // App title
        ResponsiveText::new(
            &config.app_title,
            ResponsiveFontSize::Large,
            theme.base_content,
        )
        .responsive(responsive)
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
    });
}

fn render_credits_sections(
    ui: &mut egui::Ui,
    config: &CreditsConfig,
    _nav_state: &CreditsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    let section_spacing = responsive.spacing(ResponsiveSpacing::Large);

    // Team members section
    if !config.team_members.is_empty() {
        render_credits_section(ui, theme, responsive, "Team", |ui| {
            for (name, role) in &config.team_members {
                render_credits_item(ui, theme, responsive, name, role);
            }
        });
        ui.add_space(section_spacing);
    }

    // Assets section
    if !config.assets.is_empty() {
        render_credits_section(ui, theme, responsive, "Assets & Attributions", |ui| {
            for (asset_name, attribution) in &config.assets {
                render_credits_item(ui, theme, responsive, asset_name, attribution);
            }
        });
        ui.add_space(section_spacing);
    }

    // Special thanks section
    if !config.special_thanks.is_empty() {
        render_credits_section(ui, theme, responsive, "Special Thanks", |ui| {
            for (name, reason) in &config.special_thanks {
                render_credits_item(ui, theme, responsive, name, reason);
            }
        });
        ui.add_space(section_spacing);
    }

    // Custom extension widget
    if let Some(extension_widget) = config.extension_widget {
        extension_widget(ui, theme, responsive);
        ui.add_space(section_spacing);
    }

    // Custom sections
    for custom_section in &config.custom_sections {
        render_credits_section(ui, theme, responsive, &custom_section.title, |ui| {
            (custom_section.renderer)(ui, theme, responsive);
        });
        ui.add_space(section_spacing);
    }

    // Technology section
    if !config.technologies.is_empty() {
        render_credits_section(ui, theme, responsive, "Built With", |ui| {
            for (tech_name, description) in &config.technologies {
                render_tech_item(ui, theme, responsive, tech_name, description);
            }
        });
        ui.add_space(section_spacing);
    }

    // Copyright information
    if let Some(copyright_info) = &config.copyright_info {
        ui.vertical_centered(|ui| {
            ResponsiveText::new(copyright_info, ResponsiveFontSize::Small, theme.accent)
                .responsive(responsive)
                .ui(ui);
        });
        ui.add_space(section_spacing);
    }

    ui.add_space(responsive.spacing(ResponsiveSpacing::XLarge));
}

fn render_credits_section<F>(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    title: &str,
    content: F,
) where
    F: FnOnce(&mut egui::Ui),
{
    let margin = if responsive.is_mobile() { 12 } else { 16 };
    let frame = egui::Frame {
        inner_margin: egui::Margin::same(margin),
        corner_radius: egui::CornerRadius::same(8),
        fill: theme.base_200,
        stroke: egui::Stroke::new(1.0, theme.accent.linear_multiply(0.3)),
        ..Default::default()
    };

    frame.show(ui, |ui| {
        ResponsiveText::new(title, ResponsiveFontSize::Large, theme.primary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        content(ui);
    });
}

fn render_credits_item(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    name: &str,
    description: &str,
) {
    if responsive.is_mobile() {
        ui.vertical(|ui| {
            ResponsiveText::new(name, ResponsiveFontSize::Medium, theme.secondary)
                .responsive(responsive)
                .strong()
                .ui(ui);
            ResponsiveText::new(description, ResponsiveFontSize::Small, theme.base_content)
                .responsive(responsive)
                .ui(ui);
        });
    } else {
        ui.horizontal(|ui| {
            ResponsiveText::new(name, ResponsiveFontSize::Medium, theme.secondary)
                .responsive(responsive)
                .strong()
                .ui(ui);
            ui.label(" - ");
            ResponsiveText::new(description, ResponsiveFontSize::Medium, theme.base_content)
                .responsive(responsive)
                .ui(ui);
        });
    }

    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));
}

fn render_tech_item(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    tech_name: &str,
    description: &str,
) {
    ui.horizontal(|ui| {
        let tech_badge = egui::Frame {
            inner_margin: egui::Margin::symmetric(8, 4),
            corner_radius: egui::CornerRadius::same(4),
            fill: theme.info,
            ..Default::default()
        };

        tech_badge.show(ui, |ui| {
            ResponsiveText::new(tech_name, ResponsiveFontSize::Small, theme.primary_content)
                .responsive(responsive)
                .strong()
                .ui(ui);
        });

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        ResponsiveText::new(description, ResponsiveFontSize::Medium, theme.base_content)
            .responsive(responsive)
            .ui(ui);
    });

    ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));
}

fn render_credits_dismiss_button(
    ui: &mut egui::Ui,
    config: &CreditsConfig,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    _nav_state: &CreditsNavigationState,
    entity: Entity,
    dismiss_events: &mut EventWriter<CreditsDismissed>,
) {
    ui.vertical_centered(|ui| {
        let back_button = ThemedButton::new(&config.dismiss_button_text, theme)
            .responsive(responsive)
            .width(if responsive.is_mobile() { 200.0 } else { 250.0 });

        if ui.add(back_button).clicked() {
            dismiss_events.write(CreditsDismissed { entity });
        }
    });
}

/// System to handle credits completion
fn handle_credits_completion(
    mut commands: Commands,
    mut dismiss_events: EventReader<CreditsDismissed>,
) {
    for event in dismiss_events.read() {
        info!("Dismissing credits screen for entity {:?}", event.entity);
        commands.entity(event.entity).remove::<ActiveCredits>();
    }
}

/// Helper trait for easy credits screen setup
pub trait CreditsScreenExt {
    /// Add a credits screen with the given configuration
    fn spawn_credits(&mut self, config: CreditsConfig) -> Entity;

    /// Add a simple credits screen with just a title
    fn spawn_simple_credits(&mut self, title: impl Into<String>) -> Entity;

    /// Add a game-focused credits screen
    fn spawn_game_credits(&mut self, title: impl Into<String>) -> Entity;
}

impl CreditsScreenExt for Commands<'_, '_> {
    fn spawn_credits(&mut self, config: CreditsConfig) -> Entity {
        self.spawn((Name::new("Credits Screen"), config)).id()
    }

    fn spawn_simple_credits(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_credits(CreditsConfig::new(title))
    }

    fn spawn_game_credits(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_credits(CreditsConfig::for_game(title))
    }
}
