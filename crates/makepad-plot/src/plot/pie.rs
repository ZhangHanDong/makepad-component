use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub PieChart = {{PieChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub DonutChart = {{DonutChart}} {
        width: Fill,
        height: Fill,
        draw_arc: {}
        label: <PlotLabel> {}
    }
}

/// Pie slice data
#[derive(Clone, Debug, Default)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl PieSlice {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PieChart {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_slice: DrawPieSlice,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    label: PlotLabel,

    #[rust]
    slices: Vec<PieSlice>,

    #[rust]
    title: String,

    #[rust(0.8)]
    radius_ratio: f64,

    #[rust]
    show_labels: bool,

    #[rust]
    show_percentages: bool,

    #[rust]
    legend_position: LegendPosition,
}

impl Widget for PieChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.slices.is_empty() {
            self.draw_pie(cx, rect);
            self.draw_title(cx, rect);
            self.draw_legend(cx, rect);
        }

        DrawStep::done()
    }
}

impl PieChart {
    pub fn add_slice(&mut self, slice: PieSlice) {
        self.slices.push(slice);
    }

    pub fn set_slices(&mut self, slices: Vec<PieSlice>) {
        self.slices = slices;
    }

    pub fn set_data(&mut self, labels: Vec<String>, values: Vec<f64>) {
        self.slices = labels.into_iter().zip(values).map(|(l, v)| PieSlice::new(l, v)).collect();
    }

    pub fn clear(&mut self) {
        self.slices.clear();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    pub fn set_legend(&mut self, position: LegendPosition) {
        self.legend_position = position;
    }

    fn draw_pie(&mut self, cx: &mut Cx2d, rect: Rect) {
        let total: f64 = self.slices.iter().map(|s| s.value).sum();
        if total <= 0.0 {
            return;
        }

        // Account for title and legend margins
        let title_margin = if !self.title.is_empty() { 30.0 } else { 10.0 };
        let bottom_margin = 10.0;
        let available_height = rect.size.y - title_margin - bottom_margin;
        let available_width = rect.size.x - 20.0; // Side margins

        let center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + title_margin + available_height / 2.0,
        );
        let radius = (available_width.min(available_height) / 2.0) * self.radius_ratio;

        let mut start_angle = -std::f64::consts::FRAC_PI_2;

        for (idx, slice) in self.slices.iter().enumerate() {
            let slice_angle = (slice.value / total) * std::f64::consts::TAU;
            let end_angle = start_angle + slice_angle;

            let shader_start = (start_angle + std::f64::consts::TAU) % std::f64::consts::TAU;
            let shader_end = shader_start + slice_angle;

            let color = slice.color.unwrap_or_else(|| get_color(idx));
            self.draw_slice.color = color;
            self.draw_slice.draw_slice(cx, center, radius, shader_start, shader_end);

            if self.show_percentages {
                let mid_angle = start_angle + slice_angle / 2.0;
                let label_radius = radius * 0.65;
                let label_x = center.x + mid_angle.cos() * label_radius;
                let label_y = center.y + mid_angle.sin() * label_radius;

                let percentage = (slice.value / total) * 100.0;
                let label_text = format!("{:.1}%", percentage);

                self.label.set_color(vec4(1.0, 1.0, 1.0, 1.0));
                self.label.draw_at(cx, dvec2(label_x, label_y), &label_text, TextAnchor::Center);
            }

            start_angle = end_angle;
        }
    }

    fn draw_title(&mut self, cx: &mut Cx2d, rect: Rect) {
        if !self.title.is_empty() {
            let center_x = rect.pos.x + rect.size.x / 2.0;
            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(cx, dvec2(center_x, rect.pos.y + 10.0), &self.title, TextAnchor::TopCenter);
        }
    }

    fn draw_legend(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.legend_position == LegendPosition::None || self.slices.is_empty() {
            return;
        }

        let padding = 8.0;
        let line_height = 16.0;
        let marker_size = 10.0;
        let marker_text_gap = 6.0;
        let legend_height = self.slices.len() as f64 * line_height + padding * 2.0;
        let legend_width = 100.0;

        let (legend_x, legend_y) = match self.legend_position {
            LegendPosition::TopRight => (
                rect.pos.x + rect.size.x - legend_width - 10.0,
                rect.pos.y + 30.0,
            ),
            LegendPosition::TopLeft => (
                rect.pos.x + 10.0,
                rect.pos.y + 30.0,
            ),
            LegendPosition::BottomRight => (
                rect.pos.x + rect.size.x - legend_width - 10.0,
                rect.pos.y + rect.size.y - legend_height - 10.0,
            ),
            LegendPosition::BottomLeft => (
                rect.pos.x + 10.0,
                rect.pos.y + rect.size.y - legend_height - 10.0,
            ),
            LegendPosition::None => return,
        };

        self.draw_line.color = vec4(0.95, 0.95, 0.95, 0.9);
        let bg_rect = Rect {
            pos: dvec2(legend_x, legend_y),
            size: dvec2(legend_width, legend_height),
        };
        self.draw_line.draw_abs(cx, bg_rect);

        self.draw_line.color = vec4(0.8, 0.8, 0.8, 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x + legend_width, legend_y), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y + legend_height), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x + legend_width, legend_y), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);

        for (idx, slice) in self.slices.iter().enumerate() {
            let color = slice.color.unwrap_or_else(|| get_color(idx));
            let entry_y = legend_y + padding + idx as f64 * line_height + line_height / 2.0;

            self.draw_line.color = color;
            let marker_rect = Rect {
                pos: dvec2(legend_x + padding, entry_y - marker_size / 2.0),
                size: dvec2(marker_size, marker_size),
            };
            self.draw_line.draw_abs(cx, marker_rect);

            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(
                cx,
                dvec2(legend_x + padding + marker_size + marker_text_gap, entry_y),
                &slice.label,
                TextAnchor::MiddleLeft,
            );
        }
    }
}

