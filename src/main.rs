use eframe::egui::Context;
use eframe::egui::*;
use rhai::{packages::Package, Engine, Scope, AST};
pub struct App {
    pub engine: Engine,
    pub ast: AST,
    pub scope: Scope<'static>,
    pub rhai_code: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
        Window::new("rhai editor")
            .scroll2([true, true])
            .show(ctx, |ui| {
                ui.add(
                    egui::widgets::TextEdit::multiline(&mut self.rhai_code)
                        .code_editor()
                        .desired_width(500.0),
                );
                if ui.button("compile").clicked() {
                    match self.engine.compile_with_scope(&self.scope, &self.rhai_code) {
                        Ok(ast) => self.ast = ast,
                        Err(e) => panic!("failed to compile rhai_code due to error: {e}"),
                    }
                    tracing::warn!("compile button clicked");
                }
            });
        self.scope.set_value("CTX", ctx.clone());
        self.engine
            .eval_ast_with_scope::<()>(&mut self.scope, &self.ast)
            .expect("failed to eval script");
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 100).into()

        // _visuals.window_fill() would also be a natural choice
    }
}

fn make_app() -> App {
    let mut engine = Engine::new();
    engine.register_fn("warn", |content: &str| {
        tracing::warn!(content);
    });
    engine.register_type_with_name::<Context>("Context");
    engine.register_global_module(rhai_egui::EguiPackage::new().as_shared_module());
    let scope = Scope::new();
    let rhai_code = r#"
    CTX.request_repaint();
    CTX.window("my window", |ui| {

        ui.label("hello");

        if ui.button("random button").clicked() {
            warn("clicked me");
        };

        ui.checkbox(true, "always checked");

        ui.hyperlink("https://egui.rs").context_menu( |ui| {
            ui.button("menu button");
        });
        });"#
        .to_string();

    let ast = engine
        .compile_with_scope(&scope, &rhai_code)
        .expect("failed to compile");

    App {
        engine,
        ast,
        scope,
        rhai_code,
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();
    let app = make_app();
    eframe::run_native(
        "rhai_egui",
        eframe::NativeOptions {
            initial_window_size: Some(vec2(800.0, 600.0)),
            ..Default::default()
        },
        Box::new(|_| Box::new(app)),
    );
}
#[cfg(target_arch = "wasm32")]
fn main() {
    tracing_wasm::set_as_global_default();
    let app = make_app();

    eframe::start_web("the_canvas_id", Box::new(|_| Box::new(app)))
        .expect("failed to start eframe web");
}
