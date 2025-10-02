use crate::{
    theme::KonnektorenTheme,
    ui::{
        responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
        widgets::{ResponsiveText, ThemedButton},
    },
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Widget},
    EguiContexts,
};
use chrono::Utc;

/// Plugin for reusable about screen functionality
pub struct AboutPlugin;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AboutDismissed>().add_systems(
            Update,
            (check_about_config, handle_about_completion, render_about_ui),
        );
    }
}

/// Configuration for the about screen
#[derive(Component, Clone)]
pub struct AboutConfig {
    /// Main title of the application/game
    pub app_title: String,
    /// Subtitle describing the application
    pub subtitle: String,
    /// Version information (optional)
    pub version: Option<String>,
    /// Main description text
    pub description: String,
    /// Why choose us section content
    pub why_choose_us: Option<String>,
    /// List of features with titles and descriptions
    pub features: Vec<(String, String)>,
    /// List of technologies used
    pub technologies: Vec<(String, String)>,
    /// Beta/status message (optional)
    pub status_message: Option<(String, Color32)>, // (message, color)
    /// Website links
    pub websites: Vec<WebsiteLink>,
    /// Copyright holder name
    pub copyright_holder: String,
    /// Allow manual dismissal (back button/escape)
    pub manual_dismissal: bool,
    /// Custom extension widget renderer
    pub extension_widget: Option<fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo)>,
    /// Additional custom sections
    pub custom_sections: Vec<CustomSection>,
    /// Button text for dismissal
    pub dismiss_button_text: String,
}

/// Website link configuration
#[derive(Clone)]
pub struct WebsiteLink {
    pub title: String,
    pub description: String,
    pub url: String,
    pub icon: Option<String>, // Optional emoji or icon
}

/// Custom section for extending the about screen
#[derive(Clone)]
pub struct CustomSection {
    pub title: String,
    pub renderer: fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo),
}

impl Default for AboutConfig {
    fn default() -> Self {
        Self {
            app_title: "Konnektoren".to_string(),
            subtitle: "Educational Platform".to_string(),
            version: Some("Beta".to_string()),
            description: "Welcome to Konnektoren, an innovative educational platform designed to make learning engaging and effective.".to_string(),
            why_choose_us: Some("We offer interactive experiences designed for different proficiency levels, making learning both fun and effective.".to_string()),
            features: vec![
                ("🎮 Interactive Experience".to_string(), "Engaging activities that make learning enjoyable".to_string()),
                ("📊 Progress Tracking".to_string(), "Monitor your improvement and see your strengths".to_string()),
                ("🏆 Achievements".to_string(), "Earn rewards as you master new concepts".to_string()),
                ("📱 Cross-Platform".to_string(), "Access on desktop or in your web browser".to_string()),
            ],
            technologies: vec![
                ("Rust".to_string(), "Safe, fast, and modern programming language".to_string()),
                ("Bevy".to_string(), "Data-driven game engine".to_string()),
                ("egui".to_string(), "Immediate mode GUI library".to_string()),
                ("WebAssembly".to_string(), "For web browser support".to_string()),
            ],
            status_message: Some(("Currently in Beta - Thank you for being part of our journey!".to_string(), egui::Color32::from_rgb(255, 193, 7))),
            websites: vec![
                WebsiteLink {
                    title: "Konnektoren Web App".to_string(),
                    description: "Visit our main web application".to_string(),
                    url: "https://konnektoren.help".to_string(),
                    icon: Some("🌐".to_string()),
                },
            ],
            copyright_holder: "Konnektoren".to_string(),
            manual_dismissal: true,
            extension_widget: None,
            custom_sections: vec![],
            dismiss_button_text: "← Back".to_string(),
        }
    }
}

impl AboutConfig {
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

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_why_choose_us(mut self, text: impl Into<String>) -> Self {
        self.why_choose_us = Some(text.into());
        self
    }

    pub fn with_features(mut self, features: Vec<(String, String)>) -> Self {
        self.features = features;
        self
    }

