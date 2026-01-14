use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Icon paths
    ICON_SEARCH = dep("crate://self/resources/icons/search.svg")
    ICON_CLOSE = dep("crate://self/resources/icons/close.svg")
    ICON_EYE = dep("crate://self/resources/icons/eye.svg")
    ICON_EYE_OFF = dep("crate://self/resources/icons/eye-off.svg")

    // Base input style
    MpInputBase = <TextInput> {
        width: Fill,
        height: Fit,

        padding: { left: 12, right: 12, top: 10, bottom: 10 }

        empty_text: "Enter text..."

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance disabled: 0.0

            uniform border_radius: 6.0
            uniform border_width: 1.0

            // Colors
            uniform bg_color: #FFFFFF
            uniform bg_color_hover: #FAFAFA
            uniform bg_color_focus: #FFFFFF
            uniform bg_color_disabled: #F5F5F5

            uniform border_color: #E0E0E0
            uniform border_color_hover: #BDBDBD
            uniform border_color_focus: #4A90D9
            uniform border_color_disabled: #EEEEEE

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background
                let bg = mix(
                    mix(self.bg_color, self.bg_color_hover, self.hover),
                    self.bg_color_focus,
                    self.focus
                );
                let bg_final = mix(bg, self.bg_color_disabled, self.disabled);

                // Border
                let border = mix(
                    mix(self.border_color, self.border_color_hover, self.hover),
                    self.border_color_focus,
                    self.focus
                );
                let border_final = mix(border, self.border_color_disabled, self.disabled);

                // Draw rounded rectangle
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.border_radius
                );

                sdf.fill_keep(bg_final);

                // Focus ring (thicker border when focused)
                let border_w = mix(self.border_width, 2.0, self.focus);
                sdf.stroke(border_final, border_w);

                return sdf.result;
            }
        }

        draw_text: {
            instance disabled: 0.0

            uniform color: #333333
            uniform color_disabled: #9E9E9E
            uniform color_empty: #9E9E9E

            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }

            fn get_color(self) -> vec4 {
                return mix(
                    mix(self.color, self.color_empty, self.empty),
                    self.color_disabled,
                    self.disabled
                );
            }
        }

        draw_cursor: {
            instance focus: 0.0
            uniform color: #4A90D9

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 1.0);
                sdf.fill(mix(#0000, self.color, self.focus * (1.0 - self.blink)));
                return sdf.result;
            }
        }

        draw_selection: {
            uniform color: #4A90D920

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
            }
            focus = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {focus: 0.0}
                        draw_cursor: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {focus: 1.0}
                        draw_cursor: {focus: 1.0}
                    }
                }
            }
            disabled = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {disabled: 0.0}
                        draw_text: {disabled: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {disabled: 1.0}
                        draw_text: {disabled: 1.0}
                    }
                }
            }
        }
    }

    // Default input
    pub MpInput = <MpInputBase> {}

    // Small input
    pub MpInputSmall = <MpInputBase> {
        padding: { left: 8, right: 8, top: 6, bottom: 6 }

        draw_bg: {
            uniform border_radius: 4.0
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 12.0
            }
        }
    }

    // Large input
    pub MpInputLarge = <MpInputBase> {
        padding: { left: 16, right: 16, top: 14, bottom: 14 }

        draw_bg: {
            uniform border_radius: 8.0
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 16.0
            }
        }
    }

    // Password input with toggle icon
    pub MpInputPassword = {{MpInputPassword}} {
        width: Fill,
        height: Fit,

        flow: Right,
        align: { y: 0.5 },
        padding: { left: 12, right: 12, top: 10, bottom: 10 }
        spacing: 8

        show_bg: true,
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0

            uniform border_radius: 6.0
            uniform border_width: 1.0

            uniform bg_color: #FFFFFF
            uniform bg_color_hover: #FAFAFA
            uniform bg_color_focus: #FFFFFF

            uniform border_color: #E0E0E0
            uniform border_color_hover: #BDBDBD
            uniform border_color_focus: #4A90D9

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(
                    mix(self.bg_color, self.bg_color_hover, self.hover),
                    self.bg_color_focus,
                    self.focus
                );

                let border = mix(
                    mix(self.border_color, self.border_color_hover, self.hover),
                    self.border_color_focus,
                    self.focus
                );

                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.border_radius
                );

                sdf.fill_keep(bg);
                let border_w = mix(self.border_width, 2.0, self.focus);
                sdf.stroke(border, border_w);

                return sdf.result;
            }
        }

        // Password text input (borderless)
        input = <TextInput> {
            width: Fill,
            height: Fit,
            is_password: true,
            empty_text: "Enter password..."

            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }

            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix(#333333, #9E9E9E, self.empty);
                }
            }

            draw_cursor: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 1.0);
                    sdf.fill(mix(#0000, #4A90D9, self.focus * (1.0 - self.blink)));
                    return sdf.result;
                }
            }

            draw_selection: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                    sdf.fill(#4A90D920);
                    return sdf.result;
                }
            }
        }

        // Eye icon for toggle visibility (clickable)
        eye_icon = <Button> {
            width: Fit,
            height: Fit,
            padding: 4,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }
            draw_icon: {
                svg_file: (ICON_EYE_OFF)
                color: #9E9E9E
                color_hover: #666666
            }
            icon_walk: { width: 18.0, height: 18.0 }
        }
    }

    // Numeric input
    pub MpInputNumeric = <MpInputBase> {
        is_numeric_only: true
        empty_text: "Enter number..."
    }

    // Borderless input (for inline editing)
    pub MpInputBorderless = <MpInputBase> {
        draw_bg: {
            uniform bg_color: #00000000
            uniform bg_color_hover: #F5F5F5
            uniform bg_color_focus: #00000000
            uniform border_color: #00000000
            uniform border_color_hover: #00000000
            uniform border_color_focus: #4A90D9
        }
    }

    // Search input with icon (capsule/pill shape)
    pub MpInputSearch = <View> {
        width: Fill,
        height: Fit,

        flow: Right,
        align: { y: 0.5 },
        padding: { left: 16, right: 16, top: 10, bottom: 10 }
        spacing: 8

        show_bg: true,
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0

            uniform bg_color: #F5F5F5
            uniform bg_color_hover: #EEEEEE
            uniform bg_color_focus: #FFFFFF

            uniform border_color: #E0E0E0
            uniform border_color_hover: #BDBDBD
            uniform border_color_focus: #4A90D9

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;

                let bg = mix(
                    mix(self.bg_color, self.bg_color_hover, self.hover),
                    self.bg_color_focus,
                    self.focus
                );

                let border = mix(
                    mix(self.border_color, self.border_color_hover, self.hover),
                    self.border_color_focus,
                    self.focus
                );

                // Draw capsule: left circle + rectangle + right circle
                sdf.circle(r, r, r);
                sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf.circle(sz.x - r, r, r);

                sdf.fill_keep(bg);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        // Search icon
        search_icon = <Icon> {
            icon_walk: { width: 16.0 }
            draw_icon: {
                svg_file: (ICON_SEARCH)
                color: #9E9E9E
            }
        }

        // Text input (borderless)
        input = <TextInput> {
            width: Fill,
            height: Fit,
            empty_text: "Search..."

            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }

            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix(#333333, #9E9E9E, self.empty);
                }
            }

            draw_cursor: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 1.0);
                    sdf.fill(mix(#0000, #4A90D9, self.focus * (1.0 - self.blink)));
                    return sdf.result;
                }
            }

            draw_selection: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 2.0);
                    sdf.fill(#4A90D920);
                    return sdf.result;
                }
            }
        }
    }
}

pub use makepad_widgets::text_input::TextInputAction as MpInputAction;

// Password input widget with toggle visibility
#[derive(Live, LiveHook, Widget)]
pub struct MpInputPassword {
    #[deref]
    view: View,
    #[rust]
    password_visible: bool,
}

impl Widget for MpInputPassword {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MpInputPassword {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle eye icon button click
        if self.view.button(ids!(eye_icon)).clicked(actions) {
            self.password_visible = !self.password_visible;

            // Toggle password visibility on the input
            let input = self.view.text_input(ids!(input));
            input.set_is_password(cx, !self.password_visible);

            self.view.redraw(cx);
        }
    }
}

impl MpInputPasswordRef {
    /// Get the current password text
    pub fn text(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.view.text_input(ids!(input)).text()
        } else {
            String::new()
        }
    }

    /// Set the password text
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(inner) = self.borrow() {
            inner.view.text_input(ids!(input)).set_text(cx, text);
        }
    }
}
