use eframe::egui::Context;
use eframe::egui::*;
use rhai::{packages::Package, Engine, Scope, AST};
pub struct App {
    pub engine: Engine,
    pub ast: AST,
    pub scope: Scope<'static>,
    pub rhai_code: String,
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        Window::new("rhai editor")
        .scroll2([true, true])
        .show(ctx, |ui| {
            ui.add(egui::widgets::TextEdit::multiline(&mut self.rhai_code).code_editor().desired_width(500.0));
            if ui.button("compile").clicked() {
                match self.engine.compile_with_scope(&self.scope, &self.rhai_code) {
                    Ok(ast) => self.ast = ast,
                    Err(e) => panic!("failed to compile rhai_code due to error: {e}"),
                }
            }
        });
        self.scope.set_value("CTX", ctx.clone());
        self.engine
            .eval_ast_with_scope::<()>(&mut self.scope, &self.ast)
            .expect("failed to eval script");
    }

    fn name(&self) -> &str {
        "rhai machine"
    }
}
fn main() {
    let mut engine = Engine::new();
    engine.register_type_with_name::<Context>("Context");
    engine.register_global_module(rhai_egui::EguiPackage::new().as_shared_module());
    let scope = Scope::new();
    let rhai_code = r#"CTX.window("my window", |ui| {

        ui.label("hello");

        if ui.button("random button").clicked() {
            print("clicked me");
        };

        ui.checkbox(true, "always checked");

        ui.hyperlink("https://egui.rs").context_menu( |ui| {

            if ui.button("menu button").clicked() {
                print("clicking menu");          
            }

        });
        });"#
    .to_string();

    let ast = engine
        .compile_with_scope(&scope, &rhai_code)
        .expect("failed to compile");

    let app = App {
        engine,
        ast,
        scope,
        rhai_code,
    };

    eframe::run_native(Box::new(app), eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 600.0)),
        ..Default::default()
    })
}
