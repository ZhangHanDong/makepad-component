use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub ContourPlot = {{ContourPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub QuiverPlot = {{QuiverPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ContourPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_fill: DrawPlotFill,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] data: Vec<Vec<f64>>,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] filled: bool,
    #[rust] colormap: Colormap,
    #[rust] n_levels: usize,
    #[rust] plot_area: PlotArea,
    #[live(50.0)] left_margin: f64,
    #[live(30.0)] right_margin: f64,
    #[live(30.0)] top_margin: f64,
    #[live(50.0)] bottom_margin: f64,
}

impl Widget for ContourPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { self.view.handle_event(cx, event, scope); }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            self.plot_area = PlotArea::new(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin,
                rect.pos.x + rect.size.x - self.right_margin, rect.pos.y + rect.size.y - self.bottom_margin);
            self.draw_contours(cx);
            self.draw_axis_labels(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl ContourPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn set_data(&mut self, data: Vec<Vec<f64>>) { self.data = data; }
    pub fn set_x_range(&mut self, min: f64, max: f64) { self.x_range = (min, max); }
    pub fn set_y_range(&mut self, min: f64, max: f64) { self.y_range = (min, max); }
    pub fn set_filled(&mut self, filled: bool) { self.filled = filled; }
    pub fn set_colormap(&mut self, colormap: Colormap) { self.colormap = colormap; }
    pub fn set_n_levels(&mut self, n: usize) { self.n_levels = n; }
    pub fn clear(&mut self) { self.data.clear(); self.n_levels = 0; }

    fn draw_contours(&mut self, cx: &mut Cx2d) {
        let rows = self.data.len(); if rows < 2 { return; }
        let cols = self.data[0].len(); if cols < 2 { return; }
        let (mut v_min, mut v_max) = (f64::MAX, f64::MIN);
        for row in &self.data { for &v in row { v_min = v_min.min(v); v_max = v_max.max(v); } }
        let v_range = (v_max - v_min).max(1e-10);
        let cell_w = self.plot_area.width() / (cols - 1) as f64;
        let cell_h = self.plot_area.height() / (rows - 1) as f64;

        if self.filled {
            for row in 0..rows - 1 {
                for col in 0..cols - 1 {
                    let avg = (self.data[row][col] + self.data[row][col + 1] + self.data[row + 1][col] + self.data[row + 1][col + 1]) / 4.0;
                    self.draw_fill.color = self.colormap.sample((avg - v_min) / v_range);
                    self.draw_fill.draw_abs(cx, Rect { pos: dvec2(self.plot_area.left + col as f64 * cell_w, self.plot_area.top + row as f64 * cell_h), size: dvec2(cell_w, cell_h) });
                }
            }
        }

        let n_levels = if self.n_levels > 0 { self.n_levels } else { 10 };
        for lvl in 1..=n_levels {
            let level = v_min + lvl as f64 * (v_max - v_min) / (n_levels + 1) as f64;
            self.draw_line.color = if self.filled { vec4(0.2, 0.2, 0.2, 0.8) } else { self.colormap.sample((level - v_min) / v_range) };
            for row in 0..rows - 1 {
                for col in 0..cols - 1 {
                    let (v00, v10, v01, v11) = (self.data[row][col], self.data[row][col + 1], self.data[row + 1][col], self.data[row + 1][col + 1]);
                    let case = ((v00 >= level) as u8) | (((v10 >= level) as u8) << 1) | (((v01 >= level) as u8) << 2) | (((v11 >= level) as u8) << 3);
                    if case == 0 || case == 15 { continue; }
                    let x0 = self.plot_area.left + col as f64 * cell_w;
                    let y0 = self.plot_area.top + row as f64 * cell_h;
                    let interp = |a: f64, b: f64| if (b - a).abs() < 1e-10 { 0.5 } else { (level - a) / (b - a) };
                    let (tx, bx, ly, ry) = (x0 + interp(v00, v10) * cell_w, x0 + interp(v01, v11) * cell_w, y0 + interp(v00, v01) * cell_h, y0 + interp(v10, v11) * cell_h);
                    match case {
                        1 | 14 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(tx, y0), 1.5),
                        2 | 13 => self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(x0 + cell_w, ry), 1.5),
                        3 | 12 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(x0 + cell_w, ry), 1.5),
                        4 | 11 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(bx, y0 + cell_h), 1.5),
                        6 | 9 => self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(bx, y0 + cell_h), 1.5),
                        7 | 8 => self.draw_line.draw_line(cx, dvec2(bx, y0 + cell_h), dvec2(x0 + cell_w, ry), 1.5),
                        5 => { self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(tx, y0), 1.5); self.draw_line.draw_line(cx, dvec2(bx, y0 + cell_h), dvec2(x0 + cell_w, ry), 1.5); }
                        10 => { self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(x0 + cell_w, ry), 1.5); self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(bx, y0 + cell_h), 1.5); }
                        _ => {}
                    }
                }
            }
        }

        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.right, self.plot_area.bottom), 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.left, self.plot_area.top), 1.0);
    }

    fn draw_axis_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.4, 0.4, 0.4, 1.0));
        let n_ticks = 5;
        // X-axis ticks
        for i in 0..=n_ticks {
            let frac = i as f64 / n_ticks as f64;
            let x_pos = self.plot_area.left + frac * self.plot_area.width();
            let val = self.x_range.0 + frac * (self.x_range.1 - self.x_range.0);
            let txt = format!("{:.1}", val);
            self.label.draw_at(cx, dvec2(x_pos, self.plot_area.bottom + 12.0), &txt, TextAnchor::Center);
            // Tick mark
            self.draw_line.color = vec4(0.4, 0.4, 0.4, 0.6);
            self.draw_line.draw_line(cx, dvec2(x_pos, self.plot_area.bottom), dvec2(x_pos, self.plot_area.bottom + 4.0), 1.0);
        }
        // Y-axis ticks
        for i in 0..=n_ticks {
            let frac = i as f64 / n_ticks as f64;
            let y_pos = self.plot_area.bottom - frac * self.plot_area.height();
            let val = self.y_range.0 + frac * (self.y_range.1 - self.y_range.0);
            let txt = format!("{:.1}", val);
            self.label.draw_at(cx, dvec2(self.plot_area.left - 8.0, y_pos), &txt, TextAnchor::MiddleRight);
            self.draw_line.color = vec4(0.4, 0.4, 0.4, 0.6);
            self.draw_line.draw_line(cx, dvec2(self.plot_area.left - 4.0, y_pos), dvec2(self.plot_area.left, y_pos), 1.0);
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2((self.plot_area.left + self.plot_area.right) / 2.0, self.plot_area.top - 15.0), &self.title, TextAnchor::Center);
        }
    }
}

