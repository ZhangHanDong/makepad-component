//! Chart Bridge: Maps A2UI ChartComponent → makepad-plot widgets
//!
//! Bridges the gap between the A2UI protocol's ChartComponent data model
//! and the makepad-plot library's widget API. Each A2UI chart type maps
//! to a corresponding makepad-plot widget.

use makepad_widgets::*;
use makepad_plot::*;
use super::message::*;
use super::value::StringValue;
use super::data_model::DataModel;
use super::processor::resolve_string_value_scoped;


/// Get chart color from ChartComponent palette or fallback to makepad-plot default
fn get_bridge_color(chart: &ChartComponent, index: usize) -> Vec4 {
    if index < chart.colors.len() {
        if let Some(color) = parse_hex_color(&chart.colors[index]) {
            return color;
        }
    }
    makepad_plot::get_color(index)
}

/// Parse a hex color string to Vec4
fn parse_hex_color(hex: &str) -> Option<Vec4> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
    Some(Vec4 { x: r, y: g, z: b, w: 1.0 })
}

/// Resolve a chart title from StringValue
fn resolve_title(title: &Option<StringValue>, data_model: &DataModel, scope: Option<&str>) -> Option<String> {
    title.as_ref().map(|sv| resolve_string_value_scoped(sv, data_model, scope))
}

/// Parse a colormap name string to Colormap enum
fn parse_colormap(name: &str) -> Colormap {
    match name.to_lowercase().as_str() {
        "viridis" => Colormap::Viridis,
        "plasma" => Colormap::Plasma,
        "inferno" => Colormap::Inferno,
        "magma" => Colormap::Magma,
        "cividis" => Colormap::Cividis,
        "coolwarm" => Colormap::Coolwarm,
        "rdbu" => Colormap::RdBu,
        "spectral" => Colormap::Spectral,
        "blues" => Colormap::Blues,
        "greens" => Colormap::Greens,
        "oranges" => Colormap::Oranges,
        "reds" => Colormap::Reds,
        "greys" => Colormap::Greys,
        "jet" => Colormap::Jet,
        "hot" => Colormap::Hot,
        "turbo" => Colormap::Turbo,
        _ => Colormap::Viridis,
    }
}

// ============================================================================
// Line Chart Bridge
// ============================================================================

