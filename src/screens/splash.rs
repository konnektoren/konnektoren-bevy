use crate::{
    theme::KonnektorenTheme,
    ui::responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, StrokeKind, TextureId},
    EguiContexts, EguiUserTextures,
};
use std::collections::HashMap;

/// Plugin for reusable splash screen functionality
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SplashDismissed>()
            .add_systems(
                Update,
                (
                    check_splash_config,
                    update_splash_timer,
                    handle_splash_completion,
                    load_splash_images,
                ),
            )
            .add_systems(bevy_egui::EguiContextPass, render_splash_ui);
    }
}

/// Component to track loaded textures for splash screens
#[derive(Component)]
pub struct LoadedTextures {
    pub textures: HashMap<String, TextureId>,
    // Store handles to keep assets alive
    pub handles: HashMap<String, Handle<Image>>,
}

/// Component to track loading images (keeps handles alive)
#[derive(Component)]
pub struct LoadingImages {
    pub handles: HashMap<String, Handle<Image>>,
}

/// Logo display options for the splash screen
#[derive(Clone, Debug)]
pub enum LogoDisplay {
    /// No logo
    None,
    /// Show an emoji icon
    Emoji(String),
    /// Show text as logo
    Text(String),
    /// Load image from asset path (requires asset server)
    Image(String),
    /// Custom logo renderer (advanced usage)
    Custom(fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo)),
}

impl Default for LogoDisplay {
    fn default() -> Self {
        LogoDisplay::Emoji("🎮".to_string())
    }
}

/// Component that configures splash screen behavior
#[derive(Component, Clone)]
pub struct SplashConfig {
    /// Logo display configuration
    pub logo: LogoDisplay,
    /// Title text
    pub title: String,
    /// Subtitle text (optional)
    pub subtitle: Option<String>,
    /// Duration in seconds (0.0 = infinite, requires manual dismissal)
    pub duration: f32,
    /// Auto transition to next state when timer finishes
    pub auto_transition: bool,
    /// Allow manual dismissal (click/key press)
    pub manual_dismissal: bool,
    /// Background color (optional, uses theme default if None)
    pub background_color: Option<egui::Color32>,
    /// Custom button text (if manual dismissal enabled)
    pub button_text: Option<String>,
    /// Show loading indicator
    pub show_loading: bool,
    /// Logo size multiplier (1.0 = default size)
    pub logo_size_multiplier: f32,
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            logo: LogoDisplay::default(),
            title: "Loading...".to_string(),
            subtitle: None,
            duration: 2.0,
            auto_transition: true,
            manual_dismissal: true,
            background_color: None,
            button_text: None,
            show_loading: true,
            logo_size_multiplier: 1.0,
        }
    }
}

