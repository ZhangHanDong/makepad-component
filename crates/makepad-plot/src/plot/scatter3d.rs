use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub Scatter3D = {{Scatter3D}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub Line3D = {{Line3D}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Clone, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub color: Option<Vec4>,
    pub size: Option<f64>,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, color: None, size: None }
    }
    pub fn with_color(mut self, c: Vec4) -> Self { self.color = Some(c); self }
    pub fn with_size(mut self, s: f64) -> Self { self.size = Some(s); self }
}

#[derive(Live, LiveHook, Widget)]
pub struct Scatter3D {
    #[deref] #[live] view: View,
    #[live] draw_point: DrawPlotPoint,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] points: Vec<Point3D>,
    #[rust] default_color: Vec4,
    #[rust] default_size: f64,
    #[rust] view3d: View3D,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
}

impl Scatter3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn set_data(&mut self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) {
        self.points.clear();
        let n = x.len().min(y.len()).min(z.len());
        for i in 0..n {
            self.points.push(Point3D::new(x[i], y[i], z[i]));
        }
        self.auto_range();
    }

    pub fn add_point(&mut self, p: Point3D) {
        self.points.push(p);
    }

    fn auto_range(&mut self) {
        if self.points.is_empty() { return; }
        let mut x_min = f64::MAX; let mut x_max = f64::MIN;
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;
        let mut z_min = f64::MAX; let mut z_max = f64::MIN;
        for p in &self.points {
            if p.x < x_min { x_min = p.x; } if p.x > x_max { x_max = p.x; }
            if p.y < y_min { y_min = p.y; } if p.y > y_max { y_max = p.y; }
            if p.z < z_min { z_min = p.z; } if p.z > z_max { z_max = p.z; }
        }
        let pad_x = (x_max - x_min).max(0.1) * 0.1;
        let pad_y = (y_max - y_min).max(0.1) * 0.1;
        let pad_z = (z_max - z_min).max(0.1) * 0.1;
        self.x_range = (x_min - pad_x, x_max + pad_x);
        self.y_range = (y_min - pad_y, y_max + pad_y);
        self.z_range = (z_min - pad_z, z_max + pad_z);
    }

    pub fn set_color(&mut self, c: Vec4) { self.default_color = c; }
    pub fn set_point_size(&mut self, s: f64) { self.default_size = s; }
    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn clear(&mut self) { self.points.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }
}

impl Widget for Scatter3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.points.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if self.default_size == 0.0 { self.default_size = 6.0; }
            if self.default_color == Vec4::default() { self.default_color = vec4(0.12, 0.47, 0.71, 1.0); }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            // Normalize to [-1, 1]
            let x_scale = if self.x_range.1 != self.x_range.0 { 2.0 / (self.x_range.1 - self.x_range.0) } else { 1.0 };
            let y_scale = if self.y_range.1 != self.y_range.0 { 2.0 / (self.y_range.1 - self.y_range.0) } else { 1.0 };
            let z_scale = if self.z_range.1 != self.z_range.0 { 2.0 / (self.z_range.1 - self.z_range.0) } else { 1.0 };
            let x_off = (self.x_range.0 + self.x_range.1) * 0.5;
            let y_off = (self.y_range.0 + self.y_range.1) * 0.5;
            let z_off = (self.z_range.0 + self.z_range.1) * 0.5;

            // Sort points back to front
            let mut sorted: Vec<(f64, usize)> = self.points.iter().enumerate().map(|(i, p)| {
                let x = (p.x - x_off) * x_scale;
                let y = (p.y - y_off) * y_scale;
                let z = (p.z - z_off) * z_scale;
                (self.view3d.depth(x, y, z), i)
            }).collect();
            sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

            // Draw points
            for (_, idx) in sorted {
                let p = &self.points[idx];
                let x = (p.x - x_off) * x_scale;
                let y = (p.y - y_off) * y_scale;
                let z = (p.z - z_off) * z_scale;

                let (sx, sy) = self.view3d.project(x, y, z);
                let screen_x = cx_center + sx * scale;
                let screen_y = cy_center - sy * scale;

                let color = p.color.unwrap_or(self.default_color);
                let size = p.size.unwrap_or(self.default_size);

                self.draw_point.color = color;
                self.draw_point.draw_point(cx, dvec2(screen_x, screen_y), size);
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

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.drag_start = Some(fe.abs);
                self.start_azimuth = self.view3d.azimuth;
                self.start_elevation = self.view3d.elevation;
            }
            Hit::FingerMove(fe) => {
                if let Some(start) = self.drag_start {
                    let delta = fe.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    self.view.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.drag_start = None;
            }
            Hit::FingerScroll(fe) => {
                if self.zoom == 0.0 { self.zoom = 1.0; }
                let zoom_delta = 1.0 + fe.scroll.y * 0.001;
                self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                self.view.redraw(cx);
            }
            _ => {}
        }
    }
}

