//! A2uiSurface widget definition and core implementation

use makepad_widgets::*;
use makepad_plot::*;

use crate::a2ui::{
    chart_bridge,
    data_model::DataModel,
    message::*,
    processor::{
        resolve_boolean_value_scoped, resolve_number_value_scoped,
        resolve_string_value_scoped, A2uiMessageProcessor, ProcessorEvent,
    },
};

use super::draw_types::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_plot::plot::line::LinePlot;
    use makepad_plot::plot::bar::BarPlot;
    use makepad_plot::plot::scatter::ScatterPlot;
    use makepad_plot::plot::pie::PieChart;
    use makepad_plot::plot::area::AreaChart;
    use makepad_plot::plot::polar::RadarChart;
    use makepad_plot::plot::gauge::GaugeChart;
    use makepad_plot::plot::bubble::BubbleChart;
    use makepad_plot::plot::financial::CandlestickChart;
    use makepad_plot::plot::heatmap::HeatmapChart;
    use makepad_plot::plot::treemap::Treemap;
    use makepad_plot::plot::hexbin::SankeyDiagram;
    use makepad_plot::plot::histogram::HistogramChart;
    use makepad_plot::plot::histogram::BoxPlotChart;
    use makepad_plot::plot::pie::DonutChart;
    use makepad_plot::plot::stem::StemPlot;
    use makepad_plot::plot::stem::ViolinPlot;
    use makepad_plot::plot::polar::PolarPlot;
    use makepad_plot::plot::contour::ContourPlot;
    use makepad_plot::plot::financial::WaterfallChart;
    use makepad_plot::plot::gauge::FunnelChart;
    use makepad_plot::plot::area::StepPlot;
    use makepad_plot::plot::stack::Stackplot;
    use makepad_plot::plot::hexbin::HexbinChart;
    use makepad_plot::plot::stack::Streamgraph;
    use makepad_plot::plot::surface3d::Surface3D;
    use makepad_plot::plot::scatter3d::Scatter3D;
    use makepad_plot::plot::scatter3d::Line3D;

    use crate::theme::colors::*;

    use crate::a2ui::surface::draw_types::DrawA2uiImage;
    use crate::a2ui::surface::draw_types::DrawA2uiTextField;
    use crate::a2ui::surface::draw_types::DrawA2uiCheckBox;
    use crate::a2ui::surface::draw_types::DrawA2uiSliderTrack;
    use crate::a2ui::surface::draw_types::DrawA2uiSliderThumb;
    use crate::a2ui::surface::draw_types::DrawA2uiChartLine;
    use crate::a2ui::surface::draw_types::DrawA2uiArc;
    use crate::a2ui::surface::draw_types::DrawA2uiQuad;
    use crate::a2ui::surface::draw_types::DrawAudioBars;

    pub A2uiSurface = {{A2uiSurface}} {
        width: Fill
        height: Fill
        flow: Down

        draw_bg: {
            instance bg_color: #1a1a2e

            fn pixel(self) -> vec4 {
                return self.bg_color;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        draw_card_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        draw_card: {
            color: #2a3a5a
            instance border_color: #5588bb
            instance border_radius: 8.0
            instance border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.border_radius)
                );
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        draw_button: {
            instance border_radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        draw_button_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        draw_image_placeholder: {
            instance border_radius: 4.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                let stripe_width = 8.0;
                let pos = self.pos * self.rect_size;
                let stripe = mod(pos.x + pos.y, stripe_width * 2.0);
                let is_stripe = step(stripe_width, stripe);
                let color1 = vec4(0.25, 0.28, 0.35, 1.0);
                let color2 = vec4(0.30, 0.33, 0.40, 1.0);
                let bg_color = mix(color1, color2, is_stripe);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        draw_image_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 11.0
            }
            color: #888888
        }

        draw_image: <DrawA2uiImage> {}

        draw_text_field: <DrawA2uiTextField> {
            border_color: #5588bb
            bg_color: #2a3a5a
        }

        draw_text_field_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #FFFFFF
        }

        draw_text_field_placeholder: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #888888
        }

        draw_checkbox: <DrawA2uiCheckBox> {
            border_color: #5588bb
            bg_color: #2a3a5a
            check_color: #3B82F6
        }

        draw_checkbox_label: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #FFFFFF
        }

        draw_slider_track: <DrawA2uiSliderTrack> {
            track_color: #3a4a6a
            fill_color: #3B82F6
        }

        draw_slider_thumb: <DrawA2uiSliderThumb> {
            thumb_color: #FFFFFF
        }

        draw_chart_line: <DrawA2uiChartLine> {}
        draw_chart_arc: <DrawA2uiArc> {}
        draw_chart_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 10.0
            }
            color: #AABBCC
        }
        draw_chart_quad: <DrawA2uiQuad> {}

        plot_line: <LinePlot> {}
        plot_bar: <BarPlot> {}
        plot_scatter: <ScatterPlot> {}
        plot_pie: <PieChart> {}
        plot_area: <AreaChart> {}
        plot_radar: <RadarChart> {}
        plot_gauge: <GaugeChart> {}
        plot_bubble: <BubbleChart> {}
        plot_candlestick: <CandlestickChart> {}
        plot_heatmap: <HeatmapChart> {}
        plot_treemap: <Treemap> {}
        plot_sankey: <SankeyDiagram> {}
        plot_histogram: <HistogramChart> {}
        plot_boxplot: <BoxPlotChart> {}
        plot_donut: <DonutChart> {}
        plot_stem: <StemPlot> {}
        plot_violin: <ViolinPlot> {}
        plot_polar: <PolarPlot> {}
        plot_contour: <ContourPlot> {}
        plot_waterfall: <WaterfallChart> {}
        plot_funnel: <FunnelChart> {}
        plot_step: <StepPlot> {}
        plot_stackplot: <Stackplot> {}
        plot_hexbin: <HexbinChart> {}
        plot_streamgraph: <Streamgraph> {}
        plot_surface3d: <Surface3D> {}
        plot_scatter3d: <Scatter3D> {}
        plot_line3d: <Line3D> {}
        draw_audio_bars: <DrawAudioBars> {}

        img_headphones: dep("crate://self/resources/headphones.jpg")
        img_mouse: dep("crate://self/resources/mouse.jpg")
        img_keyboard: dep("crate://self/resources/keyboard.jpg")
        img_alipay: dep("crate://self/resources/alipay.png")
        img_wechat: dep("crate://self/resources/wechat.png")
    }
}

