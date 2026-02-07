use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub GaugeChart = {{GaugeChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub FunnelChart = {{FunnelChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct GaugeChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] value: f64,
    #[rust] min_value: f64,
    #[rust] max_value: f64,
    #[rust] thresholds: Vec<(f64, Vec4)>,  // (value, color) pairs
    #[rust] show_value: bool,
    #[rust] unit: String,
    #[rust] arc_width: f64,
}

impl GaugeChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = min;
        self.max_value = max;
    }

    pub fn set_thresholds(&mut self, thresholds: Vec<(f64, Vec4)>) {
        self.thresholds = thresholds;
    }

    pub fn set_unit(&mut self, unit: impl Into<String>) {
        self.unit = unit.into();
    }

    pub fn set_show_value(&mut self, show: bool) {
        self.show_value = show;
    }

    pub fn set_arc_width(&mut self, width: f64) {
        self.arc_width = width;
    }

    fn get_color_for_value(&self, value: f64) -> Vec4 {
        if self.thresholds.is_empty() {
            return vec4(0.12, 0.47, 0.71, 1.0);
        }
        for &(threshold, color) in self.thresholds.iter().rev() {
            if value >= threshold {
                return color;
            }
        }
        self.thresholds.first().map(|&(_, c)| c).unwrap_or(vec4(0.12, 0.47, 0.71, 1.0))
    }
}

impl Widget for GaugeChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            // Initialize defaults
            if self.max_value == 0.0 && self.min_value == 0.0 {
                self.max_value = 100.0;
            }
            if self.arc_width == 0.0 {
                self.arc_width = 20.0;
            }
            if self.thresholds.is_empty() {
                self.thresholds = vec![
                    (0.0, vec4(0.17, 0.63, 0.17, 1.0)),    // Green
                    (60.0, vec4(1.0, 0.65, 0.0, 1.0)),     // Orange
                    (80.0, vec4(0.84, 0.15, 0.16, 1.0)),   // Red
                ];
            }

            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y * 0.6);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 40.0).max(30.0);

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            // Draw arc segments (from -135° to 135°, i.e., 270° total)
            let start_angle = -std::f64::consts::PI * 0.75;  // -135°
            let end_angle = std::f64::consts::PI * 0.75;     // 135°
            let total_angle = end_angle - start_angle;

            // Draw background arc
            let num_segments = 60;
            self.draw_line.color = vec4(0.85, 0.85, 0.85, 1.0);
            for i in 0..num_segments {
                let t1 = i as f64 / num_segments as f64;
                let t2 = (i + 1) as f64 / num_segments as f64;
                let a1 = start_angle + t1 * total_angle;
                let a2 = start_angle + t2 * total_angle;
                let p1 = dvec2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = dvec2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                self.draw_line.draw_line(cx, p1, p2, self.arc_width);
            }

            // Draw colored arc based on value
            let value_ratio = ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0);
            let value_angle = start_angle + value_ratio * total_angle;

            let color = self.get_color_for_value(self.value);
            self.draw_line.color = color;

            let value_segments = (value_ratio * num_segments as f64) as usize;
            for i in 0..value_segments {
                let t1 = i as f64 / num_segments as f64;
                let t2 = (i + 1) as f64 / num_segments as f64;
                let a1 = start_angle + t1 * total_angle;
                let a2 = start_angle + t2 * total_angle;
                let p1 = dvec2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = dvec2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                self.draw_line.draw_line(cx, p1, p2, self.arc_width);
            }

            // Draw needle
            let needle_length = radius - self.arc_width / 2.0 - 5.0;
            let needle_end = dvec2(
                center.x + needle_length * value_angle.cos(),
                center.y + needle_length * value_angle.sin()
            );
            self.draw_line.color = vec4(0.2, 0.2, 0.2, 1.0);
            self.draw_line.draw_line(cx, center, needle_end, 3.0);

            // Draw center circle
            self.draw_fill.color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_fill.draw_abs(cx, Rect {
                pos: dvec2(center.x - 8.0, center.y - 8.0),
                size: dvec2(16.0, 16.0),
            });

            // Draw value text
            let value_text = if self.unit.is_empty() {
                format!("{:.1}", self.value)
            } else {
                format!("{:.1}{}", self.value, self.unit)
            };
            self.label.draw_at(cx, dvec2(center.x, center.y + 30.0), &value_text, TextAnchor::TopCenter);

            // Draw min/max labels
            let min_pos = dvec2(center.x + radius * start_angle.cos(), center.y + radius * start_angle.sin() + 15.0);
            let max_pos = dvec2(center.x + radius * end_angle.cos(), center.y + radius * end_angle.sin() + 15.0);
            self.label.draw_at(cx, min_pos, &format!("{:.0}", self.min_value), TextAnchor::TopCenter);
            self.label.draw_at(cx, max_pos, &format!("{:.0}", self.max_value), TextAnchor::TopCenter);
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl GaugeChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_value(&self, value: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_value(value); }
    }
    pub fn set_range(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_range(min, max); }
    }
    pub fn set_thresholds(&self, thresholds: Vec<(f64, Vec4)>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_thresholds(thresholds); }
    }
    pub fn set_unit(&self, unit: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_unit(unit); }
    }
    pub fn set_show_value(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_value(show); }
    }
    pub fn set_arc_width(&self, width: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_arc_width(width); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}