impl Scatter3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(x, y, z); } }
    pub fn set_color(&self, c: Vec4) { if let Some(mut inner) = self.borrow_mut() { inner.set_color(c); } }
    pub fn set_point_size(&self, s: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_point_size(s); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// Line3D Widget
// =============================================================================

#[derive(Clone, Debug)]
pub struct Line3DSeries {
    pub label: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub color: Vec4,
    pub width: f64,
}

impl Line3DSeries {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), x: Vec::new(), y: Vec::new(), z: Vec::new(),
               color: vec4(0.12, 0.47, 0.71, 1.0), width: 1.5 }
    }
    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) -> Self {
        self.x = x; self.y = y; self.z = z; self
    }
    pub fn with_color(mut self, c: Vec4) -> Self { self.color = c; self }
    pub fn with_width(mut self, w: f64) -> Self { self.width = w; self }
}

#[derive(Live, LiveHook, Widget)]
pub struct Line3D {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<Line3DSeries>,
    #[rust] view3d: View3D,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
}

impl Line3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn add_series(&mut self, s: Line3DSeries) {
        self.series.push(s);
        self.auto_range();
    }

    fn auto_range(&mut self) {
        let mut x_min = f64::MAX; let mut x_max = f64::MIN;
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;
        let mut z_min = f64::MAX; let mut z_max = f64::MIN;

        for s in &self.series {
            for &v in &s.x { if v < x_min { x_min = v; } if v > x_max { x_max = v; } }
            for &v in &s.y { if v < y_min { y_min = v; } if v > y_max { y_max = v; } }
            for &v in &s.z { if v < z_min { z_min = v; } if v > z_max { z_max = v; } }
        }

        if x_min != f64::MAX {
            let pad_x = (x_max - x_min).max(0.1) * 0.1;
            let pad_y = (y_max - y_min).max(0.1) * 0.1;
            let pad_z = (z_max - z_min).max(0.1) * 0.1;
            self.x_range = (x_min - pad_x, x_max + pad_x);
            self.y_range = (y_min - pad_y, y_max + pad_y);
            self.z_range = (z_min - pad_z, z_max + pad_z);
        }
    }

    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn clear(&mut self) { self.series.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }
}

impl Widget for Line3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            // Normalize to [-1, 1]
            let x_scale = if self.x_range.1 != self.x_range.0 { 2.0 / (self.x_range.1 - self.x_range.0) } else { 1.0 };
            let y_scale = if self.y_range.1 != self.y_range.0 { 2.0 / (self.y_range.1 - self.y_range.0) } else { 1.0 };
            let z_scale = if self.z_range.1 != self.z_range.0 { 2.0 / (self.z_range.1 - self.z_range.0) } else { 1.0 };
            let x_off = (self.x_range.0 + self.x_range.1) * 0.5;
            let y_off = (self.y_range.0 + self.y_range.1) * 0.5;
            let z_off = (self.z_range.0 + self.z_range.1) * 0.5;

            // Draw 3D axes first
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

            // Draw series
            for s in &self.series {
                self.draw_line.color = s.color;
                let n = s.x.len().min(s.y.len()).min(s.z.len());

                for i in 1..n {
                    let x0 = (s.x[i-1] - x_off) * x_scale;
                    let y0 = (s.y[i-1] - y_off) * y_scale;
                    let z0 = (s.z[i-1] - z_off) * z_scale;
                    let (sx0, sy0) = self.view3d.project(x0, y0, z0);

                    let x1 = (s.x[i] - x_off) * x_scale;
                    let y1 = (s.y[i] - y_off) * y_scale;
                    let z1 = (s.z[i] - z_off) * z_scale;
                    let (sx1, sy1) = self.view3d.project(x1, y1, z1);

                    self.draw_line.draw_line(cx,
                        dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                        dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                        s.width);
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &self.title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.drag_start = Some(fe.abs);
                self.start_azimuth = self.view3d.azimuth;
                self.start_elevation = self.view3d.elevation;
            }
            Hit::FingerMove(fe) => {
                if let Some(start) = self.drag_start {
                    let delta = fe.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    self.view.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.drag_start = None;
            }
            Hit::FingerScroll(fe) => {
                if self.zoom == 0.0 { self.zoom = 1.0; }
                let zoom_delta = 1.0 + fe.scroll.y * 0.001;
                self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                self.view.redraw(cx);
            }
            _ => {}
        }
    }
}

impl Line3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn add_series(&self, s: Line3DSeries) { if let Some(mut inner) = self.borrow_mut() { inner.add_series(s); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

