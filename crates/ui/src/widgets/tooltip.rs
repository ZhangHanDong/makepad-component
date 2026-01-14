use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    TOOLTIP_BG = #1f2937
    TOOLTIP_BORDER = #374151
    TOOLTIP_TEXT = #f9fafb

    // The popup that appears on hover
    MpTooltipPopup = <View> {
        width: Fit,
        height: Fit,
        padding: { left: 8, right: 8, top: 5, bottom: 5 }
        show_bg: true,

        draw_bg: {
            instance bg_color: (TOOLTIP_BG)
            instance border_color: (TOOLTIP_BORDER)
            instance radius: 6.0
            instance arrow_dir: 0.0
            instance arrow_size: vec2(12.0, 6.0)
            instance arrow_pos: vec2(0.0, 0.0)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let border = 1.0;
                let overlap = 2.0; // Overlap to avoid gaps between box and arrow

                let arrow_depth = self.arrow_size.y;
                let arrow_width = self.arrow_size.x;
                let arrow_half_w = arrow_width * 0.5;

                let mut box_pos = vec2(border, border);
                let mut box_size = self.rect_size - vec2(border * 2.0, border * 2.0);

                // Adjust box size to make room for arrow
                if self.arrow_dir < 0.5 {
                    // Top: arrow at bottom
                    box_size.y -= arrow_depth;
                } else if self.arrow_dir < 1.5 {
                    // Bottom: arrow at top
                    box_pos.y += arrow_depth;
                    box_size.y -= arrow_depth;
                } else if self.arrow_dir < 2.5 {
                    // Left: arrow at right
                    box_size.x -= arrow_depth;
                } else {
                    // Right: arrow at left
                    box_pos.x += arrow_depth;
                    box_size.x -= arrow_depth;
                }

                // Calculate arrow position bounds (keep arrow away from rounded corners)
                let min_x = box_pos.x + self.radius + arrow_half_w;
                let max_x = box_pos.x + box_size.x - self.radius - arrow_half_w;
                let min_y = box_pos.y + self.radius + arrow_half_w;
                let max_y = box_pos.y + box_size.y - self.radius - arrow_half_w;

                // Draw the rounded box
                sdf.box(box_pos.x, box_pos.y, box_size.x, box_size.y, self.radius);
                sdf.fill_keep(self.bg_color);

                // Draw arrow triangle and fill it
                // All triangles drawn starting from tip, going clockwise
                if self.arrow_dir < 0.5 {
                    // Top: arrow points down (at bottom of tooltip)
                    let cx = clamp(self.arrow_pos.x, min_x, max_x);
                    let base_y = box_pos.y + box_size.y - overlap;
                    let tip_y = self.rect_size.y - border;
                    // tip -> left base -> right base (clockwise when arrow points down)
                    sdf.move_to(cx, tip_y);
                    sdf.line_to(cx - arrow_half_w, base_y);
                    sdf.line_to(cx + arrow_half_w, base_y);
                    sdf.close_path();
                    sdf.fill_keep(self.bg_color);
                } else if self.arrow_dir < 1.5 {
                    // Bottom: arrow points up (at top of tooltip)
                    let cx = clamp(self.arrow_pos.x, min_x, max_x);
                    let base_y = box_pos.y + overlap;
                    let tip_y = border;
                    // tip -> right base -> left base (clockwise when arrow points up)
                    sdf.move_to(cx, tip_y);
                    sdf.line_to(cx + arrow_half_w, base_y);
                    sdf.line_to(cx - arrow_half_w, base_y);
                    sdf.close_path();
                    sdf.fill_keep(self.bg_color);
                } else if self.arrow_dir < 2.5 {
                    // Left: arrow points right (at right of tooltip)
                    let cy = clamp(self.arrow_pos.y, min_y, max_y);
                    let base_x = box_pos.x + box_size.x - overlap;
                    let tip_x = self.rect_size.x - border;
                    // tip -> bottom base -> top base (clockwise when arrow points right)
                    sdf.move_to(tip_x, cy);
                    sdf.line_to(base_x, cy + arrow_half_w);
                    sdf.line_to(base_x, cy - arrow_half_w);
                    sdf.close_path();
                    sdf.fill_keep(self.bg_color);
                } else {
                    // Right: arrow points left (at left of tooltip)
                    let cy = clamp(self.arrow_pos.y, min_y, max_y);
                    let base_x = box_pos.x + overlap;
                    let tip_x = border;
                    // tip -> top base -> bottom base (clockwise when arrow points left)
                    sdf.move_to(tip_x, cy);
                    sdf.line_to(base_x, cy - arrow_half_w);
                    sdf.line_to(base_x, cy + arrow_half_w);
                    sdf.close_path();
                    sdf.fill_keep(self.bg_color);
                }

                sdf.stroke(self.border_color, border);
                return sdf.result;
            }
        }

        popup_label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                color: (TOOLTIP_TEXT)
            }
        }
    }

    // Wrapper widget - wraps content and shows tooltip on hover
    pub MpTooltip = {{MpTooltip}} {
        width: Fit,
        height: Fit,
        flow: Overlay,
        popup_padding: { left: 8, right: 8, top: 5, bottom: 5 }
        popup_radius: 6.0
        arrow_size: vec2(12.0, 6.0)
        arrow_offset: 0.0
        gap: 2.0
        top_nudge: vec2(0.0, 0.0)
        bottom_nudge: vec2(0.0, 0.0)
        left_nudge: vec2(0.0, 0.0)
        right_nudge: vec2(0.0, 0.0)
        offset: vec2(0.0, 0.0)

        // The content slot - user puts their widget here
        content = <View> {
            width: Fit,
            height: Fit,
        }

        // Enable hover capture on the wrapper
        capture_overload: true,
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }

        // The popup (hidden until hover, drawn as overlay)
        popup = <MpTooltipPopup> {}
    }

    pub MpTooltipTop = <MpTooltip> { position: Top }
    pub MpTooltipBottom = <MpTooltip> { position: Bottom }
    pub MpTooltipLeft = <MpTooltip> { position: Left }
    pub MpTooltipRight = <MpTooltip> { position: Right }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl LiveHook for TooltipPosition {}

