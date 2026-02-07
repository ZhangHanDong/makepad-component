use makepad_widgets::*;
use makepad_component::widgets::*;

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
        if self.ui.mp_tab(ids!(cat_nav)).clicked(&actions) {
            self.select_category(cx, 2);
        }
        if self.ui.mp_tab(ids!(cat_feedback)).clicked(&actions) {
            self.select_category(cx, 3);
        }
        if self.ui.mp_tab(ids!(cat_data)).clicked(&actions) {
            self.select_category(cx, 4);
        }
        if self.ui.mp_tab(ids!(cat_shader)).clicked(&actions) {
            self.select_category(cx, 5);
        }
        if self.ui.mp_tab(ids!(cat_shader_art)).clicked(&actions) {
            self.select_category(cx, 6);
        }
        if self.ui.mp_tab(ids!(cat_shader_art2)).clicked(&actions) {
            self.select_category(cx, 7);
        }
        if self.ui.mp_tab(ids!(cat_shader_math)).clicked(&actions) {
            self.select_category(cx, 8);
        }
        if self.ui.mp_tab(ids!(cat_splash)).clicked(&actions) {
            self.select_category(cx, 9);
        }
        if self.ui.mp_tab(ids!(cat_json)).clicked(&actions) {
            self.select_category(cx, 10);
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

        // Handle shader art speed slider
        if let Some(value) = self.ui.mp_slider(ids!(shader_art_speed)).changed(&actions) {
            let speed = value.end();
            self.ui.label(ids!(shader_art_speed_label)).set_text(cx, &format!("{:.1}x", speed));
            self.ui.view(ids!(shader_art_canvas)).apply_over(cx, live!{
                speed: (speed)
            });
        }

        // Handle shader art2 speed slider
        if let Some(value) = self.ui.mp_slider(ids!(shader_art2_speed)).changed(&actions) {
            let speed = value.end();
            self.ui.label(ids!(shader_art2_speed_label)).set_text(cx, &format!("{:.1}x", speed));
            self.ui.view(ids!(shader_art2_canvas)).apply_over(cx, live!{
                speed: (speed)
            });
        }

        // Handle shader math speed slider
        if let Some(value) = self.ui.mp_slider(ids!(shader_math_speed)).changed(&actions) {
            let speed = value.end();
            self.ui.label(ids!(shader_math_speed_label)).set_text(cx, &format!("{:.1}x", speed));
            self.ui.view(ids!(shader_math_canvas)).apply_over(cx, live!{
                speed: (speed)
            });
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
        self.ui.mp_tab(ids!(cat_nav)).set_selected(cx, index == 2);
        self.ui.mp_tab(ids!(cat_feedback)).set_selected(cx, index == 3);
        self.ui.mp_tab(ids!(cat_data)).set_selected(cx, index == 4);
        self.ui.mp_tab(ids!(cat_shader)).set_selected(cx, index == 5);
        self.ui.mp_tab(ids!(cat_shader_art)).set_selected(cx, index == 6);
        self.ui.mp_tab(ids!(cat_shader_art2)).set_selected(cx, index == 7);
        self.ui.mp_tab(ids!(cat_shader_math)).set_selected(cx, index == 8);
        self.ui.mp_tab(ids!(cat_splash)).set_selected(cx, index == 9);
        self.ui.mp_tab(ids!(cat_json)).set_selected(cx, index == 10);

        // Switch page
        let page_id = match index {
            0 => id!(page_form),
            1 => id!(page_display),
            2 => id!(page_nav),
            3 => id!(page_feedback),
            4 => id!(page_data),
            5 => id!(page_shader),
            6 => id!(page_shader_art),
            7 => id!(page_shader_art2),
            8 => id!(page_shader_math),
            9 => id!(page_splash),
            10 => id!(page_json),
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
