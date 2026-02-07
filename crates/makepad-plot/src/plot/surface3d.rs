use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub Surface3D = {{Surface3D}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}


/// Camera/view settings for 3D plots
#[derive(Clone, Debug, Default)]
pub struct View3D {
    pub azimuth: f64,     // Horizontal rotation (degrees)
    pub elevation: f64,   // Vertical rotation (degrees)
    pub distance: f64,    // Distance from origin
}

impl View3D {
    pub fn new() -> Self {
        Self {
            azimuth: -60.0,
            elevation: 30.0,
            distance: 3.0,
        }
    }
}

impl View3D {
    /// Project 3D point to 2D screen coordinates
    pub fn project(&self, x: f64, y: f64, z: f64) -> (f64, f64) {
        let az = self.azimuth.to_radians();
        let el = self.elevation.to_radians();

        // Rotate around Z axis (azimuth)
        let x1 = x * az.cos() - y * az.sin();
        let y1 = x * az.sin() + y * az.cos();
        let z1 = z;

        // Rotate around X axis (elevation)
        let x2 = x1;
        let y2 = y1 * el.cos() - z1 * el.sin();
        let z2 = y1 * el.sin() + z1 * el.cos();

        // Simple perspective projection
        let perspective = self.distance / (self.distance + y2 + 2.0);
        let screen_x = x2 * perspective;
        let screen_y = z2 * perspective;

        (screen_x, screen_y)
    }

    /// Get depth for z-sorting (larger = further away)
    pub fn depth(&self, x: f64, y: f64, z: f64) -> f64 {
        let az = self.azimuth.to_radians();
        let el = self.elevation.to_radians();

        let x1 = x * az.cos() - y * az.sin();
        let y1 = x * az.sin() + y * az.cos();
        let z1 = z;

        let y2 = y1 * el.cos() - z1 * el.sin();
        y2
    }
}

// =============================================================================
// Surface3D Widget
// =============================================================================
//
// Interactive 3D surface plot with mouse controls:
// - Click and drag horizontally: rotate view (azimuth)
// - Click and drag vertically: tilt view (elevation)
// - Scroll wheel: zoom in/out
//
// Supports multiple charts via per-chart state tracking: each chart_id gets
// its own data, view angles, zoom, and drag state stored in a HashMap.
//
// Implementation note: Uses manual coordinate-based hit testing instead of
// Makepad's event.hits() system because Surface3D is used as a render helper
// inside A2uiSurface (not in the widget tree), so area-based hit testing fails.
// =============================================================================

/// Per-chart instance state for multi-chart support
#[derive(Clone, Debug)]
pub struct Surface3DChart {
    pub title: String,
    pub z_data: Vec<Vec<f64>>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub z_range: (f64, f64),
    pub view3d: View3D,
    pub colormap: Colormap,
    pub show_wireframe: bool,
    pub show_surface: bool,
    pub zoom: f64,
    pub drag_start: Option<DVec2>,
    pub start_azimuth: f64,
    pub start_elevation: f64,
    pub hit_rect: Rect,
}

impl Default for Surface3DChart {
    fn default() -> Self {
        Self {
            title: String::new(),
            z_data: Vec::new(),
            x_range: (0.0, 1.0),
            y_range: (0.0, 1.0),
            z_range: (0.0, 1.0),
            view3d: View3D::new(),
            colormap: Colormap::default(),
            show_wireframe: true,
            show_surface: false,
            zoom: 1.0,
            drag_start: None,
            start_azimuth: 0.0,
            start_elevation: 0.0,
            hit_rect: Rect::default(),
        }
    }
}

impl Surface3DChart {
    pub fn set_data(&mut self, z: Vec<Vec<f64>>) {
        if z.is_empty() || z[0].is_empty() { return; }
        let mut z_min = f64::MAX;
        let mut z_max = f64::MIN;
        for row in &z {
            for &val in row {
                if val < z_min { z_min = val; }
                if val > z_max { z_max = val; }
            }
        }
        self.x_range = (0.0, (z[0].len() - 1) as f64);
        self.y_range = (0.0, (z.len() - 1) as f64);
        self.z_range = (z_min, z_max);
        self.z_data = z;
    }

    fn normalize_z(&self, z: f64) -> f64 {
        if self.z_range.1 == self.z_range.0 { return 0.5; }
        (z - self.z_range.0) / (self.z_range.1 - self.z_range.0)
    }
}

/// Wrapper for HashMap to work with Makepad's #[rust] derive
#[derive(Clone, Debug, Default)]
pub struct Surface3DCharts {
    pub map: std::collections::HashMap<String, Surface3DChart>,
}

