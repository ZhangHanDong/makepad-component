use makepad_widgets::*;
use makepad_widgets::turtle::RowAlign;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpSpace - spacing layout helper
    // ============================================================

    pub MpSpaceBase = {{MpSpace}} {
        width: Fit
        height: Fit
        flow: Right
        spacing: 8

        direction: Horizontal
        align: Auto
        size: Small
        wrap: false
    }

    // Default (horizontal, small)
    pub MpSpace = <MpSpaceBase> {}

    // Direction variants
    pub MpSpaceVertical = <MpSpaceBase> { direction: Vertical }
    pub MpSpaceWrap = <MpSpaceBase> { wrap: true }

    // Size variants
    pub MpSpaceMini = <MpSpaceBase> { size: Mini }
    pub MpSpaceSmall = <MpSpaceBase> { size: Small }
    pub MpSpaceMedium = <MpSpaceBase> { size: Medium }
    pub MpSpaceLarge = <MpSpaceBase> { size: Large }

    // Optional split item (manual use between items)
    pub MpSpaceSplit = <Label> {
        width: Fit
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
            color: (MUTED_FOREGROUND)
        }
        text: "|"
    }

    // Item wrapper (optional)
    pub MpSpaceItem = <View> {
        width: Fit
        height: Fit
    }
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpSpaceDirection {
    #[pick]
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpSpaceAlign {
    #[pick]
    Auto,
    Start,
    End,
    Center,
    Baseline,
}

#[derive(Copy, Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum MpSpaceSize {
    Mini,
    #[pick]
    Small,
    Medium,
    Large,
    #[live(8.0)]
    Fixed(f64),
}

impl MpSpaceSize {
    fn value(self) -> f64 {
        match self {
            MpSpaceSize::Mini => 4.0,
            MpSpaceSize::Small => 8.0,
            MpSpaceSize::Medium => 16.0,
            MpSpaceSize::Large => 24.0,
            MpSpaceSize::Fixed(value) => value,
        }
    }
}

#[derive(Live, Widget)]
pub struct MpSpace {
    #[deref]
    view: View,
    #[live]
    direction: MpSpaceDirection,
    #[live]
    align: MpSpaceAlign,
    #[live]
    size: MpSpaceSize,
    #[live(false)]
    wrap: bool,
}

impl LiveHook for MpSpace {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_layout(cx);
    }
}

impl Widget for MpSpace {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpSpace {
    fn sync_layout(&mut self, cx: &mut Cx) {
        let spacing = self.size.value();
        self.view.layout.spacing = spacing;

        let mut align = self.view.layout.align;
        let mut flow = Flow::Right {
            row_align: RowAlign::Top,
            wrap: self.wrap,
        };

        match self.direction {
            MpSpaceDirection::Horizontal => {
                let align_value = match self.align {
                    MpSpaceAlign::Auto => 0.5,
                    MpSpaceAlign::Start => 0.0,
                    MpSpaceAlign::Center => 0.5,
                    MpSpaceAlign::End => 1.0,
                    MpSpaceAlign::Baseline => 0.0,
                };
                align.x = 0.0;
                align.y = align_value;

                if matches!(self.align, MpSpaceAlign::Baseline) {
                    flow = Flow::Right {
                        row_align: RowAlign::Bottom,
                        wrap: self.wrap,
                    };
                }
            }
            MpSpaceDirection::Vertical => {
                let align_value = match self.align {
                    MpSpaceAlign::Auto => 0.0,
                    MpSpaceAlign::Start => 0.0,
                    MpSpaceAlign::Center => 0.5,
                    MpSpaceAlign::End => 1.0,
                    MpSpaceAlign::Baseline => 0.0,
                };
                align.x = align_value;
                align.y = 0.0;
                flow = Flow::Down;
            }
        }

        self.view.layout.flow = flow;
        self.view.layout.align = align;
        self.view.redraw(cx);
    }
}