impl SplashConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set logo as emoji
    pub fn with_emoji_logo(mut self, emoji: impl Into<String>) -> Self {
        self.logo = LogoDisplay::Emoji(emoji.into());
        self
    }

    /// Set logo as text
    pub fn with_text_logo(mut self, text: impl Into<String>) -> Self {
        self.logo = LogoDisplay::Text(text.into());
        self
    }

    /// Set logo as image from asset path
    pub fn with_image_logo(mut self, path: impl Into<String>) -> Self {
        self.logo = LogoDisplay::Image(path.into());
        self
    }

    /// Set custom logo renderer
    pub fn with_custom_logo(
        mut self,
        renderer: fn(&mut egui::Ui, &KonnektorenTheme, &ResponsiveInfo),
    ) -> Self {
        self.logo = LogoDisplay::Custom(renderer);
        self
    }

    /// Remove logo entirely
    pub fn without_logo(mut self) -> Self {
        self.logo = LogoDisplay::None;
        self
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_auto_transition(mut self, auto_transition: bool) -> Self {
        self.auto_transition = auto_transition;
        self
    }

    pub fn with_manual_dismissal(mut self, manual_dismissal: bool) -> Self {
        self.manual_dismissal = manual_dismissal;
        self
    }

    pub fn with_background_color(mut self, color: egui::Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_button_text(mut self, text: impl Into<String>) -> Self {
        self.button_text = Some(text.into());
        self
    }

    pub fn with_loading_indicator(mut self, show: bool) -> Self {
        self.show_loading = show;
        self
    }

    pub fn with_logo_size(mut self, multiplier: f32) -> Self {
        self.logo_size_multiplier = multiplier;
        self
    }

    /// Infinite splash that requires manual dismissal
    pub fn infinite(mut self) -> Self {
        self.duration = 0.0;
        self.auto_transition = false;
        self.manual_dismissal = true;
        self
    }

    /// Create a Konnektoren-branded splash screen
    pub fn konnektoren() -> Self {
        Self {
            logo: LogoDisplay::Image("logo.png".to_string()),
            title: "Konnektoren".to_string(),
            subtitle: Some("Educational Games Platform".to_string()),
            duration: 3.0,
            auto_transition: true,
            manual_dismissal: true,
            background_color: None,
            button_text: Some("Enter".to_string()),
            show_loading: true,
            logo_size_multiplier: 1.2,
        }
    }
}

/// Component marking an active splash screen
#[derive(Component)]
pub struct ActiveSplash {
    timer: Timer,
    config: SplashConfig,
}

/// Event sent when splash screen should be dismissed
#[derive(Event)]
pub struct SplashDismissed {
    pub entity: Entity,
}

/// System to check for new splash configurations and set them up
#[allow(clippy::type_complexity)]
fn check_splash_config(
    mut commands: Commands,
    query: Query<(Entity, &SplashConfig), (Without<ActiveSplash>, Changed<SplashConfig>)>,
) {
    for (entity, config) in query.iter() {
        info!("Setting up splash screen for entity {:?}", entity);

        // Create timer
        let timer = if config.duration > 0.0 {
            Timer::from_seconds(config.duration, TimerMode::Once)
        } else {
            Timer::from_seconds(f32::MAX, TimerMode::Once) // Infinite timer
        };

        // Add ActiveSplash component
        commands.entity(entity).insert(ActiveSplash {
            timer,
            config: config.clone(),
        });
    }
}

/// System to load images for splash screens
fn load_splash_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    // Query for entities that need image loading
    active_query: Query<(Entity, &ActiveSplash), (Without<LoadedTextures>, Without<LoadingImages>)>,
    // Query for entities that are currently loading
    mut loading_query: Query<(Entity, &ActiveSplash, &mut LoadingImages), Without<LoadedTextures>>,
) {
    // Start loading for new splash screens
    for (entity, splash) in active_query.iter() {
        if let LogoDisplay::Image(path) = &splash.config.logo {
            info!("Starting to load image: {}", path);

            // Load the image asset and store the handle
            let image_handle: Handle<Image> = asset_server.load(path);

            let mut handles = HashMap::new();
            handles.insert(path.clone(), image_handle);

            commands.entity(entity).insert(LoadingImages { handles });
        }
    }

    // Check loading progress
    for (entity, splash, mut loading_images) in loading_query.iter_mut() {
        if let LogoDisplay::Image(path) = &splash.config.logo {
            if let Some(image_handle) = loading_images.handles.get(path) {
                // Check if the image is now loaded
                if let Some(_image) = images.get(image_handle) {
                    info!("Image loaded successfully: {}", path);

                    // Convert to egui texture
                    let texture_id = egui_user_textures.add_image(image_handle.clone());

                    let mut textures = HashMap::new();
                    textures.insert(path.clone(), texture_id);

                    // Move handles to keep them alive
                    let handles = std::mem::take(&mut loading_images.handles);

                    commands
                        .entity(entity)
                        .remove::<LoadingImages>()
                        .insert(LoadedTextures { textures, handles });
                }
            }
        }
    }
}

/// System to update splash timers
fn update_splash_timer(
    time: Res<Time>,
    mut query: Query<(Entity, &mut ActiveSplash)>,
    mut dismiss_events: EventWriter<SplashDismissed>,
) {
    for (entity, mut splash) in query.iter_mut() {
        if splash.config.duration > 0.0 {
            splash.timer.tick(time.delta());

            if splash.timer.just_finished() && splash.config.auto_transition {
                info!("Splash timer finished for entity {:?}", entity);
                dismiss_events.write(SplashDismissed { entity });
            }
        }
    }
}