// ============================================================================
// A2UI Surface Widget
// ============================================================================

/// The root container for rendering A2UI component trees.
#[derive(Live, LiveHook, Widget)]
pub struct A2uiSurface {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    /// Draw text for rendering text components (outside cards)
    #[live]
    draw_text: DrawText,

    /// Draw text for content inside cards (separate draw item for correct z-order)
    #[live]
    draw_card_text: DrawText,

    /// Draw card background
    #[redraw]
    #[live]
    draw_card: DrawColor,

    /// Draw button background (with rounded corners shader)
    #[redraw]
    #[live]
    draw_button: DrawColor,

    /// Draw text for button labels (drawn after button background)
    #[live]
    draw_button_text: DrawText,

    /// Draw image placeholder background
    #[redraw]
    #[live]
    draw_image_placeholder: DrawColor,

    /// Draw text for image placeholder
    #[live]
    draw_image_text: DrawText,

    /// Draw actual image
    #[redraw]
    #[live]
    draw_image: DrawA2uiImage,

    /// Draw text field background
    #[redraw]
    #[live]
    draw_text_field: DrawA2uiTextField,

    /// Draw text for text field input
    #[live]
    draw_text_field_text: DrawText,

    /// Draw text for text field placeholder
    #[live]
    draw_text_field_placeholder: DrawText,

    /// Draw checkbox
    #[redraw]
    #[live]
    draw_checkbox: DrawA2uiCheckBox,

    /// Draw checkbox label
    #[live]
    draw_checkbox_label: DrawText,

    /// Draw slider track
    #[redraw]
    #[live]
    draw_slider_track: DrawA2uiSliderTrack,

    /// Draw slider thumb
    #[redraw]
    #[live]
    draw_slider_thumb: DrawA2uiSliderThumb,