impl LiveNew for TooltipPosition {
    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_type: std::any::TypeId::of::<Self>(),
            live_ignore: true,
            fields: Vec::new(),
            type_name: LiveId::from_str("TooltipPosition"),
        }
    }

    fn new(_cx: &mut Cx) -> Self {
        Self::default()
    }
}

impl LiveApply for TooltipPosition {
    fn apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::BareEnum(v) = &nodes[index].value {
            *self = match *v {
                live_id!(Top) => TooltipPosition::Top,
                live_id!(Bottom) => TooltipPosition::Bottom,
                live_id!(Left) => TooltipPosition::Left,
                live_id!(Right) => TooltipPosition::Right,
                _ => TooltipPosition::Top,
            };
        }
        index + 1
    }
}

impl LiveRead for TooltipPosition {
    fn live_read_to(&self, _id: LiveId, _out: &mut Vec<LiveNode>) {}
}

#[derive(Live, LiveHook, Widget)]
pub struct MpTooltip {
    #[deref]
    view: View,

    #[live]
    tip: ArcStringMut,

    #[live]
    position: TooltipPosition,

    #[live]
    popup_padding: Padding,

    #[live]
    popup_radius: f64,

    #[live]
    arrow_size: DVec2,

    #[live]
    arrow_offset: f64,

    #[live]
    gap: f64,

    #[live]
    top_nudge: DVec2,

    #[live]
    bottom_nudge: DVec2,

    #[live]
    left_nudge: DVec2,

    #[live]
    right_nudge: DVec2,

    #[live]
    offset: DVec2,

    #[live(0.3)]
    show_delay: f64,

    #[rust]
    delay_timer: Timer,

    #[rust]
    hovering: bool,

    #[rust]
    popup_opened: bool,

    #[rust]
    content_rect: Rect,

    #[rust]
    anchor_rect: Rect,

    #[rust]
    popup_size: DVec2,

    #[rust(DrawList2d::new(cx))]
    draw_list: DrawList2d,
}