// =============================================================================
// FunnelChart Widget
// =============================================================================

pub struct FunnelStage {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl FunnelStage {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct FunnelChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] stages: Vec<FunnelStage>,
    #[rust] show_percentages: bool,
    #[rust] show_values: bool,
    #[rust(30.0)] left_margin: f64,
    #[rust(20.0)] bottom_margin: f64,
    #[rust(30.0)] right_margin: f64,
    #[rust(30.0)] top_margin: f64,
}

impl FunnelChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, stages: Vec<FunnelStage>) {
        self.stages = stages;
    }

    pub fn add_stage(&mut self, stage: FunnelStage) {
        self.stages.push(stage);
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn clear(&mut self) {
        self.stages.clear();
    }
}

impl Widget for FunnelChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.stages.is_empty() {
            let plot_rect = Rect {
                pos: dvec2(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin),
                size: dvec2(
                    rect.size.x - self.left_margin - self.right_margin,
                    rect.size.y - self.top_margin - self.bottom_margin
                ),
            };

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            let max_value = self.stages.iter().map(|s| s.value).fold(0.0f64, f64::max);
            if max_value == 0.0 { return DrawStep::done(); }

            let num_stages = self.stages.len();
            let stage_height = plot_rect.size.y / num_stages as f64;
            let center_x = plot_rect.pos.x + plot_rect.size.x / 2.0;
            let max_width = plot_rect.size.x * 0.9;

            for (i, stage) in self.stages.iter().enumerate() {
                let ratio = stage.value / max_value;
                let width = max_width * ratio;
                let y = plot_rect.pos.y + i as f64 * stage_height;

                // Get color
                let color = stage.color.unwrap_or_else(|| get_color(i));

                // Draw trapezoid (approximated as rectangle for simplicity)
                // For a proper funnel, we'd need next stage's width
                let next_ratio = if i + 1 < num_stages {
                    self.stages[i + 1].value / max_value
                } else {
                    ratio * 0.3  // Taper at bottom
                };
                let next_width = max_width * next_ratio;

                // Draw as a series of lines to create trapezoid effect
                let top_left = dvec2(center_x - width / 2.0, y);
                let top_right = dvec2(center_x + width / 2.0, y);
                let bottom_left = dvec2(center_x - next_width / 2.0, y + stage_height);
                let bottom_right = dvec2(center_x + next_width / 2.0, y + stage_height);

                // Fill trapezoid using horizontal lines
                self.draw_fill.color = color;
                let num_lines = (stage_height as usize).max(1);
                for j in 0..num_lines {
                    let t = j as f64 / num_lines as f64;
                    let line_y = y + t * stage_height;
                    let line_width = width + (next_width - width) * t;
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(center_x - line_width / 2.0, line_y),
                        size: dvec2(line_width, 2.0),
                    });
                }

                // Draw outline
                self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.8);
                self.draw_line.draw_line(cx, top_left, top_right, 1.0);
                self.draw_line.draw_line(cx, top_left, bottom_left, 1.0);
                self.draw_line.draw_line(cx, top_right, bottom_right, 1.0);

                // Draw label on left
                self.label.draw_at(cx, dvec2(plot_rect.pos.x - 5.0, y + stage_height / 2.0),
                    &stage.label, TextAnchor::MiddleRight);

                // Draw value/percentage on right
                let value_text = if self.show_percentages {
                    format!("{:.1}%", ratio * 100.0)
                } else {
                    format!("{:.0}", stage.value)
                };
                self.label.draw_at(cx, dvec2(plot_rect.pos.x + plot_rect.size.x + 5.0, y + stage_height / 2.0),
                    &value_text, TextAnchor::MiddleLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl FunnelChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, stages: Vec<FunnelStage>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(stages); }
    }
    pub fn add_stage(&self, stage: FunnelStage) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_stage(stage); }
    }
    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_percentages(show); }
    }
    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_values(show); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

