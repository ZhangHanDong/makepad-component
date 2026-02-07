use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub HeatmapChart = {{HeatmapChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub Heatmap = {{Heatmap}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HeatmapChart {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_bar: DrawPlotBar,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    label: PlotLabel,

    #[rust]
    data: Vec<Vec<f64>>,

    #[rust]
    x_labels: Option<Vec<String>>,

    #[rust]
    y_labels: Option<Vec<String>>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust]
    colormap: Colormap,

    #[rust]
    vmin: Option<f64>,

    #[rust]
    vmax: Option<f64>,

    #[rust(true)]
    show_values: bool,

    #[rust(60.0)]
    left_margin: f64,

    #[rust(30.0)]
    bottom_margin: f64,

    #[rust(60.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,
}

impl Widget for HeatmapChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            self.update_plot_area(rect);
            self.draw_cells(cx);
            self.draw_labels(cx);
            self.draw_colorbar(cx, rect);
        }

        DrawStep::done()
    }
}

impl HeatmapChart {
    pub fn set_data(&mut self, data: Vec<Vec<f64>>) {
        self.data = data;
    }

    pub fn set_x_labels(&mut self, labels: Vec<String>) {
        self.x_labels = Some(labels);
    }

    pub fn set_y_labels(&mut self, labels: Vec<String>) {
        self.y_labels = Some(labels);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_colormap(&mut self, colormap: Colormap) {
        self.colormap = colormap;
    }

    pub fn set_vmin(&mut self, vmin: f64) {
        self.vmin = Some(vmin);
    }

    pub fn set_vmax(&mut self, vmax: f64) {
        self.vmax = Some(vmax);
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.x_labels = None;
        self.y_labels = None;
    }

    fn update_plot_area(&mut self, rect: Rect) {
        self.plot_area = PlotArea::new(
            rect.pos.x + self.left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - self.bottom_margin,
        );
    }

    fn get_value_range(&self) -> (f64, f64) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for row in &self.data {
            for &val in row {
                min = min.min(val);
                max = max.max(val);
            }
        }

        (self.vmin.unwrap_or(min), self.vmax.unwrap_or(max))
    }

    fn draw_cells(&mut self, cx: &mut Cx2d) {
        let rows = self.data.len();
        if rows == 0 {
            return;
        }
        let cols = self.data[0].len();
        if cols == 0 {
            return;
        }

        let (vmin, vmax) = self.get_value_range();
        let range = (vmax - vmin).max(1e-10);

        let cell_width = self.plot_area.width() / cols as f64;
        let cell_height = self.plot_area.height() / rows as f64;

        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                let t = (value - vmin) / range;
                let color = self.colormap.sample(t);
                self.draw_bar.color = color;

                let x = self.plot_area.left + col_idx as f64 * cell_width;
                let y = self.plot_area.top + row_idx as f64 * cell_height;

                let rect = Rect {
                    pos: dvec2(x, y),
                    size: dvec2(cell_width, cell_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Draw value text in cell
                if self.show_values {
                    let text_color = if t > 0.5 {
                        vec4(0.0, 0.0, 0.0, 1.0)
                    } else {
                        vec4(1.0, 1.0, 1.0, 1.0)
                    };
                    self.label.set_color(text_color);
                    let label = format!("{:.1}", value);
                    self.label.draw_at(
                        cx,
                        dvec2(x + cell_width / 2.0, y + cell_height / 2.0),
                        &label,
                        TextAnchor::Center,
                    );
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };

        let cell_width = self.plot_area.width() / cols.max(1) as f64;
        let cell_height = self.plot_area.height() / rows.max(1) as f64;

        // X labels (column labels)
        if let Some(ref labels) = self.x_labels {
            for (i, label) in labels.iter().enumerate().take(cols) {
                let x = self.plot_area.left + (i as f64 + 0.5) * cell_width;
                let y = self.plot_area.bottom + 5.0;
                self.label.draw_at(cx, dvec2(x, y), label, TextAnchor::TopCenter);
            }
        }

        // Y labels (row labels)
        if let Some(ref labels) = self.y_labels {
            for (i, label) in labels.iter().enumerate().take(rows) {
                let x = self.plot_area.left - 5.0;
                let y = self.plot_area.top + (i as f64 + 0.5) * cell_height;
                self.label.draw_at(cx, dvec2(x, y), label, TextAnchor::MiddleRight);
            }
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn draw_colorbar(&mut self, cx: &mut Cx2d, rect: Rect) {
        let bar_width = 15.0;
        let bar_x = rect.pos.x + rect.size.x - self.right_margin + 10.0;
        let bar_top = self.plot_area.top;
        let bar_height = self.plot_area.height();

        // Draw colorbar gradient
        let steps = 50;
        let step_height = bar_height / steps as f64;

        for i in 0..steps {
            let t = 1.0 - i as f64 / steps as f64;
            let color = self.colormap.sample(t);
            self.draw_bar.color = color;

            let rect = Rect {
                pos: dvec2(bar_x, bar_top + i as f64 * step_height),
                size: dvec2(bar_width, step_height + 1.0),
            };
            self.draw_bar.draw_bar(cx, rect);
        }

        // Draw colorbar labels
        let (vmin, vmax) = self.get_value_range();
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let label_max = format!("{:.1}", vmax);
        let label_min = format!("{:.1}", vmin);
        let label_mid = format!("{:.1}", (vmin + vmax) / 2.0);

        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top), &label_max, TextAnchor::MiddleLeft);
        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top + bar_height / 2.0), &label_mid, TextAnchor::MiddleLeft);
        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top + bar_height), &label_min, TextAnchor::MiddleLeft);
    }
}

