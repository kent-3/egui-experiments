use egui::style::{
    Interaction, Margin, Selection, Spacing, Style, TextStyle, Visuals,
    WidgetVisuals, Widgets,
};
use egui::{vec2, Color32, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Rounding};
use epaint::{Shadow, Stroke, FontFamily::{Monospace, Proportional}};

#[inline]
pub fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
pub fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn my_style() -> Style {
    Style {
        override_font_id: None,
        override_text_style: None,
        text_styles: [
            (TextStyle::Heading, FontId::new(18.0, Proportional)),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(22.0, Proportional),
            ),
            (
                TextStyle::Name("ContextHeading".into()),
                FontId::new(19.0, Proportional),
            ),
            (TextStyle::Body, FontId::new(14.0, Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, Monospace)),
            (TextStyle::Button, FontId::new(14.0, Proportional)),
            (TextStyle::Small, FontId::new(10.0, Proportional)),
        ]
        .into(),
        drag_value_text_style: TextStyle::Button,
        wrap: None,
        spacing: my_spacing(),
        interaction: Interaction {
            show_tooltips_only_when_still: false,
            ..Default::default()
        },
        visuals: my_dark_visuals(),
        animation_time: 1.0 / 12.0,
        debug: Default::default(),
        explanation_tooltips: false,
    }
}

pub fn my_spacing() -> Spacing {
    Spacing {
        item_spacing: vec2(8.0, 3.0),
        window_margin: Margin::same(6.0),
        menu_margin: Margin::same(6.0),
        button_padding: vec2(8.0, 2.0),
        indent: 18.0, // match checkbox/radio-button with `button_padding.x + icon_width + icon_spacing`
        interact_size: vec2(40.0, 18.0),
        slider_width: 100.0,
        combo_width: 100.0,
        text_edit_width: 280.0,
        icon_width: 14.0,
        icon_width_inner: 8.0,
        icon_spacing: 4.0,
        tooltip_width: 400.0,
        combo_height: 200.0,
        scroll_bar_width: 8.0,
        scroll_handle_min_length: 12.0,
        scroll_bar_inner_margin: 4.0,
        scroll_bar_outer_margin: 0.0,
        indent_ends_with_horizontal_line: false,
    }
}

pub fn my_widgets_styles() -> Widgets {
    Widgets {
        noninteractive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(27),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines
            fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // normal text color
            rounding: Rounding::same(0.0),
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(60), // button background
            bg_fill: Color32::from_gray(60),      // checkbox background
            bg_stroke: Default::default(),
            fg_stroke: Stroke::new(1.0, Color32::from_gray(180)), // button text
            rounding: Rounding::same(20.0),
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(0, 74, 119),
            bg_fill: Color32::from_gray(70),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
            fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
            rounding: Rounding::same(20.0),
            expansion: 1.0,
        },
        active: WidgetVisuals {
            weak_bg_fill: Color32::from_rgb(0, 74, 119),
            bg_fill: Color32::from_gray(55),
            bg_stroke: Stroke::new(1.0, Color32::WHITE),
            fg_stroke: Stroke::new(2.0, Color32::WHITE),
            rounding: Rounding::same(20.0),
            expansion: 1.0,
        },
        open: WidgetVisuals {
            weak_bg_fill: Color32::from_gray(27),
            bg_fill: Color32::from_gray(27),
            bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
            fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
            rounding: Rounding::same(2.0),
            expansion: 0.0,
        },
    }
}

pub fn my_dark_visuals() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::from_rgb(227, 227, 227)),
        widgets: my_widgets_styles(),
        selection: Selection::default(),
        hyperlink_color: Color32::from_rgb(90, 170, 255),
        faint_bg_color: Color32::from_additive_luminance(5), // visible, but barely so
        extreme_bg_color: Color32::from_gray(10),            // e.g. TextEdit background
        code_bg_color: Color32::from_gray(64),
        warn_fg_color: Color32::from_rgb(255, 143, 0), // orange
        error_fg_color: Color32::from_rgb(255, 0, 0),  // red

        window_rounding: Rounding::same(0.0),
        window_shadow: Shadow {
            color: Color32::from_black_alpha(50),
            extrusion: 32.0,
        },
        window_fill: Color32::from_rgb(40, 41, 42),
        window_stroke: Stroke::new(1.0, Color32::from_gray(60)),

        menu_rounding: Rounding::same(0.0),

        panel_fill: Color32::from_gray(31),

        popup_shadow: Shadow {
            color: Color32::from_black_alpha(50),
            extrusion: 16.0,
        },
        resize_corner_size: 12.0,
        text_cursor_width: 2.0,
        text_cursor_preview: true,
        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: true,
    }
}

pub fn my_font_definitions() -> FontDefinitions {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "open_sans".to_owned(),
        FontData::from_static(include_bytes!(
            "../fonts/OpenSans-Regular.ttf"
        ))
        .tweak(FontTweak {
            scale: 1.0,
            y_offset_factor: -0.2,
            y_offset: 0.0,
        }),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "open_sans".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("open_sans".to_owned());

    fonts
}

pub fn reset_animation_time(ctx: &egui::Context) {
    let mut style: egui::Style = (*ctx.style()).clone();
    // 1.0 / 12.0 is the default
    style.animation_time = 1.0 / 12.0;
    ctx.set_style(style);
}

pub fn change_animation_time(ctx: &egui::Context, time: f32) {
    let mut style: egui::Style = (*ctx.style()).clone();
    style.animation_time = time;
    ctx.set_style(style);
}
