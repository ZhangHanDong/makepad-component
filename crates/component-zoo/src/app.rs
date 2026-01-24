use makepad_widgets::*;
use makepad_component::widgets::MpButtonWidgetRefExt;
use makepad_component::widgets::MpCheckboxWidgetRefExt;
use makepad_component::widgets::MpSwitchWidgetRefExt;
use makepad_component::widgets::MpRadioWidgetRefExt;
use makepad_component::widgets::MpProgressWidgetRefExt;
use makepad_component::widgets::MpSliderWidgetRefExt;
use makepad_component::widgets::MpBadgeWidgetRefExt;
use makepad_component::widgets::MpTabWidgetRefExt;
use makepad_component::widgets::MpCardAction;
use makepad_component::widgets::MpAvatarWidgetRefExt;
use makepad_component::widgets::MpModalAction;
use makepad_component::widgets::MpModalWidgetWidgetRefExt;
use makepad_component::widgets::MpNotificationWidgetWidgetRefExt;
use makepad_component::widgets::MpSkeletonWidgetWidgetRefExt;
use makepad_component::widgets::MpPopoverWidgetWidgetRefExt;

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
    use makepad_component::widgets::dropdown::*;
    use makepad_component::widgets::page_flip::*;
    use makepad_component::widgets::tab::*;
    use makepad_component::widgets::card::*;
    use makepad_component::widgets::layout::*;
    use makepad_component::widgets::avatar::*;
    use makepad_component::widgets::skeleton::*;
    use makepad_component::widgets::spinner::*;
    use makepad_component::widgets::accordion::*;
    use makepad_component::widgets::list::*;
    use makepad_component::widgets::notification::*;
    use makepad_component::widgets::modal::*;
    use makepad_component::widgets::popover::*;
    use makepad_component::widgets::label::*;
    use makepad_component::widgets::text::*;
    use makepad_component::widgets::alert::*;

    // ============================================================
    // Section Header Component
    // ============================================================
    SectionHeader = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
            color: (FOREGROUND)
        }
    }

    SubsectionLabel = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
            color: (MUTED_FOREGROUND)
        }
    }

    // ============================================================
    // Category Tab Style
    // ============================================================
    CategoryTab = <MpTabPill> {
        padding: { left: 16, right: 16, top: 8, bottom: 8 }
    }

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "Component Zoo"
                    inner_size: vec2(1280, 900)
                }

                show_bg: true
                draw_bg: { color: (BACKGROUND) }

                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Overlay,

                    // Main content area
                    main_content = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Down,

                    // Header area
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        padding: { left: 24, right: 24, top: 24, bottom: 16 },
                        spacing: 8,

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
                    }

                    // Category Tab Bar
                    <View> {
                        width: Fill, height: Fit,
                        padding: { left: 24, right: 24, bottom: 16 },

                        <MpTabBarPill> {
                            cat_form = <CategoryTab> { text: "Form" }
                            cat_display = <CategoryTab> { text: "Display" }
                            cat_layout = <CategoryTab> { text: "Layout" }
                            cat_nav = <CategoryTab> { text: "Navigation" }
                            cat_feedback = <CategoryTab> { text: "Feedback" }
                            cat_data = <CategoryTab> { text: "Data" }
                        }
                    }

                    <MpDivider> {}

                    // Content area with PageFlip
                    category_pages = <PageFlip> {
                        width: Fill,
                        height: Fill,
                        active_page: page_form,

                        // ============================================================
                        // Form Controls Page
                        // ============================================================
                        page_form = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 200 }

                            show_bg: true
                            draw_bg: { color: #e2e8f0 }

                            // ===== Button Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Button" }

                                // Button Variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Variants" }

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

                                    <SubsectionLabel> { text: "Sizes" }

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

                                <SectionHeader> { text: "Checkbox" }

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

                                <SectionHeader> { text: "Switch" }

                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 16,

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 12,
                                        align: { y: 0.5 }
                                        switch_wifi = <MpSwitch> {}
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
                                        switch_bluetooth = <MpSwitch> { on: true }
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
                                        switch_notifications = <MpSwitch> {}
                                        <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "Notifications"
                                        }
                                    }
                                }

                                // Multiple switches
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "All On" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSwitch> { on: true }
                                        <MpSwitch> { on: true }
                                        <MpSwitch> { on: true }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Radio Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Radio" }

                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    radio_small = <MpRadio> { text: "Small" }
                                    radio_medium = <MpRadio> { text: "Medium", checked: true }
                                    radio_large = <MpRadio> { text: "Large" }
                                }

                                radio_status = <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Selected: Medium"
                                }
                            }

                            <MpDivider> {}

                            // ===== Dropdown Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Dropdown" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Basic" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        dropdown_basic = <MpDropdown> {
                                            width: 200,
                                            labels: ["Apple", "Banana", "Cherry", "Date", "Elderberry"]
                                        }

                                        dropdown_status = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Selected: Apple"
                                        }
                                    }
                                }

                                // Dropdown variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Variants" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,

                                        <MpDropdown> {
                                            width: 180,
                                            labels: ["Default", "Option 2", "Option 3"]
                                        }

                                        <MpDropdownOutline> {
                                            width: 180,
                                            labels: ["Outline", "Option 2", "Option 3"]
                                        }

                                        <MpDropdownGhost> {
                                            width: 180,
                                            labels: ["Ghost", "Option 2", "Option 3"]
                                        }
                                    }
                                }

                                // Dropdown sizes
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Sizes" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpDropdownSmall> {
                                            width: 140,
                                            labels: ["Small", "Option 2"]
                                        }

                                        <MpDropdown> {
                                            width: 150,
                                            labels: ["Medium", "Option 2"]
                                        }

                                        <MpDropdownLarge> {
                                            width: 160,
                                            labels: ["Large", "Option 2"]
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

                                <SectionHeader> { text: "Slider" }

                                // Default Slider
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Default" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        slider_default = <MpSlider> {
                                            width: 300,
                                            min: 0.0, max: 100.0, value: 50.0, step: 1.0,
                                        }

                                        slider_default_label = <Label> {
                                            width: 100, height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "Value: 50"
                                        }
                                    }
                                }

                                // Slider Colors
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Colors" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpSlider> {
                                            width: 300,
                                            min: 0.0, max: 100.0, value: 60.0, step: 1.0,
                                        }

                                        <MpSliderSuccess> {
                                            width: 300,
                                            min: 0.0, max: 100.0, value: 80.0, step: 1.0,
                                        }

                                        <MpSliderWarning> {
                                            width: 300,
                                            min: 0.0, max: 100.0, value: 40.0, step: 1.0,
                                        }

                                        <MpSliderDanger> {
                                            width: 300,
                                            min: 0.0, max: 100.0, value: 20.0, step: 1.0,
                                        }
                                    }
                                }

                                // Vertical Slider
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Vertical" }

                                    <View> {
                                        width: Fill, height: 150,
                                        flow: Right,
                                        spacing: 16,

                                        slider_vert = <MpSliderVertical> {
                                            height: Fill,
                                            min: 0.0, max: 100.0, value: 30.0, step: 1.0,
                                        }

                                        slider_vert_label = <Label> {
                                            width: 120, height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "Vertical value: 30"
                                        }
                                    }
                                }

                                // Range Slider
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Range Slider" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        slider_range = <MpSlider> {
                                            width: 300,
                                            min: 0.0, max: 100.0,
                                            value_start: 20.0, value: 80.0,
                                            range_mode: true, step: 1.0,
                                        }

                                        slider_range_label = <Label> {
                                            width: 150, height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "Range: 20 - 80"
                                        }
                                    }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        slider_range_success = <MpSliderSuccess> {
                                            width: 300,
                                            min: 0.0, max: 100.0,
                                            value_start: 30.0, value: 70.0,
                                            range_mode: true, step: 5.0,
                                        }

                                        slider_range_success_label = <Label> {
                                            width: 150, height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "Range: 30 - 70 (step 5)"
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Input Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Input" }

                                // Input variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Variants" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,

                                        <MpInput> {
                                            width: 200,
                                            empty_text: "Default input"
                                        }

                                        <MpInputBorderless> {
                                            width: 200,
                                            empty_text: "Borderless input"
                                        }
                                    }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,

                                        <MpInputPassword> {
                                            width: 200,
                                            input = { empty_text: "Password input" }
                                        }

                                        <MpInputNumeric> {
                                            width: 200,
                                            empty_text: "Numbers only"
                                        }
                                    }
                                }

                                // Input sizes
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Sizes" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpInputSmall> {
                                            width: 150,
                                            empty_text: "Small"
                                        }

                                        <MpInput> {
                                            width: 150,
                                            empty_text: "Medium"
                                        }

                                        <MpInputLarge> {
                                            width: 150,
                                            empty_text: "Large"
                                        }
                                    }
                                }

                                // Interactive input
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Interactive" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        input_interactive = <MpInput> {
                                            width: 250,
                                            empty_text: "Type something..."
                                        }

                                        input_status = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Value: (empty)"
                                        }
                                    }
                                }
                            }

                        }

                        // ============================================================
                        // Display Page
                        // ============================================================
                        page_display = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 100 }

                            show_bg: true
                            draw_bg: { color: #bbf7d0 }

                            // ===== Label Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Label" }

                                // Size variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Size Variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 1.0 }

                                        <MpLabelXs> { text: "Extra Small" }
                                        <MpLabelSm> { text: "Small" }
                                        <MpLabel> { text: "Medium (default)" }
                                        <MpLabelLg> { text: "Large" }
                                        <MpLabelXl> { text: "Extra Large" }
                                    }
                                }

                                // Color variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Color Variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,

                                        <MpLabel> { text: "Default" }
                                        <MpLabelMuted> { text: "Muted" }
                                        <MpLabelPrimary> { text: "Primary" }
                                        <MpLabelSuccess> { text: "Success" }
                                        <MpLabelWarning> { text: "Warning" }
                                        <MpLabelDanger> { text: "Danger" }
                                        <MpLabelInfo> { text: "Info" }
                                    }
                                }

                                // Headings
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Headings" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <MpHeading1> { text: "Heading 1" }
                                        <MpHeading2> { text: "Heading 2" }
                                        <MpHeading3> { text: "Heading 3" }
                                        <MpHeading4> { text: "Heading 4" }
                                        <MpHeading5> { text: "Heading 5" }
                                        <MpHeading6> { text: "Heading 6" }
                                    }
                                }

                                // Secondary text
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "With Secondary Text" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <MpLabel> {
                                            text: "Username"
                                            secondary: "(required)"
                                        }
                                        <MpLabel> {
                                            text: "Email"
                                            secondary: "optional"
                                        }
                                    }
                                }

                                // Masked text
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Masked Text (Password)" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,

                                        <MpLabel> {
                                            text: "password123"
                                            masked: true
                                        }
                                        <MpLabel> {
                                            text: "secret"
                                            masked: true
                                        }
                                    }
                                }

                                // Highlighted text
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Text Highlighting (Search)" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <MpLabel> {
                                            text: "The quick brown fox jumps over the lazy dog"
                                            highlight: "fox"
                                        }
                                        <MpLabel> {
                                            text: "Hello World, Hello Universe"
                                            highlight: "hello"
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Text Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Text" }

                                // Paragraph text
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Paragraph Text (Word Wrap)" }

                                    <View> {
                                        width: 400, height: Fit,
                                        padding: 16,
                                        show_bg: true,
                                        draw_bg: { color: #ffffff }

                                        <MpText> {
                                            text: "This is a paragraph of text that demonstrates word wrapping. When the text is too long to fit on a single line, it automatically wraps to the next line. This is useful for displaying longer content like descriptions or articles."
                                        }
                                    }
                                }

                                // Size variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Size Variants" }

                                    <View> {
                                        width: 400, height: Fit,
                                        flow: Down,
                                        spacing: 12,
                                        padding: 16,
                                        show_bg: true,
                                        draw_bg: { color: #ffffff }

                                        <MpTextXs> { text: "Extra small text for fine print" }
                                        <MpTextSm> { text: "Small text for captions" }
                                        <MpText> { text: "Medium text (default body)" }
                                        <MpTextLg> { text: "Large text for emphasis" }
                                        <MpTextXl> { text: "Extra large text for intro" }
                                    }
                                }

                                // Color variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Color Variants" }

                                    <View> {
                                        width: 400, height: Fit,
                                        flow: Down,
                                        spacing: 8,
                                        padding: 16,
                                        show_bg: true,
                                        draw_bg: { color: #ffffff }

                                        <MpText> { text: "Default text color" }
                                        <MpTextMuted> { text: "Muted text for secondary info" }
                                        <MpTextPrimary> { text: "Primary colored text" }
                                        <MpTextSuccess> { text: "Success message text" }
                                        <MpTextWarning> { text: "Warning message text" }
                                        <MpTextDanger> { text: "Danger/error message text" }
                                    }
                                }

                                // Special variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Special Variants" }

                                    <View> {
                                        width: 500, height: Fit,
                                        flow: Down,
                                        spacing: 16,
                                        padding: 16,
                                        show_bg: true,
                                        draw_bg: { color: #ffffff }

                                        // Lead text
                                        <MpTextLead> {
                                            text: "This is lead text, perfect for introductory paragraphs that need to stand out."
                                        }

                                        // Inline code
                                        <View> {
                                            width: Fit, height: Fit,
                                            flow: Right,
                                            spacing: 4,
                                            align: { y: 0.5 }

                                            <MpTextInline> { text: "Use the " }
                                            <MpTextCode> { text: "println!()" }
                                            <MpTextInline> { text: " macro to print output." }
                                        }

                                        // Caption
                                        <MpTextCaption> {
                                            text: "Caption: This is a small caption text often used below images or figures."
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Badge Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Badge" }

                                // Badge with count (wrapping content)
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Badge with Count" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        align: { y: 0.5 }

                                        // Default (red)
                                        <MpBadge> {
                                            count: 5
                                            content = {
                                                <MpButtonSecondary> { text: "Messages" }
                                            }
                                        }

                                        // Success (green)
                                        <MpBadgeSuccess> {
                                            count: 3
                                            content = {
                                                <MpButtonSecondary> { text: "Completed" }
                                            }
                                        }

                                        // Warning (orange)
                                        <MpBadgeWarning> {
                                            count: 2
                                            content = {
                                                <MpButtonSecondary> { text: "Pending" }
                                            }
                                        }
                                    }
                                }

                                // Badge color variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Color Variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        align: { y: 0.5 }

                                        <MpBadge> {
                                            count: 9
                                            content = {
                                                <MpButtonGhost> { text: "Default" }
                                            }
                                        }

                                        <MpBadgeInfo> {
                                            count: 12
                                            content = {
                                                <MpButtonGhost> { text: "Info" }
                                            }
                                        }

                                        <MpBadgeSecondary> {
                                            count: 7
                                            content = {
                                                <MpButtonGhost> { text: "Secondary" }
                                            }
                                        }
                                    }
                                }

                                // Dot badges
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Dot Badges" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        align: { y: 0.5 }

                                        <MpBadgeDot> {
                                            content = {
                                                <MpButtonSecondary> { text: "Notifications" }
                                            }
                                        }

                                        <MpBadgeDotSuccess> {
                                            content = {
                                                <MpButtonSecondary> { text: "Online" }
                                            }
                                        }

                                        <MpBadgeDotWarning> {
                                            content = {
                                                <MpButtonSecondary> { text: "Away" }
                                            }
                                        }
                                    }
                                }

                                // Standalone badges
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Standalone (inline)" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 12,
                                        align: { y: 0.5 }

                                        <MpBadgeStandalone> {
                                            label = { text: "5" }
                                        }
                                        <MpBadgeStandaloneSuccess> {
                                            label = { text: "New" }
                                        }
                                        <MpBadgeStandaloneWarning> {
                                            label = { text: "99+" }
                                        }
                                        <MpBadgeStandaloneInfo> {
                                            label = { text: "Beta" }
                                        }
                                    }
                                }

                                // Interactive badge
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Interactive Count" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        badge_dec_btn = <MpButtonGhost> { text: "-" }
                                        interactive_badge = <MpBadge> {
                                            count: 5
                                            content = {
                                                <MpButtonSecondary> { text: "Items" }
                                            }
                                        }
                                        badge_inc_btn = <MpButtonGhost> { text: "+" }

                                        badge_count_label = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Count: 5"
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Avatar Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Avatar" }

                                // Avatar sizes
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Sizes" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpAvatarXSmall> { label = { text: "XS" } }
                                        <MpAvatarSmall> { label = { text: "SM" } }
                                        <MpAvatar> { label = { text: "MD" } }
                                        <MpAvatarLarge> { label = { text: "LG" } }
                                        <MpAvatarXLarge> { label = { text: "XL" } }
                                    }
                                }

                                // Avatar colors
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Colors" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 12,
                                        align: { y: 0.5 }

                                        <MpAvatar> { label = { text: "JD" } }
                                        <MpAvatarPrimary> { label = { text: "AB" } }
                                        <MpAvatarSuccess> { label = { text: "CD" } }
                                        <MpAvatarDanger> { label = { text: "EF" } }
                                        <MpAvatarWarning> { label = { text: "GH" } }
                                    }
                                }

                                // Dynamic avatar
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Dynamic (click to change)" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 12,
                                        align: { y: 0.5 }

                                        dynamic_avatar = <MpAvatar> { label = { text: "??" } }
                                        avatar_change_btn = <MpButtonSecondary> { text: "Random Name" }
                                        avatar_name_label = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Click button..."
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Card Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Card" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 16,

                                    // Basic Card
                                    <MpCard> {
                                        width: 250,
                                        <MpCardHeader> {
                                            <MpCardTitle> { text: "Card Title" }
                                            <MpCardDescription> { text: "Card description text." }
                                        }
                                        <MpCardContent> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "This is the card content area."
                                            }
                                        }
                                        <MpCardFooter> {
                                            <MpButtonGhost> { text: "Cancel" }
                                            <MpButtonPrimary> { text: "Save" }
                                        }
                                    }

                                    // Shadow Card
                                    <MpCardShadow> {
                                        width: 250,
                                        <MpCardHeader> {
                                            <MpCardTitle> { text: "Shadow Card" }
                                            <MpCardDescription> { text: "Card with shadow effect." }
                                        }
                                        <MpCardContent> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Shadow creates depth."
                                            }
                                        }
                                    }
                                }

                                // Card color variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Color Variants" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 12,

                                        <MpCardSuccess> {
                                            width: 180,
                                            padding: 12,
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (SUCCESS)
                                                }
                                                text: "Success Card"
                                            }
                                        }

                                        <MpCardDanger> {
                                            width: 180,
                                            padding: 12,
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (DANGER)
                                                }
                                                text: "Danger Card"
                                            }
                                        }

                                        <MpCardWarning> {
                                            width: 180,
                                            padding: 12,
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: #b45309
                                                }
                                                text: "Warning Card"
                                            }
                                        }

                                        <MpCardInfo> {
                                            width: 180,
                                            padding: 12,
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (INFO)
                                                }
                                                text: "Info Card"
                                            }
                                        }
                                    }
                                }

                                // Clickable Card
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Clickable Card (hover to see effect)" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 12,

                                        clickable_card_1 = <MpCardClickable> {
                                            width: 200,
                                            <MpCardHeader> {
                                                <MpCardTitle> { text: "Click Me" }
                                                <MpCardDescription> { text: "Hover and click" }
                                            }
                                        }

                                        clickable_card_2 = <MpCardClickable> {
                                            width: 200,
                                            <MpCardHeader> {
                                                <MpCardTitle> { text: "Interactive" }
                                                <MpCardDescription> { text: "With hover effect" }
                                            }
                                        }

                                        card_click_status = <Label> {
                                            width: Fit, height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Click a card..."
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

                                <SectionHeader> { text: "Divider" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 20,

                                    <SubsectionLabel> { text: "Horizontal (default)" }
                                    <MpDivider> {}

                                    <SubsectionLabel> { text: "With text" }
                                    <MpDividerWithLabel> { text: "OR" }

                                    <SubsectionLabel> { text: "Thick" }
                                    <MpDividerWithMargin> {}
                                }
                            }

                            <MpDivider> {}

                            // ===== Skeleton Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Skeleton" }

                                // Interactive Skeleton Demo
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Right,
                                        spacing: 8,
                                        align: { y: 0.5 }

                                        skeleton_toggle_btn = <MpButtonPrimary> { text: "Toggle Loading" }

                                        skeleton_status = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Status: Loading"
                                        }
                                    }

                                    // Interactive skeleton widget
                                    interactive_skeleton = <MpSkeletonWidget> {
                                        width: Fill,
                                        height: Fit,

                                        skeleton = <View> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            spacing: 8

                                            <MpSkeletonRounded> { width: 150, height: 20 }
                                            <MpSkeletonRounded> { width: Fill, height: 14 }
                                            <MpSkeletonRounded> { width: 200, height: 14 }
                                        }

                                        content = <View> {
                                            width: Fill, height: Fit,
                                            flow: Down,
                                            spacing: 8

                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 16.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Content Loaded!"
                                            }
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "This is the actual content that appears after loading."
                                            }
                                        }
                                    }
                                }

                                <MpDivider> { margin: { top: 8, bottom: 8 } }

                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Basic shapes" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSkeleton> {
                                            width: 200, height: 20
                                        }

                                        <MpSkeletonCircle> {
                                            width: 48, height: 48
                                        }
                                    }
                                }

                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Card skeleton" }

                                    <MpSkeletonCard> {}
                                }
                            }

                            <MpDivider> {}

                            // ===== Spinner Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Spinner" }

                                // Size variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Size variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSpinnerXs> {}
                                        <MpSpinnerSm> {}
                                        <MpSpinnerMd> {}
                                        <MpSpinnerLg> {}
                                        <MpSpinnerXl> {}
                                    }
                                }

                                // Color variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Color variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSpinnerPrimary> {}
                                        <MpSpinnerSuccess> {}
                                        <MpSpinnerWarning> {}
                                        <MpSpinnerDanger> {}
                                    }
                                }

                                // Style variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Style variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSpinnerThin> {}
                                        <MpSpinner> {}
                                        <MpSpinnerThick> {}
                                        <MpSpinnerNoTrack> {}
                                    }
                                }

                                // Speed variants
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Speed variants" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        <MpSpinnerSlow> {}
                                        <MpSpinner> {}
                                        <MpSpinnerFast> {}
                                    }
                                }

                                // Alternative styles
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Alternative styles" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        align: { y: 0.5 }

                                        <MpSpinnerDots> {}
                                        <MpSpinnerPulse> {}
                                    }
                                }

                                // With label
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "With label" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 32,
                                        align: { y: 0.5 }

                                        <MpSpinnerWithLabel> {}
                                        <MpSpinnerWithLabelVertical> {}
                                    }
                                }
                            }
                        }

                        // ============================================================
                        // Layout Page
                        // ============================================================
                        page_layout = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 100 }

                            show_bg: true
                            draw_bg: { color: #fef3c7 }

                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Layout" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Header / Content / Footer" }

                                    <MpLayout> {
                                        width: Fill, height: Fit,

                                        <MpLayoutHeader> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Header"
                                            }
                                        }

                                        <MpLayoutContent> {
                                            height: 120,
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "Content area"
                                            }
                                        }

                                        <MpLayoutFooter> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "Footer"
                                            }
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Left Sider" }

                                    <MpLayout> {
                                        width: Fill, height: Fit,

                                        <MpLayoutHeader> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Header"
                                            }
                                        }

                                        <MpLayoutBody> {
                                            height: 200,

                                            <MpLayoutSider> {
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
                                                        color: (FOREGROUND)
                                                    }
                                                    text: "Sider"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Overview"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Reports"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Settings"
                                                }
                                            }

                                            <MpLayoutContent> {
                                                height: Fill,
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Main content"
                                                }
                                            }
                                        }

                                        <MpLayoutFooter> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "Footer"
                                            }
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Right Sider" }

                                    <MpLayout> {
                                        width: Fill, height: Fit,

                                        <MpLayoutHeader> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Header"
                                            }
                                        }

                                        <MpLayoutBody> {
                                            height: 200,

                                            <MpLayoutContent> {
                                                height: Fill,
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Main content"
                                                }
                                                <View> {
                                                    width: Fit, height: Fit,
                                                    flow: Right,
                                                    spacing: 12

                                                    <RoundedView> {
                                                        width: 120, height: 48,
                                                        align: { x: 0.5, y: 0.5 }
                                                        draw_bg: {
                                                            color: #e2e8f0
                                                            border_radius: 8.0
                                                        }
                                                        <Label> {
                                                            draw_text: {
                                                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                                color: (MUTED_FOREGROUND)
                                                            }
                                                            text: "Card A"
                                                        }
                                                    }

                                                    <RoundedView> {
                                                        width: 120, height: 48,
                                                        align: { x: 0.5, y: 0.5 }
                                                        draw_bg: {
                                                            color: #e2e8f0
                                                            border_radius: 8.0
                                                        }
                                                        <Label> {
                                                            draw_text: {
                                                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                                color: (MUTED_FOREGROUND)
                                                            }
                                                            text: "Card B"
                                                        }
                                                    }
                                                }
                                            }

                                            <MpLayoutSiderRight> {
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
                                                        color: (FOREGROUND)
                                                    }
                                                    text: "Sider"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Filters"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Profile"
                                                }
                                            }
                                        }

                                        <MpLayoutFooter> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "Footer"
                                            }
                                        }
                                    }
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <SubsectionLabel> { text: "Double Sider" }

                                    <MpLayout> {
                                        width: Fill, height: Fit,

                                        <MpLayoutHeader> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                                                    color: (FOREGROUND)
                                                }
                                                text: "Header"
                                            }
                                        }

                                        <MpLayoutBody> {
                                            height: 220,

                                            <MpLayoutSider> {
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
                                                        color: (FOREGROUND)
                                                    }
                                                    text: "Navigation"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Inbox"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Projects"
                                                }
                                            }

                                            <MpLayoutContent> {
                                                height: Fill,
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Content area"
                                                }
                                                <View> {
                                                    width: Fit, height: Fit,
                                                    flow: Right,
                                                    spacing: 12

                                                    <RoundedView> {
                                                        width: 140, height: 60,
                                                        align: { x: 0.5, y: 0.5 }
                                                        draw_bg: {
                                                            color: #e2e8f0
                                                            border_radius: 10.0
                                                        }
                                                        <Label> {
                                                            draw_text: {
                                                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                                color: (MUTED_FOREGROUND)
                                                            }
                                                            text: "Block 1"
                                                        }
                                                    }

                                                    <RoundedView> {
                                                        width: 140, height: 60,
                                                        align: { x: 0.5, y: 0.5 }
                                                        draw_bg: {
                                                            color: #e2e8f0
                                                            border_radius: 10.0
                                                        }
                                                        <Label> {
                                                            draw_text: {
                                                                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                                color: (MUTED_FOREGROUND)
                                                            }
                                                            text: "Block 2"
                                                        }
                                                    }
                                                }
                                            }

                                            <MpLayoutSiderRight> {
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
                                                        color: (FOREGROUND)
                                                    }
                                                    text: "Details"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Activity"
                                                }
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                        color: (MUTED_FOREGROUND)
                                                    }
                                                    text: "Notes"
                                                }
                                            }
                                        }

                                        <MpLayoutFooter> {
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                                    color: (MUTED_FOREGROUND)
                                                }
                                                text: "Footer"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // ============================================================
                        // Navigation Page
                        // ============================================================
                        page_nav = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 100 }

                            show_bg: true
                            draw_bg: { color: #bfdbfe }

                            // ===== Tab Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Tab" }

                                // Default tabs
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Default" }

                                    <MpTabBar> {
                                        tab_home = <MpTab> { text: "Home" }
                                        tab_profile = <MpTab> { text: "Profile" }
                                        tab_settings = <MpTab> { text: "Settings" }
                                    }
                                }

                                // Underline tabs
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Underline" }

                                    <MpTabBarUnderline> {
                                        tab_u_overview = <MpTabUnderline> { text: "Overview" }
                                        tab_u_analytics = <MpTabUnderline> { text: "Analytics" }
                                        tab_u_reports = <MpTabUnderline> { text: "Reports" }
                                    }
                                }

                                // Pill tabs
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Pill" }

                                    <MpTabBarPill> {
                                        tab_p_all = <MpTabPill> { text: "All" }
                                        tab_p_active = <MpTabPill> { text: "Active" }
                                        tab_p_completed = <MpTabPill> { text: "Completed" }
                                    }
                                }

                                // Outline tabs
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Outline" }

                                    <MpTabBarOutline> {
                                        tab_o_day = <MpTabOutline> { text: "Day" }
                                        tab_o_week = <MpTabOutline> { text: "Week" }
                                        tab_o_month = <MpTabOutline> { text: "Month" }
                                    }
                                }

                                // Segmented tabs
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Segmented" }

                                    <MpTabBarSegmented> {
                                        tab_s_list = <MpTabSegmented> { text: "List" }
                                        tab_s_grid = <MpTabSegmented> { text: "Grid" }
                                        tab_s_map = <MpTabSegmented> { text: "Map" }
                                    }
                                }

                                tab_status = <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Selected: Home"
                                }
                            }

                            <MpDivider> {}

                            // ===== PageFlip Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "PageFlip" }

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "PageFlip enables switching between different pages/views."
                                }

                                // Page navigation buttons
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Right,
                                    spacing: 8,

                                    page_btn_a = <MpButtonPrimary> { text: "Page A" }
                                    page_btn_b = <MpButtonGhost> { text: "Page B" }
                                    page_btn_c = <MpButtonGhost> { text: "Page C" }
                                }

                                // PageFlip container
                                <View> {
                                    width: Fill, height: 120,
                                    show_bg: true,
                                    draw_bg: {
                                        color: (MUTED)
                                    }

                                    demo_page_flip = <PageFlip> {
                                        width: Fill, height: Fill,
                                        active_page: page_a,

                                        page_a = <View> {
                                            width: Fill, height: Fill,
                                            align: { x: 0.5, y: 0.5 }
                                            show_bg: true
                                            draw_bg: { color: #dbeafe }
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 24.0 }
                                                    color: (PRIMARY)
                                                }
                                                text: "Page A Content"
                                            }
                                        }

                                        page_b = <View> {
                                            width: Fill, height: Fill,
                                            align: { x: 0.5, y: 0.5 }
                                            show_bg: true
                                            draw_bg: { color: #dcfce7 }
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 24.0 }
                                                    color: (SUCCESS)
                                                }
                                                text: "Page B Content"
                                            }
                                        }

                                        page_c = <View> {
                                            width: Fill, height: Fill,
                                            align: { x: 0.5, y: 0.5 }
                                            show_bg: true
                                            draw_bg: { color: #fee2e2 }
                                            <Label> {
                                                draw_text: {
                                                    text_style: <THEME_FONT_BOLD>{ font_size: 24.0 }
                                                    color: (DANGER)
                                                }
                                                text: "Page C Content"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // ============================================================
                        // Feedback Page
                        // ============================================================
                        page_feedback = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 100 }

                            show_bg: true
                            draw_bg: { color: #fde68a }

                            // ===== Tooltip Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Tooltip" }

                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Positions" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        padding: { top: 40, bottom: 40 }

                                        <MpTooltipTop> {
                                            tip: "Tooltip on top"
                                            content = {
                                                <MpButtonSecondary> { text: "Top" }
                                            }
                                        }

                                        <MpTooltipBottom> {
                                            tip: "Tooltip on bottom"
                                            content = {
                                                <MpButtonSecondary> { text: "Bottom" }
                                            }
                                        }

                                        <MpTooltipLeft> {
                                            tip: "Tooltip on left"
                                            content = {
                                                <MpButtonSecondary> { text: "Left" }
                                            }
                                        }

                                        <MpTooltipRight> {
                                            tip: "Tooltip on right"
                                            content = {
                                                <MpButtonSecondary> { text: "Right" }
                                            }
                                        }
                                    }
                                }

                                // Delay examples
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Show Delay" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        padding: { top: 20, bottom: 20 }

                                        <MpTooltipTop> {
                                            tip: "Instant tooltip (0s delay)"
                                            show_delay: 0.0
                                            content = {
                                                <MpButtonOutline> { text: "Instant" }
                                            }
                                        }

                                        <MpTooltipTop> {
                                            tip: "Default delay (0.3s)"
                                            content = {
                                                <MpButtonOutline> { text: "Default 0.3s" }
                                            }
                                        }

                                        <MpTooltipTop> {
                                            tip: "Slow tooltip (1s delay)"
                                            show_delay: 1.0
                                            content = {
                                                <MpButtonOutline> { text: "Slow 1s" }
                                            }
                                        }

                                        <MpTooltipTop> {
                                            tip: "Very slow tooltip (2s delay)"
                                            show_delay: 2.0
                                            content = {
                                                <MpButtonOutline> { text: "Very Slow 2s" }
                                            }
                                        }
                                    }
                                }

                                // Tooltip on different components
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "On Components" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 24,
                                        align: { y: 0.5 }
                                        padding: { top: 20, bottom: 20 }

                                        // Tooltip on Checkbox
                                        <MpTooltipTop> {
                                            tip: "Check this to enable feature"
                                            content = {
                                                <MpCheckbox> {
                                                    text: "Checkbox"
                                                }
                                            }
                                        }

                                        // Tooltip on Switch
                                        <MpTooltipTop> {
                                            tip: "Toggle to turn on/off"
                                            content = {
                                                <View> {
                                                    width: Fit, height: Fit,
                                                    flow: Right,
                                                    spacing: 8,
                                                    align: { y: 0.5 }
                                                    <MpSwitch> {}
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (FOREGROUND)
                                                        }
                                                        text: "Switch"
                                                    }
                                                }
                                            }
                                        }

                                        // Tooltip on Radio
                                        <MpTooltipTop> {
                                            tip: "Select this option"
                                            content = {
                                                <MpRadio> {
                                                    text: "Radio"
                                                }
                                            }
                                        }

                                        // Tooltip on Icon/Label
                                        <MpTooltipTop> {
                                            tip: "This is an info icon with tooltip"
                                            content = {
                                                <Label> {
                                                    draw_text: {
                                                        text_style: <THEME_FONT_REGULAR>{ font_size: 20.0 }
                                                        color: (PRIMARY)
                                                    }
                                                    text: "ℹ️"
                                                }
                                            }
                                        }
                                    }
                                }

                                // Long text tooltip
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Long Text" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        padding: { top: 20, bottom: 20 }

                                        <MpTooltipTop> {
                                            tip: "This is a longer tooltip text that provides more detailed information about the element being hovered."
                                            content = {
                                                <MpButtonGhost> { text: "Long tooltip" }
                                            }
                                        }

                                        <MpTooltipBottom> {
                                            tip: "Tooltips can contain helpful hints, keyboard shortcuts, or additional context for users."
                                            content = {
                                                <MpButtonGhost> { text: "Helpful hints" }
                                            }
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Progress Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Progress" }

                                // Progress variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 16,

                                    <SubsectionLabel> { text: "Values" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpProgress> { width: 300, value: 25.0 }
                                        <MpProgress> { width: 300, value: 50.0 }
                                        <MpProgress> { width: 300, value: 75.0 }
                                        <MpProgress> { width: 300, value: 100.0 }
                                    }
                                }

                                // Progress colors
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Colors" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpProgress> { width: 300, value: 60.0 }
                                        <MpProgressSuccess> { width: 300, value: 60.0 }
                                        <MpProgressDanger> { width: 300, value: 60.0 }
                                        <MpProgressWarning> { width: 300, value: 60.0 }
                                    }
                                }

                                // Progress widths
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Widths" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpProgress> { width: 150, value: 50.0 }
                                        <MpProgress> { width: 250, value: 50.0 }
                                        <MpProgress> { width: 350, value: 50.0 }
                                    }
                                }

                                // Interactive progress
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Interactive" }

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Right,
                                        spacing: 16,
                                        align: { y: 0.5 }

                                        progress_dec_btn = <MpButtonGhost> { text: "-10" }
                                        interactive_progress = <MpProgress> { width: 200, value: 50.0 }
                                        progress_inc_btn = <MpButtonGhost> { text: "+10" }

                                        progress_label = <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                            }
                                            text: "50%"
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Alert Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Alert" }

                                // Alert Variants
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Variants" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpAlert> {
                                            content = {
                                                message = { text: "This is a default alert message." }
                                            }
                                        }

                                        <MpAlertInfo> {
                                            content = {
                                                message = { text: "This is an info alert for general information." }
                                            }
                                        }

                                        <MpAlertSuccess> {
                                            content = {
                                                message = { text: "Operation completed successfully!" }
                                            }
                                        }

                                        <MpAlertWarning> {
                                            content = {
                                                message = { text: "Please review your input before continuing." }
                                            }
                                        }

                                        <MpAlertError> {
                                            content = {
                                                message = { text: "Something went wrong. Please try again." }
                                            }
                                        }
                                    }
                                }

                                // Alert with Title
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "With Title" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        <MpAlertInfo> {
                                            content = {
                                                title_wrapper = { visible: true, title = { text: "Information" } }
                                                message = { text: "This alert has a title for more context." }
                                            }
                                        }

                                        <MpAlertSuccess> {
                                            content = {
                                                title_wrapper = { visible: true, title = { text: "Success!" } }
                                                message = { text: "Your changes have been saved successfully." }
                                            }
                                        }

                                        <MpAlertError> {
                                            content = {
                                                title_wrapper = { visible: true, title = { text: "Error" } }
                                                message = { text: "Failed to connect to the server. Check your network." }
                                            }
                                        }
                                    }
                                }

                                // Closable Alert
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Closable" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 12,

                                        closable_alert = <MpAlertInfo> {
                                            closable: true
                                            content = {
                                                message = { text: "This alert can be closed. Click the X button." }
                                            }
                                        }

                                        closable_alert_warning = <MpAlertWarning> {
                                            closable: true
                                            content = {
                                                title_wrapper = { visible: true, title = { text: "Warning" } }
                                                message = { text: "This is a closable warning with title." }
                                            }
                                        }
                                    }
                                }

                                // Banner Alerts
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Banner Style" }

                                    <View> {
                                        width: Fill, height: Fit,
                                        flow: Down,
                                        spacing: 0,

                                        <MpAlertBannerInfo> {
                                            content = {
                                                message = { text: "Info banner - full width, no border radius" }
                                            }
                                        }

                                        <MpAlertBannerSuccess> {
                                            closable: true
                                            content = {
                                                message = { text: "Success banner with close button" }
                                            }
                                        }

                                        <MpAlertBannerWarning> {
                                            content = {
                                                message = { text: "Warning banner alert" }
                                            }
                                        }

                                        <MpAlertBannerError> {
                                            closable: true
                                            content = {
                                                message = { text: "Error banner - something needs attention!" }
                                            }
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Notification Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Notification" }

                                // Interactive Notification Demo
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 8,

                                    show_success_notif = <MpButtonSuccess> { text: "Success" }
                                    show_error_notif = <MpButtonDanger> { text: "Error" }
                                    show_warning_notif = <MpButtonWarning> { text: "Warning" }
                                    show_info_notif = <MpButtonPrimary> { text: "Info" }
                                }

                                <MpDivider> { margin: { top: 8, bottom: 8 } }

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Notification previews (static):"
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <MpNotification> {
                                        content = {
                                            title = { text: "Notification" }
                                            message = { text: "This is a default notification message." }
                                        }
                                    }

                                    <MpNotificationSuccess> {
                                        content = {
                                            title = { text: "Success" }
                                            message = { text: "Operation completed successfully!" }
                                        }
                                    }

                                    <MpNotificationError> {
                                        content = {
                                            title = { text: "Error" }
                                            message = { text: "Something went wrong. Please try again." }
                                        }
                                    }

                                    <MpNotificationWarning> {
                                        content = {
                                            title = { text: "Warning" }
                                            message = { text: "Please review your input before continuing." }
                                        }
                                    }

                                    <MpNotificationInfo> {
                                        content = {
                                            title = { text: "Info" }
                                            message = { text: "Here's some helpful information." }
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Modal Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Modal" }

                                // Interactive Modal Demo
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 16,
                                    align: { y: 0.5 }

                                    open_modal_btn = <MpButtonPrimary> { text: "Open Modal" }

                                    modal_status = <Label> {
                                        draw_text: {
                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                            color: (MUTED_FOREGROUND)
                                        }
                                        text: "Click button to open modal"
                                    }
                                }

                                <MpDivider> { margin: { top: 8, bottom: 8 } }

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Modal previews (static):"
                                }

                                // Basic Modal preview
                                <MpModal> {
                                    width: 350,
                                    header = {
                                        title = { text: "Modal Title" }
                                    }
                                    body = {
                                        <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "This is the modal content area."
                                        }
                                    }
                                    footer = {
                                        <MpButtonGhost> { text: "Cancel" }
                                        <MpButtonPrimary> { text: "Confirm" }
                                    }
                                }

                                // Alert Dialog preview
                                <MpAlertDialog> {
                                    width: 320,
                                    header = {
                                        title = { text: "Are you sure?" }
                                    }
                                    body = {
                                        <Label> {
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "This action cannot be undone."
                                        }
                                    }
                                    footer = {
                                        <MpButtonGhost> { text: "Cancel" }
                                        <MpButtonDanger> { text: "Delete" }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Popover Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Popover" }

                                // Interactive Popover Demo
                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Down,
                                    spacing: 12,

                                    <View> {
                                        width: Fit, height: Fit,
                                        flow: Overlay,

                                        popover_trigger_btn = <MpButtonPrimary> { text: "Toggle Popover" }

                                        <View> {
                                            width: Fit, height: Fit,
                                            margin: { top: 44 }

                                            interactive_popover = <MpPopoverWidget> {
                                                content = <MpPopoverBase> {
                                                    width: 200, height: Fit,
                                                    padding: 12,
                                                    flow: Down,
                                                    spacing: 8,

                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                                                            color: (FOREGROUND)
                                                        }
                                                        text: "Interactive Popover"
                                                    }
                                                    <Label> {
                                                        width: Fill,
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
                                                            color: (MUTED_FOREGROUND)
                                                            wrap: Word
                                                        }
                                                        text: "Click the button again to close."
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                <MpDivider> { margin: { top: 8, bottom: 8 } }

                                <Label> {
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (MUTED_FOREGROUND)
                                    }
                                    text: "Popover previews (static):"
                                }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 16,

                                    // Basic Popover
                                    <MpPopover> {
                                        width: 200,
                                        <Label> {
                                            width: Fill,
                                            height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                color: (FOREGROUND)
                                                wrap: Word
                                            }
                                            text: "Basic popover content goes here."
                                        }
                                    }

                                    // Popover with Header
                                    <MpPopoverWithHeader> {
                                        width: 220,
                                        header = {
                                            title_label = { text: "Popover Title" }
                                        }
                                        body = {
                                            desc_label = { text: "Detailed description here." }
                                        }
                                    }
                                }

                                // Popover Menu
                                <View> {
                                    width: Fit, height: Fit,
                                    flow: Down,
                                    spacing: 8,

                                    <SubsectionLabel> { text: "Menu Popover" }

                                    <MpPopoverMenu> {
                                        width: 180,
                                        <MpPopoverMenuItem> { label = { text: "Edit" } }
                                        <MpPopoverMenuItem> { label = { text: "Duplicate" } }
                                        <MpPopoverMenuDivider> {}
                                        <MpPopoverMenuItemDanger> { label = { text: "Delete" } }
                                    }
                                }
                            }
                        }

                        // ============================================================
                        // Data Page
                        // ============================================================
                        page_data = <ScrollYView> {
                            width: Fill, height: Fill,
                            flow: Down,
                            spacing: 24,
                            padding: { left: 24, right: 24, top: 24, bottom: 100 }

                            show_bg: true
                            draw_bg: { color: #fbcfe8 }

                            // ===== List Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "List" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 24,

                                    // Basic List
                                    <View> {
                                        width: 280, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <SubsectionLabel> { text: "Basic List" }

                                        <MpListDivided> {
                                            <MpListItem> {
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "List Item 1" }
                                                    <MpListItemDescription> { text: "Description for item 1" }
                                                }
                                            }
                                            <MpListDividerFull> {}
                                            <MpListItem> {
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "List Item 2" }
                                                    <MpListItemDescription> { text: "Description for item 2" }
                                                }
                                            }
                                            <MpListDividerFull> {}
                                            <MpListItem> {
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "List Item 3" }
                                                    <MpListItemDescription> { text: "Description for item 3" }
                                                }
                                            }
                                        }
                                    }

                                    // List with avatars
                                    <View> {
                                        width: 280, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <SubsectionLabel> { text: "With Avatar" }

                                        <MpListDivided> {
                                            <MpListItem> {
                                                <MpListItemLeading> {
                                                    <MpAvatarSmall> { label = { text: "JD" } }
                                                }
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "John Doe" }
                                                    <MpListItemDescription> { text: "Software Engineer" }
                                                }
                                            }
                                            <MpListDividerFull> {}
                                            <MpListItem> {
                                                <MpListItemLeading> {
                                                    <MpAvatarSmall> { label = { text: "AS" } }
                                                }
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "Alice Smith" }
                                                    <MpListItemDescription> { text: "Product Manager" }
                                                }
                                            }
                                            <MpListDividerFull> {}
                                            <MpListItem> {
                                                <MpListItemLeading> {
                                                    <MpAvatarSmall> { label = { text: "BJ" } }
                                                }
                                                <MpListItemContent> {
                                                    <MpListItemTitle> { text: "Bob Johnson" }
                                                    <MpListItemDescription> { text: "Designer" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Accordion Section =====
                            <View> {
                                width: Fill, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Accordion" }

                                <View> {
                                    width: Fill, height: Fit,
                                    flow: Right,
                                    spacing: 24,

                                    // Basic Accordion
                                    <View> {
                                        width: 320, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <SubsectionLabel> { text: "Basic" }

                                        <MpAccordion> {
                                            <MpAccordionItem> {
                                                header = <MpAccordionHeaderBase> {
                                                    label = { text: "Section 1" }
                                                }
                                                body = {
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (MUTED_FOREGROUND)
                                                        }
                                                        text: "Content for section 1."
                                                    }
                                                }
                                            }

                                            <MpAccordionDivider> {}

                                            <MpAccordionItem> {
                                                header = <MpAccordionHeaderBase> {
                                                    label = { text: "Section 2" }
                                                }
                                                body = {
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (MUTED_FOREGROUND)
                                                        }
                                                        text: "Content for section 2."
                                                    }
                                                }
                                            }

                                            <MpAccordionDivider> {}

                                            <MpAccordionItem> {
                                                header = <MpAccordionHeaderBase> {
                                                    label = { text: "Section 3" }
                                                }
                                                body = {
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (MUTED_FOREGROUND)
                                                        }
                                                        text: "Content for section 3."
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Bordered Accordion
                                    <View> {
                                        width: 320, height: Fit,
                                        flow: Down,
                                        spacing: 8,

                                        <SubsectionLabel> { text: "Bordered" }

                                        <MpAccordionBordered> {
                                            <MpAccordionItemBordered> {
                                                header = <MpAccordionHeaderBase> {
                                                    label = { text: "FAQ Item 1" }
                                                }
                                                body = {
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (MUTED_FOREGROUND)
                                                        }
                                                        text: "Answer to FAQ 1."
                                                    }
                                                }
                                            }

                                            <MpAccordionItemBordered> {
                                                header = <MpAccordionHeaderBase> {
                                                    label = { text: "FAQ Item 2" }
                                                }
                                                body = {
                                                    <Label> {
                                                        draw_text: {
                                                            text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                            color: (MUTED_FOREGROUND)
                                                        }
                                                        text: "Answer to FAQ 2."
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            <MpDivider> {}

                            // ===== Interactive Demo =====
                            <View> {
                                width: Fit, height: Fit,
                                flow: Down,
                                spacing: 16,

                                <SectionHeader> { text: "Interactive Demo" }

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
                    } // close main_content

                    // Modal overlay - must be after main_content to appear on top
                    demo_modal = <MpModalWidget> {
                    content = {
                        dialog = <MpModal> {
                            width: 400,
                            header = {
                                title = { text: "Interactive Modal" }
                            }
                            body = {
                                <Label> {
                                    width: Fill,
                                    height: Fit,
                                    draw_text: {
                                        text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                        color: (MUTED_FOREGROUND)
                                        wrap: Word
                                    }
                                    text: "This is an interactive modal dialog. Click the X button or the backdrop to close it."
                                }
                            }
                            footer = {
                                modal_cancel_btn = <MpButtonGhost> { text: "Cancel" }
                                modal_confirm_btn = <MpButtonPrimary> { text: "Confirm" }
                            }
                        }
                    }
                } // close demo_modal

                    // Notification overlay - positioned at top-right
                    <View> {
                        width: Fill,
                        height: Fill,
                        align: { x: 1.0, y: 0.0 }
                        padding: { top: 20, right: 20 }

                        demo_notification = <MpNotificationWidget> {
                            content = {
                                title = { text: "Notification" }
                                message = { text: "This is an interactive notification!" }
                            }
                        }
                    }
                } // close body (Overlay)
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
    #[rust]
    current_page: usize,
    #[rust]
    current_category: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        makepad_component::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.counter = 0;
        self.current_category = 0;

        // Set initial category tab as selected
        self.ui.mp_tab(ids!(cat_form)).set_selected(cx, true);

        // Initialize skeleton in loading state
        self.ui.mp_skeleton_widget(ids!(interactive_skeleton)).set_loading(cx, true);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle category tab clicks
        if self.ui.mp_tab(ids!(cat_form)).clicked(&actions) {
            self.select_category(cx, 0);
        }
        if self.ui.mp_tab(ids!(cat_display)).clicked(&actions) {
            self.select_category(cx, 1);
        }
        if self.ui.mp_tab(ids!(cat_layout)).clicked(&actions) {
            self.select_category(cx, 2);
        }
        if self.ui.mp_tab(ids!(cat_nav)).clicked(&actions) {
            self.select_category(cx, 3);
        }
        if self.ui.mp_tab(ids!(cat_feedback)).clicked(&actions) {
            self.select_category(cx, 4);
        }
        if self.ui.mp_tab(ids!(cat_data)).clicked(&actions) {
            self.select_category(cx, 5);
        }

        // Handle counter button
        if self.ui.mp_button(ids!(counter_btn)).clicked(&actions) {
            self.counter += 1;
            self.ui.label(ids!(counter_label))
                .set_text(cx, &format!("Clicked: {} times", self.counter));
        }

        // Handle PageFlip navigation
        if self.ui.mp_button(ids!(page_btn_a)).clicked(&actions) {
            self.ui.page_flip(ids!(demo_page_flip)).set_active_page(cx, id!(page_a));
            self.current_page = 0;
            self.update_page_buttons(cx);
        }
        if self.ui.mp_button(ids!(page_btn_b)).clicked(&actions) {
            self.ui.page_flip(ids!(demo_page_flip)).set_active_page(cx, id!(page_b));
            self.current_page = 1;
            self.update_page_buttons(cx);
        }
        if self.ui.mp_button(ids!(page_btn_c)).clicked(&actions) {
            self.ui.page_flip(ids!(demo_page_flip)).set_active_page(cx, id!(page_c));
            self.current_page = 2;
            self.update_page_buttons(cx);
        }

        // Handle checkbox changes
        if self.ui.mp_checkbox(ids!(checkbox1)).changed(&actions).is_some() {
            self.update_checkbox_status(cx);
        }
        if self.ui.mp_checkbox(ids!(checkbox2)).changed(&actions).is_some() {
            self.update_checkbox_status(cx);
        }
        if self.ui.mp_checkbox(ids!(checkbox3)).changed(&actions).is_some() {
            self.update_checkbox_status(cx);
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
        }
        if self.ui.mp_radio(ids!(radio_medium)).changed(&actions).is_some() {
            self.ui.mp_radio(ids!(radio_small)).set_checked(cx, false);
            self.ui.mp_radio(ids!(radio_large)).set_checked(cx, false);
            self.ui.label(ids!(radio_status)).set_text(cx, "Selected: Medium");
        }
        if self.ui.mp_radio(ids!(radio_large)).changed(&actions).is_some() {
            self.ui.mp_radio(ids!(radio_small)).set_checked(cx, false);
            self.ui.mp_radio(ids!(radio_medium)).set_checked(cx, false);
            self.ui.label(ids!(radio_status)).set_text(cx, "Selected: Large");
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
        }

        if let Some(value) = self.ui.mp_slider(ids!(slider_vert)).changed(&actions) {
            let v = value.end();
            self.ui.label(ids!(slider_vert_label)).set_text(cx, &format!("Vertical value: {}", v as i32));
        }

        // Handle range slider changes
        if let Some(value) = self.ui.mp_slider(ids!(slider_range)).changed(&actions) {
            let start = value.start() as i32;
            let end = value.end() as i32;
            self.ui.label(ids!(slider_range_label)).set_text(cx, &format!("Range: {} - {}", start, end));
        }

        if let Some(value) = self.ui.mp_slider(ids!(slider_range_success)).changed(&actions) {
            let start = value.start() as i32;
            let end = value.end() as i32;
            self.ui.label(ids!(slider_range_success_label)).set_text(cx, &format!("Range: {} - {} (step 5)", start, end));
        }

        // Handle input changes
        if let Some(text) = self.ui.text_input(ids!(input_interactive)).changed(&actions) {
            let display = if text.is_empty() {
                "Value: (empty)".to_string()
            } else {
                format!("Value: {}", text)
            };
            self.ui.label(ids!(input_status)).set_text(cx, &display);
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

        // Handle avatar change button
        if self.ui.mp_button(ids!(avatar_change_btn)).clicked(&actions) {
            let names = ["Alice Wang", "Bob Smith", "Carol Lee", "David Kim", "Emma Chen", "Frank Zhang"];
            let idx = (cx.event_id() as usize) % names.len();
            let name = names[idx];
            self.ui.mp_avatar(ids!(dynamic_avatar)).set_initials_from_name(cx, name);
            self.ui.label(ids!(avatar_name_label)).set_text(cx, name);
        }

        // Handle clickable card clicks using as_widget_action().cast() pattern
        for action in actions {
            if let MpCardAction::Clicked = action.as_widget_action().cast() {
                self.ui.label(ids!(card_click_status)).set_text(cx, "Card clicked!");
            }
            // Handle modal close request (backdrop or X button)
            if let MpModalAction::CloseRequested = action.as_widget_action().cast() {
                self.ui.mp_modal_widget(ids!(demo_modal)).close(cx);
                self.ui.label(ids!(modal_status)).set_text(cx, "Modal closed");
            }
        }

        // Handle open modal button
        if self.ui.mp_button(ids!(open_modal_btn)).clicked(&actions) {
            self.ui.mp_modal_widget(ids!(demo_modal)).open(cx);
            self.ui.label(ids!(modal_status)).set_text(cx, "Modal opened");
        }

        // Handle modal cancel button
        if self.ui.mp_button(ids!(modal_cancel_btn)).clicked(&actions) {
            self.ui.mp_modal_widget(ids!(demo_modal)).close(cx);
            self.ui.label(ids!(modal_status)).set_text(cx, "Cancelled");
        }

        // Handle modal confirm button
        if self.ui.mp_button(ids!(modal_confirm_btn)).clicked(&actions) {
            self.ui.mp_modal_widget(ids!(demo_modal)).close(cx);
            self.ui.label(ids!(modal_status)).set_text(cx, "Confirmed!");
        }

        // Handle popover toggle button
        if self.ui.mp_button(ids!(popover_trigger_btn)).clicked(&actions) {
            self.ui.mp_popover_widget(ids!(interactive_popover)).toggle(cx);
        }

        // Handle skeleton toggle button
        if self.ui.mp_button(ids!(skeleton_toggle_btn)).clicked(&actions) {
            let skeleton = self.ui.mp_skeleton_widget(ids!(interactive_skeleton));
            let is_loading = skeleton.is_loading();
            skeleton.set_loading(cx, !is_loading);
            let status = if !is_loading { "Loading" } else { "Loaded" };
            self.ui.label(ids!(skeleton_status)).set_text(cx, &format!("Status: {}", status));
        }

        // Handle notification buttons
        if self.ui.mp_button(ids!(show_success_notif)).clicked(&actions) {
            self.ui.mp_notification_widget(ids!(demo_notification)).show_message(
                cx, "Success!", "Operation completed successfully!"
            );
        }
        if self.ui.mp_button(ids!(show_error_notif)).clicked(&actions) {
            self.ui.mp_notification_widget(ids!(demo_notification)).show_message(
                cx, "Error", "Something went wrong. Please try again."
            );
        }
        if self.ui.mp_button(ids!(show_warning_notif)).clicked(&actions) {
            self.ui.mp_notification_widget(ids!(demo_notification)).show_message(
                cx, "Warning", "Please review your input before continuing."
            );
        }
        if self.ui.mp_button(ids!(show_info_notif)).clicked(&actions) {
            self.ui.mp_notification_widget(ids!(demo_notification)).show_message(
                cx, "Info", "Here's some helpful information for you."
            );
        }

        // Handle dropdown changes
        let labels = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];
        if let Some(idx) = self.ui.drop_down(ids!(dropdown_basic)).selected(&actions) {
            let label = labels.get(idx).unwrap_or(&"Unknown");
            self.ui.label(ids!(dropdown_status)).set_text(cx, &format!("Selected: {}", label));
        }

        // Handle Tab clicks - Default style
        if self.ui.mp_tab(ids!(tab_home)).clicked(&actions) {
            self.select_tab(cx, "default", 0, "Home");
        }
        if self.ui.mp_tab(ids!(tab_profile)).clicked(&actions) {
            self.select_tab(cx, "default", 1, "Profile");
        }
        if self.ui.mp_tab(ids!(tab_settings)).clicked(&actions) {
            self.select_tab(cx, "default", 2, "Settings");
        }

        // Handle Tab clicks - Underline style
        if self.ui.mp_tab(ids!(tab_u_overview)).clicked(&actions) {
            self.select_tab(cx, "underline", 0, "Overview");
        }
        if self.ui.mp_tab(ids!(tab_u_analytics)).clicked(&actions) {
            self.select_tab(cx, "underline", 1, "Analytics");
        }
        if self.ui.mp_tab(ids!(tab_u_reports)).clicked(&actions) {
            self.select_tab(cx, "underline", 2, "Reports");
        }

        // Handle Tab clicks - Pill style
        if self.ui.mp_tab(ids!(tab_p_all)).clicked(&actions) {
            self.select_tab(cx, "pill", 0, "All");
        }
        if self.ui.mp_tab(ids!(tab_p_active)).clicked(&actions) {
            self.select_tab(cx, "pill", 1, "Active");
        }
        if self.ui.mp_tab(ids!(tab_p_completed)).clicked(&actions) {
            self.select_tab(cx, "pill", 2, "Completed");
        }

        // Handle Tab clicks - Outline style
        if self.ui.mp_tab(ids!(tab_o_day)).clicked(&actions) {
            self.select_tab(cx, "outline", 0, "Day");
        }
        if self.ui.mp_tab(ids!(tab_o_week)).clicked(&actions) {
            self.select_tab(cx, "outline", 1, "Week");
        }
        if self.ui.mp_tab(ids!(tab_o_month)).clicked(&actions) {
            self.select_tab(cx, "outline", 2, "Month");
        }

        // Handle Tab clicks - Segmented style
        if self.ui.mp_tab(ids!(tab_s_list)).clicked(&actions) {
            self.select_tab(cx, "segmented", 0, "List");
        }
        if self.ui.mp_tab(ids!(tab_s_grid)).clicked(&actions) {
            self.select_tab(cx, "segmented", 1, "Grid");
        }
        if self.ui.mp_tab(ids!(tab_s_map)).clicked(&actions) {
            self.select_tab(cx, "segmented", 2, "Map");
        }
    }
}

impl App {
    fn select_category(&mut self, cx: &mut Cx, index: usize) {
        self.current_category = index;

        // Update tab selected states
        self.ui.mp_tab(ids!(cat_form)).set_selected(cx, index == 0);
        self.ui.mp_tab(ids!(cat_display)).set_selected(cx, index == 1);
        self.ui.mp_tab(ids!(cat_layout)).set_selected(cx, index == 2);
        self.ui.mp_tab(ids!(cat_nav)).set_selected(cx, index == 3);
        self.ui.mp_tab(ids!(cat_feedback)).set_selected(cx, index == 4);
        self.ui.mp_tab(ids!(cat_data)).set_selected(cx, index == 5);

        // Switch page
        let page_id = match index {
            0 => id!(page_form),
            1 => id!(page_display),
            2 => id!(page_layout),
            3 => id!(page_nav),
            4 => id!(page_feedback),
            5 => id!(page_data),
            _ => id!(page_form),
        };
        self.ui.page_flip(ids!(category_pages)).set_active_page(cx, page_id);
        self.ui.redraw(cx);
    }

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

    fn update_page_buttons(&mut self, cx: &mut Cx) {
        let active_bg = vec4(0.231, 0.510, 0.965, 1.0);
        let active_hover = vec4(0.145, 0.388, 0.859, 1.0);
        let active_pressed = vec4(0.114, 0.310, 0.847, 1.0);
        let active_text = vec4(1.0, 1.0, 1.0, 1.0);

        let inactive_bg = vec4(0.0, 0.0, 0.0, 0.0);
        let inactive_hover = vec4(0.945, 0.961, 0.976, 1.0);
        let inactive_pressed = vec4(0.796, 0.835, 0.820, 1.0);
        let inactive_text = vec4(0.059, 0.090, 0.165, 1.0);

        let buttons = [
            (ids!(page_btn_a), 0),
            (ids!(page_btn_b), 1),
            (ids!(page_btn_c), 2),
        ];

        for (btn_id, page_idx) in buttons {
            let btn = self.ui.widget(btn_id);
            if page_idx == self.current_page {
                btn.apply_over(cx, live! {
                    draw_bg: { color: (active_bg), color_hover: (active_hover), color_pressed: (active_pressed) }
                    draw_text: { color: (active_text) }
                });
            } else {
                btn.apply_over(cx, live! {
                    draw_bg: { color: (inactive_bg), color_hover: (inactive_hover), color_pressed: (inactive_pressed) }
                    draw_text: { color: (inactive_text) }
                });
            }
        }

        self.ui.redraw(cx);
    }

    fn select_tab(&mut self, cx: &mut Cx, style: &str, index: usize, label: &str) {
        match style {
            "default" => {
                self.ui.mp_tab(ids!(tab_home)).set_selected(cx, index == 0);
                self.ui.mp_tab(ids!(tab_profile)).set_selected(cx, index == 1);
                self.ui.mp_tab(ids!(tab_settings)).set_selected(cx, index == 2);
            }
            "underline" => {
                self.ui.mp_tab(ids!(tab_u_overview)).set_selected(cx, index == 0);
                self.ui.mp_tab(ids!(tab_u_analytics)).set_selected(cx, index == 1);
                self.ui.mp_tab(ids!(tab_u_reports)).set_selected(cx, index == 2);
            }
            "pill" => {
                self.ui.mp_tab(ids!(tab_p_all)).set_selected(cx, index == 0);
                self.ui.mp_tab(ids!(tab_p_active)).set_selected(cx, index == 1);
                self.ui.mp_tab(ids!(tab_p_completed)).set_selected(cx, index == 2);
            }
            "outline" => {
                self.ui.mp_tab(ids!(tab_o_day)).set_selected(cx, index == 0);
                self.ui.mp_tab(ids!(tab_o_week)).set_selected(cx, index == 1);
                self.ui.mp_tab(ids!(tab_o_month)).set_selected(cx, index == 2);
            }
            "segmented" => {
                self.ui.mp_tab(ids!(tab_s_list)).set_selected(cx, index == 0);
                self.ui.mp_tab(ids!(tab_s_grid)).set_selected(cx, index == 1);
                self.ui.mp_tab(ids!(tab_s_map)).set_selected(cx, index == 2);
            }
            _ => {}
        }

        self.ui.label(ids!(tab_status)).set_text(cx, &format!("Selected: {}", label));
        self.ui.redraw(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