/// System to render splash UI
fn render_splash_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    query: Query<(Entity, &ActiveSplash, Option<&LoadedTextures>)>,
    mut dismiss_events: EventWriter<SplashDismissed>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Early return if no active splash screens
    if query.is_empty() {
        return;
    }

    let ctx = contexts.ctx_mut();

    for (entity, splash, loaded_textures) in query.iter() {
        let config = &splash.config;

        // Handle keyboard dismissal
        if config.manual_dismissal
            && (input.just_pressed(KeyCode::Space)
                || input.just_pressed(KeyCode::Enter)
                || input.just_pressed(KeyCode::Escape))
        {
            dismiss_events.write(SplashDismissed { entity });
            continue;
        }

        // Determine background color
        let bg_color = config.background_color.unwrap_or(theme.base_100);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(bg_color))
            .show(ctx, |ui| {
                render_splash_content(
                    ui,
                    config,
                    splash,
                    &theme,
                    &responsive,
                    entity,
                    &mut dismiss_events,
                    loaded_textures,
                );
            });
    }
}

/// Render splash screen content
fn render_splash_content(
    ui: &mut egui::Ui,
    config: &SplashConfig,
    splash: &ActiveSplash,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    dismiss_events: &mut EventWriter<SplashDismissed>,
    loaded_textures: Option<&LoadedTextures>,
) {
    ui.vertical_centered(|ui| {
        let top_spacing = if responsive.is_mobile() { 50.0 } else { 80.0 };
        ui.add_space(top_spacing);

        // Render logo with image support
        render_logo_enhanced(
            ui,
            &config.logo,
            theme,
            responsive,
            config.logo_size_multiplier,
            loaded_textures,
        );

        // Title
        ui.heading(
            egui::RichText::new(&config.title)
                .color(theme.primary)
                .size(responsive.font_size(ResponsiveFontSize::Title))
                .strong(),
        );

        // Subtitle
        if let Some(subtitle) = &config.subtitle {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
            ui.label(
                egui::RichText::new(subtitle)
                    .color(theme.base_content)
                    .size(responsive.font_size(ResponsiveFontSize::Large)),
            );
        }

        // Loading indicator for timed splashes
        if config.show_loading && config.duration > 0.0 {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

            let progress = splash.timer.elapsed_secs() / splash.timer.duration().as_secs_f32();
            let progress = progress.clamp(0.0, 1.0);

            let time = ui.input(|i| i.time);
            let dots = match ((time * 2.0) as usize) % 4 {
                0 => "",
                1 => ".",
                2 => "..",
                3 => "...",
                _ => "",
            };

            ui.label(
                egui::RichText::new(format!("Loading{}", dots))
                    .color(theme.accent)
                    .size(responsive.font_size(ResponsiveFontSize::Medium)),
            );

            ui.add_space(responsive.spacing(ResponsiveSpacing::Small));
            let progress_bar = egui::ProgressBar::new(progress)
                .desired_width(200.0)
                .animate(true);
            ui.add(progress_bar);
        }

        // Manual dismissal button
        if config.manual_dismissal {
            ui.add_space(responsive.spacing(ResponsiveSpacing::XLarge));

            let button_text = config.button_text.as_deref().unwrap_or("Continue");
            let button = egui::Button::new(
                egui::RichText::new(button_text)
                    .size(responsive.font_size(ResponsiveFontSize::Large)),
            )
            .min_size(egui::vec2(120.0, 40.0));

            if ui.add(button).clicked() {
                dismiss_events.write(SplashDismissed { entity });
            }

            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
            ui.label(
                egui::RichText::new("Press Space, Enter, or Escape to continue")
                    .color(theme.base_content.gamma_multiply(0.7))
                    .size(responsive.font_size(ResponsiveFontSize::Small)),
            );
        }
    });
}

