use eframe::egui;

#[allow(unused_imports)]
use egui::{
    Align2, Area, Frame, Margin, Rounding,
    RichText, SidePanel, TopBottomPanel, CentralPanel, Vec2, Window, vec2, Color32
};

use crate::style::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // #[serde(skip)]
    // svg_image: egui_extras::RetainedImage,

    welcome_window_open: bool,
    connect_window_open: bool,
    alert_window_open: bool,
    side_panel_open: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // svg_image: egui_extras::RetainedImage::from_svg_bytes_with_size(
            //     "scrt.svg",
            //     include_bytes!("scrt.svg"),
            //     egui_extras::image::FitTo::Original,
            // )
            // .unwrap(),
            welcome_window_open: true,
            connect_window_open: false,
            alert_window_open: false,
            side_panel_open: true,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_style(crate::style::my_style());
        cc.egui_ctx.set_fonts(crate::style::my_font_definitions());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            // svg_image: _,
            welcome_window_open,
            connect_window_open,
            alert_window_open,
            side_panel_open,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
                ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
                ui.menu_button("File", |ui| {
                    ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
                    ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    };
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                        ui.close_menu();
                    };
                });
                ui.menu_button("View", |ui| {
                    ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
                    ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
                    if ui.button("Toggle Sidebar").clicked() {
                        ui.style_mut().animation_time = 1.0;
                        *side_panel_open = !*side_panel_open;
                    }
                });
                ui.menu_button("Help", |ui| ui.button("Button"));
            });
            Area::new("notification")
                .anchor(Align2::RIGHT_TOP, Vec2::new(-12.0, 1.0))
                .show(ctx, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.style_mut().visuals.hyperlink_color = Color32::from_rgb(242, 176, 70);
                    ui.label(
                        RichText::new("Notifications will show up here")
                            .color(Color32::from_rgb(242, 176, 70))
                            .italics(),
                    );
                });
        });

        #[cfg(target_arch = "wasm32")]
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
            ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("Menu", |ui| {
                    ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
                    ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
                    if ui.button("Button").clicked() {
                        ui.close_menu()
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(0.0);
                    ui.style_mut().visuals.widgets.active.rounding = Rounding::same(0.0);
                    if ui.button("Toggle Sidebar").clicked() {
                        *side_panel_open = !*side_panel_open;
                    }
                });
                // ui.horizontal(|ui| {
                //     ui.add_space(10.0);
                //     ui.hyperlink("Telegram");
                //     ui.hyperlink("Twitter");
                //     ui.hyperlink("Github");
                // });
            });
            Area::new("notification")
                .anchor(Align2::RIGHT_TOP, Vec2::new(-12.0, 1.0))
                .show(ctx, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.style_mut().visuals.hyperlink_color = Color32::from_rgb(242, 176, 70);
                    ui.link(
                        RichText::new("Notifications will show up here")
                            .color(Color32::from_rgb(242, 176, 70))
                            .italics(),
                    );
                });
        });

        TopBottomPanel::bottom("bottom_panel")
            .exact_height(26.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // ui.hyperlink("https://github.com/kent-3/egui-experiments");
                    ui.add(egui::github_link_file!(
                        "https://github.com/kent-3/egui-experiments/blob/master/",
                        "Source code."
                    ));
                    egui::warn_if_debug_build(ui);
                    // egui::widgets::global_dark_light_mode_buttons(ui);
                });
                Area::new("my_area")
                    .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-8.0, 0.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.add(CustomHyperlink::from_label_and_url("Telegram", "https://t.me/AmberDAOscrt"));
                            ui.add(CustomHyperlink::from_label_and_url("Twitter", "https://twitter.com/AmberDAO_"));
                            ui.add(CustomHyperlink::from_label_and_url("GitHub", "https://github.com/kent-3/amber"));
                            egui::widgets::global_dark_light_mode_switch(ui);
                        });
                    });
            });

        change_animation_time(ctx, 1.0 / 6.0 );

        SidePanel::right("right_panel")
            .resizable(true)
            .frame(
                Frame::side_top_panel(&ctx.style()).inner_margin(Margin::same(10.0)), // .fill(egui::Color32::from_rgb(45, 47, 49)),
            )
            .max_width(200.0)
            .show_animated(ctx, *side_panel_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("SidePanel");
                    ui.separator();
                    ui.add_space(4.0);
                    if ui.button("Button").clicked() {
                        *connect_window_open = true;
                        *alert_window_open = true;
                    };
                    ui.add_space(4.0);

                    change_animation_time(ctx, 0.30);
                    ui.collapsing("Collapsing Header", |ui| {
                        ui.label("label");
                        ui.label("label");
                        ui.label("label");
                        ui.label("label");
                        ui.label("label");
                    });
                    reset_animation_time(ctx);

                    ui.add_space(4.0);
                })
            });

        reset_animation_time(ctx);

        CentralPanel::default().show(ctx, |ui| {
            // The central panel is the region left after adding TopPanels and SidePanels

            // Area::new("background_image")
            //     .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            //     .order(egui::Order::Background)
            //     .show(ctx, |ui| {
            //         self.svg_image.show_max_size(ui, egui::Vec2 { x: 400.0, y: 400.0 });
            //     });

            // Uncomment this window to mess with the style settings
            // Window::new("style settings").default_open(true).scroll2([true,true]).show(ctx, |ui| {
            //     ctx.style_ui(ui);
            // });

            // Create a window within the area
            // .default_pos(egui::pos2(0.0, 0.0))
            Window::new("Welcome Message")
                .resizable(true)
                .open(welcome_window_open)
                .default_width(400.0)
                .scroll2([false, false])
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.add_space(20.0);
                            ui.label(LOREM_IPSUM);
                        });
                        ui.add_space(20.0);
                    });
                });
            Window::new("Window")
                .anchor(Align2::RIGHT_TOP, vec2(-16.0, 16.0))
                .open(connect_window_open)
                .pivot(Align2::RIGHT_TOP)
                .default_width(240.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(LOREM_IPSUM_SHORT);
                        ui.add_space(20.0);
                    })
                });

            Window::new("Alert")
                .anchor(Align2::RIGHT_BOTTOM, vec2(-16.0, -16.0))
                .frame(Frame {
                    fill: Color32::from_rgb(41, 54, 31),
                    ..Frame::window(&ctx.style())
                })
                .pivot(Align2::RIGHT_BOTTOM)
                .resizable(false)
                .open(&mut self.alert_window_open)
                .default_width(240.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(LOREM_IPSUM_SHORT);
                        ui.add_space(20.0);
                    })
                });
        });
        
    }
}

