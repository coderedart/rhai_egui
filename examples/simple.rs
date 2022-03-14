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
        Window::new("rhai editor").show(ctx, |ui| {
            ui.code_editor(&mut self.rhai_code);
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
        label(ui, "hello");
        if button(ui, "random button").clicked() {
            print("clicked me");
        };
        check_box(ui, true, "always checked");
        }
        );"#.to_string();
    let ast = engine
        .compile_with_scope(&scope, &rhai_code)
        .expect("failed to compile");
    let app = App {
        engine,
        ast,
        scope,
        rhai_code,
    };
    eframe::run_native(Box::new(app), eframe::NativeOptions::default())
}