    /// Draw chart line segment (chord chart)
    #[redraw]
    #[live]
    draw_chart_line: DrawA2uiChartLine,

    /// Draw chart arc (chord chart)
    #[redraw]
    #[live]
    draw_chart_arc: DrawA2uiArc,

    /// Draw chart text (chord chart labels)
    #[live]
    draw_chart_text: DrawText,

    /// Draw chart arbitrary quadrilateral (chord ribbons)
    #[redraw]
    #[live]
    draw_chart_quad: DrawA2uiQuad,

    // makepad-plot chart widget instances
    #[live] plot_line: LinePlot,
    #[live] plot_bar: BarPlot,
    #[live] plot_scatter: ScatterPlot,
    #[live] plot_pie: PieChart,
    #[live] plot_area: AreaChart,
    #[live] plot_radar: RadarChart,
    #[live] plot_gauge: GaugeChart,
    #[live] plot_bubble: BubbleChart,
    #[live] plot_candlestick: CandlestickChart,
    #[live] plot_heatmap: HeatmapChart,
    #[live] plot_treemap: Treemap,
    #[live] plot_sankey: SankeyDiagram,
    #[live] plot_histogram: HistogramChart,
    #[live] plot_boxplot: BoxPlotChart,
    #[live] plot_donut: DonutChart,
    #[live] plot_stem: StemPlot,
    #[live] plot_violin: ViolinPlot,
    #[live] plot_polar: PolarPlot,
    #[live] plot_contour: ContourPlot,
    #[live] plot_waterfall: WaterfallChart,
    #[live] plot_funnel: FunnelChart,
    #[live] plot_step: StepPlot,
    #[live] plot_stackplot: Stackplot,
    #[live] plot_hexbin: HexbinChart,
    #[live] plot_streamgraph: Streamgraph,
    #[live] plot_surface3d: Surface3D,
    #[live] plot_scatter3d: Scatter3D,
    #[live] plot_line3d: Line3D,
    /// Draw audio bars visualization
    #[redraw]
    #[live]
    draw_audio_bars: DrawAudioBars,

    /// Image sources (preloaded)
    #[live]
    img_headphones: LiveDependency,
    #[live]
    img_mouse: LiveDependency,
    #[live]
    img_keyboard: LiveDependency,
    #[live]
    img_alipay: LiveDependency,
    #[live]
    img_wechat: LiveDependency,

    /// Loaded textures for images
    #[rust]
    texture_headphones: Option<Texture>,
    #[rust]
    texture_mouse: Option<Texture>,
    #[rust]
    texture_keyboard: Option<Texture>,
    #[rust]
    texture_alipay: Option<Texture>,
    #[rust]
    texture_wechat: Option<Texture>,

    /// Surface ID
    #[live]
    surface_id: LiveValue,

    /// The message processor (manages surfaces and data models)
    #[rust]
    processor: Option<A2uiMessageProcessor>,

    #[rust]
    area: Area,

    /// Flag to track if we're inside a card context (for correct text draw ordering)
    #[rust]
    inside_card: bool,

    /// Flag to track if we're inside a button context
    #[rust]
    inside_button: bool,

    /// Button areas for event.hits() detection - each button has independent Area
    #[rust]
    button_areas: Vec<Area>,

    /// Button metadata: (component_id, Option<ActionDefinition>, Option<scope>)
    #[rust]
    button_data: Vec<(String, Option<ActionDefinition>, Option<String>)>,

    /// Currently hovered button index (only one at a time)
    #[rust]
    hovered_button_idx: Option<usize>,

    /// Currently pressed button index (only one at a time)
    #[rust]
    pressed_button_idx: Option<usize>,

    /// Current template scope path for relative path resolution
    /// When rendering inside a template, this is set to the item path (e.g., "/products/0")
    #[rust]
    current_scope: Option<String>,

    // ============================================================================
    // TextField state tracking
    // ============================================================================

    /// TextField areas for event detection
    #[rust]
    text_field_areas: Vec<Area>,

    /// TextField metadata: (component_id, binding_path, current_value)
    #[rust]
    text_field_data: Vec<(String, Option<String>, String)>,

