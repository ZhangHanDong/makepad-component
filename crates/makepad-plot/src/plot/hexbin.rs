use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub HexbinChart = {{HexbinChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub SankeyDiagram = {{SankeyDiagram}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

pub struct HexbinPoint {
    pub x: f64,
    pub y: f64,
}

impl HexbinPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct HexBin {
    center: DVec2,
    count: usize,
    ring: i32,
}

#[derive(Live, LiveHook, Widget)]
pub struct HexbinChart {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] points: Vec<HexbinPoint>,
    #[rust] hex_radius: f64,
    #[rust] color_low: Vec4,
    #[rust] color_high: Vec4,
    #[rust] title: String,
    #[rust] area: Area,
}

impl HexbinChart {
    pub fn set_data(&mut self, points: Vec<HexbinPoint>) {
        self.points = points;
    }

    pub fn set_hex_radius(&mut self, radius: f64) {
        self.hex_radius = radius.max(5.0);
    }

    pub fn set_colors(&mut self, low: Vec4, high: Vec4) {
        self.color_low = low;
        self.color_high = high;
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    fn cube_round(q: f64, r: f64) -> (i32, i32, i32) {
        let s = -q - r;
        let mut rq = q.round();
        let mut rr = r.round();
        let mut rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        } else {
            rs = -rq - rr;
        }

        (rq as i32, rr as i32, rs as i32)
    }

    fn calculate_bins(&self, chart_x: f64, chart_y: f64, chart_w: f64, chart_h: f64) -> (Vec<HexBin>, i32) {
        let center_x = chart_x + chart_w / 2.0;
        let center_y = chart_y + chart_h / 2.0;
        let chart_size = chart_w.min(chart_h);
        let hex_radius = if self.hex_radius > 0.0 { self.hex_radius } else { 14.0 };
        let rings = ((chart_size / 2.0) / (hex_radius * 1.5)).floor() as i32;

        let mut bin_data: std::collections::HashMap<(i32, i32, i32), (usize, i32)> = std::collections::HashMap::new();

        // Generate hexagonal grid using cube coordinates
        for q in -rings..=rings {
            for r in (-rings).max(-q - rings)..=rings.min(-q + rings) {
                let s = -q - r;
                let ring = q.abs().max(r.abs()).max(s.abs());
                bin_data.insert((q, r, s), (0, ring));
            }
        }

        // Assign points to bins
        if !self.points.is_empty() {
            let x_min = self.points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let x_max = self.points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
            let y_min = self.points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let y_max = self.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

            let x_range = (x_max - x_min).max(1.0);
            let y_range = (y_max - y_min).max(1.0);

            for point in &self.points {
                let px = ((point.x - x_min) / x_range) * chart_size - chart_size / 2.0;
                let py = ((point.y - y_min) / y_range) * chart_size - chart_size / 2.0;

                // Convert pixel to cube coordinates (pointy-topped)
                let q = (px * 3.0_f64.sqrt() / 3.0 - py / 3.0) / hex_radius;
                let r = (py * 2.0 / 3.0) / hex_radius;

                let (q, r, s) = Self::cube_round(q, r);

                if let Some((count, _)) = bin_data.get_mut(&(q, r, s)) {
                    *count += 1;
                }
            }
        }

        // Convert to pixel positions
        let mut bins = Vec::new();
        for ((q, r, _s), (count, ring)) in bin_data {
            let px = hex_radius * (3.0_f64.sqrt() * q as f64 + 3.0_f64.sqrt() / 2.0 * r as f64);
            let py = hex_radius * (3.0 / 2.0 * r as f64);

            bins.push(HexBin {
                center: DVec2 { x: center_x + px, y: center_y + py },
                count,
                ring,
            });
        }

        (bins, rings)
    }

    fn interpolate_color(&self, t: f64) -> Vec4 {
        // Radial gradient: dark at center (t=0), light at edge (t=1)
        vec4(
            self.color_high.x + t as f32 * (self.color_low.x - self.color_high.x),
            self.color_high.y + t as f32 * (self.color_low.y - self.color_high.y),
            self.color_high.z + t as f32 * (self.color_low.z - self.color_high.z),
            self.color_high.w + t as f32 * (self.color_low.w - self.color_high.w),
        )
    }

    fn draw_hexagon(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, color: Vec4) {
        let corners: Vec<DVec2> = (0..6)
            .map(|i| {
                let angle = std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 2.0;
                DVec2 {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                }
            })
            .collect();

        self.draw_triangle.color = color;

        for i in 0..6 {
            self.draw_triangle.draw_triangle(cx, center, corners[i], corners[(i + 1) % 6]);
        }
    }
}

impl Widget for HexbinChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            // Initialize defaults if not set
            if self.hex_radius <= 0.0 { self.hex_radius = 14.0; }
            if self.color_high == Vec4::default() {
                self.color_high = vec4(0.05, 0.15, 0.45, 1.0);
                self.color_low = vec4(0.92, 0.95, 0.98, 1.0);
            }

            let (bins, max_ring) = self.calculate_bins(chart_x, chart_y, chart_w, chart_h);

            // Sort by ring for proper layering
            let mut sorted_bins: Vec<_> = bins.iter().collect();
            sorted_bins.sort_by_key(|b| b.ring);

            for bin in sorted_bins {
                let t = if max_ring > 0 { bin.ring as f64 / max_ring as f64 } else { 0.0 };
                let t_eased = t * t * (3.0 - 2.0 * t); // smoothstep
                let color = self.interpolate_color(t_eased);
                self.draw_hexagon(cx, bin.center, self.hex_radius * 0.94, color);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl HexbinChartRef {
    pub fn set_data(&self, points: Vec<HexbinPoint>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(points); }
    }
    pub fn set_hex_radius(&self, radius: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_hex_radius(radius); }
    }
    pub fn set_colors(&self, low: Vec4, high: Vec4) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colors(low, high); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
}


// =============================================================================
// SankeyDiagram Widget
// =============================================================================

pub struct SankeyNode {
    pub name: String,
    pub layer: usize,
    pub value: f64,
    pub color: Vec4,
    // Layout computed values
    y: f64,
    height: f64,
}

impl SankeyNode {
    pub fn new(name: impl Into<String>, layer: usize, value: f64, color: Vec4) -> Self {
        Self {
            name: name.into(),
            layer,
            value,
            color,
            y: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SankeyLink {
    pub source: usize,
    pub target: usize,
    pub value: f64,
    // Layout computed values
    source_y: f64,
    target_y: f64,
}

impl SankeyLink {
    pub fn new(source: usize, target: usize, value: f64) -> Self {
        Self {
            source,
            target,
            value,
            source_y: 0.0,
            target_y: 0.0,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SankeyDiagram {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[redraw] #[live] draw_line: DrawPlotLine,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] nodes: Vec<SankeyNode>,
    #[rust] links: Vec<SankeyLink>,
    #[rust] title: String,
    #[rust] area: Area,
}

impl SankeyDiagram {
    pub fn set_data(&mut self, nodes: Vec<SankeyNode>, links: Vec<SankeyLink>) {
        self.nodes = nodes;
        self.links = links;
        self.compute_layout();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    fn compute_layout(&mut self) {
        if self.nodes.is_empty() { return; }

        // Calculate incoming totals
        let mut incoming_totals: Vec<f64> = vec![0.0; self.nodes.len()];
        for link in &self.links {
            incoming_totals[link.target] += link.value;
        }

        // Update node values from incoming
        for i in 0..self.nodes.len() {
            if self.nodes[i].layer > 0 {
                self.nodes[i].value = incoming_totals[i];
            }
        }

        let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);

        // Layout each layer
        for layer in 0..=max_layer {
            let layer_nodes: Vec<usize> = self.nodes.iter()
                .enumerate()
                .filter(|(_, n)| n.layer == layer)
                .map(|(i, _)| i)
                .collect();

            let total_value: f64 = layer_nodes.iter()
                .map(|&i| if layer == 0 { self.nodes[i].value } else { incoming_totals[i] })
                .sum();

            let mut y = 0.0;
            let gap_fraction = 0.08;

            for &idx in &layer_nodes {
                let node_value = if layer == 0 { self.nodes[idx].value } else { incoming_totals[idx] };
                let height = if total_value > 0.0 {
                    node_value / total_value * (1.0 - gap_fraction * (layer_nodes.len() - 1) as f64)
                } else { 0.0 };
                self.nodes[idx].y = y;
                self.nodes[idx].height = height;
                y += height + gap_fraction;
            }
        }

        // Compute source totals
        let source_totals: Vec<f64> = self.nodes.iter().enumerate().map(|(i, node)| {
            if node.layer == 0 { node.value } else { incoming_totals[i] }
        }).collect();

        // Compute link positions
        let mut source_offsets: Vec<f64> = vec![0.0; self.nodes.len()];
        let mut target_offsets: Vec<f64> = vec![0.0; self.nodes.len()];

        for link in &mut self.links {
            let source_idx = link.source;
            let target_idx = link.target;

            link.source_y = self.nodes[source_idx].y + source_offsets[source_idx];
            link.target_y = self.nodes[target_idx].y + target_offsets[target_idx];

            let source_total = source_totals[source_idx];
            let target_total = incoming_totals[target_idx];

            if source_total > 0.0 {
                source_offsets[source_idx] += link.value / source_total * self.nodes[source_idx].height;
            }
            if target_total > 0.0 {
                target_offsets[target_idx] += link.value / target_total * self.nodes[target_idx].height;
            }
        }
    }
}

impl Widget for SankeyDiagram {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 && !self.nodes.is_empty() {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + 30.0;
            let chart_width = rect.size.x - padding * 2.0;
            let chart_height = rect.size.y - padding - 40.0;

            if chart_width <= 0.0 || chart_height <= 0.0 { return DrawStep::done(); }

            let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);
            let node_width = 24.0;
            let layer_spacing = if max_layer > 0 {
                (chart_width - node_width) / max_layer as f64
            } else { chart_width };

            // Precompute totals
            let mut incoming_totals: Vec<f64> = vec![0.0; self.nodes.len()];
            for link in &self.links {
                incoming_totals[link.target] += link.value;
            }

            let source_totals: Vec<f64> = self.nodes.iter().enumerate().map(|(i, node)| {
                if node.layer == 0 { node.value } else { incoming_totals[i] }
            }).collect();

            // Draw links
            for link in &self.links {
                let source = &self.nodes[link.source];
                let target = &self.nodes[link.target];

                let sx = chart_x + source.layer as f64 * layer_spacing + node_width;
                let sy = chart_y + link.source_y * chart_height;
                let tx = chart_x + target.layer as f64 * layer_spacing;
                let ty = chart_y + link.target_y * chart_height;

                let source_total = source_totals[link.source];
                let target_total = incoming_totals[link.target];

                let link_height_source = if source_total > 0.0 {
                    (link.value / source_total) * source.height * chart_height
                } else { 0.0 };
                let link_height_target = if target_total > 0.0 {
                    (link.value / target_total) * target.height * chart_height
                } else { 0.0 };

                // Draw curved flow
                let segments = 24;
                for i in 0..segments {
                    let t1 = i as f64 / segments as f64;
                    let t2 = (i + 1) as f64 / segments as f64;

                    let ease1 = t1 * t1 * (3.0 - 2.0 * t1);
                    let ease2 = t2 * t2 * (3.0 - 2.0 * t2);

                    let x1 = sx + (tx - sx) * t1;
                    let x2 = sx + (tx - sx) * t2;
                    let y1_top = sy + (ty - sy) * ease1;
                    let y2_top = sy + (ty - sy) * ease2;

                    let h1 = link_height_source + (link_height_target - link_height_source) * ease1;
                    let h2 = link_height_source + (link_height_target - link_height_source) * ease2;

                    let t_color = t1 as f32;
                    let color = vec4(
                        source.color.x + (target.color.x - source.color.x) * t_color,
                        source.color.y + (target.color.y - source.color.y) * t_color,
                        source.color.z + (target.color.z - source.color.z) * t_color,
                        0.55,
                    );

                    self.draw_triangle.color = color;
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_top), dvec2(x2, y2_top), dvec2(x2, y2_top + h2));
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_top), dvec2(x2, y2_top + h2), dvec2(x1, y1_top + h1));
                }
            }

            // Draw nodes
            for node in &self.nodes {
                let x = chart_x + node.layer as f64 * layer_spacing;
                let y = chart_y + node.y * chart_height;
                let height = node.height * chart_height;

                self.draw_triangle.color = node.color;
                self.draw_triangle.draw_triangle(cx, dvec2(x, y), dvec2(x + node_width, y), dvec2(x + node_width, y + height));
                self.draw_triangle.draw_triangle(cx, dvec2(x, y), dvec2(x + node_width, y + height), dvec2(x, y + height));
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl SankeyDiagramRef {
    pub fn set_data(&self, nodes: Vec<SankeyNode>, links: Vec<SankeyLink>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(nodes, links); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
}
