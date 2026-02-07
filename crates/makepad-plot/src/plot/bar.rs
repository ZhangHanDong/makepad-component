use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub BarPlot = {{BarPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

/// Bar group for grouped/stacked bar charts
#[derive(Clone, Debug)]
pub struct BarGroup {
    pub label: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl BarGroup {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self { label: label.into(), values, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BarPlot {
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
    categories: Vec<String>,

    #[rust]
    values: Vec<f64>,

    #[rust]
    bar_color: Option<Vec4>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust(50.0)]
    left_margin: f64,

    #[rust(40.0)]
    bottom_margin: f64,

    #[rust(20.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,

    #[rust(0.8)]
    bar_width_ratio: f64,

    // New fields for enhanced bar charts
    #[rust]
    horizontal: bool,

    #[rust]
    stacked: bool,

    #[rust]
    groups: Vec<BarGroup>,

    #[rust]
    show_bar_labels: bool,
}

impl Widget for BarPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        let has_data = !self.values.is_empty() || !self.groups.is_empty();
        if rect.size.x > 0.0 && rect.size.y > 0.0 && has_data {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_bars(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}

impl BarPlot {
    /// Set bar data (simple mode - single series)
    pub fn set_data(&mut self, categories: Vec<String>, values: Vec<f64>) {
        self.categories = categories;
        self.values = values;
        self.groups.clear();
    }

    /// Set plot title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Set bar color (for simple mode)
    pub fn set_color(&mut self, color: Vec4) {
        self.bar_color = Some(color);
    }

    /// Set horizontal orientation (barh)
    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
    }

    /// Set stacked mode
    pub fn set_stacked(&mut self, stacked: bool) {
        self.stacked = stacked;
    }

    /// Show bar value labels
    pub fn set_show_bar_labels(&mut self, show: bool) {
        self.show_bar_labels = show;
    }

    /// Add a bar group (for grouped/stacked bars)
    pub fn add_group(&mut self, group: BarGroup) {
        self.groups.push(group);
    }

    /// Set multiple groups at once
    pub fn set_groups(&mut self, categories: Vec<String>, groups: Vec<BarGroup>) {
        self.categories = categories;
        self.groups = groups;
        self.values.clear();
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.categories.clear();
        self.values.clear();
        self.groups.clear();
    }

    fn update_plot_area(&mut self, rect: Rect) {
        let left_margin = if self.horizontal { 80.0 } else { self.left_margin };
        let bottom_margin = if self.horizontal { self.bottom_margin } else { 40.0 };
        self.plot_area = PlotArea::new(
            rect.pos.x + left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - bottom_margin,
        );
    }

    fn get_value_range(&self) -> (f64, f64) {
        if !self.groups.is_empty() {
            if self.stacked {
                // For stacked, sum up all groups per category
                let num_cats = self.categories.len();
                let mut max = 0.0f64;
                for cat_idx in 0..num_cats {
                    let sum: f64 = self.groups.iter()
                        .filter_map(|g| g.values.get(cat_idx))
                        .sum();
                    max = max.max(sum);
                }
                (0.0, max * 1.1)
            } else {
                // For grouped, find max across all values
                let max = self.groups.iter()
                    .flat_map(|g| g.values.iter())
                    .cloned()
                    .fold(0.0f64, f64::max);
                (0.0, max * 1.1)
            }
        } else {
            let max = self.values.iter().cloned().fold(0.0f64, f64::max);
            (0.0, max * 1.1)
        }
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);

        let (v_min, v_max) = self.get_value_range();
        let v_ticks = self.generate_ticks(v_min, v_max, 5);

        if self.horizontal {
            // Vertical grid lines for horizontal bars
            for v in &v_ticks {
                let x_pixel = self.plot_area.left + (*v - v_min) / (v_max - v_min) * self.plot_area.width();
                let p1 = dvec2(x_pixel, self.plot_area.top);
                let p2 = dvec2(x_pixel, self.plot_area.bottom);
                self.draw_line.draw_line(cx, p1, p2, 0.5);
            }
        } else {
            // Horizontal grid lines for vertical bars
            for v in &v_ticks {
                let y_pixel = self.plot_area.bottom - (*v - v_min) / (v_max - v_min) * self.plot_area.height();
                let p1 = dvec2(self.plot_area.left, y_pixel);
                let p2 = dvec2(self.plot_area.right, y_pixel);
                self.draw_line.draw_line(cx, p1, p2, 0.5);
            }
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

        if self.horizontal {
            // X axis (bottom)
            let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
            self.draw_line.draw_line(cx, x1, x2, 1.0);

            // Y axis (left)
            let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let y2 = dvec2(self.plot_area.left, self.plot_area.top);
            self.draw_line.draw_line(cx, y1, y2, 1.0);
        } else {
            // X axis
            let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
            self.draw_line.draw_line(cx, x1, x2, 1.0);

            // Y axis
            let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let y2 = dvec2(self.plot_area.left, self.plot_area.top);
            self.draw_line.draw_line(cx, y1, y2, 1.0);
        }
    }

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let (v_min, v_max) = self.get_value_range();

        if !self.groups.is_empty() {
            self.draw_grouped_bars(cx, v_min, v_max);
        } else {
            self.draw_simple_bars(cx, v_min, v_max);
        }
    }

    fn draw_simple_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let n = self.values.len();
        if n == 0 {
            return;
        }

        if self.horizontal {
            let band_height = self.plot_area.height() / n as f64;
            let bar_height = band_height * self.bar_width_ratio;

            for (i, value) in self.values.iter().enumerate() {
                let color = self.bar_color.unwrap_or_else(|| get_color(0));
                self.draw_bar.color = color;

                let y_center = self.plot_area.top + (i as f64 + 0.5) * band_height;
                let bar_width = (*value - v_min) / (v_max - v_min) * self.plot_area.width();

                let rect = Rect {
                    pos: dvec2(self.plot_area.left, y_center - bar_height / 2.0),
                    size: dvec2(bar_width, bar_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Bar label
                if self.show_bar_labels {
                    self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
                    let label = format!("{:.1}", value);
                    self.label.draw_at(cx, dvec2(self.plot_area.left + bar_width + 5.0, y_center), &label, TextAnchor::MiddleLeft);
                }
            }
        } else {
            let band_width = self.plot_area.width() / n as f64;
            let bar_width = band_width * self.bar_width_ratio;

            for (i, value) in self.values.iter().enumerate() {
                let color = self.bar_color.unwrap_or_else(|| get_color(0));
                self.draw_bar.color = color;

                let x_center = self.plot_area.left + (i as f64 + 0.5) * band_width;
                let bar_height = (*value - v_min) / (v_max - v_min) * self.plot_area.height();
                let bar_top = self.plot_area.bottom - bar_height;

                let rect = Rect {
                    pos: dvec2(x_center - bar_width / 2.0, bar_top),
                    size: dvec2(bar_width, bar_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Bar label
                if self.show_bar_labels {
                    self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
                    let label = format!("{:.1}", value);
                    self.label.draw_at(cx, dvec2(x_center, bar_top - 5.0), &label, TextAnchor::BottomCenter);
                }
            }
        }
    }

    fn draw_grouped_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();
        let num_groups = self.groups.len();
        if num_cats == 0 || num_groups == 0 {
            return;
        }

        if self.stacked {
            self.draw_stacked_bars(cx, v_min, v_max);
        } else {
            self.draw_side_by_side_bars(cx, v_min, v_max);
        }
    }

    fn draw_stacked_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();

        if self.horizontal {
            let band_height = self.plot_area.height() / num_cats as f64;
            let bar_height = band_height * self.bar_width_ratio;

            for cat_idx in 0..num_cats {
                let y_center = self.plot_area.top + (cat_idx as f64 + 0.5) * band_height;
                let mut x_start = self.plot_area.left;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let bar_width = (value - v_min) / (v_max - v_min) * self.plot_area.width();
                        let rect = Rect {
                            pos: dvec2(x_start, y_center - bar_height / 2.0),
                            size: dvec2(bar_width, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                        x_start += bar_width;
                    }
                }
            }
        } else {
            let band_width = self.plot_area.width() / num_cats as f64;
            let bar_width = band_width * self.bar_width_ratio;

            for cat_idx in 0..num_cats {
                let x_center = self.plot_area.left + (cat_idx as f64 + 0.5) * band_width;
                let mut y_bottom = self.plot_area.bottom;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let bar_height = (value - v_min) / (v_max - v_min) * self.plot_area.height();
                        let bar_top = y_bottom - bar_height;
                        let rect = Rect {
                            pos: dvec2(x_center - bar_width / 2.0, bar_top),
                            size: dvec2(bar_width, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                        y_bottom = bar_top;
                    }
                }
            }
        }
    }

    fn draw_side_by_side_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();
        let num_groups = self.groups.len();

        if self.horizontal {
            let band_height = self.plot_area.height() / num_cats as f64;
            let group_height = band_height * self.bar_width_ratio / num_groups as f64;

            for cat_idx in 0..num_cats {
                let y_start = self.plot_area.top + (cat_idx as f64 + 0.5) * band_height
                    - (band_height * self.bar_width_ratio) / 2.0;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let y_pos = y_start + group_idx as f64 * group_height;
                        let bar_width = (value - v_min) / (v_max - v_min) * self.plot_area.width();
                        let rect = Rect {
                            pos: dvec2(self.plot_area.left, y_pos),
                            size: dvec2(bar_width, group_height * 0.9),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                    }
                }
            }
        } else {
            let band_width = self.plot_area.width() / num_cats as f64;
            let group_width = band_width * self.bar_width_ratio / num_groups as f64;

            for cat_idx in 0..num_cats {
                let x_start = self.plot_area.left + (cat_idx as f64 + 0.5) * band_width
                    - (band_width * self.bar_width_ratio) / 2.0;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let x_pos = x_start + group_idx as f64 * group_width;
                        let bar_height = (value - v_min) / (v_max - v_min) * self.plot_area.height();
                        let bar_top = self.plot_area.bottom - bar_height;
                        let rect = Rect {
                            pos: dvec2(x_pos, bar_top),
                            size: dvec2(group_width * 0.9, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                    }
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let n = self.categories.len().max(self.values.len());
        let (v_min, v_max) = self.get_value_range();

        if self.horizontal {
            // Category labels on Y axis
            let band_height = self.plot_area.height() / n as f64;
            for (i, cat) in self.categories.iter().enumerate() {
                let y = self.plot_area.top + (i as f64 + 0.5) * band_height;
                self.label.draw_at(cx, dvec2(self.plot_area.left - 5.0, y), cat, TextAnchor::MiddleRight);
            }

            // Value tick labels on X axis
            let v_ticks = self.generate_ticks(v_min, v_max, 5);
            for v in &v_ticks {
                let x_pixel = self.plot_area.left + (*v - v_min) / (v_max - v_min) * self.plot_area.width();
                let label = format!("{:.0}", v);
                self.label.draw_at(cx, dvec2(x_pixel, self.plot_area.bottom + 5.0), &label, TextAnchor::TopCenter);
            }
        } else {
            // Category labels on X axis
            let band_width = self.plot_area.width() / n as f64;
            for (i, cat) in self.categories.iter().enumerate() {
                let x = self.plot_area.left + (i as f64 + 0.5) * band_width;
                self.label.draw_at(cx, dvec2(x, self.plot_area.bottom + 5.0), cat, TextAnchor::TopCenter);
            }

            // Value tick labels on Y axis
            let v_ticks = self.generate_ticks(v_min, v_max, 5);
            for v in &v_ticks {
                let y_pixel = self.plot_area.bottom - (*v - v_min) / (v_max - v_min) * self.plot_area.height();
                let label = format!("{:.0}", v);
                self.label.draw_at(cx, dvec2(self.plot_area.left - 5.0, y_pixel), &label, TextAnchor::MiddleRight);
            }
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        let step = (max - min) / count as f64;
        (0..=count).map(|i| min + i as f64 * step).collect()
    }
}

impl BarPlotRef {
    pub fn set_data(&self, categories: Vec<String>, values: Vec<f64>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(categories, values);
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_color(&self, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_color(color);
        }
    }

    pub fn set_horizontal(&self, horizontal: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_horizontal(horizontal);
        }
    }

    pub fn set_stacked(&self, stacked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stacked(stacked);
        }
    }

    pub fn set_show_bar_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_bar_labels(show);
        }
    }

    pub fn add_group(&self, group: BarGroup) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_group(group);
        }
    }

    pub fn set_groups(&self, categories: Vec<String>, groups: Vec<BarGroup>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_groups(categories, groups);
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