    /// Currently focused text field index
    #[rust]
    focused_text_field_idx: Option<usize>,

    /// Text input buffer for focused field
    #[rust]
    text_input_buffer: String,

    /// Cursor position in text input
    #[rust]
    cursor_pos: usize,

    // ============================================================================
    // CheckBox state tracking
    // ============================================================================

    /// CheckBox areas for event detection
    #[rust]
    checkbox_areas: Vec<Area>,

    /// CheckBox metadata: (component_id, binding_path, current_value)
    #[rust]
    checkbox_data: Vec<(String, Option<String>, bool)>,

    /// Currently hovered checkbox index
    #[rust]
    hovered_checkbox_idx: Option<usize>,

    // ============================================================================
    // Slider state tracking
    // ============================================================================

    /// Slider areas for event detection
    #[rust]
    slider_areas: Vec<Area>,

    /// Slider metadata: (component_id, binding_path, min, max, current_value)
    #[rust]
    slider_data: Vec<(String, Option<String>, f64, f64, f64)>,

    /// Currently dragging slider index
    #[rust]
    dragging_slider_idx: Option<usize>,

    /// Currently hovered slider index
    #[rust]
    hovered_slider_idx: Option<usize>,

    // ============================================================================
    // AudioPlayer state tracking
    // ============================================================================

    /// AudioPlayer button areas for event detection (play buttons)
    #[rust]
    audio_player_areas: Vec<Area>,

    /// AudioPlayer metadata: (component_id, audio_url, title)
    #[rust]
    audio_player_data: Vec<(String, String, String)>,

    /// Currently hovered audio player index
    #[rust]
    hovered_audio_player_idx: Option<usize>,

    /// Currently playing audio component ID (for Play/Stop toggle)
    #[rust]
    playing_component_id: Option<String>,
}

impl A2uiSurface {
    /// Initialize the surface with a processor
    pub fn init_processor(&mut self) {
        if self.processor.is_none() {
            self.processor = Some(A2uiMessageProcessor::with_standard_catalog());
        }
    }

    /// Clear all surfaces and reset the processor
    pub fn clear(&mut self) {
        // Reset the processor to clear all surfaces and components
        self.processor = Some(A2uiMessageProcessor::with_standard_catalog());
    }

    /// Apply theme colors to all A2UI components
    pub fn set_theme_colors(&mut self, cx: &mut Cx, colors: &A2uiThemeColors) {
        // Apply surface background
        self.draw_bg.apply_over(cx, live! {
            bg_color: (colors.bg_surface)
        });

        // Apply text colors
        self.draw_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        self.draw_card_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        // Apply card colors
        self.draw_card.apply_over(cx, live! {
            color: (colors.bg_card)
            border_color: (colors.border_color)
        });

        // Apply button colors - the shader uses hardcoded colors, so we update via instance
        self.draw_button.apply_over(cx, live! {
            color: (colors.accent)
        });

        self.draw_button_text.apply_over(cx, live! {
            color: (vec4(1.0, 1.0, 1.0, 1.0))
        });

        // Apply text field colors
        self.draw_text_field.apply_over(cx, live! {
            bg_color: (colors.input_bg)
            border_color: (colors.border_color)
        });

        self.draw_text_field_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        self.draw_text_field_placeholder.apply_over(cx, live! {
            color: (colors.text_secondary)
        });

        // Apply checkbox colors
        self.draw_checkbox.apply_over(cx, live! {
            bg_color: (colors.input_bg)
            border_color: (colors.border_color)
            check_color: (colors.control_fill)
        });

        self.draw_checkbox_label.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        // Apply slider colors
        self.draw_slider_track.apply_over(cx, live! {
            track_color: (colors.slider_track)
            fill_color: (colors.control_fill)
        });

        // Apply image placeholder text
        self.draw_image_text.apply_over(cx, live! {
            color: (colors.text_secondary)
        });
    }