pub const LOREM_IPSUM_SHORT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

//---------------------------------------------------------------
// I needed to make a CustomHyperlink struct to open links in new tab by default.
// The normal Hyperlink only opens in a new tab if a keyboard modifer is active.
// ui.add(CustomHyperlink::from_label_and_url("new tab", "about:blank"));

use egui::{WidgetText, Ui, Widget, Response, Link};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct CustomHyperlink {
    url: String,
    text: WidgetText,
}

#[allow(dead_code)]
impl CustomHyperlink {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(url: impl ToString) -> Self {
        let url = url.to_string();
        Self {
            url: url.clone(),
            text: url.into(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn from_label_and_url(text: impl Into<WidgetText>, url: impl ToString) -> Self {
        Self {
            url: url.to_string(),
            text: text.into(),
        }
    }
}

impl Widget for CustomHyperlink {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { url, text } = self;

        let response = ui.add(Link::new(text));
        if response.clicked() {
            ui.ctx().output_mut(|o| {
                o.open_url = Some(egui::output::OpenUrl {
                    url: url.clone(),
                    new_tab: true,
                });
            });
        }
        if response.middle_clicked() {
            ui.ctx().output_mut(|o| {
                o.open_url = Some(egui::output::OpenUrl {
                    url: url.clone(),
                    new_tab: true,
                });
            });
        }
        response.on_hover_text(url)
    }
}