    pub fn add_feature(mut self, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.features.push((title.into(), description.into()));
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

    pub fn with_status_message(mut self, message: impl Into<String>, color: egui::Color32) -> Self {
        self.status_message = Some((message.into(), color));
        self
    }

    pub fn with_websites(mut self, websites: Vec<WebsiteLink>) -> Self {
        self.websites = websites;
        self
    }

    pub fn add_website(
        mut self,
        title: impl Into<String>,
        description: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        self.websites.push(WebsiteLink {
            title: title.into(),
            description: description.into(),
            url: url.into(),
            icon: Some("🌐".to_string()),
        });
        self
    }

    pub fn with_copyright_holder(mut self, holder: impl Into<String>) -> Self {
        self.copyright_holder = holder.into();
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
        self.custom_sections.push(CustomSection {
            title: title.into(),
            renderer,
        });
        self
    }

    pub fn with_dismiss_button_text(mut self, text: impl Into<String>) -> Self {
        self.dismiss_button_text = text.into();
        self
    }

    /// Create a default game-focused about config
    pub fn for_game(title: impl Into<String>) -> Self {
        Self {
            app_title: title.into(),
            subtitle: "Educational Game".to_string(),
            description: "An engaging educational game designed to make learning fun and effective through interactive gameplay.".to_string(),
            features: vec![
                ("🎮 Interactive Gameplay".to_string(), "Fun challenges that test your knowledge".to_string()),
                ("🗺️ Challenge Map".to_string(), "Visual progression system to track your learning journey".to_string()),
                ("📊 Progress Tracking".to_string(), "Monitor your improvement and see your strengths".to_string()),
                ("🏆 Achievements".to_string(), "Earn rewards as you master new concepts".to_string()),
                ("📱 Cross-Platform".to_string(), "Play on desktop or in your web browser".to_string()),
            ],
            ..Default::default()
        }
    }
}

/// Component marking an active about screen
#[derive(Component)]
pub struct ActiveAbout {
    config: AboutConfig,
    navigation_state: NavigationState,
}

/// Navigation state for keyboard/gamepad support
#[derive(Clone)]
pub struct NavigationState {
    pub current_index: usize,
    pub max_index: usize,
    pub enabled: bool,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_index: 0,
            max_index: 0,
            enabled: true,
        }
    }
}

/// Event sent when about screen should be dismissed
#[derive(Message)]
pub struct AboutDismissed {
    pub entity: Entity,
}

/// System to check for new about configurations and set them up
#[allow(clippy::type_complexity)]
fn check_about_config(
    mut commands: Commands,
    query: Query<(Entity, &AboutConfig), (Without<ActiveAbout>, Changed<AboutConfig>)>,
    existing_about: Query<Entity, With<ActiveAbout>>,
) {
    for (entity, config) in query.iter() {
        info!("Setting up about screen for entity {:?}", entity);

        // Clean up any existing about screens first
        for existing_entity in existing_about.iter() {
            info!("Cleaning up existing about screen: {:?}", existing_entity);
            commands.entity(existing_entity).remove::<ActiveAbout>();
        }

        // Calculate navigation indices
        let mut nav_state = NavigationState {
            max_index: config.websites.len(),
            ..Default::default()
        };
        if config.manual_dismissal {
            nav_state.max_index += 1;
        }

        commands.entity(entity).insert(ActiveAbout {
            config: config.clone(),
            navigation_state: nav_state,
        });
    }
}

