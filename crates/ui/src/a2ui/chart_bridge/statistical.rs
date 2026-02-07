use makepad_widgets::*;
use makepad_plot::*;
use crate::a2ui::message::*;
use crate::a2ui::data_model::DataModel;
use crate::a2ui::processor::resolve_string_value_scoped;

use super::{get_bridge_color, resolve_title, parse_colormap};

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

