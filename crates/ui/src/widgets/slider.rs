use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Slider component
    pub MpSlider = {{MpSlider}} {
        width: Fill,
        height: 24,

        draw_track: {
            instance progress_start: 0.0
            instance progress_end: 0.0
            instance disabled: 0.0
            instance vertical: 0.0
            instance track_color: #e2e8f0
            instance fill_color: #3b82f6
            instance disabled_track_color: #f1f5f9
            instance disabled_fill_color: #94a3b8

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;

                // Choose track color based on disabled state
                let track_col = mix(self.track_color, self.disabled_track_color, self.disabled);
                let fill_col = mix(self.fill_color, self.disabled_fill_color, self.disabled);

                // For vertical, swap coordinates conceptually
                let is_vert = self.vertical;
                let length = mix(sz.x, sz.y, is_vert);
                let thickness = mix(sz.y, sz.x, is_vert);
                let r = thickness * 0.5;

                // Draw track capsule
                let pos_main = mix(self.pos.x, 1.0 - self.pos.y, is_vert);
                let pos_cross = mix(self.pos.y, self.pos.x, is_vert);

                // Create capsule shape based on orientation
                if is_vert > 0.5 {
                    sdf.circle(r, r, r);
                    sdf.rect(0.0, r, sz.x, sz.y - sz.x);
                    sdf.circle(r, sz.y - r, r);
                } else {
                    sdf.circle(r, r, r);
                    sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                    sdf.circle(sz.x - r, r, r);
                }

                sdf.fill(track_col);

                // Draw fill based on progress
                let fill_start = length * self.progress_start;
                let fill_end = length * self.progress_end;
                let px = pos_main * length;

                // Check if pixel is in fill region
                let in_fill = step(fill_start, px) * step(px, fill_end);

                // Create second SDF for fill
                let sdf2 = Sdf2d::viewport(self.pos * self.rect_size);
                if is_vert > 0.5 {
                    sdf2.circle(r, r, r);
                    sdf2.rect(0.0, r, sz.x, sz.y - sz.x);
                    sdf2.circle(r, sz.y - r, r);
                } else {
                    sdf2.circle(r, r, r);
                    sdf2.rect(r, 0.0, sz.x - sz.y, sz.y);
                    sdf2.circle(sz.x - r, r, r);
                }
                sdf2.fill(fill_col);

                let result = mix(sdf.result, sdf2.result, in_fill * sdf2.result.w);
                return result;
            }
        }

        draw_thumb: {
            instance hover: 0.0
            instance pressed: 0.0
            instance disabled: 0.0
            instance border_color: #3b82f6
            instance disabled_border_color: #94a3b8

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Choose border color based on disabled state
                let border_col = mix(self.border_color, self.disabled_border_color, self.disabled);

                // Shadow (only when not disabled)
                let shadow_alpha = mix(0.2, 0.0, self.disabled);
                let shadow_offset = 2.0;
                sdf.circle(c.x + shadow_offset, c.y + shadow_offset, c.x - 2.0);
                sdf.fill(vec4(0.0, 0.0, 0.0, shadow_alpha));

                // Main circle
                sdf.circle(c.x, c.y, c.x - 2.0);

                let base_color = mix(#ffffff, #f8fafc, self.disabled);
                let hover_color = #f0f9ff;
                let pressed_color = #e0f2fe;

                // Only apply hover/pressed when not disabled
                let active_hover = self.hover * (1.0 - self.disabled);
                let active_pressed = self.pressed * (1.0 - self.disabled);

                let color = mix(base_color, hover_color, active_hover);
                let color = mix(color, pressed_color, active_pressed);

                sdf.fill(color);

                // Border
                sdf.stroke(border_col, 2.5);

                return sdf.result;
            }
        }

        // Second thumb for range mode
        draw_thumb_start: {
            instance hover: 0.0
            instance pressed: 0.0
            instance disabled: 0.0
            instance border_color: #3b82f6
            instance disabled_border_color: #94a3b8

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                let border_col = mix(self.border_color, self.disabled_border_color, self.disabled);

                let shadow_alpha = mix(0.2, 0.0, self.disabled);
                let shadow_offset = 2.0;
                sdf.circle(c.x + shadow_offset, c.y + shadow_offset, c.x - 2.0);
                sdf.fill(vec4(0.0, 0.0, 0.0, shadow_alpha));

                sdf.circle(c.x, c.y, c.x - 2.0);

                let base_color = mix(#ffffff, #f8fafc, self.disabled);
                let hover_color = #f0f9ff;
                let pressed_color = #e0f2fe;

                let active_hover = self.hover * (1.0 - self.disabled);
                let active_pressed = self.pressed * (1.0 - self.disabled);

                let color = mix(base_color, hover_color, active_hover);
                let color = mix(color, pressed_color, active_pressed);

                sdf.fill(color);
                sdf.stroke(border_col, 2.5);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_thumb: { hover: 0.0 }, draw_thumb_start: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_thumb: { hover: 1.0 }, draw_thumb_start: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: { draw_thumb: { pressed: 0.0 }, draw_thumb_start: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_thumb: { pressed: 1.0 }, draw_thumb_start: { pressed: 1.0 } }
                }
            }
        }
    }

    // Vertical slider
    pub MpSliderVertical = <MpSlider> {
        width: 24,
        height: Fill,
        vertical: true,
    }

    // Slider variants
    pub MpSliderSuccess = <MpSlider> {
        draw_track: { fill_color: #22c55e }
        draw_thumb: { border_color: #22c55e }
        draw_thumb_start: { border_color: #22c55e }
    }

    pub MpSliderWarning = <MpSlider> {
        draw_track: { fill_color: #f59e0b }
        draw_thumb: { border_color: #f59e0b }
        draw_thumb_start: { border_color: #f59e0b }
    }

    pub MpSliderDanger = <MpSlider> {
        draw_track: { fill_color: #dc2626 }
        draw_thumb: { border_color: #dc2626 }
        draw_thumb_start: { border_color: #dc2626 }
    }
}

/// The value of the slider, can be a single value or a range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SliderValue {
    Single(f64),
    Range(f64, f64),
}

impl Default for SliderValue {
    fn default() -> Self {
        SliderValue::Single(0.0)
    }
}

impl SliderValue {
    pub fn single(value: f64) -> Self {
        SliderValue::Single(value)
    }

    pub fn range(start: f64, end: f64) -> Self {
        SliderValue::Range(start.min(end), start.max(end))
    }

    pub fn is_single(&self) -> bool {
        matches!(self, SliderValue::Single(_))
    }

    pub fn is_range(&self) -> bool {
        matches!(self, SliderValue::Range(_, _))
    }

    pub fn start(&self) -> f64 {
        match self {
            SliderValue::Single(v) => *v,
            SliderValue::Range(start, _) => *start,
        }
    }

    pub fn end(&self) -> f64 {
        match self {
            SliderValue::Single(v) => *v,
            SliderValue::Range(_, end) => *end,
        }
    }

    pub fn clamp(self, min: f64, max: f64) -> Self {
        match self {
            SliderValue::Single(v) => SliderValue::Single(v.clamp(min, max)),
            SliderValue::Range(start, end) => {
                SliderValue::Range(start.clamp(min, max), end.clamp(min, max))
            }
        }
    }
}

impl std::fmt::Display for SliderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliderValue::Single(v) => write!(f, "{}", v),
            SliderValue::Range(start, end) => write!(f, "{}..{}", start, end),
        }
    }
}


#[derive(Clone, Debug, DefaultNone)]
pub enum MpSliderAction {
    Changed(SliderValue),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSlider {
    #[redraw]
    #[live]
    draw_track: DrawQuad,

    #[live]
    draw_thumb: DrawQuad,

    #[live]
    draw_thumb_start: DrawQuad,

    #[animator]
    animator: Animator,

    #[walk]
    walk: Walk,

    #[live(0.0)]
    value: f64,

    #[live(0.0)]
    value_start: f64,

    #[live(false)]
    range_mode: bool,

    #[live(0.0)]
    min: f64,

    #[live(100.0)]
    max: f64,

    #[live(1.0)]
    step: f64,

    #[live(false)]
    logarithmic: bool,

    #[live(false)]
    vertical: bool,

    #[live(false)]
    disabled: bool,

    #[rust]
    dragging: bool,

    #[rust]
    dragging_start_thumb: bool,

    #[rust]
    track_area: Area,

    #[rust]
    thumb_area: Area,

    #[rust]
    thumb_start_area: Area,
}

impl Widget for MpSlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.disabled {
            return;
        }

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        // Handle track area events
        match event.hits(cx, self.track_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                if !self.dragging {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(fe) => {
                self.dragging = true;
                // Determine which thumb to drag in range mode
                if self.range_mode {
                    let progress = self.position_to_progress(cx, fe.abs);
                    let start_progress = self.value_to_progress(self.value_start);
                    let end_progress = self.value_to_progress(self.value);
                    let mid = (start_progress + end_progress) / 2.0;
                    self.dragging_start_thumb = progress < mid;
                }
                self.animator_play(cx, ids!(pressed.on));
                self.update_value_from_position(cx, fe.abs, scope);
            }
            Hit::FingerMove(fe) => {
                if self.dragging {
                    self.update_value_from_position(cx, fe.abs, scope);
                }
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
                self.dragging_start_thumb = false;
                self.animator_play(cx, ids!(pressed.off));
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Calculate progress values
        let progress_start = if self.range_mode {
            self.value_to_progress(self.value_start)
        } else {
            0.0
        };
        let progress_end = self.value_to_progress(self.value);

        let disabled_f = if self.disabled { 1.0 } else { 0.0 };
        let vertical_f = if self.vertical { 1.0 } else { 0.0 };

        // Update track
        self.draw_track.apply_over(cx, live! {
            progress_start: (progress_start),
            progress_end: (progress_end),
            disabled: (disabled_f),
            vertical: (vertical_f)
        });

        // Update thumbs disabled state
        self.draw_thumb.apply_over(cx, live! {
            disabled: (disabled_f)
        });
        self.draw_thumb_start.apply_over(cx, live! {
            disabled: (disabled_f)
        });

        // Get the rect for drawing
        let rect = cx.walk_turtle(walk);
        let track_thickness = 6.0;
        let thumb_size = 20.0;

        if self.vertical {
            // Vertical layout
            let track_x = (rect.size.x - track_thickness) / 2.0;
            let track_rect = Rect {
                pos: DVec2 { x: rect.pos.x + track_x, y: rect.pos.y },
                size: DVec2 { x: track_thickness, y: rect.size.y },
            };
            self.draw_track.draw_abs(cx, track_rect);
            self.track_area = self.draw_track.area();

            // Draw end thumb (main value)
            let thumb_padding = thumb_size / 2.0;
            let track_height = rect.size.y - thumb_size;
            // For vertical, progress 0 is at bottom, 1 is at top
            let thumb_y = rect.pos.y + rect.size.y - thumb_padding - (track_height * progress_end) - (thumb_size / 2.0);
            let thumb_x = rect.pos.x + (rect.size.x - thumb_size) / 2.0;

            let thumb_rect = Rect {
                pos: DVec2 { x: thumb_x, y: thumb_y },
                size: DVec2 { x: thumb_size, y: thumb_size },
            };
            self.draw_thumb.draw_abs(cx, thumb_rect);
            self.thumb_area = self.draw_thumb.area();

            // Draw start thumb if range mode
            if self.range_mode {
                let start_thumb_y = rect.pos.y + rect.size.y - thumb_padding - (track_height * progress_start) - (thumb_size / 2.0);
                let start_thumb_rect = Rect {
                    pos: DVec2 { x: thumb_x, y: start_thumb_y },
                    size: DVec2 { x: thumb_size, y: thumb_size },
                };
                self.draw_thumb_start.draw_abs(cx, start_thumb_rect);
                self.thumb_start_area = self.draw_thumb_start.area();
            }
        } else {
            // Horizontal layout
            let track_y = (rect.size.y - track_thickness) / 2.0;
            let track_rect = Rect {
                pos: DVec2 { x: rect.pos.x, y: rect.pos.y + track_y },
                size: DVec2 { x: rect.size.x, y: track_thickness },
            };
            self.draw_track.draw_abs(cx, track_rect);
            self.track_area = self.draw_track.area();

            // Draw end thumb (main value)
            let thumb_padding = thumb_size / 2.0;
            let track_width = rect.size.x - thumb_size;
            let thumb_x = rect.pos.x + thumb_padding + (track_width * progress_end) - (thumb_size / 2.0);
            let thumb_y = rect.pos.y + (rect.size.y - thumb_size) / 2.0;

            let thumb_rect = Rect {
                pos: DVec2 { x: thumb_x, y: thumb_y },
                size: DVec2 { x: thumb_size, y: thumb_size },
            };
            self.draw_thumb.draw_abs(cx, thumb_rect);
            self.thumb_area = self.draw_thumb.area();

            // Draw start thumb if range mode
            if self.range_mode {
                let start_thumb_x = rect.pos.x + thumb_padding + (track_width * progress_start) - (thumb_size / 2.0);
                let start_thumb_rect = Rect {
                    pos: DVec2 { x: start_thumb_x, y: thumb_y },
                    size: DVec2 { x: thumb_size, y: thumb_size },
                };
                self.draw_thumb_start.draw_abs(cx, start_thumb_rect);
                self.thumb_start_area = self.draw_thumb_start.area();
            }
        }

        DrawStep::done()
    }
}

impl MpSlider {
    /// Convert a value to progress (0.0 to 1.0)
    fn value_to_progress(&self, value: f64) -> f64 {
        if self.max <= self.min {
            return 0.0;
        }

        if self.logarithmic && self.min > 0.0 {
            let base = self.max / self.min;
            ((value / self.min).ln() / base.ln()).clamp(0.0, 1.0)
        } else {
            ((value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
        }
    }

    /// Convert progress (0.0 to 1.0) to a value
    fn progress_to_value(&self, progress: f64) -> f64 {
        if self.logarithmic && self.min > 0.0 {
            let base = self.max / self.min;
            base.powf(progress) * self.min
        } else {
            self.min + (self.max - self.min) * progress
        }
    }

    /// Convert mouse position to progress
    fn position_to_progress(&self, cx: &Cx, pos: DVec2) -> f64 {
        let rect = self.track_area.rect(cx);
        if self.vertical {
            let thumb_radius = 10.0;
            let _track_start = rect.pos.y + thumb_radius;
            let track_height = rect.size.y - thumb_radius * 2.0;
            if track_height <= 0.0 {
                return 0.0;
            }
            // Invert for vertical (bottom = 0, top = 1)
            let relative_y = (rect.pos.y + rect.size.y - thumb_radius - pos.y).clamp(0.0, track_height);
            relative_y / track_height
        } else {
            let thumb_radius = 10.0;
            let track_start = rect.pos.x + thumb_radius;
            let track_width = rect.size.x - thumb_radius * 2.0;
            if track_width <= 0.0 {
                return 0.0;
            }
            let relative_x = (pos.x - track_start).clamp(0.0, track_width);
            relative_x / track_width
        }
    }

    fn update_value_from_position(&mut self, cx: &mut Cx, pos: DVec2, scope: &mut Scope) {
        let progress = self.position_to_progress(cx, pos);
        let raw_value = self.progress_to_value(progress);

        // Apply step
        let stepped_value = if self.step > 0.0 {
            let steps = ((raw_value - self.min) / self.step).round();
            self.min + steps * self.step
        } else {
            raw_value
        };

        let new_value = stepped_value.clamp(self.min, self.max);

        if self.range_mode {
            if self.dragging_start_thumb {
                // Update start value, ensure it doesn't exceed end
                let clamped = new_value.min(self.value);
                if (clamped - self.value_start).abs() > f64::EPSILON {
                    self.value_start = clamped;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpSliderAction::Changed(SliderValue::Range(self.value_start, self.value)),
                    );
                    self.redraw(cx);
                }
            } else {
                // Update end value, ensure it doesn't go below start
                let clamped = new_value.max(self.value_start);
                if (clamped - self.value).abs() > f64::EPSILON {
                    self.value = clamped;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpSliderAction::Changed(SliderValue::Range(self.value_start, self.value)),
                    );
                    self.redraw(cx);
                }
            }
        } else {
            if (new_value - self.value).abs() > f64::EPSILON {
                self.value = new_value;
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    MpSliderAction::Changed(SliderValue::Single(self.value)),
                );
                self.redraw(cx);
            }
        }
    }

    pub fn value(&self) -> SliderValue {
        if self.range_mode {
            SliderValue::Range(self.value_start, self.value)
        } else {
            SliderValue::Single(self.value)
        }
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: SliderValue) {
        match value {
            SliderValue::Single(v) => {
                self.value = v.clamp(self.min, self.max);
                self.range_mode = false;
            }
            SliderValue::Range(start, end) => {
                self.value_start = start.clamp(self.min, self.max);
                self.value = end.clamp(self.min, self.max);
                // Ensure start <= end
                if self.value_start > self.value {
                    std::mem::swap(&mut self.value_start, &mut self.value);
                }
                self.range_mode = true;
            }
        }
        self.redraw(cx);
    }

    pub fn set_single_value(&mut self, cx: &mut Cx, value: f64) {
        self.value = value.clamp(self.min, self.max);
        self.redraw(cx);
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
        self.value_start = self.value_start.clamp(min, max);
    }

    pub fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.disabled = disabled;
        self.redraw(cx);
    }
}

impl MpSliderRef {
    pub fn value(&self) -> SliderValue {
        if let Some(inner) = self.borrow() {
            inner.value()
        } else {
            SliderValue::Single(0.0)
        }
    }

    pub fn set_value(&self, cx: &mut Cx, value: SliderValue) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn set_single_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_single_value(cx, value);
        }
    }

    pub fn set_disabled(&self, cx: &mut Cx, disabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_disabled(cx, disabled);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<SliderValue> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpSliderAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }
}
