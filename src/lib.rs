
use egui::*;
use rhai::*;
use rhai::plugin::*;
pub fn cast_i64_to_ref(ui: i64) -> &'static mut Ui {
    let ui = ui as usize as *mut Ui;
     unsafe { ui.as_mut().expect("failed to cast back to ptr") }
}
pub fn cast_ref_to_i64(ui: &mut Ui) -> i64 {
    ui as *mut Ui as i64
}

def_package! {
    /// Package to interact with egui context (to add UI elements dynamically)
    pub EguiPackage(lib) {
        combine_with_exported_module!(lib, "context_api", egui_context_api);

        combine_with_exported_module!(lib, "ui_api", egui_ui_api);
    }
}
#[export_module]
mod egui_context_api {
    pub fn request_repaint(ctx: &mut Context) {
        ctx.request_repaint();
    }
    pub fn style_ui(ctx: &mut Context, ui: i64) {
        ctx.style_ui(cast_i64_to_ref(ui));
    }

    pub fn window(rtx: NativeCallContext, ctx: &mut Context, title: &str, cb: FnPtr) {
        let window = {
            Window::new(title)
        };
        window.show(ctx, |ui| {
            let _: Result<(), _> = cb.call_within_context(&rtx, (cast_ref_to_i64(ui),));
        });
    }
}

#[export_module]
mod egui_ui_api {
    pub fn button(ui: i64, text: &str) -> Response {
        cast_i64_to_ref(ui).button(text)
    }
    pub fn check_box(ui: i64, checked: bool, text: &str) -> Response {
        let mut checked = checked;
        cast_i64_to_ref(ui).checkbox(&mut checked, text)
    }
    pub fn label(ui: i64, text: &str) -> Response {
        cast_i64_to_ref(ui).label(text)
    }
}