pub fn render_line(
    plot: &mut LinePlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let x = series.x_values.clone().unwrap_or_else(||
            (0..series.values.len()).map(|j| j as f64).collect());
        let mut s = Series::new(series.name.as_deref().unwrap_or(""))
            .with_data(x, series.values.clone());
        s = s.with_color(get_bridge_color(chart, i));
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    if let Some(ref xl) = chart.x_label {
        plot.set_xlabel(xl.as_str());
    }
    if let Some(ref yl) = chart.y_label {
        plot.set_ylabel(yl.as_str());
    }

    if let Some(true) = chart.show_legend {
        plot.set_legend(LegendPosition::TopRight);
    }

    if let Some(true) = chart.interactive {
        plot.set_interactive(true);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Bar Chart Bridge
// ============================================================================

pub fn render_bar(
    plot: &mut BarPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if chart.series.len() == 1 {
        // Single series → simple bar chart
        plot.set_data(chart.labels.clone(), chart.series[0].values.clone());
        plot.set_color(get_bridge_color(chart, 0));
    } else {
        // Multiple series → grouped bar chart
        let categories = chart.labels.clone();
        let groups: Vec<BarGroup> = chart.series.iter().enumerate().map(|(i, s)| {
            BarGroup::new(s.name.as_deref().unwrap_or(""), s.values.clone())
                .with_color(get_bridge_color(chart, i))
        }).collect();
        plot.set_groups(categories, groups);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    plot.set_show_bar_labels(true);

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Scatter Chart Bridge
// ============================================================================

pub fn render_scatter(
    plot: &mut ScatterPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let x = series.x_values.clone().unwrap_or_else(||
            (0..series.values.len()).map(|j| j as f64).collect());
        let mut s = Series::new(series.name.as_deref().unwrap_or(""))
            .with_data(x, series.values.clone());
        s = s.with_color(get_bridge_color(chart, i));
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    if let Some(true) = chart.show_legend {
        plot.set_legend(LegendPosition::TopRight);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Pie Chart Bridge
// ============================================================================

pub fn render_pie(
    plot: &mut PieChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if let Some(first_series) = chart.series.first() {
        let labels = &chart.labels;
        let values = &first_series.values;
        let count = labels.len().min(values.len());

        for i in 0..count {
            let mut slice = PieSlice::new(&labels[i], values[i]);
            slice = slice.with_color(get_bridge_color(chart, i));
            plot.add_slice(slice);
        }
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    plot.set_show_percentages(true);
    plot.set_show_labels(true);

    if let Some(true) = chart.show_legend {
        plot.set_legend(LegendPosition::TopRight);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Area Chart Bridge
// ============================================================================

pub fn render_area(
    plot: &mut AreaChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let x = series.x_values.clone().unwrap_or_else(||
            (0..series.values.len()).map(|j| j as f64).collect());
        let color = get_bridge_color(chart, i);
        let s = AreaSeries::new(series.name.as_deref().unwrap_or(""))
            .with_data(x, series.values.clone())
            .with_color(color);
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Radar Chart Bridge
// ============================================================================

pub fn render_radar(
    plot: &mut RadarChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();
    plot.set_axes(chart.labels.clone());

    for (i, series) in chart.series.iter().enumerate() {
        let color = get_bridge_color(chart, i);
        let s = RadarSeries::new(series.name.as_deref().unwrap_or(""), series.values.clone())
            .with_color(color);
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Gauge Chart Bridge
// ============================================================================

pub fn render_gauge(
    plot: &mut GaugeChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    // Gauge uses first series, first value as the gauge value
    if let Some(first_series) = chart.series.first() {
        if let Some(&value) = first_series.values.first() {
            plot.set_value(value);
        }
    }

    let max_val = chart.max_value.unwrap_or(100.0);
    plot.set_range(0.0, max_val);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Bubble Chart Bridge
// ============================================================================

pub fn render_bubble(
    plot: &mut BubbleChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // A2UI bubble format: series[0]=x, series[1]=y, series[2]=size
    // OR: each series has values as y, x is implicit indices, size is proportional to value
    if chart.series.len() >= 3 {
        let xs = &chart.series[0].values;
        let ys = &chart.series[1].values;
        let sizes = &chart.series[2].values;
        let count = xs.len().min(ys.len()).min(sizes.len());

        let mut bs = BubbleSeries::new(chart.series[0].name.as_deref().unwrap_or(""));
        let mut points = Vec::new();
        for i in 0..count {
            let mut p = BubblePoint::new(xs[i], ys[i], sizes[i]);
            p = p.with_color(get_bridge_color(chart, i));
            if i < chart.labels.len() {
                p = p.with_label(&chart.labels[i]);
            }
            points.push(p);
        }
        bs = bs.with_points(points);
        bs = bs.with_color(get_bridge_color(chart, 0));
        plot.add_series(bs);
    } else {
        // Fallback: each series is a bubble series
        for (si, series) in chart.series.iter().enumerate() {
            let color = get_bridge_color(chart, si);
            let mut bs = BubbleSeries::new(series.name.as_deref().unwrap_or(""));
            let points: Vec<BubblePoint> = series.values.iter().enumerate().map(|(i, &v)| {
                BubblePoint::new(i as f64, v, v.abs().sqrt().max(2.0))
            }).collect();
            bs = bs.with_points(points).with_color(color);
            plot.add_series(bs);
        }
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Candlestick Chart Bridge
// ============================================================================

pub fn render_candlestick(
    plot: &mut CandlestickChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // A2UI candlestick: series[0]=open, series[1]=high, series[2]=low, series[3]=close
    // Optional: series[4]=volume
    if chart.series.len() >= 4 {
        let opens = &chart.series[0].values;
        let highs = &chart.series[1].values;
        let lows = &chart.series[2].values;
        let closes = &chart.series[3].values;
        let count = opens.len().min(highs.len()).min(lows.len()).min(closes.len());

        let mut candles = Vec::with_capacity(count);
        for i in 0..count {
            let mut candle = Candle::new(i as f64, opens[i], highs[i], lows[i], closes[i]);
            if chart.series.len() > 4 && i < chart.series[4].values.len() {
                candle = candle.with_volume(chart.series[4].values[i]);
            }
            candles.push(candle);
        }
        plot.set_data(candles);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Heatmap Chart Bridge
// ============================================================================

pub fn render_heatmap(
    plot: &mut HeatmapChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // Each series is a row of the heatmap matrix
    let data: Vec<Vec<f64>> = chart.series.iter().map(|s| s.values.clone()).collect();
    plot.set_data(data);

    if !chart.labels.is_empty() {
        plot.set_x_labels(chart.labels.clone());
    }

    // Y labels from series names
    let y_labels: Vec<String> = chart.series.iter()
        .map(|s| s.name.as_deref().unwrap_or("").to_string())
        .collect();
    if y_labels.iter().any(|l| !l.is_empty()) {
        plot.set_y_labels(y_labels);
    }

    plot.set_show_values(false);
    plot.set_colormap(chart.colormap.as_ref().map(|cm| parse_colormap(cm)).unwrap_or(Colormap::Viridis));

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Treemap Bridge
// ============================================================================

pub fn render_treemap(
    plot: &mut Treemap,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    // Treemap: labels = node names, first series = values
    if let Some(first_series) = chart.series.first() {
        let count = chart.labels.len().min(first_series.values.len());
        let mut nodes = Vec::with_capacity(count);
        for i in 0..count {
            let mut node = TreemapNode::new(&chart.labels[i], first_series.values[i]);
            node = node.with_color(get_bridge_color(chart, i));
            nodes.push(node);
        }
        plot.set_data(nodes);
    }

    plot.set_show_labels(true);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Sankey Diagram Bridge
// ============================================================================

pub fn render_sankey(
    plot: &mut SankeyDiagram,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    // A2UI sankey: labels = node names, series = flow matrix
    // series[i].values[j] = flow from node i to node j
    let node_count = chart.labels.len();
    let mut nodes = Vec::with_capacity(node_count);
    let mut links = Vec::new();

    // Compute layers using topological ordering
    // Simple heuristic: nodes with no incoming flow are layer 0,
    // then layer = max(source_layer) + 1 for each downstream node
    let mut layers = vec![0usize; node_count];
    let mut has_incoming = vec![false; node_count];

    for (i, series) in chart.series.iter().enumerate() {
        for (j, &val) in series.values.iter().enumerate() {
            if val > 0.0 && i != j && j < node_count {
                has_incoming[j] = true;
            }
        }
    }

    // Simple BFS layering
    for _pass in 0..node_count {
        for (i, series) in chart.series.iter().enumerate() {
            for (j, &val) in series.values.iter().enumerate() {
                if val > 0.0 && i != j && j < node_count {
                    if layers[j] <= layers[i] {
                        layers[j] = layers[i] + 1;
                    }
                }
            }
        }
    }

    // Create nodes with auto-calculated values
    for i in 0..node_count {
        let mut value = 0.0f64;
        // Calculate outgoing
        if i < chart.series.len() {
            for &v in &chart.series[i].values {
                if v > 0.0 { value += v; }
            }
        }
        // Calculate incoming
        let mut incoming = 0.0;
        for (src, series) in chart.series.iter().enumerate() {
            if src < node_count && i < series.values.len() && series.values[i] > 0.0 && src != i {
                incoming += series.values[i];
            }
        }
        value = value.max(incoming).max(1.0);

        let color = get_bridge_color(chart, i);
        nodes.push(SankeyNode::new(&chart.labels[i], layers[i], value, color));
    }

    // Create links
    for (i, series) in chart.series.iter().enumerate() {
        if i >= node_count { break; }
        for (j, &val) in series.values.iter().enumerate() {
            if val > 0.0 && i != j && j < node_count {
                links.push(SankeyLink::new(i, j, val));
            }
        }
    }

    plot.set_data(nodes, links);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Histogram Bridge
// ============================================================================

pub fn render_histogram(
    plot: &mut HistogramChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if let Some(first_series) = chart.series.first() {
        plot.set_values(first_series.values.clone());
        plot.set_color(get_bridge_color(chart, 0));
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Box Plot Bridge
// ============================================================================

pub fn render_boxplot(
    plot: &mut BoxPlotChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let label = series.name.as_deref()
            .or(chart.labels.get(i).map(|s| s.as_str()))
            .unwrap_or("");
        plot.add_from_values(label, &series.values);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Donut Chart Bridge
// ============================================================================

pub fn render_donut(
    plot: &mut DonutChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if let Some(first_series) = chart.series.first() {
        let count = chart.labels.len().min(first_series.values.len());
        let mut slices = Vec::with_capacity(count);
        for i in 0..count {
            let slice = DonutSlice::new(&chart.labels[i], first_series.values[i])
                .with_color(get_bridge_color(chart, i));
            slices.push(slice);
        }
        plot.set_data(slices);
    }

    plot.set_show_percentages(true);
    plot.set_show_labels(true);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Stem Plot Bridge
// ============================================================================

pub fn render_stem(
    plot: &mut StemPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let x = series.x_values.clone().unwrap_or_else(||
            (0..series.values.len()).map(|j| j as f64).collect());
        let s = Series::new(series.name.as_deref().unwrap_or(""))
            .with_data(x, series.values.clone())
            .with_color(get_bridge_color(chart, i));
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Violin Plot Bridge
// ============================================================================

pub fn render_violin(
    plot: &mut ViolinPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let label = series.name.as_deref()
            .or(chart.labels.get(i).map(|s| s.as_str()))
            .unwrap_or("");
        plot.add_from_values(label, &series.values);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Polar Plot Bridge
// ============================================================================

pub fn render_polar(
    plot: &mut PolarPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // Polar: series[0] = theta (angles in radians), series[1] = r (radii)
    if chart.series.len() >= 2 {
        let theta = chart.series[0].values.clone();
        let r = chart.series[1].values.clone();
        let s = PolarSeries::new(chart.series[0].name.as_deref().unwrap_or(""))
            .with_data(theta, r)
            .with_color(get_bridge_color(chart, 0));
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Contour Plot Bridge
// ============================================================================

pub fn render_contour(
    plot: &mut ContourPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    let data: Vec<Vec<f64>> = chart.series.iter().map(|s| s.values.clone()).collect();
    plot.set_data(data);
    plot.set_filled(true);

    // Use x_values from first series as x_range hint, labels[0..2] as y_range hint
    if let Some(first) = chart.series.first() {
        if let Some(ref xv) = first.x_values {
            if xv.len() >= 2 {
                plot.set_x_range(xv[0], xv[xv.len() - 1]);
            }
        }
    }
    // Use labels as range hints: labels[0]=x_min, labels[1]=x_max, labels[2]=y_min, labels[3]=y_max
    if chart.labels.len() >= 4 {
        if let (Ok(xmin), Ok(xmax), Ok(ymin), Ok(ymax)) = (
            chart.labels[0].parse::<f64>(),
            chart.labels[1].parse::<f64>(),
            chart.labels[2].parse::<f64>(),
            chart.labels[3].parse::<f64>(),
        ) {
            plot.set_x_range(xmin, xmax);
            plot.set_y_range(ymin, ymax);
        }
    }

    if let Some(ref cm) = chart.colormap {
        plot.set_colormap(parse_colormap(cm));
    }

    // Scale contour levels based on grid resolution for better detail
    let grid_size = chart.series.len();
    if grid_size >= 100 {
        plot.set_n_levels(25);
    } else if grid_size >= 50 {
        plot.set_n_levels(15);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Waterfall Chart Bridge
// ============================================================================

pub fn render_waterfall(
    plot: &mut WaterfallChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if let Some(first_series) = chart.series.first() {
        let count = chart.labels.len().min(first_series.values.len());
        let mut entries = Vec::with_capacity(count);
        for i in 0..count {
            entries.push(WaterfallEntry::new(&chart.labels[i], first_series.values[i]));
        }
        plot.set_data(entries);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Funnel Chart Bridge
// ============================================================================

pub fn render_funnel(
    plot: &mut FunnelChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    if let Some(first_series) = chart.series.first() {
        let count = chart.labels.len().min(first_series.values.len());
        let mut stages = Vec::with_capacity(count);
        for i in 0..count {
            let stage = FunnelStage::new(&chart.labels[i], first_series.values[i])
                .with_color(get_bridge_color(chart, i));
            stages.push(stage);
        }
        plot.set_data(stages);
    }

    plot.set_show_percentages(true);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Step Plot Bridge
// ============================================================================

pub fn render_step(
    plot: &mut StepPlot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    for (i, series) in chart.series.iter().enumerate() {
        let x = series.x_values.clone().unwrap_or_else(||
            (0..series.values.len()).map(|j| j as f64).collect());
        let s = StepSeries::new(series.name.as_deref().unwrap_or(""))
            .with_data(x, series.values.clone())
            .with_color(get_bridge_color(chart, i));
        plot.add_series(s);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Stackplot Bridge
// ============================================================================

pub fn render_stackplot(
    plot: &mut Stackplot,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    let series: Vec<StackSeries> = chart.series.iter().enumerate().map(|(i, s)| {
        StackSeries::new(s.name.as_deref().unwrap_or(""), s.values.clone())
            .with_color(get_bridge_color(chart, i))
    }).collect();

    plot.set_data(series, chart.labels.clone());

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Hexbin Chart Bridge
// ============================================================================

pub fn render_hexbin(
    plot: &mut HexbinChart,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    // Hexbin: series[0] = x values, series[1] = y values
    if chart.series.len() >= 2 {
        let xs = &chart.series[0].values;
        let ys = &chart.series[1].values;
        let count = xs.len().min(ys.len());
        let points: Vec<HexbinPoint> = (0..count)
            .map(|i| HexbinPoint { x: xs[i], y: ys[i] })
            .collect();
        plot.set_data(points);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// Streamgraph Bridge
// ============================================================================

pub fn render_streamgraph(
    plot: &mut Streamgraph,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    let series: Vec<StreamSeries> = chart.series.iter().enumerate().map(|(i, s)| {
        StreamSeries::new(s.name.as_deref().unwrap_or(""), s.values.clone())
            .with_color(get_bridge_color(chart, i))
    }).collect();

    plot.set_data(series, chart.labels.clone());

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// 3D Surface Plot Bridge
// ============================================================================

pub fn render_surface3d(
    plot: &mut Surface3D,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    log!("[render_surface3d] Rendering 3D surface with {} series, size {}x{}",
         chart.series.len(), chart.width, chart.height);
    plot.clear();

    // Convert series to 2D grid: each series is a row of z-values
    let z_data: Vec<Vec<f64>> = chart.series.iter().map(|s| s.values.clone()).collect();
    plot.set_data(z_data);

    // Set ranges from labels if provided: [x_min, x_max, y_min, y_max]
    if chart.labels.len() >= 4 {
        if let (Ok(xmin), Ok(xmax), Ok(ymin), Ok(ymax)) = (
            chart.labels[0].parse::<f64>(),
            chart.labels[1].parse::<f64>(),
            chart.labels[2].parse::<f64>(),
            chart.labels[3].parse::<f64>(),
        ) {
            plot.set_x_range(xmin, xmax);
            plot.set_y_range(ymin, ymax);
        }
    }

    if let Some(ref cm) = chart.colormap {
        plot.set_colormap(parse_colormap(cm));
    }

    // Show surface with wireframe overlay for nice look
    plot.set_surface(true);
    plot.set_wireframe(true);

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// 3D Scatter Plot Bridge
// ============================================================================

pub fn render_scatter3d(
    plot: &mut Scatter3D,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // Expect 3 series: x, y, z coordinates
    if chart.series.len() >= 3 {
        let x = chart.series[0].values.clone();
        let y = chart.series[1].values.clone();
        let z = chart.series[2].values.clone();
        plot.set_data(x, y, z);
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}

// ============================================================================
// 3D Line Plot Bridge
// ============================================================================

pub fn render_line3d(
    plot: &mut Line3D,
    cx: &mut Cx2d,
    scope: &mut Scope,
    chart: &ChartComponent,
    data_model: &DataModel,
    current_scope: Option<&str>,
) {
    plot.clear();

    // Each 3 consecutive series form a line: x, y, z
    let mut i = 0;
    let mut series_idx = 0;
    while i + 2 < chart.series.len() {
        let x = chart.series[i].values.clone();
        let y = chart.series[i + 1].values.clone();
        let z = chart.series[i + 2].values.clone();

        let name = chart.series[i].name.as_deref().unwrap_or("");
        let color = get_bridge_color(chart, series_idx);
        plot.add_series(Line3DSeries::new(name).with_data(x, y, z).with_color(color));

        i += 3;
        series_idx += 1;
    }

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        plot.set_title(title);
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_walk(cx, scope, walk);
}
