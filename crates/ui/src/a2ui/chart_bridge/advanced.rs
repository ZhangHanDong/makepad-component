use makepad_widgets::*;
use makepad_plot::*;
use crate::a2ui::message::*;
use crate::a2ui::data_model::DataModel;
use crate::a2ui::processor::resolve_string_value_scoped;

use super::{get_bridge_color, resolve_title, parse_colormap};

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
    chart_id: &str,
) {
    log!("[render_surface3d] Rendering 3D surface '{}' with {} series, size {}x{}",
         chart_id, chart.series.len(), chart.width, chart.height);

    // Use per-chart instance state (preserves view angles/zoom across redraws)
    let instance = plot.get_chart_mut(chart_id);

    // Update data (but preserve interactive state like view3d, zoom)
    let z_data: Vec<Vec<f64>> = chart.series.iter().map(|s| s.values.clone()).collect();
    instance.set_data(z_data);

    // Set ranges from labels if provided: [x_min, x_max, y_min, y_max]
    if chart.labels.len() >= 4 {
        if let (Ok(xmin), Ok(xmax), Ok(ymin), Ok(ymax)) = (
            chart.labels[0].parse::<f64>(),
            chart.labels[1].parse::<f64>(),
            chart.labels[2].parse::<f64>(),
            chart.labels[3].parse::<f64>(),
        ) {
            instance.x_range = (xmin, xmax);
            instance.y_range = (ymin, ymax);
        }
    }

    if let Some(ref cm) = chart.colormap {
        instance.colormap = parse_colormap(cm);
    }

    // Show surface with wireframe overlay for nice look
    instance.show_surface = true;
    instance.show_wireframe = true;

    if let Some(title) = resolve_title(&chart.title, data_model, current_scope) {
        instance.title = title;
    }

    let walk = Walk::new(Size::Fixed(chart.width), Size::Fixed(chart.height));
    let _ = plot.draw_chart_instance(cx, scope, walk, chart_id);
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