impl Widget for MpTooltip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle delay timer first
        if self.delay_timer.is_event(event).is_some() {
            if self.hovering && !self.popup_opened {
                self.popup_opened = true;
                self.popup_size = DVec2::default();
                self.draw_list.redraw(cx);
                self.redraw(cx);
            }
        }

        // Manual hover tracking so child widgets keep their own hover behavior.
        let hover_rect = if self.anchor_rect.size.x > 0.0 {
            self.anchor_rect
        } else {
            self.content_rect
        };
        if hover_rect.size.x > 0.0 {
            match event {
                Event::MouseMove(me) => {
                    let is_over = hover_rect.contains(me.abs);
                    if is_over && !self.hovering {
                        self.hovering = true;
                        if self.show_delay > 0.0 {
                            self.delay_timer = cx.start_timeout(self.show_delay);
                        } else {
                            self.popup_opened = true;
                            self.popup_size = DVec2::default();
                            self.draw_list.redraw(cx);
                            self.redraw(cx);
                        }
                    } else if !is_over && self.hovering {
                        self.hovering = false;
                        self.popup_opened = false;
                        self.delay_timer = Timer::default();
                        self.draw_list.redraw(cx);
                        self.redraw(cx);
                    }
                }
                Event::MouseLeave(_) => {
                    if self.hovering {
                        self.hovering = false;
                        self.popup_opened = false;
                        self.delay_timer = Timer::default();
                        self.draw_list.redraw(cx);
                        self.redraw(cx);
                    }
                }
                _ => {}
            }
        }

        // Forward events to children
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Hide popup - it will be drawn as overlay
        self.view.view(ids!(popup)).set_visible(cx, false);

        // Draw the view (which contains user content)
        let result = self.view.draw_walk(cx, scope, walk);

        // Store rect for positioning
        let content_area = self.view.view(ids!(content)).area();
        self.content_rect = content_area.rect(cx);
        self.anchor_rect = content_area.clipped_rect(cx);

        // Draw popup overlay if opened
        if self.popup_opened && self.content_rect.size.x > 0.0 && !self.tip.as_ref().is_empty() {
            self.draw_popup_overlay(cx, scope);
        }

        result
    }
}

impl MpTooltip {
    fn draw_popup_overlay(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        // Update label text
        self.view.label(ids!(popup.popup_label)).set_text(cx, self.tip.as_ref());

        // Begin overlay rendering
        self.draw_list.begin_overlay_reuse(cx);

        let pass_size = cx.current_pass_size();
        cx.begin_root_turtle_for_pass(Layout::flow_overlay());

        // Calculate popup position with edge detection
        let anchor = if self.anchor_rect.size.x > 0.0 {
            self.anchor_rect
        } else {
            self.content_rect
        };
        let (calculated_pos, actual_position) = self.calculate_position(anchor, pass_size);

        // First frame: we don't have popup size yet, draw off-screen to measure
        let is_measuring = self.popup_size.x == 0.0;
        let pos = if is_measuring {
            // Draw off-screen for measurement
            DVec2 { x: -10000.0, y: -10000.0 }
        } else {
            self.snap_position(cx, calculated_pos)
        };

        // Position and show popup
        let popup = self.view.view(ids!(popup));
        popup.set_visible(cx, true);

        // Use actual_position (may be flipped due to edge detection)
        let arrow_dir = match actual_position {
            TooltipPosition::Top => 0.0,
            TooltipPosition::Bottom => 1.0,
            TooltipPosition::Left => 2.0,
            TooltipPosition::Right => 3.0,
        };

        let arrow_depth = self.arrow_size.y.max(0.0);
        let mut padding = self.popup_padding;
        match actual_position {
            TooltipPosition::Top => padding.bottom += arrow_depth,
            TooltipPosition::Bottom => padding.top += arrow_depth,
            TooltipPosition::Left => padding.right += arrow_depth,
            TooltipPosition::Right => padding.left += arrow_depth,
        }

        let anchor_center = DVec2 {
            x: anchor.pos.x + anchor.size.x * 0.5,
            y: anchor.pos.y + anchor.size.y * 0.5,
        };
        let mut arrow_pos = DVec2 {
            x: anchor_center.x - pos.x,
            y: anchor_center.y - pos.y,
        };
        match actual_position {
            TooltipPosition::Top | TooltipPosition::Bottom => {
                arrow_pos.x += self.arrow_offset;
                if self.popup_size.x > 0.0 {
                    let arrow_half = self.arrow_size.x * 0.5;
                    let inset = 1.0;
                    let min_x = inset + self.popup_radius + arrow_half;
                    let max_x = self.popup_size.x - inset - self.popup_radius - arrow_half;
                    if max_x > min_x {
                        arrow_pos.x = arrow_pos.x.max(min_x).min(max_x);
                    }
                }
            }
            TooltipPosition::Left | TooltipPosition::Right => {
                arrow_pos.y += self.arrow_offset;
                if self.popup_size.y > 0.0 {
                    let arrow_half = self.arrow_size.x * 0.5;
                    let inset = 1.0;
                    let min_y = inset + self.popup_radius + arrow_half;
                    let max_y = self.popup_size.y - inset - self.popup_radius - arrow_half;
                    if max_y > min_y {
                        arrow_pos.y = arrow_pos.y.max(min_y).min(max_y);
                    }
                }
            }
        }

        popup.apply_over(
            cx,
            live! {
                padding: {
                    left: (padding.left),
                    right: (padding.right),
                    top: (padding.top),
                    bottom: (padding.bottom)
                }
                draw_bg: {
                    arrow_dir: (arrow_dir),
                    arrow_size: (vec2(self.arrow_size.x as f32, self.arrow_size.y as f32)),
                    arrow_pos: (vec2(arrow_pos.x as f32, arrow_pos.y as f32)),
                    radius: (self.popup_radius)
                }
            },
        );

        // Draw popup
        let mut walk = popup.walk(cx);
        walk.abs_pos = Some(pos);
        walk.margin = Margin::default();
        let _ = popup.draw_walk(cx, scope, walk);

        // Update popup size for positioning
        let new_size = popup.area().rect(cx).size;
        if (new_size.x - self.popup_size.x).abs() > 1.0 || (new_size.y - self.popup_size.y).abs() > 1.0 {
            self.popup_size = new_size;
            cx.redraw_all();
        }

        cx.end_pass_sized_turtle();
        self.draw_list.end(cx);
    }

