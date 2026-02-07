use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub Treemap = {{Treemap}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

pub struct TreemapNode {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl TreemapNode {
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
pub struct Treemap {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] nodes: Vec<TreemapNode>,
    #[rust] show_labels: bool,
}

impl Treemap {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, nodes: Vec<TreemapNode>) {
        self.nodes = nodes;
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }
}

impl Widget for Treemap {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.nodes.is_empty() {
            let padding = 20.0;
            let title_space = if self.title.is_empty() { 0.0 } else { 30.0 };

            let plot_left = rect.pos.x + padding;
            let plot_top = rect.pos.y + padding + title_space;
            let plot_width = rect.size.x - padding * 2.0;
            let plot_height = rect.size.y - padding * 2.0 - title_space;

            let total: f64 = self.nodes.iter().map(|n| n.value).sum();
            if total > 0.0 {
                let area = plot_width * plot_height;
                let mut x = plot_left;
                let mut y = plot_top;
                let mut remaining_width = plot_width;
                let mut remaining_height = plot_height;
                let horizontal = plot_width > plot_height;

                for (i, node) in self.nodes.iter().enumerate() {
                    let node_area = (node.value / total) * area;
                    let (node_x, node_y, node_w, node_h) = if horizontal {
                        let w = if remaining_height > 0.0 { node_area / remaining_height } else { 0.0 };
                        let w = w.min(remaining_width);
                        let result = (x, y, w, remaining_height);
                        x += w;
                        remaining_width -= w;
                        result
                    } else {
                        let h = if remaining_width > 0.0 { node_area / remaining_width } else { 0.0 };
                        let h = h.min(remaining_height);
                        let result = (x, y, remaining_width, h);
                        y += h;
                        remaining_height -= h;
                        result
                    };

                    if node_w > 2.0 && node_h > 2.0 {
                        let color = node.color.unwrap_or_else(|| get_color(i));

                        // Draw filled rectangle
                        self.draw_fill.color = color;
                        self.draw_fill.draw_abs(cx, Rect {
                            pos: dvec2(node_x, node_y),
                            size: dvec2(node_w - 2.0, node_h - 2.0),
                        });

                        // Draw border
                        self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.8);
                        let corners = [
                            dvec2(node_x, node_y),
                            dvec2(node_x + node_w - 2.0, node_y),
                            dvec2(node_x + node_w - 2.0, node_y + node_h - 2.0),
                            dvec2(node_x, node_y + node_h - 2.0),
                        ];
                        for j in 0..4 {
                            self.draw_line.draw_line(cx, corners[j], corners[(j + 1) % 4], 1.0);
                        }

                        // Draw label
                        if self.show_labels && node_w > 40.0 && node_h > 25.0 {
                            let center = dvec2(node_x + node_w / 2.0, node_y + node_h / 2.0);
                            let brightness = color.x * 0.299 + color.y * 0.587 + color.z * 0.114;
                            self.label.draw_text.color = if brightness > 0.5 { vec4(0.0, 0.0, 0.0, 1.0) } else { vec4(1.0, 1.0, 1.0, 1.0) };
                            self.label.draw_at(cx, center, &node.label, TextAnchor::Center);
                        }
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

impl Treemap {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl TreemapRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, nodes: Vec<TreemapNode>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(nodes); }
    }
    pub fn set_show_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_labels(show); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