impl ContourPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, data: Vec<Vec<f64>>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(data); } }
    pub fn set_x_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_x_range(min, max); } }
    pub fn set_y_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_y_range(min, max); } }
    pub fn set_filled(&self, filled: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_filled(filled); } }
    pub fn set_colormap(&self, colormap: Colormap) { if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(colormap); } }
    pub fn set_n_levels(&self, n: usize) { if let Some(mut inner) = self.borrow_mut() { inner.set_n_levels(n); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// QuiverPlot Widget (Vector Field)
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct QuiverPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] x: Vec<f64>,
    #[rust] y: Vec<f64>,
    #[rust] u: Vec<f64>,
    #[rust] v: Vec<f64>,
    #[rust] scale: f64,
    #[rust] arrow_color: Vec4,
    #[rust] plot_area: PlotArea,
    #[live(50.0)] left_margin: f64,
    #[live(30.0)] right_margin: f64,
    #[live(30.0)] top_margin: f64,
    #[live(50.0)] bottom_margin: f64,
}

impl Widget for QuiverPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { self.view.handle_event(cx, event, scope); }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.x.is_empty() {
            self.plot_area = PlotArea::new(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin,
                rect.pos.x + rect.size.x - self.right_margin, rect.pos.y + rect.size.y - self.bottom_margin);
            self.draw_arrows(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl QuiverPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn set_data(&mut self, x: Vec<f64>, y: Vec<f64>, u: Vec<f64>, v: Vec<f64>) { self.x = x; self.y = y; self.u = u; self.v = v; }
    pub fn set_scale(&mut self, scale: f64) { self.scale = scale; }
    pub fn set_color(&mut self, color: Vec4) { self.arrow_color = color; }
    pub fn clear(&mut self) { self.x.clear(); self.y.clear(); self.u.clear(); self.v.clear(); }

    fn draw_arrows(&mut self, cx: &mut Cx2d) {
        let n = self.x.len().min(self.y.len()).min(self.u.len()).min(self.v.len());
        if n == 0 { return; }
        let (x_min, x_max) = (self.x.iter().cloned().fold(f64::MAX, f64::min), self.x.iter().cloned().fold(f64::MIN, f64::max));
        let (y_min, y_max) = (self.y.iter().cloned().fold(f64::MAX, f64::min), self.y.iter().cloned().fold(f64::MIN, f64::max));
        let (xr, yr) = ((x_max - x_min).max(1e-10), (y_max - y_min).max(1e-10));
        let max_mag = self.u.iter().zip(self.v.iter()).map(|(&u, &v)| (u * u + v * v).sqrt()).fold(0.0f64, f64::max);
        let scale = if self.scale > 0.0 { self.scale } else if max_mag > 0.0 { 0.1 * self.plot_area.width().min(self.plot_area.height()) / max_mag } else { 1.0 };
        self.draw_line.color = if self.arrow_color.w > 0.0 { self.arrow_color } else { vec4(0.12, 0.47, 0.71, 1.0) };

        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.right, self.plot_area.bottom), 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.left, self.plot_area.top), 1.0);

        let color = self.draw_line.color;
        for i in 0..n {
            let px = self.plot_area.left + (self.x[i] - x_min) / xr * self.plot_area.width();
            let py = self.plot_area.bottom - (self.y[i] - y_min) / yr * self.plot_area.height();
            let (dx, dy) = (self.u[i] * scale, -self.v[i] * scale);
            let (p1, p2) = (dvec2(px, py), dvec2(px + dx, py + dy));
            self.draw_line.color = color;
            self.draw_line.draw_line(cx, p1, p2, 1.5);
            let len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
            if len > 1.0 {
                let (dirx, diry) = ((p2.x - p1.x) / len, (p2.y - p1.y) / len);
                let (perpx, perpy) = (-diry, dirx);
                let (al, aa) = (5.0, 0.4);
                self.draw_line.draw_line(cx, p2, dvec2(p2.x - dirx * al + perpx * al * aa, p2.y - diry * al + perpy * al * aa), 1.5);
                self.draw_line.draw_line(cx, p2, dvec2(p2.x - dirx * al - perpx * al * aa, p2.y - diry * al - perpy * al * aa), 1.5);
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2((self.plot_area.left + self.plot_area.right) / 2.0, self.plot_area.top - 15.0), &self.title, TextAnchor::Center);
        }
    }
}

impl QuiverPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, x: Vec<f64>, y: Vec<f64>, u: Vec<f64>, v: Vec<f64>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(x, y, u, v); } }
    pub fn set_scale(&self, scale: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_scale(scale); } }
    pub fn set_color(&self, color: Vec4) { if let Some(mut inner) = self.borrow_mut() { inner.set_color(color); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

