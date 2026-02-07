//! Math Charts Demo - Famous High-Order Functions
//!
//! Generates A2UI JSON showcasing famous mathematical functions rendered
//! as 2D and 3D charts via the makepad-plot integration.
//!
//! Usage:
//!   cargo run -p a2ui-demo --bin math-charts
//!
//! This writes `math_test.json` and copies to `ui_live.json`.
//! Then run the watch-server + demo app to see the charts.

use serde_json::{json, Value};
use std::f64::consts::PI;

fn main() {
    let messages = generate_math_ui();
    let json_str = serde_json::to_string_pretty(&messages).unwrap();

    // Write to math_test.json
    std::fs::write("math_test.json", &json_str).unwrap();
    println!("Written math_test.json ({} bytes)", json_str.len());

    // Also copy to ui_live.json for live preview
    std::fs::write("ui_live.json", &json_str).unwrap();
    println!("Written ui_live.json ({} bytes)", json_str.len());
    println!("\nTo view: run watch-server + a2ui-demo, or just run a2ui-demo and click 'Live Editor'");
}

fn generate_math_ui() -> Vec<Value> {
    let mut messages = Vec::new();

    // 1. Begin rendering
    messages.push(json!({
        "beginRendering": {
            "surfaceId": "main",
            "root": "root",
            "styles": {
                "primaryColor": "#1E88E5",
                "font": "Roboto"
            }
        }
    }));

    // 2. Build component tree
    let mut components = Vec::new();

    // Root layout
    components.push(json!({
        "id": "root",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "title",
                    "subtitle",
                    "row-1",
                    "row-2",
                    "row-3"
                ]}
            }
        }
    }));

    // Title
    components.push(json!({
        "id": "title",
        "component": {
            "Text": {
                "text": { "literalString": "Famous Mathematical Functions" },
                "usageHint": "h1"
            }
        }
    }));

    components.push(json!({
        "id": "subtitle",
        "component": {
            "Text": {
                "text": { "literalString": "Rendered via A2UI Protocol → makepad-plot" },
                "usageHint": "body"
            }
        }
    }));

    // =========================================================================
    // Row 1: Chebyshev Polynomials (Line) + Fourier Square Wave (Line)
    // =========================================================================
    components.push(json!({
        "id": "row-1",
        "component": {
            "Row": {
                "children": { "explicitList": ["chebyshev-col", "fourier-col"] }
            }
        }
    }));

    // -- Chebyshev column --
    components.push(json!({
        "id": "chebyshev-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "cheb-title", "cheb-formula", "cheb-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "cheb-title",
        "component": {
            "Text": {
                "text": { "literalString": "Chebyshev Polynomials of the First Kind" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "cheb-formula",
        "component": {
            "Text": {
                "text": { "literalString": "T\u{2099}(cos\u{03B8}) = cos(n\u{03B8})  |  T\u{2080}=1, T\u{2081}=x, T\u{2082}=2x\u{00B2}\u{2212}1, T\u{2083}=4x\u{00B3}\u{2212}3x, T\u{2084}=8x\u{2074}\u{2212}8x\u{00B2}+1" },
                "usageHint": "body"
            }
        }
    }));

    // Compute Chebyshev T0-T4 over [-1, 1]
    let n_points = 201;
    let x_cheb: Vec<f64> = (0..n_points).map(|i| -1.0 + 2.0 * i as f64 / (n_points - 1) as f64).collect();

    let t0: Vec<f64> = x_cheb.iter().map(|_| 1.0).collect();
    let t1: Vec<f64> = x_cheb.iter().map(|&x| x).collect();
    let t2: Vec<f64> = x_cheb.iter().map(|&x| 2.0 * x * x - 1.0).collect();
    let t3: Vec<f64> = x_cheb.iter().map(|&x| 4.0 * x * x * x - 3.0 * x).collect();
    let t4: Vec<f64> = x_cheb.iter().map(|&x| {
        let x2 = x * x;
        8.0 * x2 * x2 - 8.0 * x2 + 1.0
    }).collect();

    components.push(json!({
        "id": "cheb-chart",
        "component": {
            "Chart": {
                "chartType": "line",
                "title": { "literalString": "Chebyshev Polynomials T\u{2080}\u{2013}T\u{2084}" },
                "width": 560.0,
                "height": 380.0,
                "showLegend": true,
                "interactive": true,
                "xLabel": "x",
                "yLabel": "T\u{2099}(x)",
                "colors": ["#1E88E5", "#43A047", "#E53935", "#FB8C00", "#8E24AA"],
                "series": [
                    { "name": "T\u{2080}(x) = 1", "values": t0, "xValues": x_cheb.clone() },
                    { "name": "T\u{2081}(x) = x", "values": t1, "xValues": x_cheb.clone() },
                    { "name": "T\u{2082}(x) = 2x\u{00B2}\u{2212}1", "values": t2, "xValues": x_cheb.clone() },
                    { "name": "T\u{2083}(x) = 4x\u{00B3}\u{2212}3x", "values": t3, "xValues": x_cheb.clone() },
                    { "name": "T\u{2084}(x) = 8x\u{2074}\u{2212}8x\u{00B2}+1", "values": t4, "xValues": x_cheb.clone() },
                ],
                "labels": []
            }
        }
    }));

    // -- Fourier column --
    components.push(json!({
        "id": "fourier-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "fourier-title", "fourier-formula", "fourier-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "fourier-title",
        "component": {
            "Text": {
                "text": { "literalString": "Fourier Series: Square Wave Approximation" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "fourier-formula",
        "component": {
            "Text": {
                "text": { "literalString": "f(x) = (4/\u{03C0}) \u{2211}\u{2099}\u{208C}\u{2081}\u{207E}\u{221E} sin((2n\u{2212}1)x) / (2n\u{2212}1)" },
                "usageHint": "body"
            }
        }
    }));

    // Compute Fourier square wave with 1, 3, 7, 15 harmonics
    let n_pts = 401;
    let x_four: Vec<f64> = (0..n_pts).map(|i| -PI + 2.0 * PI * i as f64 / (n_pts - 1) as f64).collect();

    let fourier_sum = |x: f64, n_terms: usize| -> f64 {
        let mut s = 0.0;
        for k in 0..n_terms {
            let n = (2 * k + 1) as f64;
            s += (n * x).sin() / n;
        }
        s * 4.0 / PI
    };

    let f1: Vec<f64> = x_four.iter().map(|&x| fourier_sum(x, 1)).collect();
    let f3: Vec<f64> = x_four.iter().map(|&x| fourier_sum(x, 3)).collect();
    let f7: Vec<f64> = x_four.iter().map(|&x| fourier_sum(x, 7)).collect();
    let f15: Vec<f64> = x_four.iter().map(|&x| fourier_sum(x, 15)).collect();
    // The target square wave
    let sq: Vec<f64> = x_four.iter().map(|&x| if x.sin() >= 0.0 { 1.0 } else { -1.0 }).collect();

    components.push(json!({
        "id": "fourier-chart",
        "component": {
            "Chart": {
                "chartType": "line",
                "title": { "literalString": "Fourier Square Wave Approximation" },
                "width": 560.0,
                "height": 380.0,
                "showLegend": true,
                "interactive": true,
                "xLabel": "x",
                "yLabel": "f(x)",
                "colors": ["#90A4AE", "#E53935", "#FB8C00", "#43A047", "#1E88E5"],
                "series": [
                    { "name": "Square Wave", "values": sq, "xValues": x_four.clone() },
                    { "name": "N=1", "values": f1, "xValues": x_four.clone() },
                    { "name": "N=3", "values": f3, "xValues": x_four.clone() },
                    { "name": "N=7", "values": f7, "xValues": x_four.clone() },
                    { "name": "N=15", "values": f15, "xValues": x_four.clone() },
                ],
                "labels": []
            }
        }
    }));

    // =========================================================================
    // Row 2: Rosenbrock Contour + Himmelblau Heatmap (3D-like visualization)
    // =========================================================================
    components.push(json!({
        "id": "row-2",
        "component": {
            "Row": {
                "children": { "explicitList": ["rosenbrock-col", "himmelblau-col"] }
            }
        }
    }));

    // -- Rosenbrock Contour --
    components.push(json!({
        "id": "rosenbrock-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "rosen-title", "rosen-formula", "rosen-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "rosen-title",
        "component": {
            "Text": {
                "text": { "literalString": "Rosenbrock Function (Banana Function)" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "rosen-formula",
        "component": {
            "Text": {
                "text": { "literalString": "f(x,y) = (1\u{2212}x)\u{00B2} + 100(y\u{2212}x\u{00B2})\u{00B2}   |   Global minimum at (1,1)" },
                "usageHint": "body"
            }
        }
    }));

    // Compute Rosenbrock on grid, log-scaled for visibility
    let grid_size = 50;
    let x_min = -2.0_f64;
    let x_max = 2.0_f64;
    let y_min = -1.0_f64;
    let y_max = 3.0_f64;

    let mut rosen_grid: Vec<Value> = Vec::new();
    for j in 0..grid_size {
        let y = y_min + (y_max - y_min) * j as f64 / (grid_size - 1) as f64;
        let row: Vec<f64> = (0..grid_size).map(|i| {
            let x = x_min + (x_max - x_min) * i as f64 / (grid_size - 1) as f64;
            let f = (1.0 - x).powi(2) + 100.0 * (y - x * x).powi(2);
            (1.0 + f).ln() // log scale for better visualization
        }).collect();
        rosen_grid.push(json!({
            "values": row
        }));
    }

    components.push(json!({
        "id": "rosen-chart",
        "component": {
            "Chart": {
                "chartType": "contour",
                "title": { "literalString": "Rosenbrock: ln(1 + f(x,y))  |  Contour Plot" },
                "width": 560.0,
                "height": 420.0,
                "colormap": "plasma",
                "labels": ["-2", "2", "-1", "3"],
                "series": rosen_grid,
            }
        }
    }));

    // -- Himmelblau Heatmap --
    components.push(json!({
        "id": "himmelblau-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "himmel-title", "himmel-formula", "himmel-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "himmel-title",
        "component": {
            "Text": {
                "text": { "literalString": "Himmelblau's Function" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "himmel-formula",
        "component": {
            "Text": {
                "text": { "literalString": "f(x,y) = (x\u{00B2}+y\u{2212}11)\u{00B2} + (x+y\u{00B2}\u{2212}7)\u{00B2}   |   Four identical local minima" },
                "usageHint": "body"
            }
        }
    }));

    let hgrid = 40;
    let hrange = 5.0_f64;
    let mut himmel_series: Vec<Value> = Vec::new();
    for j in 0..hgrid {
        let y = -hrange + 2.0 * hrange * j as f64 / (hgrid - 1) as f64;
        let row: Vec<f64> = (0..hgrid).map(|i| {
            let x = -hrange + 2.0 * hrange * i as f64 / (hgrid - 1) as f64;
            let f = (x * x + y - 11.0).powi(2) + (x + y * y - 7.0).powi(2);
            (1.0 + f).ln()
        }).collect();
        himmel_series.push(json!({
            "name": format!("y={:.1}", y),
            "values": row
        }));
    }

    components.push(json!({
        "id": "himmel-chart",
        "component": {
            "Chart": {
                "chartType": "heatmap",
                "title": { "literalString": "Himmelblau: ln(1 + f(x,y))  |  Heatmap" },
                "width": 560.0,
                "height": 420.0,
                "colormap": "inferno",
                "series": himmel_series,
                "labels": []
            }
        }
    }));

    // =========================================================================
    // Row 3: Legendre Polynomials (Area) + Rastrigin Contour
    // =========================================================================
    components.push(json!({
        "id": "row-3",
        "component": {
            "Row": {
                "children": { "explicitList": ["legendre-col", "rastrigin-col"] }
            }
        }
    }));

    // -- Legendre Polynomials --
    components.push(json!({
        "id": "legendre-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "leg-title", "leg-formula", "leg-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "leg-title",
        "component": {
            "Text": {
                "text": { "literalString": "Legendre Polynomials" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "leg-formula",
        "component": {
            "Text": {
                "text": { "literalString": "P\u{2080}=1, P\u{2081}=x, P\u{2082}=\u{00BD}(3x\u{00B2}\u{2212}1), P\u{2083}=\u{00BD}(5x\u{00B3}\u{2212}3x), P\u{2084}=\u{215B}(35x\u{2074}\u{2212}30x\u{00B2}+3)" },
                "usageHint": "body"
            }
        }
    }));

    let x_leg: Vec<f64> = (0..n_points).map(|i| -1.0 + 2.0 * i as f64 / (n_points - 1) as f64).collect();
    let p0: Vec<f64> = x_leg.iter().map(|_| 1.0).collect();
    let p1: Vec<f64> = x_leg.iter().map(|&x| x).collect();
    let p2: Vec<f64> = x_leg.iter().map(|&x| 0.5 * (3.0 * x * x - 1.0)).collect();
    let p3: Vec<f64> = x_leg.iter().map(|&x| 0.5 * (5.0 * x * x * x - 3.0 * x)).collect();
    let p4: Vec<f64> = x_leg.iter().map(|&x| {
        let x2 = x * x;
        0.125 * (35.0 * x2 * x2 - 30.0 * x2 + 3.0)
    }).collect();

    components.push(json!({
        "id": "leg-chart",
        "component": {
            "Chart": {
                "chartType": "line",
                "title": { "literalString": "Legendre Polynomials P\u{2080}\u{2013}P\u{2084}" },
                "width": 560.0,
                "height": 380.0,
                "showLegend": true,
                "interactive": true,
                "xLabel": "x",
                "yLabel": "P\u{2099}(x)",
                "colors": ["#1E88E5", "#43A047", "#E53935", "#FB8C00", "#8E24AA"],
                "series": [
                    { "name": "P\u{2080}(x)", "values": p0, "xValues": x_leg.clone() },
                    { "name": "P\u{2081}(x)", "values": p1, "xValues": x_leg.clone() },
                    { "name": "P\u{2082}(x)", "values": p2, "xValues": x_leg.clone() },
                    { "name": "P\u{2083}(x)", "values": p3, "xValues": x_leg.clone() },
                    { "name": "P\u{2084}(x)", "values": p4, "xValues": x_leg.clone() },
                ],
                "labels": []
            }
        }
    }));

    // -- Rastrigin Function Contour --
    components.push(json!({
        "id": "rastrigin-col",
        "component": {
            "Column": {
                "children": { "explicitList": [
                    "rast-title", "rast-formula", "rast-chart"
                ]}
            }
        }
    }));

    components.push(json!({
        "id": "rast-title",
        "component": {
            "Text": {
                "text": { "literalString": "Rastrigin Function (Multimodal Optimization)" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "rast-formula",
        "component": {
            "Text": {
                "text": { "literalString": "f(x,y) = 20 + (x\u{00B2} \u{2212} 10cos(2\u{03C0}x)) + (y\u{00B2} \u{2212} 10cos(2\u{03C0}y))   |   Global min at (0,0)" },
                "usageHint": "body"
            }
        }
    }));

    let rgrid = 120;
    let rrange = 5.12_f64;
    let mut rast_series: Vec<Value> = Vec::new();
    for j in 0..rgrid {
        let y = -rrange + 2.0 * rrange * j as f64 / (rgrid - 1) as f64;
        let row: Vec<f64> = (0..rgrid).map(|i| {
            let x = -rrange + 2.0 * rrange * i as f64 / (rgrid - 1) as f64;
            20.0 + (x * x - 10.0 * (2.0 * PI * x).cos()) + (y * y - 10.0 * (2.0 * PI * y).cos())
        }).collect();
        rast_series.push(json!({
            "values": row
        }));
    }

    components.push(json!({
        "id": "rast-chart",
        "component": {
            "Chart": {
                "chartType": "contour",
                "title": { "literalString": "Rastrigin f(x,y)  |  Filled Contour" },
                "width": 560.0,
                "height": 420.0,
                "colormap": "turbo",
                "labels": ["-5.12", "5.12", "-5.12", "5.12"],
                "series": rast_series,
            }
        }
    }));

    // Package the surface update
    messages.push(json!({
        "surfaceUpdate": {
            "surfaceId": "main",
            "components": components
        }
    }));

    messages
}