/// Enhanced logo rendering with actual image support
fn render_logo_enhanced(
    ui: &mut egui::Ui,
    logo: &LogoDisplay,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    size_multiplier: f32,
    loaded_textures: Option<&LoadedTextures>,
) {
    let base_size = if responsive.is_mobile() { 80.0 } else { 100.0 };
    let logo_size = base_size * size_multiplier;

    match logo {
        LogoDisplay::None => {
            // No logo, no space
        }
        LogoDisplay::Emoji(emoji) => {
            ui.label(egui::RichText::new(emoji).size(logo_size));
            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }
        LogoDisplay::Text(text) => {
            // Create a circular background for text logos
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(logo_size, logo_size), egui::Sense::hover());

            // Draw circular background
            ui.painter()
                .circle_filled(rect.center(), logo_size / 2.0, theme.primary);

            // Draw border
            ui.painter().circle_stroke(
                rect.center(),
                logo_size / 2.0,
                egui::Stroke::new(3.0, theme.primary_content),
            );

            // Draw text in center
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(logo_size * 0.5),
                theme.primary_content,
            );

            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }
        LogoDisplay::Image(path) => {
            // Try to render actual image if loaded
            if let Some(textures) = loaded_textures {
                if let Some(texture_id) = textures.textures.get(path) {
                    render_actual_image(ui, texture_id, logo_size, responsive);
                } else {
                    render_image_loading(ui, path, theme, responsive, logo_size);
                }
            } else {
                render_image_loading(ui, path, theme, responsive, logo_size);
            }
        }
        LogoDisplay::Custom(renderer) => {
            renderer(ui, theme, responsive);
            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }
    }
}

/// Render actual loaded image
fn render_actual_image(
    ui: &mut egui::Ui,
    texture_id: &TextureId,
    size: f32,
    responsive: &ResponsiveInfo,
) {
    // Create a square image with proper sizing
    let image_widget = egui::Image::from_texture((*texture_id, egui::vec2(size, size)))
        .fit_to_exact_size(egui::vec2(size, size))
        .corner_radius(egui::CornerRadius::same(8));

    ui.add(image_widget);
    ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
}

/// Render loading state for image
fn render_image_loading(
    ui: &mut egui::Ui,
    path: &str,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    size: f32,
) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());

    // Draw placeholder background
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(8), theme.base_200);

    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(8),
        egui::Stroke::new(2.0, theme.accent),
        StrokeKind::Outside,
    );

    // Show loading spinner
    let time = ui.input(|i| i.time);
    let angle = time % 2.0 * std::f64::consts::PI;
    let spinner_center = rect.center();
    let spinner_radius = size * 0.15;

    for i in 0..8 {
        let i_angle = angle + (i as f64 * std::f64::consts::PI / 4.0);
        let alpha = ((8 - i) as f32 / 8.0) * 0.8 + 0.2;
        let pos = egui::pos2(
            spinner_center.x + (spinner_radius * i_angle.cos() as f32),
            spinner_center.y + (spinner_radius * i_angle.sin() as f32),
        );

        ui.painter()
            .circle_filled(pos, 3.0, theme.primary.linear_multiply(alpha));
    }

    // Show filename
    ui.painter().text(
        rect.center() + egui::vec2(0.0, size * 0.35),
        egui::Align2::CENTER_CENTER,
        &format!("Loading {}", path.split('/').last().unwrap_or(path)),
        egui::FontId::proportional(size * 0.08),
        theme.base_content,
    );

    ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
}

/// System to handle splash completion
fn handle_splash_completion(
    mut commands: Commands,
    mut dismiss_events: EventReader<SplashDismissed>,
) {
    for event in dismiss_events.read() {
        info!("Dismissing splash screen for entity {:?}", event.entity);

        // Remove all splash-related components
        commands
            .entity(event.entity)
            .remove::<ActiveSplash>()
            .remove::<LoadedTextures>()
            .remove::<LoadingImages>();
    }
}

/// Helper trait for easy splash screen setup
pub trait SplashScreenExt {
    /// Add a splash screen with the given configuration
    fn spawn_splash(&mut self, config: SplashConfig) -> Entity;

    /// Add a simple splash screen with just a title
    fn spawn_simple_splash(&mut self, title: impl Into<String>) -> Entity;

    /// Add a Konnektoren-branded splash screen
    fn spawn_konnektoren_splash(&mut self) -> Entity;
}

impl SplashScreenExt for Commands<'_, '_> {
    fn spawn_splash(&mut self, config: SplashConfig) -> Entity {
        self.spawn((Name::new("Splash Screen"), config)).id()
    }

    fn spawn_simple_splash(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_splash(SplashConfig::new(title))
    }

    fn spawn_konnektoren_splash(&mut self) -> Entity {
        self.spawn_splash(SplashConfig::konnektoren())
    }
}
