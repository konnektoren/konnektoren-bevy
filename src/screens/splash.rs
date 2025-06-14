use crate::{
    theme::KonnektorenTheme,
    ui::responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, StrokeKind},
    EguiContexts,
};

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
                ),
            )
            .add_systems(bevy_egui::EguiContextPass, render_splash_ui);
    }
}

/// Component that configures splash screen behavior
/// Add this to any entity to trigger a splash screen
#[derive(Component, Clone)]
pub struct SplashConfig {
    /// Logo asset path (optional) - simplified to just show a placeholder
    pub logo_path: Option<String>,
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
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            logo_path: None,
            title: "Loading...".to_string(),
            subtitle: None,
            duration: 2.0,
            auto_transition: true,
            manual_dismissal: true,
            background_color: None,
            button_text: None,
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

    pub fn with_logo(mut self, logo_path: impl Into<String>) -> Self {
        self.logo_path = Some(logo_path.into());
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

    /// Infinite splash that requires manual dismissal
    pub fn infinite(mut self) -> Self {
        self.duration = 0.0;
        self.auto_transition = false;
        self.manual_dismissal = true;
        self
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

/// System to render splash UI - simplified without texture loading
fn render_splash_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    query: Query<(Entity, &ActiveSplash)>,
    mut dismiss_events: EventWriter<SplashDismissed>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Early return if no active splash screens
    if query.is_empty() {
        return;
    }

    let ctx = contexts.ctx_mut();

    for (entity, splash) in query.iter() {
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
                );
            });
    }
}

/// Simplified splash content without actual logo loading
fn render_splash_content(
    ui: &mut egui::Ui,
    config: &SplashConfig,
    splash: &ActiveSplash,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    dismiss_events: &mut EventWriter<SplashDismissed>,
) {
    ui.vertical_centered(|ui| {
        let top_spacing = if responsive.is_mobile() { 50.0 } else { 80.0 };
        ui.add_space(top_spacing);

        // Logo placeholder - simplified to just show an icon
        if config.logo_path.is_some() {
            render_logo_placeholder(ui, theme, responsive);
        }

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
        if config.duration > 0.0 {
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

fn render_logo_placeholder(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    let logo_size = if responsive.is_mobile() { 60.0 } else { 80.0 };

    let (rect, _) = ui.allocate_exact_size(egui::vec2(logo_size, logo_size), egui::Sense::hover());

    // Draw a nice placeholder with border
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(8), theme.base_200);

    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(8),
        egui::Stroke::new(2.0, theme.primary.linear_multiply(0.5)),
        StrokeKind::Outside,
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "🎮",
        egui::FontId::proportional(logo_size * 0.4),
        theme.primary,
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

        // Remove ActiveSplash component
        commands.entity(event.entity).remove::<ActiveSplash>();
    }
}

/// Helper trait for easy splash screen setup
pub trait SplashScreenExt {
    /// Add a splash screen with the given configuration
    fn spawn_splash(&mut self, config: SplashConfig) -> Entity;

    /// Add a simple splash screen with just a title
    fn spawn_simple_splash(&mut self, title: impl Into<String>) -> Entity;
}

impl SplashScreenExt for Commands<'_, '_> {
    fn spawn_splash(&mut self, config: SplashConfig) -> Entity {
        self.spawn((Name::new("Splash Screen"), config)).id()
    }

    fn spawn_simple_splash(&mut self, title: impl Into<String>) -> Entity {
        self.spawn_splash(SplashConfig::new(title))
    }
}
