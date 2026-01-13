use makepad_widgets::*;
use makepad_component::widgets::MpButtonWidgetRefExt;
use makepad_component::widgets::MpCheckboxWidgetRefExt;
use makepad_component::widgets::MpSwitchWidgetRefExt;
use makepad_component::widgets::MpRadioWidgetRefExt;
use makepad_component::widgets::MpProgressWidgetRefExt;
use makepad_component::widgets::MpSliderWidgetRefExt;
use makepad_component::widgets::MpBadgeWidgetRefExt;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_component::theme::colors::*;
    use makepad_component::widgets::button::*;
    use makepad_component::widgets::checkbox::*;
    use makepad_component::widgets::switch::*;
    use makepad_component::widgets::divider::*;
    use makepad_component::widgets::radio::*;
    use makepad_component::widgets::progress::*;
    use makepad_component::widgets::slider::*;
    use makepad_component::widgets::input::*;
    use makepad_component::widgets::badge::*;
    use makepad_component::widgets::tooltip::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "Component Zoo"
                    inner_size: vec2(800, 700)
                }

                show_bg: true
                draw_bg: { color: (BACKGROUND) }

                body = <ScrollYView> {
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    spacing: 24,
                    padding: 24,

                    // Header
                    <Label> {
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{ font_size: 24.0 }
                            color: (FOREGROUND)
                        }
                        text: "Component Zoo"
                    }

                    <Label> {
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                            color: (MUTED_FOREGROUND)
                        }
                        text: "A showcase of makepad-component widgets"
                    }

                    <MpDivider> {}

                    // ===== Button Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Button"
                        }

                        // Button Variants
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Variants"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 12,

                                btn_primary = <MpButtonPrimary> { text: "Primary" }
                                btn_secondary = <MpButtonSecondary> { text: "Secondary" }
                                btn_danger = <MpButtonDanger> { text: "Danger" }
                                btn_ghost = <MpButtonGhost> { text: "Ghost" }
                            }
                        }

                        // Button Sizes
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Sizes"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 12,
                                align: { y: 0.5 }

                                <MpButtonSmall> { text: "Small" }
                                <MpButton> { text: "Medium" }
                                <MpButtonLarge> { text: "Large" }
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Checkbox Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Checkbox"
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 12,

                            checkbox1 = <MpCheckbox> { text: "Option 1" }
                            checkbox2 = <MpCheckbox> { text: "Option 2", checked: true }
                            checkbox3 = <MpCheckbox> { text: "Option 3" }
                        }

                        checkbox_status = <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                color: (MUTED_FOREGROUND)
                            }
                            text: "Selected: Option 2"
                        }
                    }

                    <MpDivider> {}

                    // ===== Switch Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Switch"
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 16,

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 12,
                                align: { y: 0.5 }

                                switch_wifi = <MpSwitch> { on: true }
                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (FOREGROUND)
                                    }
                                    text: "Wi-Fi"
                                }
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 12,
                                align: { y: 0.5 }

                                switch_bluetooth = <MpSwitch> {}
                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (FOREGROUND)
                                    }
                                    text: "Bluetooth"
                                }
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 12,
                                align: { y: 0.5 }

                                switch_notifications = <MpSwitch> { on: true }
                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (FOREGROUND)
                                    }
                                    text: "Notifications"
                                }
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Divider Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Divider"
                        }

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                color: (MUTED_FOREGROUND)
                            }
                            text: "Horizontal"
                        }

                        <MpDivider> {}

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                color: (MUTED_FOREGROUND)
                            }
                            text: "With Label"
                        }

                        <MpDividerWithLabel> { text: "OR" }
                    }

                    <MpDivider> {}

                    // ===== Radio Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Radio"
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 12,

                            radio_small = <MpRadio> { text: "Small", value: "small", checked: true }
                            radio_medium = <MpRadio> { text: "Medium", value: "medium" }
                            radio_large = <MpRadio> { text: "Large", value: "large" }
                        }

                        radio_status = <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                color: (MUTED_FOREGROUND)
                            }
                            text: "Selected: Small"
                        }
                    }

                    <MpDivider> {}

                    // ===== Progress Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Progress"
                        }

                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 12,

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 4,

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Default (25%)"
                                }
                                progress1 = <MpProgress> { width: Fill, value: 25.0 }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 4,

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Success (50%)"
                                }
                                <MpProgressSuccess> { width: Fill, value: 50.0 }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 4,

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Warning (75%)"
                                }
                                <MpProgressWarning> { width: Fill, value: 75.0 }
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 4,

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Danger (100%)"
                                }
                                <MpProgressDanger> { width: Fill, value: 100.0 }
                            }
                        }

                        // Interactive progress
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,
                            margin: { top: 8 }

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Interactive"
                            }

                            interactive_progress = <MpProgress> { width: Fill, value: 0.0 }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 8,

                                progress_dec_btn = <MpButtonSecondary> { text: "-10%" }
                                progress_inc_btn = <MpButtonPrimary> { text: "+10%" }
                                progress_label = <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (FOREGROUND)
                                    }
                                    text: "0%"
                                }
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Slider Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Slider"
                        }

                        // Color variants
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Variants"
                            }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 12,

                                slider_default = <MpSlider> { value: 50.0 }
                                <MpSliderSuccess> { value: 75.0 }
                                <MpSliderWarning> { value: 30.0 }
                                <MpSliderDanger> { value: 90.0 }
                            }

                            slider_default_label = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Value: 50"
                            }
                        }

                        // Disabled
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Disabled"
                            }

                            <MpSlider> { value: 40.0, disabled: true }
                        }

                        // Range mode
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Range Mode (dual thumbs)"
                            }

                            slider_range = <MpSlider> {
                                value_start: 25.0,
                                value: 75.0,
                                range_mode: true
                            }

                            slider_range_label = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Range: 25 - 75"
                            }
                        }

                        // Logarithmic scale
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Logarithmic Scale (1-1000)"
                            }

                            slider_log = <MpSlider> {
                                min: 1.0,
                                max: 1000.0,
                                value: 100.0,
                                logarithmic: true,
                                step: 0.0
                            }

                            slider_log_label = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Value: 100"
                            }
                        }

                        // Vertical sliders
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Vertical"
                            }

                            <View> {
                                width: Fit, height: 120,
                                flow: Right,
                                spacing: 24,

                                slider_vert = <MpSlider> { width: 24, height: Fill, vertical: true, value: 60.0 }
                                <MpSliderSuccess> { width: 24, height: Fill, vertical: true, value: 80.0 }
                                <MpSliderWarning> { width: 24, height: Fill, vertical: true, value: 40.0 }
                                <MpSliderDanger> { width: 24, height: Fill, vertical: true, value: 20.0 }
                            }

                            slider_vert_label = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Vertical value: 60"
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Input Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Input"
                        }

                        // Default input
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Default"
                            }

                            input_default = <MpInput> {
                                width: 300,
                                empty_text: "Enter your name..."
                            }
                        }

                        // Sizes
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Sizes"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Down,
                                spacing: 12,

                                <MpInputSmall> {
                                    width: 250,
                                    empty_text: "Small input..."
                                }
                                <MpInput> {
                                    width: 300,
                                    empty_text: "Medium input..."
                                }
                                <MpInputLarge> {
                                    width: 350,
                                    empty_text: "Large input..."
                                }
                            }
                        }

                        // Special inputs
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Special Types"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Down,
                                spacing: 12,

                                input_password = <MpInputPassword> {
                                    width: 300
                                }
                                <MpInputNumeric> {
                                    width: 200,
                                    empty_text: "Numbers only..."
                                }
                                <MpInputSearch> {
                                    width: 300
                                }
                            }
                        }

                        // Borderless input
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Borderless (inline editing)"
                            }

                            <MpInputBorderless> {
                                width: 300,
                                empty_text: "Click to edit..."
                            }
                        }

                        // Input with value display
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Interactive"
                            }

                            input_interactive = <MpInput> {
                                width: 300,
                                empty_text: "Type something..."
                            }

                            input_status = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Value: (empty)"
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Badge Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Badge"
                        }

                        // Badge with numbers
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "With Count"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 32,
                                padding: { top: 8, bottom: 8 }

                                // Badge on button
                                <MpBadge> {
                                    count: 5
                                    content = { <MpButton> { text: "Messages" } }
                                }

                                // Badge on button (large count)
                                <MpBadge> {
                                    count: 128
                                    content = { <MpButton> { text: "Notifications" } }
                                }

                                // Badge on icon (simulated with view)
                                <MpBadge> {
                                    count: 3
                                    content = {
                                        <View> {
                                            width: 40, height: 40,
                                            show_bg: true,
                                            draw_bg: { color: #E5E7EB }
                                        }
                                    }
                                }
                            }
                        }

                        // Color variants
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Color Variants"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 32,
                                padding: { top: 8, bottom: 8 }

                                <MpBadge> {
                                    count: 5
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeSuccess> {
                                    count: 12
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeWarning> {
                                    count: 8
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeInfo> {
                                    count: 99
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeSecondary> {
                                    count: 7
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }
                            }
                        }

                        // Dot badges
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Dot Style"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 32,
                                padding: { top: 8, bottom: 8 }

                                <MpBadgeDot> {
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeDotSuccess> {
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }

                                <MpBadgeDotWarning> {
                                    content = { <View> { width: 40, height: 40, show_bg: true, draw_bg: { color: #E5E7EB } } }
                                }
                            }
                        }

                        // Standalone badges (inline)
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Standalone (inline)"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 8,
                                align: { y: 0.5 }
                                padding: { top: 8, bottom: 8 }

                                <MpBadgeStandalone> { label = { text: "5" } }
                                <MpBadgeStandaloneSuccess> { label = { text: "New" } }
                                <MpBadgeStandaloneWarning> { label = { text: "99+" } }
                                <MpBadgeStandaloneInfo> { label = { text: "Beta" } }
                            }
                        }

                        // Interactive badge
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Interactive"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 16,
                                align: { y: 0.5 }
                                padding: { top: 8, bottom: 8 }

                                interactive_badge = <MpBadge> {
                                    count: 0
                                    show_zero: true
                                    content = {
                                        <View> {
                                            width: 48, height: 48,
                                            show_bg: true,
                                            draw_bg: {
                                                color: #3B82F6
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                    sdf.fill(self.color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }

                                badge_inc_btn = <MpButtonPrimary> { text: "+1" }
                                badge_dec_btn = <MpButtonSecondary> { text: "-1" }

                                badge_count_label = <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (FOREGROUND)
                                    }
                                    text: "Count: 0"
                                }
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Tooltip Section =====
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Tooltip"
                        }

                        // Tooltip positions
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Positions"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 24,
                                padding: { top: 32, bottom: 16 }

                                <MpTooltipTop> {
                                    tip: "This tooltip appears on top"
                                    content = { <MpButtonSecondary> { text: "Top" } }
                                }

                                <MpTooltipBottom> {
                                    tip: "This tooltip appears below"
                                    content = { <MpButtonSecondary> { text: "Bottom" } }
                                }

                                <MpTooltipLeft> {
                                    tip: "Left side"
                                    content = { <MpButtonSecondary> { text: "Left" } }
                                }

                                <MpTooltipRight> {
                                    tip: "Right side"
                                    content = { <MpButtonSecondary> { text: "Right" } }
                                }
                            }
                        }

                        // Tooltip examples
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "On Buttons"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 24,
                                padding: { top: 16, bottom: 16 }

                                <MpTooltipTop> {
                                    tip: "Primary action button"
                                    content = { <MpButtonPrimary> { text: "Primary" } }
                                }

                                <MpTooltipTop> {
                                    tip: "Secondary action button"
                                    content = { <MpButtonSecondary> { text: "Secondary" } }
                                }

                                <MpTooltipTop> {
                                    tip: "This will delete the item permanently"
                                    content = { <MpButtonDanger> { text: "Delete" } }
                                }
                            }
                        }

                        // Tooltip on different elements
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "On Different Elements"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 24,
                                align: { y: 0.5 }
                                padding: { top: 16, bottom: 16 }

                                <MpTooltipTop> {
                                    tip: "This is a helpful tip for the icon"
                                    content = {
                                        <View> {
                                            width: 40, height: 40,
                                            show_bg: true,
                                            draw_bg: {
                                                color: #3B82F6
                                                fn pixel(self) -> vec4 {
                                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                    sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 20.0);
                                                    sdf.fill(self.color);
                                                    return sdf.result;
                                                }
                                            }
                                        }
                                    }
                                }

                                <MpTooltipTop> {
                                    tip: "Click to learn more"
                                    content = {
                                        <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: #3B82F6
                                            }
                                            text: "Hover over me"
                                        }
                                    }
                                }

                                <MpTooltipTop> {
                                    tip: "Danger zone! This action cannot be undone."
                                    content = { <MpButtonDanger> { text: "Delete" } }
                                }
                            }
                        }

                        // Advanced options
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Advanced Options"
                            }

                            <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: 24,
                                padding: { top: 16, bottom: 16 }

                                // Custom delay (0.5s)
                                <MpTooltipTop> {
                                    tip: "This tooltip has a 0.5s delay"
                                    show_delay: 0.5
                                    content = { <MpButtonSecondary> { text: "Delay 0.5s" } }
                                }

                                // Instant (no delay)
                                <MpTooltipTop> {
                                    tip: "This tooltip appears instantly"
                                    show_delay: 0.0
                                    content = { <MpButtonSecondary> { text: "Instant" } }
                                }

                                // Custom gap
                                <MpTooltipTop> {
                                    tip: "This tooltip has a larger gap (10px)"
                                    gap: 10.0
                                    content = { <MpButtonSecondary> { text: "Gap 10px" } }
                                }

                                // Custom arrow size
                                <MpTooltipTop> {
                                    tip: "This tooltip has a larger arrow"
                                    arrow_size: vec2(16.0, 10.0)
                                    content = { <MpButtonSecondary> { text: "Large Arrow" } }
                                }
                            }
                        }

                        // Edge detection demo
                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            spacing: 8,

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Edge Detection (try near window edges)"
                            }

                            <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
                                    color: (MUTED_FOREGROUND)
                                }
                                text: "Tooltips automatically flip when they would go off-screen"
                            }
                        }
                    }

                    <MpDivider> {}

                    // ===== Interactive Demo =====
                    <View> {
                        width: Fit, height: Fit,
                        flow: Down,
                        spacing: 16,

                        <Label> {
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                                color: (FOREGROUND)
                            }
                            text: "Interactive Demo"
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Right,
                            spacing: 16,
                            align: { y: 0.5 }

                            counter_btn = <MpButtonPrimary> { text: "Click me!" }

                            counter_label = <Label> {
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                    color: (FOREGROUND)
                                }
                                text: "Clicked: 0 times"
                            }
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        makepad_component::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        self.counter = 0;
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle counter button
        if self.ui.mp_button(ids!(counter_btn)).clicked(&actions) {
            self.counter += 1;
            self.ui.label(ids!(counter_label))
                .set_text(cx, &format!("Clicked: {} times", self.counter));
        }

        // Handle checkbox changes
        if let Some(checked) = self.ui.mp_checkbox(ids!(checkbox1)).changed(&actions) {
            self.update_checkbox_status(cx);
            log!("Checkbox 1 changed: {}", checked);
        }
        if let Some(checked) = self.ui.mp_checkbox(ids!(checkbox2)).changed(&actions) {
            self.update_checkbox_status(cx);
            log!("Checkbox 2 changed: {}", checked);
        }
        if let Some(checked) = self.ui.mp_checkbox(ids!(checkbox3)).changed(&actions) {
            self.update_checkbox_status(cx);
            log!("Checkbox 3 changed: {}", checked);
        }

        // Handle switch changes
        if let Some(on) = self.ui.mp_switch(ids!(switch_wifi)).changed(&actions) {
            log!("Wi-Fi: {}", if on { "ON" } else { "OFF" });
        }
        if let Some(on) = self.ui.mp_switch(ids!(switch_bluetooth)).changed(&actions) {
            log!("Bluetooth: {}", if on { "ON" } else { "OFF" });
        }
        if let Some(on) = self.ui.mp_switch(ids!(switch_notifications)).changed(&actions) {
            log!("Notifications: {}", if on { "ON" } else { "OFF" });
        }

        // Handle radio changes (mutually exclusive)
        if self.ui.mp_radio(ids!(radio_small)).changed(&actions).is_some() {
            self.ui.mp_radio(ids!(radio_medium)).set_checked(cx, false);
            self.ui.mp_radio(ids!(radio_large)).set_checked(cx, false);
            self.ui.label(ids!(radio_status)).set_text(cx, "Selected: Small");
            log!("Radio: Small");
        }
        if self.ui.mp_radio(ids!(radio_medium)).changed(&actions).is_some() {
            self.ui.mp_radio(ids!(radio_small)).set_checked(cx, false);
            self.ui.mp_radio(ids!(radio_large)).set_checked(cx, false);
            self.ui.label(ids!(radio_status)).set_text(cx, "Selected: Medium");
            log!("Radio: Medium");
        }
        if self.ui.mp_radio(ids!(radio_large)).changed(&actions).is_some() {
            self.ui.mp_radio(ids!(radio_small)).set_checked(cx, false);
            self.ui.mp_radio(ids!(radio_medium)).set_checked(cx, false);
            self.ui.label(ids!(radio_status)).set_text(cx, "Selected: Large");
            log!("Radio: Large");
        }

        // Handle progress buttons
        if self.ui.mp_button(ids!(progress_inc_btn)).clicked(&actions) {
            let current = self.ui.mp_progress(ids!(interactive_progress)).value();
            let new_value = (current + 10.0).min(100.0);
            self.ui.mp_progress(ids!(interactive_progress)).set_value(cx, new_value);
            self.ui.label(ids!(progress_label)).set_text(cx, &format!("{}%", new_value as i32));
        }
        if self.ui.mp_button(ids!(progress_dec_btn)).clicked(&actions) {
            let current = self.ui.mp_progress(ids!(interactive_progress)).value();
            let new_value = (current - 10.0).max(0.0);
            self.ui.mp_progress(ids!(interactive_progress)).set_value(cx, new_value);
            self.ui.label(ids!(progress_label)).set_text(cx, &format!("{}%", new_value as i32));
        }

        // Handle slider changes
        if let Some(value) = self.ui.mp_slider(ids!(slider_default)).changed(&actions) {
            let v = value.end();
            self.ui.label(ids!(slider_default_label)).set_text(cx, &format!("Value: {}", v as i32));
            log!("Slider value: {}", v);
        }

        // Handle range slider changes
        if let Some(value) = self.ui.mp_slider(ids!(slider_range)).changed(&actions) {
            let start = value.start();
            let end = value.end();
            self.ui.label(ids!(slider_range_label)).set_text(cx, &format!("Range: {} - {}", start as i32, end as i32));
            log!("Range slider: {} - {}", start, end);
        }

        // Handle logarithmic slider changes
        if let Some(value) = self.ui.mp_slider(ids!(slider_log)).changed(&actions) {
            let v = value.end();
            self.ui.label(ids!(slider_log_label)).set_text(cx, &format!("Value: {:.1}", v));
            log!("Log slider value: {}", v);
        }

        // Handle vertical slider changes
        if let Some(value) = self.ui.mp_slider(ids!(slider_vert)).changed(&actions) {
            let v = value.end();
            self.ui.label(ids!(slider_vert_label)).set_text(cx, &format!("Vertical value: {}", v as i32));
            log!("Vertical slider value: {}", v);
        }

        // Handle input changes
        if let Some(text) = self.ui.text_input(ids!(input_interactive)).changed(&actions) {
            let display = if text.is_empty() {
                "Value: (empty)".to_string()
            } else {
                format!("Value: {}", text)
            };
            self.ui.label(ids!(input_status)).set_text(cx, &display);
            log!("Input changed: {}", text);
        }

        // Handle badge buttons
        if self.ui.mp_button(ids!(badge_inc_btn)).clicked(&actions) {
            let current = self.ui.mp_badge(ids!(interactive_badge)).count();
            let new_count = current + 1;
            self.ui.mp_badge(ids!(interactive_badge)).set_count(cx, new_count);
            self.ui.label(ids!(badge_count_label)).set_text(cx, &format!("Count: {}", new_count));
        }
        if self.ui.mp_button(ids!(badge_dec_btn)).clicked(&actions) {
            let current = self.ui.mp_badge(ids!(interactive_badge)).count();
            let new_count = (current - 1).max(0);
            self.ui.mp_badge(ids!(interactive_badge)).set_count(cx, new_count);
            self.ui.label(ids!(badge_count_label)).set_text(cx, &format!("Count: {}", new_count));
        }
    }
}

impl App {
    fn update_checkbox_status(&mut self, cx: &mut Cx) {
        let mut selected = Vec::new();

        if self.ui.mp_checkbox(ids!(checkbox1)).is_checked() {
            selected.push("Option 1");
        }
        if self.ui.mp_checkbox(ids!(checkbox2)).is_checked() {
            selected.push("Option 2");
        }
        if self.ui.mp_checkbox(ids!(checkbox3)).is_checked() {
            selected.push("Option 3");
        }

        let status = if selected.is_empty() {
            "Selected: None".to_string()
        } else {
            format!("Selected: {}", selected.join(", "))
        };

        self.ui.label(ids!(checkbox_status)).set_text(cx, &status);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