#[derive(Live, LiveHook, Widget)]
pub struct Surface3D {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    // Legacy single-chart fields (used when no chart_id is provided)
    #[rust] title: String,
    #[rust] z_data: Vec<Vec<f64>>,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] view3d: View3D,
    #[rust] colormap: Colormap,
    #[rust] show_wireframe: bool,
    #[rust] show_surface: bool,
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
    #[rust] hit_rect: Rect,
    // Multi-chart support: per-chart state keyed by chart component ID
    #[rust] charts: Surface3DCharts,
    // Which chart is currently being dragged (by chart_id)
    #[rust] active_drag_id: String,
}

impl Surface3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn set_data(&mut self, z: Vec<Vec<f64>>) {
        if z.is_empty() || z[0].is_empty() { return; }

        // Calculate z range
        let mut z_min = f64::MAX;
        let mut z_max = f64::MIN;
        for row in &z {
            for &val in row {
                if val < z_min { z_min = val; }
                if val > z_max { z_max = val; }
            }
        }

        self.z_data = z;
        self.z_range = (z_min, z_max);
        self.x_range = (0.0, (self.z_data[0].len() - 1) as f64);
        self.y_range = (0.0, (self.z_data.len() - 1) as f64);
    }

    pub fn set_x_range(&mut self, min: f64, max: f64) { self.x_range = (min, max); }
    pub fn set_y_range(&mut self, min: f64, max: f64) { self.y_range = (min, max); }
    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn set_colormap(&mut self, cm: Colormap) { self.colormap = cm; }
    pub fn set_wireframe(&mut self, show: bool) { self.show_wireframe = show; }
    pub fn set_surface(&mut self, show: bool) { self.show_surface = show; }
    pub fn clear(&mut self) { self.z_data.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }

    fn normalize_z(&self, z: f64) -> f64 {
        if self.z_range.1 == self.z_range.0 { return 0.5; }
        (z - self.z_range.0) / (self.z_range.1 - self.z_range.0)
    }

    // ── Multi-chart API ──────────────────────────────────────────────

    /// Get or create a per-chart instance by ID. Preserves interactive state
    /// (view angles, zoom) across redraws while allowing data updates.
    pub fn get_chart_mut(&mut self, chart_id: &str) -> &mut Surface3DChart {
        self.charts.map.entry(chart_id.to_string()).or_insert_with(Surface3DChart::default)
    }

    /// Draw a specific chart instance using its per-chart state.
    pub fn draw_chart_instance(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk, chart_id: &str) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        // Store hit_rect in the per-chart state
        if let Some(chart) = self.charts.map.get_mut(chart_id) {
            chart.hit_rect = rect;
        }

        // Get chart data (need to clone to avoid borrow issues with self.draw_*)
        let chart_data = match self.charts.map.get(chart_id) {
            Some(c) if !c.z_data.is_empty() && rect.size.x > 0.0 && rect.size.y > 0.0 => {
                Some((
                    c.z_data.clone(),
                    c.x_range, c.y_range, c.z_range,
                    c.view3d.clone(),
                    c.colormap.clone(),
                    c.show_wireframe, c.show_surface,
                    c.zoom,
                    c.title.clone(),
                ))
            }
            _ => None,
        };

        if let Some((z_data, x_range, y_range, z_range, view3d, colormap, show_wireframe, show_surface, zoom, title)) = chart_data {
            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let zoom_val = if zoom == 0.0 { 1.0 } else { zoom };
            let scale = rect.size.x.min(rect.size.y) * 0.35 * zoom_val;

            let rows = z_data.len();
            let cols = z_data[0].len();

            let x_scale = 2.0 / (cols - 1).max(1) as f64;
            let y_scale = 2.0 / (rows - 1).max(1) as f64;
            let z_scale = if z_range.1 != z_range.0 { 1.5 / (z_range.1 - z_range.0) } else { 1.0 };
            let z_offset = (z_range.0 + z_range.1) * 0.5;

            let normalize_z = |z: f64| -> f64 {
                if z_range.1 == z_range.0 { 0.5 } else { (z - z_range.0) / (z_range.1 - z_range.0) }
            };

            // Draw filled surface quads with depth sorting
            if show_surface {
                let mut quads: Vec<(f64, usize, usize, Vec4)> = Vec::new();
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let avg_z = (z_data[i][j] + z_data[i+1][j] + z_data[i][j+1] + z_data[i+1][j+1]) * 0.25;
                        let t = normalize_z(avg_z);
                        let color = colormap.sample(t);
                        let cx_q = (j as f64 + 0.5) * x_scale - 1.0;
                        let cy_q = (i as f64 + 0.5) * y_scale - 1.0;
                        let cz_q = (avg_z - z_offset) * z_scale;
                        let depth = view3d.depth(cx_q, cy_q, cz_q);
                        quads.push((depth, i, j, color));
                    }
                }
                quads.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

                for (_, i, j, color) in quads {
                    let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1)];
                    let mut pts: Vec<DVec2> = Vec::new();
                    for &(cj, ci) in &corners {
                        let x = cj as f64 * x_scale - 1.0;
                        let y = ci as f64 * y_scale - 1.0;
                        let z = (z_data[ci][cj] - z_offset) * z_scale;
                        let (sx, sy) = view3d.project(x, y, z);
                        pts.push(dvec2(cx_center + sx * scale, cy_center - sy * scale));
                    }
                    self.draw_fill.color = color;
                    let min_x = pts.iter().map(|p| p.x).fold(f64::MAX, f64::min);
                    let max_x = pts.iter().map(|p| p.x).fold(f64::MIN, f64::max);
                    let min_y = pts.iter().map(|p| p.y).fold(f64::MAX, f64::min);
                    let max_y = pts.iter().map(|p| p.y).fold(f64::MIN, f64::max);
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(min_x, min_y),
                        size: dvec2(max_x - min_x + 1.0, max_y - min_y + 1.0),
                    });
                }
            }

            // Draw wireframe
            if show_wireframe {
                let wire_color = if show_surface { vec4(0.0, 0.0, 0.0, 0.5) } else { vec4(0.2, 0.4, 0.8, 1.0) };
                self.draw_line.color = wire_color;
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1), (j, i)];
                        for k in 0..4 {
                            let (j0, i0) = corners[k];
                            let (j1, i1) = corners[k+1];
                            let x0 = j0 as f64 * x_scale - 1.0;
                            let y0 = i0 as f64 * y_scale - 1.0;
                            let z0 = (z_data[i0][j0] - z_offset) * z_scale;
                            let (sx0, sy0) = view3d.project(x0, y0, z0);
                            let x1 = j1 as f64 * x_scale - 1.0;
                            let y1 = i1 as f64 * y_scale - 1.0;
                            let z1 = (z_data[i1][j1] - z_offset) * z_scale;
                            let (sx1, sy1) = view3d.project(x1, y1, z1);
                            self.draw_line.draw_line(cx,
                                dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                                dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                                1.0);
                        }
                    }
                }
            }

            // Draw 3D axes
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = view3d.project(x0, y0, z0);
                let (sx1, sy1) = view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw title
            if !title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    /// Handle events for all chart instances (multi-chart drag support).
    pub fn handle_multi_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::MouseDown(me) => {
                // Find which chart was clicked
                for (chart_id, chart) in self.charts.map.iter_mut() {
                    let r = chart.hit_rect;
                    if r.size.x > 0.0 && r.size.y > 0.0 && r.contains(me.abs) {
                        chart.drag_start = Some(me.abs);
                        chart.start_azimuth = chart.view3d.azimuth;
                        chart.start_elevation = chart.view3d.elevation;
                        self.active_drag_id = chart_id.clone();
                        return;
                    }
                }
            }
            Event::MouseMove(me) => {
                if !self.active_drag_id.is_empty() {
                    if let Some(chart) = self.charts.map.get_mut(&self.active_drag_id) {
                        if let Some(start) = chart.drag_start {
                            let delta = me.abs - start;
                            chart.view3d.azimuth = chart.start_azimuth + delta.x * 0.5;
                            chart.view3d.elevation = (chart.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                            cx.redraw_all();
                        }
                    }
                }
            }
            Event::MouseUp(_) => {
                if !self.active_drag_id.is_empty() {
                    if let Some(chart) = self.charts.map.get_mut(&self.active_drag_id) {
                        chart.drag_start = None;
                    }
                    self.active_drag_id.clear();
                }
            }
            Event::Scroll(se) => {
                for (_chart_id, chart) in self.charts.map.iter_mut() {
                    let r = chart.hit_rect;
                    if r.size.x > 0.0 && r.size.y > 0.0 && r.contains(se.abs) {
                        if chart.zoom == 0.0 { chart.zoom = 1.0; }
                        let zoom_delta = 1.0 + se.scroll.y * 0.001;
                        chart.zoom = (chart.zoom * zoom_delta).clamp(0.2, 5.0);
                        cx.redraw_all();
                        return;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Widget for Surface3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        self.hit_rect = rect;  // Store for manual hit testing

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.z_data.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if !self.show_wireframe && !self.show_surface { self.show_wireframe = true; }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            let rows = self.z_data.len();
            let cols = self.z_data[0].len();

            // Normalize coordinates to [-1, 1] range
            let x_scale = 2.0 / (cols - 1).max(1) as f64;
            let y_scale = 2.0 / (rows - 1).max(1) as f64;
            let z_scale = if self.z_range.1 != self.z_range.0 {
                1.5 / (self.z_range.1 - self.z_range.0)
            } else { 1.0 };
            let z_offset = (self.z_range.0 + self.z_range.1) * 0.5;

            // Draw filled surface quads with depth sorting (painter's algorithm)
            if self.show_surface {
                // Collect all quads with their depth for sorting
                let mut quads: Vec<(f64, usize, usize, Vec4)> = Vec::new();
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let avg_z = (self.z_data[i][j] + self.z_data[i+1][j] +
                                     self.z_data[i][j+1] + self.z_data[i+1][j+1]) * 0.25;
                        let t = self.normalize_z(avg_z);
                        let color = self.colormap.sample(t);

                        // Calculate center point for depth
                        let cx_q = (j as f64 + 0.5) * x_scale - 1.0;
                        let cy_q = (i as f64 + 0.5) * y_scale - 1.0;
                        let cz_q = (avg_z - z_offset) * z_scale;
                        let depth = self.view3d.depth(cx_q, cy_q, cz_q);

                        quads.push((depth, i, j, color));
                    }
                }

                // Sort by depth (back to front)
                quads.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

                // Draw quads in sorted order
                for (_, i, j, color) in quads {
                    // Get projected corners
                    let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1)];
                    let mut pts: Vec<DVec2> = Vec::new();
                    for &(cj, ci) in &corners {
                        let x = cj as f64 * x_scale - 1.0;
                        let y = ci as f64 * y_scale - 1.0;
                        let z = (self.z_data[ci][cj] - z_offset) * z_scale;
                        let (sx, sy) = self.view3d.project(x, y, z);
                        pts.push(dvec2(cx_center + sx * scale, cy_center - sy * scale));
                    }

                    // Draw filled quad as two triangles using lines
                    self.draw_fill.color = color;
                    let min_x = pts.iter().map(|p| p.x).fold(f64::MAX, f64::min);
                    let max_x = pts.iter().map(|p| p.x).fold(f64::MIN, f64::max);
                    let min_y = pts.iter().map(|p| p.y).fold(f64::MAX, f64::min);
                    let max_y = pts.iter().map(|p| p.y).fold(f64::MIN, f64::max);
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(min_x, min_y),
                        size: dvec2(max_x - min_x + 1.0, max_y - min_y + 1.0),
                    });
                }
            }

            // Draw wireframe
            if self.show_wireframe {
                let wire_color = if self.show_surface { vec4(0.0, 0.0, 0.0, 0.5) } else { vec4(0.2, 0.4, 0.8, 1.0) };
                self.draw_line.color = wire_color;

                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1), (j, i)];
                        for k in 0..4 {
                            let (j0, i0) = corners[k];
                            let (j1, i1) = corners[k+1];

                            let x0 = j0 as f64 * x_scale - 1.0;
                            let y0 = i0 as f64 * y_scale - 1.0;
                            let z0 = (self.z_data[i0][j0] - z_offset) * z_scale;
                            let (sx0, sy0) = self.view3d.project(x0, y0, z0);

                            let x1 = j1 as f64 * x_scale - 1.0;
                            let y1 = i1 as f64 * y_scale - 1.0;
                            let z1 = (self.z_data[i1][j1] - z_offset) * z_scale;
                            let (sx1, sy1) = self.view3d.project(x1, y1, z1);

                            self.draw_line.draw_line(cx,
                                dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                                dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                                1.0);
                        }
                    }
                }
            }

            // Draw 3D axes
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = self.view3d.project(x0, y0, z0);
                let (sx1, sy1) = self.view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &self.title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        // Handle multi-chart instances (used by A2uiSurface for multiple 3D charts)
        if !self.charts.map.is_empty() {
            self.handle_multi_event(cx, event);
            return;
        }

        // Legacy single-chart path (used when Surface3D is used standalone)
        let r = self.hit_rect;
        if r.size.x <= 0.0 || r.size.y <= 0.0 { return; }

        match event {
            Event::MouseDown(me) => {
                if r.contains(me.abs) {
                    self.drag_start = Some(me.abs);
                    self.start_azimuth = self.view3d.azimuth;
                    self.start_elevation = self.view3d.elevation;
                }
            }
            Event::MouseMove(me) => {
                if let Some(start) = self.drag_start {
                    let delta = me.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    cx.redraw_all();
                }
            }
            Event::MouseUp(_) => {
                self.drag_start = None;
            }
            Event::Scroll(se) => {
                if r.contains(se.abs) {
                    if self.zoom == 0.0 { self.zoom = 1.0; }
                    let zoom_delta = 1.0 + se.scroll.y * 0.001;
                    self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                    cx.redraw_all();
                }
            }
            _ => {}
        }
    }
}

impl Surface3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, z: Vec<Vec<f64>>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(z); } }
    pub fn set_x_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_x_range(min, max); } }
    pub fn set_y_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_y_range(min, max); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn set_colormap(&self, cm: Colormap) { if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(cm); } }
    pub fn set_wireframe(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_wireframe(show); } }
    pub fn set_surface(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_surface(show); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