impl PieChartRef {
    pub fn add_slice(&self, slice: PieSlice) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_slice(slice);
        }
    }

    pub fn set_slices(&self, slices: Vec<PieSlice>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_slices(slices);
        }
    }

    pub fn set_data(&self, labels: Vec<String>, values: Vec<f64>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(labels, values);
        }
    }

    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_percentages(show);
        }
    }

    pub fn set_legend(&self, position: LegendPosition) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_legend(position);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}


// =============================================================================
// DonutChart Widget
// =============================================================================

#[derive(Clone)]
pub struct DonutSlice {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl DonutSlice {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DonutChart {
    #[deref] #[live] view: View,
    #[live] draw_arc: DrawArc,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] slices: Vec<DonutSlice>,
    #[rust] inner_radius_ratio: f64,
    #[rust] center_label: String,
    #[rust] show_labels: bool,
    #[rust] show_percentages: bool,
    #[rust] use_gradient: bool,
}

impl DonutChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, slices: Vec<DonutSlice>) {
        self.slices = slices;
    }

    pub fn add_slice(&mut self, slice: DonutSlice) {
        self.slices.push(slice);
    }

    pub fn set_inner_radius_ratio(&mut self, ratio: f64) {
        self.inner_radius_ratio = ratio.clamp(0.0, 0.9);
    }

    pub fn set_center_label(&mut self, label: impl Into<String>) {
        self.center_label = label.into();
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_use_gradient(&mut self, use_gradient: bool) {
        self.use_gradient = use_gradient;
    }

    pub fn clear(&mut self) {
        self.slices.clear();
    }
}

impl Widget for DonutChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.slices.is_empty() {
            // Set defaults
            if self.inner_radius_ratio == 0.0 { self.inner_radius_ratio = 0.5; }
            // Enable gradient by default for better visuals
            self.use_gradient = true;

            let total: f64 = self.slices.iter().map(|s| s.value).sum();
            if total <= 0.0 { return DrawStep::done(); }

            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let outer_radius = (rect.size.x.min(rect.size.y) / 2.0 - 40.0).max(20.0);

            let mut start_angle = -std::f64::consts::PI / 2.0;

            for (i, slice) in self.slices.iter().enumerate() {
                let sweep_angle = (slice.value / total) * 2.0 * std::f64::consts::PI;
                let end_angle = start_angle + sweep_angle;
                let color = slice.color.unwrap_or_else(|| get_color(i));

                // Use proper arc shader for clean rendering
                self.draw_arc.color = color;
                if self.use_gradient {
                    // Create a subtle radial gradient
                    let lighter = vec4(
                        (color.x * 1.3).min(1.0),
                        (color.y * 1.3).min(1.0),
                        (color.z * 1.3).min(1.0),
                        color.w
                    );
                    self.draw_arc.draw_arc_gradient(
                        cx, center, outer_radius, self.inner_radius_ratio,
                        start_angle, end_angle, lighter, color, 0  // radial gradient
                    );
                } else {
                    self.draw_arc.draw_arc(cx, center, outer_radius, self.inner_radius_ratio, start_angle, end_angle);
                }

                // Draw label
                if self.show_labels || self.show_percentages {
                    let mid_angle = start_angle + sweep_angle / 2.0;
                    let label_radius = outer_radius + 15.0;
                    let label_pos = dvec2(center.x + label_radius * mid_angle.cos(), center.y + label_radius * mid_angle.sin());

                    let label_text = if self.show_percentages {
                        let pct = (slice.value / total) * 100.0;
                        if self.show_labels {
                            format!("{} ({:.1}%)", slice.label, pct)
                        } else {
                            format!("{:.1}%", pct)
                        }
                    } else {
                        slice.label.clone()
                    };

                    self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
                    let anchor = if mid_angle.cos() > 0.0 { TextAnchor::MiddleLeft } else { TextAnchor::MiddleRight };
                    self.label.draw_at(cx, label_pos, &label_text, anchor);
                }

                start_angle = end_angle;
            }

            // Draw center label
            if !self.center_label.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, center, &self.center_label, TextAnchor::Center);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl DonutChart {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl DonutChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, slices: Vec<DonutSlice>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(slices); }
    }
    pub fn add_slice(&self, slice: DonutSlice) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_slice(slice); }
    }
    pub fn set_inner_radius_ratio(&self, ratio: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_inner_radius_ratio(ratio); }
    }
    pub fn set_center_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_center_label(label); }
    }
    pub fn set_show_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_labels(show); }
    }
    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_percentages(show); }
    }
    pub fn set_use_gradient(&self, use_gradient: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_use_gradient(use_gradient); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

