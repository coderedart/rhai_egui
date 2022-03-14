
use std::ops::{DerefMut, Deref};

use egui::*;
use rhai::*;
use rhai::plugin::*;
pub fn cast_usize_to_ref(ui: usize) -> &'static mut Ui {
    let ui = ui as *mut Ui;
     unsafe { ui.as_mut().expect("failed to cast back to ptr") }
}
pub fn cast_ref_to_usize(ui: &mut Ui) -> usize {
    ui as *mut Ui as usize
}

def_package! {
    /// Package to interact with egui context (to add UI elements dynamically)
    pub EguiPackage(lib) {
        combine_with_exported_module!(lib, "context_api", egui_context_api);

        combine_with_exported_module!(lib, "ui_api", egui_ui_api);

        combine_with_exported_module!(lib, "response:api", egui_response_api);
    }
}
#[export_module]
mod egui_context_api {
    pub fn request_repaint(ctx: &mut Context) {
        ctx.request_repaint();
    }
    

    pub fn window(rtx: NativeCallContext, ctx: &mut Context, title: &str, cb: FnPtr) {
        let window = {
            Window::new(title)
        };
        window.show(ctx, |ui| {
            let my_ui: MyUi = ui.into();
            let _: Result<(), _> = cb.call_within_context(&rtx, (my_ui,));
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MyUi(usize);
impl From<&mut Ui> for MyUi {
    fn from(ui: &mut Ui) -> Self {
        MyUi(cast_ref_to_usize(ui))
    }
}
impl Deref for MyUi {
    type Target = Ui;

    fn deref(&self) -> &Self::Target {
        cast_usize_to_ref(self.0)
    }
}
impl DerefMut for MyUi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        cast_usize_to_ref(self.0)
    }
}
#[export_module]
mod egui_ui_api {
    pub fn button(ui: &mut MyUi, text: &str) -> Response {
        ui.button(text)
    }
    pub fn checkbox(ui: &mut MyUi, checked: bool, text: &str) -> Response {
        let mut checked = checked;
        ui.checkbox(&mut checked, text)
    }
    pub fn label(ui: &mut MyUi, text: &str) -> Response {
        ui.label(text)
    }
 
    /// Reset to the default style set in [`Context`].
    pub fn reset_style(ui: &mut MyUi) {
        ui.reset_style();
    }




    /// If `false`, the `Ui` does not allow any interaction and
    /// the widgets in it will draw with a gray look.
    pub fn is_enabled(ui: &mut MyUi) -> bool {
        ui.is_enabled()
    }

    /// Calling `set_enabled(false)` will cause the `Ui` to deny all future interaction
    /// and all the widgets will draw with a gray look.
    ///
    /// Usually it is more convenient to use [`Self::add_enabled_ui`] or [`Self::add_enabled`].
    ///
    /// Calling `set_enabled(true)` has no effect - it will NOT re-enable the `Ui` once disabled.
    ///
    /// ### Example
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// # let mut enabled = true;
    /// ui.group(|ui| {
    ///     ui.checkbox(&mut enabled, "Enable subsection");
    ///     ui.set_enabled(enabled);
    ///     if ui.button("Button that is not always clickable").clicked() {
    ///         /* … */
    ///     }
    /// });
    /// # });
    /// ```
    pub fn set_enabled(ui:&mut MyUi, enabled: bool) {
        ui.set_enabled(enabled)
    }

    /// If `false`, any widgets added to the `Ui` will be invisible and non-interactive.
    pub fn is_visible(ui: &mut MyUi) -> bool {
        ui.is_visible()
    }



    /// Calling `set_visible(false)` will cause all further widgets to be invisible,
    /// yet still allocate space.
    ///
    /// The widgets will not be interactive (`set_visible(false)` implies `set_enabled(false)`).
    ///
    /// Calling `set_visible(true)` has no effect.
    ///
    /// ### Example
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// # let mut visible = true;
    /// ui.group(|ui| {
    ///     ui.checkbox(&mut visible, "Show subsection");
    ///     ui.set_visible(visible);
    ///     if ui.button("Button that is not always shown").clicked() {
    ///         /* … */
    ///     }
    /// });
    /// # });
    /// ```
    pub fn set_visible(ui: &mut MyUi, visible: bool) {
        ui.set_visible(visible);
    }


    /// Should text wrap in this `Ui`?
    ///
    /// This is determined first by [`Style::wrap`], and then by the layout of this `Ui`.
    pub fn wrap_text(ui: &mut MyUi) -> bool {
        ui.wrap_text()
    }

    /// Is the pointer (mouse/touch) above this `Ui`?
    /// Equivalent to `ui.rect_contains_pointer(ui.min_rect())`
    pub fn ui_contains_pointer(ui: &mut MyUi) -> bool {
        ui.ui_contains_pointer()
    }

    /// Add extra space before the next widget.
    ///
    /// The direction is dependent on the layout.
    /// This will be in addition to the [`crate::style::Spacing::item_spacing`].
    ///
    /// [`Self::min_rect`] will expand to contain the space.
    pub fn add_space(ui: &mut MyUi, amount: f32) {
        ui.add_space(amount)
    }

    /// Show large text.
    ///
    /// Shortcut for `ui.label(RichText::new(text).heading())`
    pub fn heading(ui: &mut MyUi, text: &str) -> Response {
        ui.heading(text)
    }

    /// Show monospace (fixed width) text.
    ///
    /// Shortcut for `ui.label(RichText::new(text).monospace())`
    pub fn monospace(ui: &mut MyUi, text: &str) -> Response {
        ui.monospace(text)
    }

    /// Show text as monospace with a gray background.
    ///
    /// Shortcut for `ui.label(RichText::new(text).code())`
    pub fn code(ui: &mut MyUi, text: &str) -> Response {
        ui.code(text)
    }

    /// Show small text.
    ///
    /// Shortcut for `ui.label(RichText::new(text).small())`
    pub fn small(ui: &mut MyUi, text: &str) -> Response {
        ui.small(text)
    }

    /// Show text that stand out a bit (e.g. slightly brighter).
    ///
    /// Shortcut for `ui.label(RichText::new(text).strong())`
    pub fn strong(ui: &mut MyUi, text: &str) -> Response {
        ui.strong(text)
    }

    /// Show text that is waker (fainter color).
    ///
    /// Shortcut for `ui.label(RichText::new(text).weak())`
    pub fn weak(ui: &mut MyUi, text: &str) -> Response {
        ui.weak(text)
    }

    /// Shortcut for `add(Hyperlink::new(url))`
    ///
    /// See also [`Hyperlink`].
    pub fn hyperlink(ui: &mut MyUi, url: &str) -> Response {
        ui.hyperlink(url)
    }

    /// Shortcut for `add(Hyperlink::new(url).text(label))`
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// ui.hyperlink_to("egui on GitHub", "https://www.github.com/emilk/egui/");
    /// # });
    /// ```
    ///
    /// See also [`Hyperlink`].
    pub fn hyperlink_to(ui: &mut MyUi, label: &str, url: &str) -> Response {
        ui.hyperlink_to(label, url)
    }


    /// A button as small as normal body text.
    ///
    /// Usage: `if ui.small_button("Click me").clicked() { … }`
    ///
    /// Shortcut for `add(Button::new(text).small())`
    pub fn small_button(ui: &mut MyUi, text: &str) -> Response {
        ui.small_button(text)
    }


    /// Show a [`RadioButton`].
    /// Often you want to use [`Self::radio_value`] instead.

    pub fn radio(ui: &mut MyUi, selected: bool, text: &str) -> Response {
        ui.radio(selected, text)
    }

    /// Show a label which can be selected or not.
    ///
    /// See also [`SelectableLabel`].
    pub fn selectable_label(ui: &mut MyUi, checked: bool, text: &str) -> Response {
        ui.selectable_label(checked, text)
    }

    /// Shortcut for `add(Separator::default())` (see [`Separator`]).
    pub fn separator(ui: &mut MyUi) -> Response {
        ui.separator()
    }

}

#[export_module]
mod egui_response_api {
    #[inline(always)]
    pub fn clicked(resp: &mut Response) -> bool {
       resp.clicked()
    }

    /// Returns true if this widget was clicked this frame by the given button.
    pub fn clicked_by(resp: &mut Response, button: PointerButton) -> bool {
       resp.clicked_by(button)
    }

    /// Returns true if this widget was clicked this frame by the secondary mouse button (e.g. the right mouse button).
    pub fn secondary_clicked(resp: &mut Response) -> bool {
       resp.secondary_clicked()
    }

    /// Returns true if this widget was clicked this frame by the middle mouse button.
    pub fn middle_clicked(resp: &mut Response) -> bool {
       resp.middle_clicked()
    }

    /// Returns true if this widget was double-clicked this frame by the primary button.
    pub fn double_clicked(resp: &mut Response) -> bool {
       resp.double_clicked()
    }

    /// Returns true if this widget was double-clicked this frame by the given button.
    pub fn double_clicked_by(resp: &mut Response, button: PointerButton) -> bool {
       resp.double_clicked_by(button)
    }

    /// `true` if there was a click *outside* this widget this frame.
    pub fn clicked_elsewhere(resp: &mut Response) -> bool {
        resp.clicked_elsewhere()
    }

    /// Was the widget enabled?
    /// If false, there was no interaction attempted
    /// and the widget should be drawn in a gray disabled look.
    #[inline(always)]
    pub fn enabled(resp: &mut Response) -> bool {
       resp.enabled()
    }

    /// The pointer is hovering above this widget or the widget was clicked/tapped this frame.
    ///
    /// Note that this is slightly different from checking `response.rect.contains(pointer_pos)`.
    /// For one, the hover rectangle is slightly larger, by half of the current item spacing
    /// (to make it easier to click things). But `hovered` also checks that no other area
    /// is covering this response rectangle.
    #[inline(always)]
    pub fn hovered(resp: &mut Response) -> bool {
       resp.hovered()
    }

    /// This widget has the keyboard focus (i.e. is receiving key presses).
    pub fn has_focus(resp: &mut Response) -> bool {
       resp.has_focus()
    }

    /// True if this widget has keyboard focus this frame, but didn't last frame.
    pub fn gained_focus(resp: &mut Response) -> bool {
       resp.gained_focus()
    }

    /// The widget had keyboard focus and lost it,
    /// either because the user pressed tab or clicked somewhere else,
    /// or (in case of a [`crate::TextEdit`]) because the user pressed enter.
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// # let mut my_text = String::new();
    /// # fn do_request(_: &str) {}
    /// let response = ui.text_edit_singleline(&mut my_text);
    /// if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
    ///     do_request(&my_text);
    /// }
    /// # });
    /// ```
    pub fn lost_focus(resp: &mut Response) -> bool {
       resp.lost_focus()
    }

    /// Request that this widget get keyboard focus.
    pub fn request_focus(resp: &mut Response) {
       resp.request_focus();
    }

    /// Surrender keyboard focus for this widget.
    pub fn surrender_focus(resp: &mut Response) {
       resp.surrender_focus();
    }

    /// The widgets is being dragged.
    ///
    /// To find out which button(s), query [`crate::PointerState::button_down`]
    /// (`ui.input().pointer.button_down(…)`).
    ///
    /// Note that the widget must be sensing drags with [`Sense::drag`].
    /// [`crate::DragValue`] senses drags; [`crate::Label`] does not (unless you call [`crate::Label::sense`]).
    ///
    /// You can use [`resp::interact`] to sense more things *after* adding a widget.
    #[inline(always)]
    pub fn dragged(resp: &mut Response) -> bool {
       resp.dragged()
    }

    pub fn dragged_by(resp: &mut Response, button: PointerButton) -> bool {
       resp.dragged_by(button)
    }

    /// Did a drag on this widgets begin this frame?
    pub fn drag_started(resp: &mut Response) -> bool {
       resp.drag_started()
    }

    /// The widget was being dragged, but now it has been released.
    pub fn drag_released(resp: &mut Response) -> bool {
       resp.drag_released()
    }

    /// If dragged, how many points were we dragged and in what direction?
    pub fn drag_delta(resp: &mut Response) -> Vec2 {
        resp.drag_delta()
    }

    /// Where the pointer (mouse/touch) were when when this widget was clicked or dragged.
    /// `None` if the widget is not being interacted with.
    pub fn interact_pointer_pos(resp: &mut Response) -> Option<Pos2> {
       resp.interact_pointer_pos()
    }

    /// If it is a good idea to show a tooltip, where is pointer?
    /// None if the pointer is outside the response area.
    pub fn hover_pos(resp: &mut Response) -> Option<Pos2> {
        resp.hover_pos()
    }

    /// Is the pointer button currently down on this widget?
    /// This is true if the pointer is pressing down or dragging a widget
    #[inline(always)]
    pub fn is_pointer_button_down_on(resp: &mut Response) -> bool {
       resp.is_pointer_button_down_on()
    }

    /// What the underlying data changed?
    ///
    /// e.g. the slider was dragged, text was entered in a `TextEdit` etc.
    /// Always `false` for something like a `Button`.
    ///
    /// Can sometimes be `true` even though the data didn't changed
    /// (e.g. if the user entered a character and erased it the same frame).
    ///
    /// This is not set if the *view* of the data was changed.
    /// For instance, moving the cursor in a `TextEdit` does not set this to `true`.
    #[inline(always)]
    pub fn changed(resp: &mut Response) -> bool {
       resp.changed()
    }

    /// Report the data shown by this widget changed.
    ///
    /// This must be called by widgets that represent some mutable data,
    /// e.g. checkboxes, sliders etc.
    ///
    /// This should be called when the *content* changes, but not when the view does.
    /// So we call this when the text of a [`crate::TextEdit`], but not when the cursors changes.
    #[inline(always)]
    pub fn mark_changed(resp: &mut Response) {
       resp.mark_changed()
    }

    /// Show this UI if the widget was hovered (i.e. a tooltip).
    ///
    /// The text will not be visible if the widget is not enabled.
    /// For that, use [`resp::on_disabled_hover_ui`] instead.
    ///
    /// If you call this multiple times the tooltips will stack underneath the previous ones.
    // #[doc(alias = "tooltip")]
    // pub fn on_hover_ui(resp: &mut Response, add_contents: impl FnOnce(&mut Ui)) -> resp {
    //     if resp.should_show_hover_ui() {
    //         egui::containers::show_tooltip_for(
    //             &resp.ctx,
    //            resp.id.with("__tooltip"),
    //             &resp.rect,
    //             add_contents,
    //         );
    //     }
    //    resp
    // }

    // /// Show this UI when hovering if the widget is disabled.
    // pub fn on_disabled_hover_ui(resp: &mut Response, add_contents: impl FnOnce(&mut Ui)) -> resp {
    //     if !resp.enabled &&resp.ctx.rect_contains_pointer(resp.layer_id,resp.rect) {
    //         crate::containers::show_tooltip_for(
    //             &resp.ctx,
    //            resp.id.with("__tooltip"),
    //             &resp.rect,
    //             add_contents,
    //         );
    //     }
    //    resp
    // }

    // /// Like `on_hover_ui`, but show the ui next to cursor.
    // pub fn on_hover_ui_at_pointer(resp: &mut Response, add_contents: impl FnOnce(&mut Ui)) -> Response {
    //     if resp.should_show_hover_ui() {
    //         crate::containers::show_tooltip_at_pointer(
    //             &resp.ctx,
    //            resp.id.with("__tooltip"),
    //             add_contents,
    //         );
    //     }
    //    resp
    // }


    /// Like `on_hover_text`, but show the text next to cursor.
    #[doc(alias = "tooltip")]
    pub fn on_hover_text_at_pointer(resp: &mut Response, text: &str) -> Response {
       resp.clone().on_hover_text_at_pointer(text)
    }

    // /// Show this text if the widget was hovered (i.e. a tooltip).
    // ///
    // /// The text will not be visible if the widget is not enabled.
    // /// For that, use [`resp::on_disabled_hover_text`] instead.
    // ///
    // /// If you call this multiple times the tooltips will stack underneath the previous ones.
    // #[doc(alias = "tooltip")]
    // pub fn on_hover_text(resp: &mut Response, text: impl Into<WidgetText>) -> Response {
    //    resp.on_hover_ui(|ui| {
    //         ui.add(crate::widgets::Label::new(text));
    //     })
    // }

    // /// Show this text when hovering if the widget is disabled.
    // pub fn on_disabled_hover_text(resp: &mut Response, text: impl Into<WidgetText>) -> Response {
    //    resp.on_disabled_hover_ui(|ui| {
    //         ui.add(crate::widgets::Label::new(text));
    //     })
    // }

    /// When hovered, use this icon for the mouse cursor.
    pub fn on_hover_cursor(resp: &mut Response, cursor: CursorIcon) -> Response {
        resp.clone().on_hover_cursor(cursor)
    }

    /// Check for more interactions (e.g. sense clicks on a `Response` returned from a label).
    ///
    /// Note that this call will not add any hover-effects to the widget, so when possible
    /// it is better to give the widget a `Sense` instead, e.g. using [`crate::Label::sense`].
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// let response = ui.label("hello");
    /// assert!(!response.clicked()); // labels don't sense clicks by default
    /// let response = response.interact(egui::Sense::click());
    /// if response.clicked() { /* … */ }
    /// # });
    /// ```
    pub fn interact(resp: &mut Response, sense: Sense) -> Response {
       resp.interact(sense)
    }

    /// Adjust the scroll position until this UI becomes visible.
    ///
    /// If `align` is `None`, it'll scroll enough to bring the UI into view.
    ///
    /// See also: [`Ui::scroll_to_cursor`], [`Ui::scroll_to_rect`].
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// egui::ScrollArea::vertical().show(ui, |ui| {
    ///     for i in 0..1000 {
    ///         let response = ui.button("Scroll to me");
    ///         if response.clicked() {
    ///             response.scroll_to_me(Some(egui::Align::Center));
    ///         }
    ///     }
    /// });
    /// # });
    /// ```
    pub fn scroll_to_me(resp: &mut Response, align: Option<Align>) {
       resp.scroll_to_me(align)
    }

    // /// For accessibility.
    // ///
    // /// Call after interacting and potential calls to [`resp::mark_changed`].
    // pub fn widget_info(resp: &mut Response, make_info: impl Fn() -> crate::WidgetInfo) {
    //     use crate::output::OutputEvent;
    //     let event = if resp.clicked() {
    //         Some(OutputEvent::Clicked(make_info()))
    //     } else if resp.double_clicked() {
    //         Some(OutputEvent::DoubleClicked(make_info()))
    //     } else if resp.gained_focus() {
    //         Some(OutputEvent::FocusGained(make_info()))
    //     } else if resp.changed {
    //         Some(OutputEvent::ValueChanged(make_info()))
    //     } else {
    //         None
    //     };
    //     if let Some(event) = event {
    //        resp.ctx.output().events.push(event);
    //     }
    // }

    /// Response to secondary clicks (right-clicks) by showing the given menu.
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// let response = ui.label("Right-click me!");
    /// response.context_menu(|ui| {
    ///     if ui.button("Close the menu").clicked() {
    ///         ui.close_menu();
    ///     }
    /// });
    /// # });
    /// ```
    ///
    /// See also: [`Ui::menu_button`] and [`Ui::close_menu`].
    pub fn context_menu(rtx: NativeCallContext, resp: &mut Response, cb: FnPtr) -> Response {
        resp.clone().context_menu(|ui| {
            
            let _: () = cb.call_within_context(&rtx, (MyUi::from(ui),)).expect("failed to execute callback code on ui");
        })
    }
}
