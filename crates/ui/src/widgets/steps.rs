use makepad_widgets::*;
use makepad_widgets::turtle::RowAlign;

const STEP_ICON_SIZE_DEFAULT: f64 = 28.0;
const STEP_ICON_SIZE_SMALL: f64 = 20.0;
const STEP_DOT_SIZE_DEFAULT: f64 = 8.0;
const STEP_DOT_SIZE_SMALL: f64 = 6.0;
const STEP_TAIL_LENGTH_DEFAULT: f64 = 48.0;
const STEP_TAIL_LENGTH_SMALL: f64 = 32.0;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpSteps - stepper component
    // ============================================================

    STEP_ICON_SIZE = 28.0
    STEP_ICON_SIZE_SM = 20.0
    STEP_DOT_SIZE = 8.0
    STEP_DOT_SIZE_SM = 6.0
    STEP_TAIL_LENGTH = 48.0
    STEP_TAIL_LENGTH_SM = 32.0

    // Steps container (horizontal)
    MpStepsBase = {{MpSteps}} {
        width: Fit
        height: Fit
        flow: Right
        spacing: 24
        align: { y: 0.0 }

        current: 1
        current_status: Process
        step_type: Default
        size: Default
        direction: Horizontal
        label_placement: Horizontal
        lineless: false
        step_spacing: -1.0
    }

    // Steps container (vertical)
    pub MpSteps = <MpStepsBase> {}
    pub MpStepsVertical = <MpStepsBase> { direction: Vertical }
    pub MpStepsDot = <MpStepsBase> { step_type: Dot }
    pub MpStepsArrow = <MpStepsBase> { step_type: Arrow }
    pub MpStepsNavigation = <MpStepsBase> { step_type: Navigation }

    MpStepIcon = <View> {
        width: (STEP_ICON_SIZE)
        height: (STEP_ICON_SIZE)
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance border_color: (BORDER)
            instance border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y) - self.border_width;

                sdf.circle(c.x, c.y, r);
                sdf.fill_keep(self.bg_color);

                if self.border_width > 0.0 {
                    sdf.stroke(self.border_color, self.border_width);
                }

                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
                color: (MUTED_FOREGROUND)
            }
            text: "1"
        }
    }

    MpStepTail = <View> {
        width: (STEP_TAIL_LENGTH)
        height: 2
        show_bg: true
        draw_bg: {
            instance color: (BORDER)

            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }

    MpStepContent = <View> {
        width: Fit
        height: Fit
        flow: Down
        spacing: 4

        title = <Label> {
            width: Fit
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: "Step"
        }

        description = <Label> {
            width: Fit
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                color: (MUTED_FOREGROUND)
            }
            text: ""
        }
    }

    pub MpStep = {{MpStep}} {
        width: Fit
        height: Fit
        flow: Down
        spacing: 8
        align: { x: 0.0 }

        show_bg: false
        draw_bg: {
            instance bg_color: #00000000
            instance border_color: #00000000
            instance border_width: 0.0
            instance border_radius: 6.0
            instance arrow: 0.0
            instance arrow_size: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let w = self.rect_size.x;
                let h = self.rect_size.y;
                let bw = self.border_width;

                if self.arrow > 0.5 {
                    let tip = min(self.arrow_size, w * 0.5);
                    let left = bw;
                    let right = w - bw;
                    let top = bw;
                    let bottom = h - bw;

                    sdf.move_to(left, top);
                    sdf.line_to(right - tip, top);
                    sdf.line_to(right, h * 0.5);
                    sdf.line_to(right - tip, bottom);
                    sdf.line_to(left, bottom);
                    sdf.close_path();
                    sdf.fill_keep(self.bg_color);

                    if bw > 0.0 {
                        sdf.stroke(self.border_color, bw);
                    }

                    return sdf.result;
                }

                let r = min(self.border_radius, (h - bw * 2.0) * 0.5);
                sdf.box(bw, bw, w - bw * 2.0, h - bw * 2.0, r);
                sdf.fill_keep(self.bg_color);
                if bw > 0.0 {
                    sdf.stroke(self.border_color, bw);
                }

                return sdf.result;
            }
        }

        direction: Horizontal
        label_placement: Vertical
        size: Default
        status: Process
        step_type: Default
        lineless: false
        show_tail: true
        index: 1
        disabled: false

        icon_wrap = <View> {
            width: Fit
            height: Fit
            flow: Right
            spacing: 8
            align: { y: 0.5 }

            icon = <MpStepIcon> {}
            tail = <MpStepTail> {}
        }

        content = <MpStepContent> {}
    }

    pub MpStepSmall = <MpStep> { size: Small }
    pub MpStepDot = <MpStep> { step_type: Dot label_placement: Vertical }
    pub MpStepVertical = <MpStep> { direction: Vertical label_placement: Horizontal }
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpStepDirection {
    #[pick]
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpStepLabelPlacement {
    #[pick]
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpStepSize {
    #[pick]
    Default,
    Small,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpStepStatus {
    #[pick]
    Wait,
    Process,
    Finish,
    Error,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpStepType {
    #[pick]
    Default,
    Dot,
    Arrow,
    Navigation,
}

#[derive(Live, Widget)]
pub struct MpStep {
    #[deref]
    view: View,
    #[live]
    direction: MpStepDirection,
    #[live]
    label_placement: MpStepLabelPlacement,
    #[live]
    size: MpStepSize,
    #[live]
    status: MpStepStatus,
    #[live]
    step_type: MpStepType,
    #[live(false)]
    lineless: bool,
    #[live(true)]
    show_tail: bool,
    #[live(1)]
    index: i64,
    #[live(false)]
    disabled: bool,
}

impl LiveHook for MpStep {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_visuals(cx);
    }
}

impl Widget for MpStep {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                if !self.disabled {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.disabled {
                    cx.set_cursor(MouseCursor::Default);
                }
            }
            Hit::FingerUp(fe) => {
                if !self.disabled && fe.is_over {
                    cx.widget_action(self.widget_uid(), &scope.path, MpStepAction::Clicked(self.index));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpStep {
    fn sync_visuals(&mut self, cx: &mut Cx) {
        let (icon_size, dot_size, tail_length, title_size, desc_size, icon_spacing, content_spacing) =
            match self.size {
                MpStepSize::Default => (
                    STEP_ICON_SIZE_DEFAULT,
                    STEP_DOT_SIZE_DEFAULT,
                    STEP_TAIL_LENGTH_DEFAULT,
                    14.0,
                    12.0,
                    8.0,
                    4.0,
                ),
                MpStepSize::Small => (
                    STEP_ICON_SIZE_SMALL,
                    STEP_DOT_SIZE_SMALL,
                    STEP_TAIL_LENGTH_SMALL,
                    12.0,
                    11.0,
                    6.0,
                    2.0,
                ),
            };

        let border_color = vec4(0.824, 0.847, 0.941, 1.0);
        let muted = vec4(0.392, 0.455, 0.545, 1.0);
        let foreground = vec4(0.059, 0.090, 0.165, 1.0);
        let primary = vec4(0.231, 0.510, 0.965, 1.0);
        let success = vec4(0.086, 0.639, 0.290, 1.0);
        let danger = vec4(0.863, 0.149, 0.149, 1.0);
        let white = vec4(1.0, 1.0, 1.0, 1.0);
        let muted_bg = vec4(0.945, 0.961, 0.976, 1.0);

        let (bg_color, border, text_color, tail_color, title_color, border_width) = match self.status {
            MpStepStatus::Wait => (vec4(0.0, 0.0, 0.0, 0.0), border_color, muted, border_color, muted, 1.0),
            MpStepStatus::Process => (primary, primary, white, primary, foreground, 0.0),
            MpStepStatus::Finish => (success, success, white, success, foreground, 0.0),
            MpStepStatus::Error => (danger, danger, white, danger, danger, 0.0),
        };

        let icon = self.view.view(ids!(icon_wrap.icon));
        let icon_label = self.view.label(ids!(icon_wrap.icon.label));
        let tail = self.view.view(ids!(icon_wrap.tail));
        let title = self.view.label(ids!(content.title));
        let description = self.view.label(ids!(content.description));
        let content = self.view.view(ids!(content));
        let icon_wrap = self.view.view(ids!(icon_wrap));

        icon_label.set_text(cx, &self.index.to_string());
        title.apply_over(cx, live! { draw_text: { color: (title_color), text_style: { font_size: (title_size) } } });
        description.apply_over(cx, live! { draw_text: { text_style: { font_size: (desc_size) } } });
        content.apply_over(cx, live! { spacing: (content_spacing) });

        if matches!(self.step_type, MpStepType::Dot) {
            icon.apply_over(cx, live! {
                width: (dot_size)
                height: (dot_size)
                draw_bg: { bg_color: (tail_color), border_color: (tail_color), border_width: 0.0 }
            });
            icon_label.set_visible(cx, false);
        } else {
            icon.apply_over(cx, live! {
                width: (icon_size)
                height: (icon_size)
                draw_bg: { bg_color: (bg_color), border_color: (border), border_width: (border_width) }
            });
            icon_label.set_visible(cx, true);
            icon_label.apply_over(cx, live! { draw_text: { color: (text_color) } });
        }

        let show_tail = self.show_tail && !self.lineless;
        tail.set_visible(cx, show_tail);
        if show_tail {
            match self.direction {
                MpStepDirection::Horizontal => {
                    tail.apply_over(cx, live! { width: (tail_length), height: 2.0, draw_bg: { color: (tail_color) } });
                }
                MpStepDirection::Vertical => {
                    tail.apply_over(cx, live! { width: 2.0, height: (tail_length), draw_bg: { color: (tail_color) } });
                }
            }
        }

        let effective_label = if matches!(self.direction, MpStepDirection::Vertical) {
            MpStepLabelPlacement::Horizontal
        } else {
            self.label_placement
        };

        let is_arrow = matches!(self.step_type, MpStepType::Arrow | MpStepType::Navigation);
        if is_arrow {
            let (arrow_bg, arrow_text) = match self.status {
                MpStepStatus::Wait => (muted_bg, muted),
                MpStepStatus::Process => (primary, white),
                MpStepStatus::Finish => (success, white),
                MpStepStatus::Error => (danger, white),
            };
            let arrow_size = match self.size {
                MpStepSize::Default => 12.0,
                MpStepSize::Small => 8.0,
            };
            self.view.apply_over(cx, live! {
                show_bg: true
                draw_bg: { bg_color: (arrow_bg), border_color: (arrow_bg), border_width: 0.0, arrow: 1.0, arrow_size: (arrow_size) }
                padding: { left: 12, right: 16, top: 8, bottom: 8 }
                flow: Right
                spacing: 8
            });
            title.apply_over(cx, live! { draw_text: { color: (arrow_text) } });
            description.apply_over(cx, live! { draw_text: { color: (arrow_text) } });
            icon_wrap.set_visible(cx, false);
        } else {
            self.view.apply_over(cx, live! {
                show_bg: false
                draw_bg: { arrow: 0.0 }
                padding: { left: 0, right: 0, top: 0, bottom: 0 }
            });
            icon_wrap.set_visible(cx, true);
            description.apply_over(cx, live! { draw_text: { color: (muted) } });

            match (self.direction, effective_label) {
                (MpStepDirection::Horizontal, MpStepLabelPlacement::Vertical) => {
                    self.view.layout.flow = Flow::Down;
                    self.view.layout.spacing = icon_spacing;
                    icon_wrap.apply_over(cx, live! {
                        flow: Right
                        spacing: (icon_spacing)
                        align: { y: 0.5 }
                    });
                }
                (MpStepDirection::Horizontal, MpStepLabelPlacement::Horizontal) => {
                    self.view.layout.flow = Flow::Right { row_align: RowAlign::Top, wrap: false };
                    self.view.layout.spacing = icon_spacing + 4.0;
                    icon_wrap.apply_over(cx, live! {
                        flow: Right
                        spacing: (icon_spacing)
                        align: { y: 0.5 }
                    });
                }
                (MpStepDirection::Vertical, _) => {
                    self.view.layout.flow = Flow::Right { row_align: RowAlign::Top, wrap: false };
                    self.view.layout.spacing = icon_spacing + 4.0;
                    icon_wrap.apply_over(cx, live! {
                        flow: Down
                        spacing: (icon_spacing)
                        align: { x: 0.5 }
                    });
                }
            }
        }

        self.view.redraw(cx);
    }

    pub fn apply_state(
        &mut self,
        cx: &mut Cx,
        index: i64,
        status: MpStepStatus,
        size: MpStepSize,
        direction: MpStepDirection,
        label_placement: MpStepLabelPlacement,
        step_type: MpStepType,
        lineless: bool,
        show_tail: bool,
    ) {
        self.index = index;
        self.status = status;
        self.size = size;
        self.direction = direction;
        self.label_placement = label_placement;
        self.step_type = step_type;
        self.lineless = lineless;
        self.show_tail = show_tail;
        self.sync_visuals(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpStepAction {
    Clicked(i64),
    None,
}

impl MpStep {
    pub fn clicked(&self, actions: &Actions) -> Option<i64> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let MpStepAction::Clicked(index) = action.cast::<MpStepAction>() {
                return Some(index);
            }
        }
        None
    }
}

impl MpStepRef {
    pub fn clicked(&self, actions: &Actions) -> Option<i64> {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            None
        }
    }
}

#[derive(Live, Widget)]
pub struct MpSteps {
    #[deref]
    view: View,
    #[live(1)]
    current: i64,
    #[live]
    current_status: MpStepStatus,
    #[live]
    step_type: MpStepType,
    #[live]
    size: MpStepSize,
    #[live]
    direction: MpStepDirection,
    #[live]
    label_placement: MpStepLabelPlacement,
    #[live(false)]
    lineless: bool,
    #[live(-1.0)]
    step_spacing: f64,
}

impl LiveHook for MpSteps {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_steps(cx);
    }
}

impl Widget for MpSteps {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpSteps {
    pub fn set_current(&mut self, cx: &mut Cx, current: i64) {
        self.current = current.max(1);
        self.sync_steps(cx);
    }

    fn sync_steps(&mut self, cx: &mut Cx) {
        let mut steps = Vec::new();
        let count = self.view.child_count();
        for index in 0..count {
            if let Some(child) = self.view.child_at_index(index) {
                if child.borrow::<MpStep>().is_some() {
                    steps.push(child.clone());
                }
            }
        }

        let total = steps.len() as i64;
        if total == 0 {
            return;
        }
        if self.current > total {
            self.current = total;
        }

        let mut direction = self.direction;
        if matches!(self.step_type, MpStepType::Arrow | MpStepType::Navigation) {
            direction = MpStepDirection::Horizontal;
        }

        let mut label_placement = self.label_placement;
        if matches!(self.step_type, MpStepType::Dot) {
            label_placement = if matches!(direction, MpStepDirection::Vertical) {
                MpStepLabelPlacement::Horizontal
            } else {
                MpStepLabelPlacement::Vertical
            };
        }
        if matches!(self.step_type, MpStepType::Arrow | MpStepType::Navigation) {
            label_placement = MpStepLabelPlacement::Horizontal;
        }

        match direction {
            MpStepDirection::Horizontal => {
                self.view.layout.flow = Flow::Right { row_align: RowAlign::Top, wrap: false };
                let default_spacing = if matches!(self.step_type, MpStepType::Arrow | MpStepType::Navigation) {
                    8.0
                } else {
                    24.0
                };
                self.view.layout.spacing = if self.step_spacing >= 0.0 {
                    self.step_spacing
                } else {
                    default_spacing
                };
            }
            MpStepDirection::Vertical => {
                self.view.layout.flow = Flow::Down;
                self.view.layout.spacing = if self.step_spacing >= 0.0 {
                    self.step_spacing
                } else {
                    16.0
                };
            }
        }

        for (idx, step_ref) in steps.iter().enumerate() {
            let index = (idx as i64) + 1;
            let status = if self.current > index {
                MpStepStatus::Finish
            } else if self.current == index {
                self.current_status
            } else {
                MpStepStatus::Wait
            };

            let force_lineless = matches!(self.step_type, MpStepType::Arrow | MpStepType::Navigation);
            let lineless = self.lineless || force_lineless;
            let show_tail = index < total && !force_lineless;

            if let Some(mut step) = step_ref.borrow_mut::<MpStep>() {
                step.apply_state(
                    cx,
                    index,
                    status,
                    self.size,
                    direction,
                    label_placement,
                    self.step_type,
                    lineless,
                    show_tail,
                );
            }
        }

        self.view.redraw(cx);
    }
}

impl MpStepsRef {
    pub fn set_current(&self, cx: &mut Cx, current: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_current(cx, current);
        }
    }
}
