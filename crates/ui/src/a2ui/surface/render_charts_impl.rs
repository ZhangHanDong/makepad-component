impl A2uiSurface {
    fn render_chart(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        chart: &ChartComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let current_scope = self.current_scope.clone();
        let cs = current_scope.as_deref();
        match chart.chart_type {
            ChartType::Line => chart_bridge::render_line(&mut self.plot_line, cx, scope, chart, data_model, cs),
            ChartType::Bar => chart_bridge::render_bar(&mut self.plot_bar, cx, scope, chart, data_model, cs),
            ChartType::Scatter => chart_bridge::render_scatter(&mut self.plot_scatter, cx, scope, chart, data_model, cs),
            ChartType::Pie => chart_bridge::render_pie(&mut self.plot_pie, cx, scope, chart, data_model, cs),
            ChartType::Area => chart_bridge::render_area(&mut self.plot_area, cx, scope, chart, data_model, cs),
            ChartType::Radar => chart_bridge::render_radar(&mut self.plot_radar, cx, scope, chart, data_model, cs),
            ChartType::Gauge => chart_bridge::render_gauge(&mut self.plot_gauge, cx, scope, chart, data_model, cs),
            ChartType::Bubble => chart_bridge::render_bubble(&mut self.plot_bubble, cx, scope, chart, data_model, cs),
            ChartType::Candlestick => chart_bridge::render_candlestick(&mut self.plot_candlestick, cx, scope, chart, data_model, cs),
            ChartType::Heatmap => chart_bridge::render_heatmap(&mut self.plot_heatmap, cx, scope, chart, data_model, cs),
            ChartType::Treemap => chart_bridge::render_treemap(&mut self.plot_treemap, cx, scope, chart, data_model, cs),
            ChartType::Sankey => chart_bridge::render_sankey(&mut self.plot_sankey, cx, scope, chart, data_model, cs),
            ChartType::Chord => self.render_chord_chart(cx, chart, data_model),
            // New chart types from makepad-plot
            ChartType::Histogram => chart_bridge::render_histogram(&mut self.plot_histogram, cx, scope, chart, data_model, cs),
            ChartType::BoxPlot => chart_bridge::render_boxplot(&mut self.plot_boxplot, cx, scope, chart, data_model, cs),
            ChartType::Donut => chart_bridge::render_donut(&mut self.plot_donut, cx, scope, chart, data_model, cs),
            ChartType::Stem => chart_bridge::render_stem(&mut self.plot_stem, cx, scope, chart, data_model, cs),
            ChartType::Violin => chart_bridge::render_violin(&mut self.plot_violin, cx, scope, chart, data_model, cs),
            ChartType::Polar => chart_bridge::render_polar(&mut self.plot_polar, cx, scope, chart, data_model, cs),
            ChartType::Contour => chart_bridge::render_contour(&mut self.plot_contour, cx, scope, chart, data_model, cs),
            ChartType::Waterfall => chart_bridge::render_waterfall(&mut self.plot_waterfall, cx, scope, chart, data_model, cs),
            ChartType::Funnel => chart_bridge::render_funnel(&mut self.plot_funnel, cx, scope, chart, data_model, cs),
            ChartType::Step => chart_bridge::render_step(&mut self.plot_step, cx, scope, chart, data_model, cs),
            ChartType::Stackplot => chart_bridge::render_stackplot(&mut self.plot_stackplot, cx, scope, chart, data_model, cs),
            ChartType::Hexbin => chart_bridge::render_hexbin(&mut self.plot_hexbin, cx, scope, chart, data_model, cs),
            ChartType::Streamgraph => chart_bridge::render_streamgraph(&mut self.plot_streamgraph, cx, scope, chart, data_model, cs),
            // 3D chart types - pass component_id for per-chart state tracking
            ChartType::Surface3d => chart_bridge::render_surface3d(&mut self.plot_surface3d, cx, scope, chart, data_model, cs, component_id),
            ChartType::Scatter3d => chart_bridge::render_scatter3d(&mut self.plot_scatter3d, cx, scope, chart, data_model, cs),
            ChartType::Line3d => chart_bridge::render_line3d(&mut self.plot_line3d, cx, scope, chart, data_model, cs),
        }
    }


    // ─── Chord diagram ────────────────────────────────────────────
    // Data convention:
    //   labels: entity names (["A", "B", "C", "D"])
    //   series: flow matrix rows — series[i].values[j] = flow from i to j
    fn render_chord_chart(
        &mut self,
        cx: &mut Cx2d,
        chart: &ChartComponent,
        data_model: &DataModel,
    ) {
        let chart_width = chart.width;
        let chart_height = chart.height;

        let walk = Walk::new(Size::Fixed(chart_width), Size::Fixed(chart_height));
        cx.begin_turtle(walk, Layout::default());
        let origin = cx.turtle().pos();

        let mut title_height = 0.0;
        if let Some(ref title_val) = chart.title {
            let title = resolve_string_value_scoped(title_val, data_model, self.current_scope.as_deref());
            if !title.is_empty() {
                self.draw_chart_text.text_style.font_size = 14.0;
                self.draw_chart_text.draw_walk(cx, Walk {
                    abs_pos: Some(dvec2(origin.x + (chart_width - Self::estimate_text_width(&title, 14.0)) / 2.0, origin.y + 4.0)),
                    ..Walk::fit()
                }, Align::default(), &title);
                title_height = 24.0;
            }
        }

        let n = chart.labels.len().min(chart.series.len());
        if n < 2 { cx.end_turtle(); return; }

        // Compute row sums for arc sizing
        let mut row_sums: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            let vals = &chart.series[i].values;
            for j in 0..n.min(vals.len()) {
                row_sums[i] += vals[j];
            }
        }
        let grand_total: f64 = row_sums.iter().sum();
        if grand_total <= 0.0 { cx.end_turtle(); return; }

        let center_x = origin.x + chart_width / 2.0;
        let center_y = origin.y + title_height + (chart_height - title_height) / 2.0;
        let radius = ((chart_width.min(chart_height - title_height)) / 2.0 - 40.0).max(30.0);
        let inner_frac = 0.85; // inner_radius as fraction of outer
        let gap_angle = 0.04_f64; // radians gap between arcs
        let total_gap = gap_angle * n as f64;
        let available = std::f64::consts::TAU - total_gap;
        let pi = std::f64::consts::PI;

        // Compute arc start/end angles (starting at -PI/2 = 12 o'clock)
        let mut arc_starts: Vec<f64> = vec![0.0; n];
        let mut arc_ends: Vec<f64> = vec![0.0; n];
        let mut angle = -pi / 2.0;
        for i in 0..n {
            arc_starts[i] = angle;
            let span = (row_sums[i] / grand_total) * available;
            arc_ends[i] = angle + span;
            angle += span + gap_angle;
        }

        // Draw outer arcs using DrawA2uiArc (proper GPU arc shader)
        let arc_size = radius * 2.0 + 4.0;
        for i in 0..n {
            self.draw_chart_arc.color = self.get_chart_color(chart, i);
            self.draw_chart_arc.start_angle = arc_starts[i] as f32;
            self.draw_chart_arc.end_angle = arc_ends[i] as f32;
            self.draw_chart_arc.inner_radius = inner_frac as f32;
            self.draw_chart_arc.draw_walk(cx, Walk {
                abs_pos: Some(dvec2(center_x - radius, center_y - radius)),
                width: Size::Fixed(arc_size),
                height: Size::Fixed(arc_size),
                ..Walk::default()
            });
        }

        // Draw chord ribbons using DrawA2uiQuad (proper quadrilateral shader)
        let inner_r = radius * inner_frac;
        let mut dst_angle_cursors: Vec<f64> = arc_starts.clone();
        for i in 0..n {
            let vals = &chart.series[i].values;
            let mut src_angle_cursor = arc_starts[i];
            for j in 0..n.min(vals.len()) {
                let flow = vals[j];
                if flow <= 0.0 || i == j {
                    let span = (flow / grand_total) * available;
                    src_angle_cursor += span;
                    continue;
                }
                let src_span = (flow / grand_total) * available;
                let src_a0 = src_angle_cursor;
                let src_a1 = src_angle_cursor + src_span;
                src_angle_cursor += src_span;

                // Target position on target arc — consume from destination cursor
                let dst_a0 = dst_angle_cursors[j];
                let dst_a1 = dst_a0 + src_span;
                dst_angle_cursors[j] += src_span;

                let color = self.get_chart_color(chart, i);
                let mut fill_color = color;
                fill_color.w = 0.35;

                // Boundary curves: top (src_a0→dst_a0) and bottom (src_a1→dst_a1)
                // Both are quadratic beziers with control point at center
                let s0x = center_x + inner_r * src_a0.cos();
                let s0y = center_y + inner_r * src_a0.sin();
                let s1x = center_x + inner_r * src_a1.cos();
                let s1y = center_y + inner_r * src_a1.sin();
                let d0x = center_x + inner_r * dst_a0.cos();
                let d0y = center_y + inner_r * dst_a0.sin();
                let d1x = center_x + inner_r * dst_a1.cos();
                let d1y = center_y + inner_r * dst_a1.sin();

                // Helper: evaluate quadratic bezier at t
                #[inline]
                fn qbez(t: f64, p0: f64, p1: f64, p2: f64) -> f64 {
                    let u = 1.0 - t;
                    u * u * p0 + 2.0 * u * t * p1 + t * t * p2
                }

                // Render ribbon fill as a strip of proper quadrilaterals
                let t_steps = 64;
                let pad = 2.0;
                for step in 0..t_steps {
                    let t = step as f64 / t_steps as f64;
                    let t_next = (step + 1) as f64 / t_steps as f64;

                    let tx0 = qbez(t, s0x, center_x, d0x);
                    let ty0 = qbez(t, s0y, center_y, d0y);
                    let tx1 = qbez(t_next, s0x, center_x, d0x);
                    let ty1 = qbez(t_next, s0y, center_y, d0y);

                    let bx0 = qbez(t, s1x, center_x, d1x);
                    let by0 = qbez(t, s1y, center_y, d1y);
                    let bx1 = qbez(t_next, s1x, center_x, d1x);
                    let by1 = qbez(t_next, s1y, center_y, d1y);

                    let corners = [(tx0, ty0), (tx1, ty1), (bx1, by1), (bx0, by0)];
                    let min_x = corners.iter().map(|c| c.0).fold(f64::MAX, f64::min) - pad;
                    let max_x = corners.iter().map(|c| c.0).fold(f64::MIN, f64::max) + pad;
                    let min_y = corners.iter().map(|c| c.1).fold(f64::MAX, f64::min) - pad;
                    let max_y = corners.iter().map(|c| c.1).fold(f64::MIN, f64::max) + pad;
                    let w = (max_x - min_x).max(1.0);
                    let h = (max_y - min_y).max(1.0);

                    self.draw_chart_quad.color = fill_color;
                    self.draw_chart_quad.opacity = 0.4;
                    self.draw_chart_quad.p0x = (tx0 - min_x) as f32;
                    self.draw_chart_quad.p0y = (ty0 - min_y) as f32;
                    self.draw_chart_quad.p1x = (tx1 - min_x) as f32;
                    self.draw_chart_quad.p1y = (ty1 - min_y) as f32;
                    self.draw_chart_quad.p2x = (bx1 - min_x) as f32;
                    self.draw_chart_quad.p2y = (by1 - min_y) as f32;
                    self.draw_chart_quad.p3x = (bx0 - min_x) as f32;
                    self.draw_chart_quad.p3y = (by0 - min_y) as f32;
                    self.draw_chart_quad.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(min_x, min_y)),
                        width: Size::Fixed(w),
                        height: Size::Fixed(h),
                        ..Walk::default()
                    });
                }

                // Draw smooth anti-aliased boundary curves on top
                let border_color = Vec4 { x: color.x, y: color.y, z: color.z, w: 0.6 };
                let edge_steps = 48;
                for step in 0..edge_steps {
                    let t = step as f64 / edge_steps as f64;
                    let t_next = (step + 1) as f64 / edge_steps as f64;

                    // Top boundary line segments
                    let ax0 = qbez(t, s0x, center_x, d0x);
                    let ay0 = qbez(t, s0y, center_y, d0y);
                    let ax1 = qbez(t_next, s0x, center_x, d0x);
                    let ay1 = qbez(t_next, s0y, center_y, d0y);

                    let seg_w = ((ax1 - ax0).abs() + 2.0).max(3.0);
                    let seg_h = ((ay1 - ay0).abs() + 2.0).max(3.0);
                    let seg_x = ax0.min(ax1) - 1.0;
                    let seg_y = ay0.min(ay1) - 1.0;

                    self.draw_chart_line.color = border_color;
                    self.draw_chart_line.line_width = 0.08;
                    self.draw_chart_line.x1 = ((ax0 - seg_x) / seg_w) as f32;
                    self.draw_chart_line.y1 = ((ay0 - seg_y) / seg_h) as f32;
                    self.draw_chart_line.x2 = ((ax1 - seg_x) / seg_w) as f32;
                    self.draw_chart_line.y2 = ((ay1 - seg_y) / seg_h) as f32;
                    self.draw_chart_line.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(seg_x, seg_y)),
                        width: Size::Fixed(seg_w),
                        height: Size::Fixed(seg_h),
                        ..Walk::default()
                    });

                    // Bottom boundary line segments
                    let bx0l = qbez(t, s1x, center_x, d1x);
                    let by0l = qbez(t, s1y, center_y, d1y);
                    let bx1l = qbez(t_next, s1x, center_x, d1x);
                    let by1l = qbez(t_next, s1y, center_y, d1y);

                    let seg_w2 = ((bx1l - bx0l).abs() + 2.0).max(3.0);
                    let seg_h2 = ((by1l - by0l).abs() + 2.0).max(3.0);
                    let seg_x2 = bx0l.min(bx1l) - 1.0;
                    let seg_y2 = by0l.min(by1l) - 1.0;

                    self.draw_chart_line.x1 = ((bx0l - seg_x2) / seg_w2) as f32;
                    self.draw_chart_line.y1 = ((by0l - seg_y2) / seg_h2) as f32;
                    self.draw_chart_line.x2 = ((bx1l - seg_x2) / seg_w2) as f32;
                    self.draw_chart_line.y2 = ((by1l - seg_y2) / seg_h2) as f32;
                    self.draw_chart_line.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(seg_x2, seg_y2)),
                        width: Size::Fixed(seg_w2),
                        height: Size::Fixed(seg_h2),
                        ..Walk::default()
                    });
                }
            }
        }

        // Draw entity labels around the outside
        for i in 0..n {
            let mid_angle = (arc_starts[i] + arc_ends[i]) / 2.0;
            let label_r = radius + 18.0;
            let lx = center_x + label_r * mid_angle.cos();
            let ly = center_y + label_r * mid_angle.sin();
            let label = &chart.labels[i];
            self.draw_chart_text.text_style.font_size = 10.0;
            self.draw_chart_text.draw_walk(cx, Walk {
                abs_pos: Some(dvec2(lx - Self::estimate_text_width(label, 10.0) / 2.0, ly - 5.0)),
                ..Walk::fit()
            }, Align::default(), label);
        }

        cx.end_turtle();
    }

    // AudioPlayer Rendering
    // ============================================================================

    fn render_audio_player(
        &mut self,
        cx: &mut Cx2d,
        audio_player: &AudioPlayerComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let audio_player_idx = self.audio_player_data.len();
        let is_hovered = self.hovered_audio_player_idx == Some(audio_player_idx);

        // Resolve URL and title
        let url = resolve_string_value_scoped(
            &audio_player.url,
            data_model,
            self.current_scope.as_deref(),
        );

        // Check if this audio component is currently playing
        let is_playing = self.playing_component_id.as_ref().map(|s| s.as_str()) == Some(component_id);

        let title = audio_player
            .title
            .as_ref()
            .map(|t| resolve_string_value_scoped(t, data_model, self.current_scope.as_deref()))
            .unwrap_or_else(|| "Audio".to_string());

        let artist = audio_player
            .artist
            .as_ref()
            .map(|a| resolve_string_value_scoped(a, data_model, self.current_scope.as_deref()));

        // Check if we're already inside a Card - avoid nested card backgrounds
        let already_in_card = self.inside_card;

        // Only create card background if not already inside a card
        if !already_in_card {
            let walk = Walk {
                width: Size::fill(),
                height: Size::fit(),
                margin: Margin { top: 8.0, bottom: 8.0, left: 0.0, right: 0.0 },
                ..Walk::default()
            };
            let layout = Layout {
                flow: Flow::Down,
                padding: Padding {
                    left: 16.0,
                    right: 16.0,
                    top: 12.0,
                    bottom: 12.0,
                },
                spacing: 8.0,
                ..Layout::default()
            };
            self.draw_card.begin(cx, walk, layout);
            self.inside_card = true;
        } else {
            // When inside a card, create a container for audio player content
            let walk = Walk::fill_fit();
            let layout = Layout {
                flow: Flow::Down,
                spacing: 8.0,
                ..Layout::default()
            };
            cx.begin_turtle(walk, layout);
        }

        // Title row with audio bars
        let title_walk = Walk::fill_fit();
        let title_layout = Layout {
            flow: Flow::right(),
            spacing: 8.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };
        cx.begin_turtle(title_walk, title_layout);

        // Music icon (🎵)
        self.draw_card_text.text_style.font_size = 20.0;
        self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), "🎵");

        // Title and artist column (flexible width, not fill)
        let info_walk = Walk::fit();
        let info_layout = Layout {
            flow: Flow::Down,
            spacing: 2.0,
            ..Layout::default()
        };
        cx.begin_turtle(info_walk, info_layout);

        // Title
        self.draw_card_text.text_style.font_size = 16.0;
        self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), &title);

        // Artist (if present)
        if let Some(artist_name) = &artist {
            self.draw_card_text.text_style.font_size = 12.0;
            self.draw_card_text.color = vec4(0.6, 0.6, 0.6, 1.0);
            self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), artist_name);
            self.draw_card_text.color = vec4(1.0, 1.0, 1.0, 1.0); // Reset color
        }

        cx.end_turtle();

        // Audio bars visualization (placed right after title/artist)
        let bars_walk = Walk {
            width: Size::Fixed(50.0),
            height: Size::Fixed(35.0),
            margin: Margin { left: 16.0, ..Margin::default() },
            ..Walk::default()
        };
        self.draw_audio_bars.is_playing = if is_playing { 1.0 } else { 0.0 };
        self.draw_audio_bars.draw_walk(cx, bars_walk);

        // Request next frame for continuous animation when playing
        if is_playing {
            cx.new_next_frame();
        }

        cx.end_turtle();

        // Play button
        let button_walk = Walk::fit();
        let button_layout = Layout {
            padding: Padding {
                left: 20.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        // Button colors - different for play vs stop
        let (base_color, hover_color, button_text) = if is_playing {
            (
                vec4(0.9, 0.3, 0.3, 1.0),    // Red for stop
                vec4(0.8, 0.2, 0.2, 1.0),    // Darker red
                "⏹ Stop"
            )
        } else {
            (
                vec4(0.231, 0.51, 0.965, 1.0),   // Blue for play
                vec4(0.145, 0.388, 0.922, 1.0), // Darker blue
                "▶ Play"
            )
        };
        let color = if is_hovered { hover_color } else { base_color };

        self.draw_button.color = color;
        self.draw_button.begin(cx, button_walk, button_layout);

        // Play/Stop button text
        self.draw_button_text.text_style.font_size = 14.0;
        self.draw_button_text.draw_walk(cx, Walk::fit(), Align::default(), button_text);

        self.draw_button.end(cx);

        // Use draw_button's area directly for hit testing
        let button_area = self.draw_button.area();

        // Update or create Area for this audio player button
        if audio_player_idx < self.audio_player_areas.len() {
            self.audio_player_areas[audio_player_idx] = button_area;
        } else {
            self.audio_player_areas.push(button_area);
        }

        // Only end card if we started it
        if !already_in_card {
            self.inside_card = false;
            self.draw_card.end(cx);
        } else {
            // End the container turtle we created
            cx.end_turtle();
        }

        // Store metadata
        self.audio_player_data.push((
            component_id.to_string(),
            url.clone(),
            title.clone(),
        ));

        // Debug: check area rect
        let rect = button_area.rect(cx);
        log!("[render_audio_player] idx={}, id={}, rect=({:.0},{:.0} {:.0}x{:.0})",
             audio_player_idx, component_id, rect.pos.x, rect.pos.y, rect.size.x, rect.size.y);
    }
}
