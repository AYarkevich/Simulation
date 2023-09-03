use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use egui::{
    plot::{Line, Plot, PlotPoints},
    Rgba,
};

use crate::cameras::main_camera::GameCapCamera;
use crate::utils::ring_buffer::RingBuffer;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Resource)]
pub(crate) struct InfoBoardSettings {
    occupied_screen_space: OccupiedScreenSpace,
    pub world_metrics: WorldMetrics,
    show_logs: bool,
    world_cursor_position: Vec3,
    window_cursor_position: Vec2,
    frames: RingBuffer<f64>,
}

#[derive(Default)]
struct OccupiedScreenSpace {
    right: f32,
    bottom: f32,
}

#[derive(Default)]
pub struct WorldMetrics {
    pub seeds_count: RingBuffer<f64>,
    pub cells_count: RingBuffer<f64>,
}

pub struct InfoBoardPlugin;

impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin, EguiPlugin))
            .init_resource::<InfoBoardSettings>()
            .add_systems(PostStartup, info_board_setup_system)
            .add_systems(Update, (change_text_system, cursor_events, ui_example_system));
    }
}
fn info_board_setup_system(mut info_board_settings: ResMut<InfoBoardSettings>) {
    setup_buffer(&mut info_board_settings.frames, 50);
    setup_buffer(&mut info_board_settings.world_metrics.seeds_count, 50);
    setup_buffer(&mut info_board_settings.world_metrics.cells_count, 50);
}

fn setup_buffer(input_buffer: &mut RingBuffer<f64>, capacity: usize) {
    input_buffer.increase_capacity(capacity);
    input_buffer.push(0.);
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut info_board_settings: ResMut<InfoBoardSettings>,
) {
    let ctx = contexts.ctx_mut();

    info_board_settings.occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .min_width(230.)
        .show(ctx, |ui| {
            ui.heading("Details:");
            ui.add_space(24.);

            //FPS
            let last_fps = info_board_settings.frames.get_last().unwrap();
            ui.heading(format!("FPS: {last_fps:.1}"));
            example_plot(ui, &info_board_settings.frames, "Frames plot");
            ui.separator();
            ui.add_space(24.);

            //POSITION
            let window_pos = info_board_settings.window_cursor_position;
            let world_pos = info_board_settings.world_cursor_position;
            ui.heading(format!("Mouse positions:"));
            ui.label(format!(
                "Window: x: {:.2}, y: {:.2}",
                window_pos.x, window_pos.y
            ));
            ui.label(format!(
                "World: x: {:.2}, y: {:.2}, z: {:.2}",
                world_pos.x, world_pos.y, world_pos.z
            ));
            ui.separator();
            ui.add_space(24.);

            //SEEDS
            let seeds_count = info_board_settings
                .world_metrics
                .seeds_count
                .get_last()
                .unwrap();
            ui.heading(format!("SEEDS: {seeds_count:}"));
            example_plot(
                ui,
                &info_board_settings.world_metrics.seeds_count,
                "Seeds plot",
            );
            ui.separator();
            ui.add_space(24.);

            //CELLS
            let cells_count = info_board_settings
                .world_metrics
                .cells_count
                .get_last()
                .unwrap();
            ui.heading(format!("CELLS: {cells_count:}"));
            example_plot(
                ui,
                &info_board_settings.world_metrics.cells_count,
                "Cells plot",
            );
            ui.separator();
            ui.add_space(24.);

            //ADVANCED
            ui.heading(format!("Advanced:"));
            ui.checkbox(&mut info_board_settings.show_logs, "Show logs");
            ui.separator();
        })
        .response
        .rect
        .width();

    if info_board_settings.show_logs {
        info_board_settings.occupied_screen_space.bottom =
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(230.)
                .show(ctx, |ui| {
                    ui.label("Bottom resizeable panel");
                    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                })
                .response
                .rect
                .height();
    }
}

fn example_plot(ui: &mut egui::Ui, frames: &RingBuffer<f64>, id: &str) -> egui::Response {
    let n = 128;
    let mut position = 0;
    let line_points: PlotPoints = frames
        .iter()
        .map(|i| {
            position = position + 1;
            [position as f64, *i]
        })
        .collect();
    let line = Line::new(line_points).color(Rgba::from_rgb(55., 55., 155.));

    Plot::new(id)
        .height(64.0)
        .show_axes([false, false])
        .allow_scroll(false)
        .allow_drag(false)
        .allow_zoom(false)
        .data_aspect(1.0)
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}

fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut info_board_settings: ResMut<InfoBoardSettings>,
) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;
        }
    }

    info_board_settings.frames.push(fps)
}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCapCamera>>,
    mut info_board_settings: ResMut<InfoBoardSettings>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    info_board_settings.window_cursor_position = window.cursor_position().unwrap_or_default();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        for _ in cursor_evr.iter() {
            info_board_settings.world_cursor_position = world_position;
        }
    }
}
