use winapi::{shared::windef::POINT, um::winuser};

/// Can be used to store native window settings (position and size).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WindowSettings {
    /// Inner position of window in physical pixels
    inner_pos: Option<egui::Pos2>,
    /// Inner size of window in logical pixels
    inner_size_points: Option<egui::Vec2>,
}

impl WindowSettings {
    pub fn from_display(window: &winit::window::Window) -> Self {
        let inner_size_points = window.inner_size().to_logical::<f32>(window.scale_factor());

        Self {
            inner_pos: window
                .inner_position()
                .ok()
                .map(|p| egui::pos2(p.x as f32, p.y as f32)),

            inner_size_points: Some(egui::vec2(
                inner_size_points.width as f32,
                inner_size_points.height as f32,
            )),
        }
    }

    pub fn initialize_window(
        &self,
        mut window: winit::window::WindowBuilder,
    ) -> winit::window::WindowBuilder {
        // If the app last ran on two monitors and only one is now connected, then
        // the given position is invalid.
        // If this happens on Mac, the window is clamped into valid area.
        // If this happens on Windows, the window is hidden and very difficult to find:
        // Check with winapi MonitorFromPoint if its outside of any screen

        if let Some(pos) = self.inner_pos {
            let mut can_restore_position = true;

            if cfg!(windows) {
                let pt: POINT = POINT {
                    x: pos.x as i32,
                    y: pos.y as i32,
                };
                unsafe {
                    let h_monitor = winuser::MonitorFromPoint(pt, winuser::MONITOR_DEFAULTTONULL);
                    if h_monitor.is_null() {
                        can_restore_position = false;
                    }
                }
            }

            if can_restore_position {
                window = window.with_position(winit::dpi::PhysicalPosition {
                    x: pos.x as f64,
                    y: pos.y as f64,
                });
            }
        }

        if let Some(inner_size_points) = self.inner_size_points {
            window.with_inner_size(winit::dpi::LogicalSize {
                width: inner_size_points.x as f64,
                height: inner_size_points.y as f64,
            })
        } else {
            window
        }
    }
}