    fn calculate_position(&self, anchor_rect: Rect, screen_size: DVec2) -> (DVec2, TooltipPosition) {
        let c = anchor_rect;
        let popup_size = self.popup_size;
        let gap = self.gap;
        let arrow_depth = self.arrow_size.y.max(0.0);

        // Calculate initial position based on requested position
        let mut actual_position = self.position;
        let mut pos = self.calc_pos_for_direction(actual_position, c, popup_size, gap);

        // Edge detection and automatic flip
        match actual_position {
            TooltipPosition::Top => {
                // Flip to bottom if would go off top
                if pos.y < 0.0 {
                    actual_position = TooltipPosition::Bottom;
                    pos = self.calc_pos_for_direction(actual_position, c, popup_size, gap);
                }
            }
            TooltipPosition::Bottom => {
                // Flip to top if would go off bottom
                if pos.y + popup_size.y + arrow_depth > screen_size.y {
                    actual_position = TooltipPosition::Top;
                    pos = self.calc_pos_for_direction(actual_position, c, popup_size, gap);
                }
            }
            TooltipPosition::Left => {
                // Flip to right if would go off left
                if pos.x < 0.0 {
                    actual_position = TooltipPosition::Right;
                    pos = self.calc_pos_for_direction(actual_position, c, popup_size, gap);
                }
            }
            TooltipPosition::Right => {
                // Flip to left if would go off right
                if pos.x + popup_size.x + arrow_depth > screen_size.x {
                    actual_position = TooltipPosition::Left;
                    pos = self.calc_pos_for_direction(actual_position, c, popup_size, gap);
                }
            }
        }

        // Apply nudge and offset
        let nudge = match actual_position {
            TooltipPosition::Top => self.top_nudge,
            TooltipPosition::Bottom => self.bottom_nudge,
            TooltipPosition::Left => self.left_nudge,
            TooltipPosition::Right => self.right_nudge,
        };
        pos.x += nudge.x + self.offset.x;
        pos.y += nudge.y + self.offset.y;

        // Clamp to screen bounds
        pos.x = pos.x.max(0.0).min(screen_size.x - popup_size.x);
        pos.y = pos.y.max(0.0).min(screen_size.y - popup_size.y);

        (pos, actual_position)
    }

    fn calc_pos_for_direction(&self, position: TooltipPosition, c: Rect, popup_size: DVec2, gap: f64) -> DVec2 {
        match position {
            TooltipPosition::Top => DVec2 {
                x: c.pos.x + (c.size.x - popup_size.x) / 2.0,
                y: c.pos.y - popup_size.y - gap,
            },
            TooltipPosition::Bottom => DVec2 {
                x: c.pos.x + (c.size.x - popup_size.x) / 2.0,
                y: c.pos.y + c.size.y + gap,
            },
            TooltipPosition::Left => DVec2 {
                x: c.pos.x - popup_size.x - gap,
                y: c.pos.y + (c.size.y - popup_size.y) / 2.0,
            },
            TooltipPosition::Right => DVec2 {
                x: c.pos.x + c.size.x + gap,
                y: c.pos.y + (c.size.y - popup_size.y) / 2.0,
            },
        }
    }

    fn snap_position(&self, cx: &Cx2d, pos: DVec2) -> DVec2 {
        let dpi = cx.current_dpi_factor().max(1.0);
        DVec2 {
            x: (pos.x * dpi).round() / dpi,
            y: (pos.y * dpi).round() / dpi,
        }
    }
}

impl MpTooltipRef {
    pub fn set_tip(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.tip.as_mut_empty().push_str(text);
            inner.redraw(cx);
        }
    }

    pub fn set_position(&self, position: TooltipPosition) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.position = position;
        }
    }

    pub fn set_show_delay(&self, delay: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_delay = delay;
        }
    }
}