impl HeatmapChartRef {
    pub fn set_data(&self, data: Vec<Vec<f64>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(data);
        }
    }

    pub fn set_x_labels(&self, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_x_labels(labels);
        }
    }

    pub fn set_y_labels(&self, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_y_labels(labels);
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_colormap(&self, colormap: Colormap) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_colormap(colormap);
        }
    }

    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_values(show);
        }
    }

    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}


// =============================================================================
// Heatmap Widget (alias)
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct Heatmap {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] data: Vec<Vec<f64>>,
    #[rust] x_labels: Vec<String>,
    #[rust] y_labels: Vec<String>,
    #[rust] colormap: Colormap,
    #[rust] show_values: bool,
    #[rust] min_value: Option<f64>,
    #[rust] max_value: Option<f64>,
}

impl Heatmap {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>) {
        self.data = data;
    }

    pub fn set_labels(&mut self, x_labels: Vec<String>, y_labels: Vec<String>) {
        self.x_labels = x_labels;
        self.y_labels = y_labels;
    }

    pub fn set_colormap(&mut self, colormap: Colormap) {
        self.colormap = colormap;
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = Some(min);
        self.max_value = Some(max);
    }

    fn get_data_range(&self) -> (f64, f64) {
        if let (Some(min), Some(max)) = (self.min_value, self.max_value) {
            return (min, max);
        }
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for row in &self.data {
            for &val in row {
                if val < min { min = val; }
                if val > max { max = val; }
            }
        }
        if min == f64::MAX { min = 0.0; }
        if max == f64::MIN { max = 1.0; }
        (min, max)
    }
}

impl Widget for Heatmap {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            let padding_left = 60.0;
            let padding_right = 20.0;
            let padding_top = 40.0;
            let padding_bottom = 40.0;

            let plot_left = rect.pos.x + padding_left;
            let plot_top = rect.pos.y + padding_top;
            let plot_right = rect.pos.x + rect.size.x - padding_right;
            let plot_bottom = rect.pos.y + rect.size.y - padding_bottom;
            let plot_width = plot_right - plot_left;
            let plot_height = plot_bottom - plot_top;

            let rows = self.data.len();
            let cols = if rows > 0 { self.data[0].len() } else { 0 };

            if cols > 0 {
                let cell_width = plot_width / cols as f64;
                let cell_height = plot_height / rows as f64;
                let (min_val, max_val) = self.get_data_range();
                let range = if (max_val - min_val).abs() < 1e-10 { 1.0 } else { max_val - min_val };

                // Draw cells
                for (row_idx, row) in self.data.iter().enumerate() {
                    for (col_idx, &val) in row.iter().enumerate() {
                        let x = plot_left + col_idx as f64 * cell_width;
                        let y = plot_top + row_idx as f64 * cell_height;

                        let normalized = (val - min_val) / range;
                        let color = self.colormap.sample(normalized);

                        self.draw_fill.color = color;
                        self.draw_fill.draw_abs(cx, Rect {
                            pos: dvec2(x, y),
                            size: dvec2(cell_width - 1.0, cell_height - 1.0),
                        });

                        // Draw value text
                        if self.show_values && cell_width > 25.0 && cell_height > 15.0 {
                            let text = if val.abs() < 10.0 {
                                format!("{:.1}", val)
                            } else {
                                format!("{:.0}", val)
                            };
                            let brightness = color.x * 0.299 + color.y * 0.587 + color.z * 0.114;
                            self.label.draw_text.color = if brightness > 0.5 { vec4(0.0, 0.0, 0.0, 1.0) } else { vec4(1.0, 1.0, 1.0, 1.0) };
                            self.label.draw_at(cx, dvec2(x + cell_width / 2.0, y + cell_height / 2.0), &text, TextAnchor::Center);
                        }
                    }
                }

                // Draw X labels
                self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
                for (i, label) in self.x_labels.iter().enumerate() {
                    if i < cols {
                        let x = plot_left + (i as f64 + 0.5) * cell_width;
                        self.label.draw_at(cx, dvec2(x, plot_bottom + 15.0), label, TextAnchor::TopCenter);
                    }
                }

                // Draw Y labels
                for (i, label) in self.y_labels.iter().enumerate() {
                    if i < rows {
                        let y = plot_top + (i as f64 + 0.5) * cell_height;
                        self.label.draw_at(cx, dvec2(plot_left - 10.0, y), label, TextAnchor::MiddleRight);
                    }
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl Heatmap {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl HeatmapRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, data: Vec<Vec<f64>>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(data); }
    }
    pub fn set_labels(&self, x_labels: Vec<String>, y_labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_labels(x_labels, y_labels); }
    }
    pub fn set_colormap(&self, colormap: Colormap) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(colormap); }
    }
    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_values(show); }
    }
    pub fn set_range(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_range(min, max); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