    /// Load image textures from LiveDependency resources
    fn load_image_textures(&mut self, cx: &mut Cx) {
        use makepad_widgets::image_cache::ImageBuffer;

        // Load headphones image (JPG)
        if self.texture_headphones.is_none() {
            let path = self.img_headphones.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_headphones = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load mouse image (JPG)
        if self.texture_mouse.is_none() {
            let path = self.img_mouse.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_mouse = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load keyboard image (JPG)
        if self.texture_keyboard.is_none() {
            let path = self.img_keyboard.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_keyboard = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load Alipay icon (PNG)
        if self.texture_alipay.is_none() {
            let path = self.img_alipay.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_png(&data) {
                        self.texture_alipay = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load WeChat icon (PNG)
        if self.texture_wechat.is_none() {
            let path = self.img_wechat.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_png(&data) {
                        self.texture_wechat = Some(image.into_new_texture(cx));
                    }
                }
            }
        }
    }

    /// Get texture index for a given URL (0=headphones, 1=mouse, 2=keyboard, 3=alipay, 4=wechat, None=not found)
    fn get_texture_index_for_url(&self, url: &str) -> Option<usize> {
        if url.contains("headphones") && self.texture_headphones.is_some() {
            Some(0)
        } else if url.contains("mouse") && self.texture_mouse.is_some() {
            Some(1)
        } else if url.contains("keyboard") && self.texture_keyboard.is_some() {
            Some(2)
        } else if url.contains("alipay") && self.texture_alipay.is_some() {
            Some(3)
        } else if url.contains("wechat") && self.texture_wechat.is_some() {
            Some(4)
        } else {
            None
        }
    }

    /// Get the processor
    pub fn processor(&self) -> Option<&A2uiMessageProcessor> {
        self.processor.as_ref()
    }

    /// Get mutable processor
    pub fn processor_mut(&mut self) -> Option<&mut A2uiMessageProcessor> {
        self.processor.as_mut()
    }

    /// Set the currently playing audio component ID (for Play/Stop toggle display)
    pub fn set_playing_component(&mut self, component_id: Option<String>) {
        self.playing_component_id = component_id;
    }

    /// Get the currently playing audio component ID
    pub fn playing_component_id(&self) -> Option<&String> {
        self.playing_component_id.as_ref()
    }

    /// Process A2UI JSON messages
    pub fn process_json(&mut self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&mut self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_message(message)
        } else {
            vec![]
        }
    }

    /// Get the current surface ID
    fn get_surface_id(&self) -> String {
        // For now, use "main" as default
        "main".to_string()
    }
}

// Widget trait implementation (handle_event + draw_walk)
include!("events_impl.rs");

// Render methods - layout and basic components
include!("render_impl.rs");

// Render methods - charts, chord, audio player
include!("render_charts_impl.rs");

impl A2uiSurfaceRef {
    /// Process A2UI JSON messages
    pub fn process_json(&self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_message(message)
        } else {
            vec![]
        }
    }

    /// Check if any user action was triggered
    /// Returns the UserAction if one was triggered
    pub fn user_action(&self, actions: &Actions) -> Option<UserAction> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let A2uiSurfaceAction::UserAction(user_action) =
                    action.cast::<A2uiSurfaceAction>()
                {
                    return Some(user_action);
                }
            }
        }
        None
    }

    /// Check if a specific action was triggered by name
    /// Returns the context HashMap if the action matches
    pub fn action_by_name(
        &self,
        actions: &Actions,
        action_name: &str,
    ) -> Option<std::collections::HashMap<String, serde_json::Value>> {
        if let Some(user_action) = self.user_action(actions) {
            if user_action.action.name == action_name {
                return Some(user_action.action.context);
            }
        }
        None
    }

    /// Check if an audio play action was triggered
    /// Returns (component_id, url, title) if PlayAudio was triggered
    pub fn play_audio(&self, actions: &Actions) -> Option<(String, String, String)> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let A2uiSurfaceAction::PlayAudio { component_id, url, title } =
                    action.cast::<A2uiSurfaceAction>()
                {
                    return Some((component_id, url, title));
                }
            }
        }
        None
    }

    /// Set the currently playing audio component ID (for Play/Stop toggle display)
    pub fn set_playing_component(&self, component_id: Option<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_playing_component(component_id);
        }
    }
}