/// System to render about UI
fn render_about_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    mut query: Query<(Entity, &mut ActiveAbout)>,
    mut dismiss_events: MessageWriter<AboutDismissed>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        // Only render the first (most recent) about screen to avoid widget ID conflicts
        if let Some((entity, mut about)) = query.iter_mut().next() {
            // Check dismissal first with separate borrow
            let should_dismiss =
                about.config.manual_dismissal && input.just_pressed(KeyCode::Escape);
            if should_dismiss {
                dismiss_events.write(AboutDismissed { entity });
                return;
            }

            // Destructure to get separate borrows
            let ActiveAbout {
                config,
                navigation_state,
            } = &mut *about;

            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(theme.base_100))
                .show(ctx, |ui| {
                    render_about_content(
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
}

/// Render about screen content
fn render_about_content(
    ui: &mut egui::Ui,
    config: &AboutConfig,
    nav_state: &mut NavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    dismiss_events: &mut MessageWriter<AboutDismissed>,
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
        render_header(ui, config, theme, responsive);

        // Main scrollable content
        let scroll_height = ui.available_height() - 80.0;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                render_content_sections(ui, config, nav_state, theme, responsive);
            });

        // Back button at bottom
        if config.manual_dismissal {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
            render_dismiss_button(
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

fn render_header(
    ui: &mut egui::Ui,
    config: &AboutConfig,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    ui.vertical_centered(|ui| {
        // Title
        ResponsiveText::new(
            &format!("About {}", config.app_title),
            ResponsiveFontSize::Title,
            theme.primary,
        )
        .responsive(responsive)
        .strong()
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        // Version badge
        if let Some(version) = &config.version {
            let badge_frame = egui::Frame {
                inner_margin: egui::Margin::symmetric(12, 6),
                corner_radius: egui::CornerRadius::same(16),
                fill: theme.secondary,
                ..Default::default()
            };

            badge_frame.show(ui, |ui| {
                ResponsiveText::new(version, ResponsiveFontSize::Small, theme.secondary_content)
                    .responsive(responsive)
                    .strong()
                    .ui(ui);
            });

            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }

        // Subtitle
        ResponsiveText::new(
            &config.subtitle,
            ResponsiveFontSize::Large,
            theme.base_content,
        )
        .responsive(responsive)
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
    });
}

fn render_content_sections(
    ui: &mut egui::Ui,
    config: &AboutConfig,
    nav_state: &NavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    let section_spacing = responsive.spacing(ResponsiveSpacing::Large);

    // Main description
    render_section(ui, theme, responsive, "About This Platform", |ui| {
        ResponsiveText::new(
            &config.description,
            ResponsiveFontSize::Medium,
            theme.base_content,
        )
        .responsive(responsive)
        .ui(ui);
    });

    ui.add_space(section_spacing);

    // Why choose us section
    if let Some(why_choose_us) = &config.why_choose_us {
        render_section(ui, theme, responsive, "Why Choose Us?", |ui| {
            ResponsiveText::new(
                why_choose_us,
                ResponsiveFontSize::Medium,
                theme.base_content,
            )
            .responsive(responsive)
            .ui(ui);
        });
        ui.add_space(section_spacing);
    }

    // Features section
    if !config.features.is_empty() {
        render_section(ui, theme, responsive, "Features", |ui| {
            for (title, description) in &config.features {
                render_feature_item(ui, theme, responsive, title, description);
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
        render_section(ui, theme, responsive, &custom_section.title, |ui| {
            (custom_section.renderer)(ui, theme, responsive);
        });
        ui.add_space(section_spacing);
    }

    // Technology section
    if !config.technologies.is_empty() {
        render_section(ui, theme, responsive, "Built With", |ui| {
            for (tech_name, description) in &config.technologies {
                render_tech_item(ui, theme, responsive, tech_name, description);
            }
        });
        ui.add_space(section_spacing);
    }

    // Status message section
    if let Some((message, color)) = &config.status_message {
        render_section(ui, theme, responsive, "Status", |ui| {
            ResponsiveText::new(message, ResponsiveFontSize::Medium, *color)
                .responsive(responsive)
                .ui(ui);
        });
        ui.add_space(section_spacing * 1.5);
    }

    // Website links
    if !config.websites.is_empty() {
        ui.vertical_centered(|ui| {
            ResponsiveText::new(
                "Visit Our Websites",
                ResponsiveFontSize::Large,
                theme.secondary,
            )
            .responsive(responsive)
            .strong()
            .ui(ui);

            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

            for (index, website) in config.websites.iter().enumerate() {
                render_website_link(ui, theme, responsive, nav_state, website, index + 1);
                ui.add_space(responsive.spacing(ResponsiveSpacing::Small));
            }
        });
        ui.add_space(section_spacing * 2.0);
    }

    // Copyright notice
    ui.vertical_centered(|ui| {
        let current_year = Utc::now().format("%Y").to_string();
        ResponsiveText::new(
            &format!(
                "© {} {}. All rights reserved.",
                current_year, config.copyright_holder
            ),
            ResponsiveFontSize::Small,
            theme.accent,
        )
        .responsive(responsive)
        .ui(ui);
    });

    ui.add_space(responsive.spacing(ResponsiveSpacing::XLarge));
}

fn render_section<F>(
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

fn render_feature_item(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    title: &str,
    description: &str,
) {
    if responsive.is_mobile() {
        ui.vertical(|ui| {
            ResponsiveText::new(title, ResponsiveFontSize::Medium, theme.secondary)
                .responsive(responsive)
                .strong()
                .ui(ui);
            ResponsiveText::new(description, ResponsiveFontSize::Small, theme.base_content)
                .responsive(responsive)
                .ui(ui);
        });
    } else {
        ui.horizontal(|ui| {
            ResponsiveText::new(title, ResponsiveFontSize::Medium, theme.secondary)
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

fn render_website_link(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    nav_state: &NavigationState,
    website: &WebsiteLink,
    nav_index: usize,
) {
    let is_focused = nav_state.enabled && nav_state.current_index == nav_index;

    let link_frame = egui::Frame {
        inner_margin: egui::Margin::same(12),
        corner_radius: egui::CornerRadius::same(8),
        fill: if is_focused {
            theme.base_300.linear_multiply(1.2)
        } else {
            theme.base_300
        },
        stroke: egui::Stroke::new(
            if is_focused { 2.0 } else { 1.0 },
            if is_focused {
                theme.primary
            } else {
                theme.accent
            },
        ),
        ..Default::default()
    };

    link_frame.show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ResponsiveText::new(&website.title, ResponsiveFontSize::Medium, theme.primary)
                .responsive(responsive)
                .strong()
                .ui(ui);

            ResponsiveText::new(
                &website.description,
                ResponsiveFontSize::Small,
                theme.base_content,
            )
            .responsive(responsive)
            .ui(ui);

            ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

            let button_text = if let Some(icon) = &website.icon {
                format!("{} {}", icon, website.url)
            } else {
                website.url.clone()
            };

            let url_button = ThemedButton::new(&button_text, theme).responsive(responsive);

            if ui.add(url_button).clicked() {
                open_url(&website.url);
            }
        });
    });
}

fn render_dismiss_button(
    ui: &mut egui::Ui,
    config: &AboutConfig,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    nav_state: &NavigationState,
    entity: Entity,
    dismiss_events: &mut MessageWriter<AboutDismissed>,
) {
    ui.vertical_centered(|ui| {
        let _is_focused = nav_state.enabled && nav_state.current_index == 0;

        let back_button = ThemedButton::new(&config.dismiss_button_text, theme)
            .responsive(responsive)
            .width(if responsive.is_mobile() { 200.0 } else { 250.0 });

        if ui.add(back_button).clicked() {
            dismiss_events.write(AboutDismissed { entity });
        }
    });
}

/// System to handle about completion
fn handle_about_completion(
    mut commands: Commands,
    mut dismiss_events: MessageReader<AboutDismissed>,
) {
    for event in dismiss_events.read() {
        info!("Dismissing about screen for entity {:?}", event.entity);
        commands.entity(event.entity).remove::<ActiveAbout>();
    }
}

/// Helper function to open URLs
fn open_url(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = ["window"], js_name = open)]
            fn window_open(url: &str);
        }

        window_open(url);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Err(e) = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .or_else(|_| std::process::Command::new("open").arg(url).spawn())
            .or_else(|_| std::process::Command::new("start").arg(url).spawn())
        {
            error!("Failed to open URL: {}", e);
        }
    }
}

/// Helper trait for easy about screen setup
pub trait AboutScreenExt {
    /// Add an about screen with the given configuration
    fn spawn_about(&mut self, config: AboutConfig) -> Entity;

    /// Add a simple about screen with just a title
    fn spawn_simple_about(&mut self, title: impl Into<String>) -> Entity;

    /// Add a game-focused about screen
    fn spawn_game_about(&mut self, title: impl Into<String>) -> Entity;
}

impl AboutScreenExt for Commands<'_, '_> {
    fn spawn_about(&mut self, config: AboutConfig) -> Entity {
        self.spawn((Name::new("About Screen"), config)).id()
    }

    fn spawn_simple_about(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_about(AboutConfig::new(title))
    }

    fn spawn_game_about(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_about(AboutConfig::for_game(title))
    }
}
