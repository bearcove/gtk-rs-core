// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::CellAreaContext;
use crate::CellEditable;
use crate::CellLayout;
use crate::CellRenderer;
use crate::CellRendererState;
use crate::DirectionType;
use crate::Orientation;
use crate::SizeRequestMode;
use crate::TreeIter;
use crate::TreeModel;
use crate::TreePath;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct CellArea(Object<ffi::GtkCellArea, ffi::GtkCellAreaClass>) @implements Buildable, CellLayout;

    match fn {
        type_ => || ffi::gtk_cell_area_get_type(),
    }
}

pub const NONE_CELL_AREA: Option<&CellArea> = None;

pub trait CellAreaExt: 'static {
    #[doc(alias = "gtk_cell_area_activate")]
    fn activate<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool;

    #[doc(alias = "gtk_cell_area_activate_cell")]
    fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>>(
        &self,
        widget: &P,
        renderer: &Q,
        event: &gdk::Event,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    #[doc(alias = "gtk_cell_area_add")]
    fn add<P: IsA<CellRenderer>>(&self, renderer: &P);

    #[doc(alias = "gtk_cell_area_add_focus_sibling")]
    fn add_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    );

    //#[doc(alias = "gtk_cell_area_add_with_properties")]
    //fn add_with_properties<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_cell_area_apply_attributes")]
    fn apply_attributes<P: IsA<TreeModel>>(
        &self,
        tree_model: &P,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    );

    #[doc(alias = "gtk_cell_area_attribute_connect")]
    fn attribute_connect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str, column: i32);

    #[doc(alias = "gtk_cell_area_attribute_disconnect")]
    fn attribute_disconnect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str);

    #[doc(alias = "gtk_cell_area_attribute_get_column")]
    fn attribute_get_column<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) -> i32;

    //#[doc(alias = "gtk_cell_area_cell_get")]
    //fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_cell_area_cell_get_property")]
    fn cell_get_property<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
    ) -> glib::Value;

    //#[doc(alias = "gtk_cell_area_cell_get_valist")]
    //fn cell_get_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //#[doc(alias = "gtk_cell_area_cell_set")]
    //fn cell_set<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_cell_area_cell_set_property")]
    fn cell_set_property<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
        value: &glib::Value,
    );

    //#[doc(alias = "gtk_cell_area_cell_set_valist")]
    //fn cell_set_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "gtk_cell_area_copy_context")]
    fn copy_context<P: IsA<CellAreaContext>>(&self, context: &P) -> Option<CellAreaContext>;

    #[doc(alias = "gtk_cell_area_create_context")]
    fn create_context(&self) -> Option<CellAreaContext>;

    #[doc(alias = "gtk_cell_area_event")]
    fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        event: &gdk::Event,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> i32;

    #[doc(alias = "gtk_cell_area_focus")]
    fn focus(&self, direction: DirectionType) -> bool;

    #[doc(alias = "gtk_cell_area_foreach")]
    fn foreach<P: FnMut(&CellRenderer) -> bool>(&self, callback: P);

    #[doc(alias = "gtk_cell_area_foreach_alloc")]
    fn foreach_alloc<
        P: IsA<CellAreaContext>,
        Q: IsA<Widget>,
        R: FnMut(&CellRenderer, &gdk::Rectangle, &gdk::Rectangle) -> bool,
    >(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        background_area: &gdk::Rectangle,
        callback: R,
    );

    #[doc(alias = "gtk_cell_area_get_cell_allocation")]
    #[doc(alias = "get_cell_allocation")]
    fn cell_allocation<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<CellRenderer>>(
        &self,
        context: &P,
        widget: &Q,
        renderer: &R,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle;

    #[doc(alias = "gtk_cell_area_get_cell_at_position")]
    #[doc(alias = "get_cell_at_position")]
    fn cell_at_position<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        x: i32,
        y: i32,
    ) -> (CellRenderer, gdk::Rectangle);

    #[doc(alias = "gtk_cell_area_get_current_path_string")]
    #[doc(alias = "get_current_path_string")]
    fn current_path_string(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_cell_area_get_edit_widget")]
    #[doc(alias = "get_edit_widget")]
    fn edit_widget(&self) -> Option<CellEditable>;

    #[doc(alias = "gtk_cell_area_get_edited_cell")]
    #[doc(alias = "get_edited_cell")]
    fn edited_cell(&self) -> Option<CellRenderer>;

    #[doc(alias = "gtk_cell_area_get_focus_cell")]
    #[doc(alias = "get_focus_cell")]
    fn focus_cell(&self) -> Option<CellRenderer>;

    #[doc(alias = "gtk_cell_area_get_focus_from_sibling")]
    #[doc(alias = "get_focus_from_sibling")]
    fn focus_from_sibling<P: IsA<CellRenderer>>(&self, renderer: &P) -> Option<CellRenderer>;

    #[doc(alias = "gtk_cell_area_get_focus_siblings")]
    #[doc(alias = "get_focus_siblings")]
    fn focus_siblings<P: IsA<CellRenderer>>(&self, renderer: &P) -> Vec<CellRenderer>;

    #[doc(alias = "gtk_cell_area_get_preferred_height")]
    #[doc(alias = "get_preferred_height")]
    fn preferred_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
    ) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_get_preferred_height_for_width")]
    #[doc(alias = "get_preferred_height_for_width")]
    fn preferred_height_for_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        width: i32,
    ) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_get_preferred_width")]
    #[doc(alias = "get_preferred_width")]
    fn preferred_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
    ) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_get_preferred_width_for_height")]
    #[doc(alias = "get_preferred_width_for_height")]
    fn preferred_width_for_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        height: i32,
    ) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_get_request_mode")]
    #[doc(alias = "get_request_mode")]
    fn request_mode(&self) -> SizeRequestMode;

    #[doc(alias = "gtk_cell_area_has_renderer")]
    fn has_renderer<P: IsA<CellRenderer>>(&self, renderer: &P) -> bool;

    #[doc(alias = "gtk_cell_area_inner_cell_area")]
    fn inner_cell_area<P: IsA<Widget>>(
        &self,
        widget: &P,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle;

    #[doc(alias = "gtk_cell_area_is_activatable")]
    fn is_activatable(&self) -> bool;

    #[doc(alias = "gtk_cell_area_is_focus_sibling")]
    fn is_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    ) -> bool;

    #[doc(alias = "gtk_cell_area_remove")]
    fn remove<P: IsA<CellRenderer>>(&self, renderer: &P);

    #[doc(alias = "gtk_cell_area_remove_focus_sibling")]
    fn remove_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    );

    #[doc(alias = "gtk_cell_area_render")]
    fn render<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cr: &cairo::Context,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    );

    #[doc(alias = "gtk_cell_area_request_renderer")]
    fn request_renderer<P: IsA<CellRenderer>, Q: IsA<Widget>>(
        &self,
        renderer: &P,
        orientation: Orientation,
        widget: &Q,
        for_size: i32,
    ) -> (i32, i32);

    #[doc(alias = "gtk_cell_area_set_focus_cell")]
    fn set_focus_cell<P: IsA<CellRenderer>>(&self, renderer: &P);

    #[doc(alias = "gtk_cell_area_stop_editing")]
    fn stop_editing(&self, canceled: bool);

    #[doc(alias = "add-editable")]
    fn connect_add_editable<
        F: Fn(&Self, &CellRenderer, &CellEditable, &gdk::Rectangle, TreePath) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "apply-attributes")]
    fn connect_apply_attributes<F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "focus-changed")]
    fn connect_focus_changed<F: Fn(&Self, &CellRenderer, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "remove-editable")]
    fn connect_remove_editable<F: Fn(&Self, &CellRenderer, &CellEditable) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "edit-widget")]
    fn connect_edit_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "edited-cell")]
    fn connect_edited_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "focus-cell")]
    fn connect_focus_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellArea>> CellAreaExt for O {
    fn activate<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        edit_only: bool,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
                edit_only.into_glib(),
            ))
        }
    }

    fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>>(
        &self,
        widget: &P,
        renderer: &Q,
        event: &gdk::Event,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate_cell(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
                cell_area.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn add<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_add(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    ) {
        unsafe {
            ffi::gtk_cell_area_add_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            );
        }
    }

    //fn add_with_properties<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_cell_area_add_with_properties() }
    //}

    fn apply_attributes<P: IsA<TreeModel>>(
        &self,
        tree_model: &P,
        iter: &TreeIter,
        is_expander: bool,
        is_expanded: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_apply_attributes(
                self.as_ref().to_glib_none().0,
                tree_model.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
                is_expander.into_glib(),
                is_expanded.into_glib(),
            );
        }
    }

    fn attribute_connect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_area_attribute_connect(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
                column,
            );
        }
    }

    fn attribute_disconnect<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) {
        unsafe {
            ffi::gtk_cell_area_attribute_disconnect(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
            );
        }
    }

    fn attribute_get_column<P: IsA<CellRenderer>>(&self, renderer: &P, attribute: &str) -> i32 {
        unsafe {
            ffi::gtk_cell_area_attribute_get_column(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                attribute.to_glib_none().0,
            )
        }
    }

    //fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_cell_area_cell_get() }
    //}

    fn cell_get_property<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
    ) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_cell_area_cell_get_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    //fn cell_get_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_cell_area_cell_get_valist() }
    //}

    //fn cell_set<P: IsA<CellRenderer>>(&self, renderer: &P, first_prop_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_cell_area_cell_set() }
    //}

    fn cell_set_property<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
        value: &glib::Value,
    ) {
        unsafe {
            ffi::gtk_cell_area_cell_set_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    //fn cell_set_valist<P: IsA<CellRenderer>>(&self, renderer: &P, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_cell_area_cell_set_valist() }
    //}

    fn copy_context<P: IsA<CellAreaContext>>(&self, context: &P) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_copy_context(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    fn create_context(&self) -> Option<CellAreaContext> {
        unsafe {
            from_glib_full(ffi::gtk_cell_area_create_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        event: &gdk::Event,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> i32 {
        unsafe {
            ffi::gtk_cell_area_event(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
                cell_area.to_glib_none().0,
                flags.into_glib(),
            )
        }
    }

    fn focus(&self, direction: DirectionType) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_focus(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
            ))
        }
    }

    fn foreach<P: FnMut(&CellRenderer) -> bool>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&CellRenderer) -> bool>(
            renderer: *mut ffi::GtkCellRenderer,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let renderer = from_glib_borrow(renderer);
            let callback: *mut P = data as *const _ as usize as *mut P;
            let res = (*callback)(&renderer);
            res.into_glib()
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gtk_cell_area_foreach(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn foreach_alloc<
        P: IsA<CellAreaContext>,
        Q: IsA<Widget>,
        R: FnMut(&CellRenderer, &gdk::Rectangle, &gdk::Rectangle) -> bool,
    >(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        background_area: &gdk::Rectangle,
        callback: R,
    ) {
        let callback_data: R = callback;
        unsafe extern "C" fn callback_func<
            P: IsA<CellAreaContext>,
            Q: IsA<Widget>,
            R: FnMut(&CellRenderer, &gdk::Rectangle, &gdk::Rectangle) -> bool,
        >(
            renderer: *mut ffi::GtkCellRenderer,
            cell_area: *const gdk::ffi::GdkRectangle,
            cell_background: *const gdk::ffi::GdkRectangle,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let renderer = from_glib_borrow(renderer);
            let cell_area = from_glib_borrow(cell_area);
            let cell_background = from_glib_borrow(cell_background);
            let callback: *mut R = data as *const _ as usize as *mut R;
            let res = (*callback)(&renderer, &cell_area, &cell_background);
            res.into_glib()
        }
        let callback = Some(callback_func::<P, Q, R> as _);
        let super_callback0: &R = &callback_data;
        unsafe {
            ffi::gtk_cell_area_foreach_alloc(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                background_area.to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn cell_allocation<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: IsA<CellRenderer>>(
        &self,
        context: &P,
        widget: &Q,
        renderer: &R,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let mut allocation = gdk::Rectangle::uninitialized();
            ffi::gtk_cell_area_get_cell_allocation(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                allocation.to_glib_none_mut().0,
            );
            allocation
        }
    }

    fn cell_at_position<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cell_area: &gdk::Rectangle,
        x: i32,
        y: i32,
    ) -> (CellRenderer, gdk::Rectangle) {
        unsafe {
            let mut alloc_area = gdk::Rectangle::uninitialized();
            let ret = from_glib_none(ffi::gtk_cell_area_get_cell_at_position(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                x,
                y,
                alloc_area.to_glib_none_mut().0,
            ));
            (ret, alloc_area)
        }
    }

    fn current_path_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_current_path_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn edit_widget(&self) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edit_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn edited_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_edited_cell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn focus_cell(&self) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_cell(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn focus_from_sibling<P: IsA<CellRenderer>>(&self, renderer: &P) -> Option<CellRenderer> {
        unsafe {
            from_glib_none(ffi::gtk_cell_area_get_focus_from_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    fn focus_siblings<P: IsA<CellRenderer>>(&self, renderer: &P) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_cell_area_get_focus_siblings(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    fn preferred_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_height(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            let minimum_height = minimum_height.assume_init();
            let natural_height = natural_height.assume_init();
            (minimum_height, natural_height)
        }
    }

    fn preferred_height_for_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        width: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_height = mem::MaybeUninit::uninit();
            let mut natural_height = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_height_for_width(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                minimum_height.as_mut_ptr(),
                natural_height.as_mut_ptr(),
            );
            let minimum_height = minimum_height.assume_init();
            let natural_height = natural_height.assume_init();
            (minimum_height, natural_height)
        }
    }

    fn preferred_width<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_width(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            let minimum_width = minimum_width.assume_init();
            let natural_width = natural_width.assume_init();
            (minimum_width, natural_width)
        }
    }

    fn preferred_width_for_height<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        height: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_width = mem::MaybeUninit::uninit();
            let mut natural_width = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_get_preferred_width_for_height(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                height,
                minimum_width.as_mut_ptr(),
                natural_width.as_mut_ptr(),
            );
            let minimum_width = minimum_width.assume_init();
            let natural_width = natural_width.assume_init();
            (minimum_width, natural_width)
        }
    }

    fn request_mode(&self) -> SizeRequestMode {
        unsafe {
            from_glib(ffi::gtk_cell_area_get_request_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_renderer<P: IsA<CellRenderer>>(&self, renderer: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_has_renderer(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            ))
        }
    }

    fn inner_cell_area<P: IsA<Widget>>(
        &self,
        widget: &P,
        cell_area: &gdk::Rectangle,
    ) -> gdk::Rectangle {
        unsafe {
            let mut inner_area = gdk::Rectangle::uninitialized();
            ffi::gtk_cell_area_inner_cell_area(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                inner_area.to_glib_none_mut().0,
            );
            inner_area
        }
    }

    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_is_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_remove(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_focus_sibling<P: IsA<CellRenderer>, Q: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        sibling: &Q,
    ) {
        unsafe {
            ffi::gtk_cell_area_remove_focus_sibling(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            );
        }
    }

    fn render<P: IsA<CellAreaContext>, Q: IsA<Widget>>(
        &self,
        context: &P,
        widget: &Q,
        cr: &cairo::Context,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
        paint_focus: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_render(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.into_glib(),
                paint_focus.into_glib(),
            );
        }
    }

    fn request_renderer<P: IsA<CellRenderer>, Q: IsA<Widget>>(
        &self,
        renderer: &P,
        orientation: Orientation,
        widget: &Q,
        for_size: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            ffi::gtk_cell_area_request_renderer(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                orientation.into_glib(),
                widget.as_ref().to_glib_none().0,
                for_size,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            let minimum_size = minimum_size.assume_init();
            let natural_size = natural_size.assume_init();
            (minimum_size, natural_size)
        }
    }

    fn set_focus_cell<P: IsA<CellRenderer>>(&self, renderer: &P) {
        unsafe {
            ffi::gtk_cell_area_set_focus_cell(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
            );
        }
    }

    fn stop_editing(&self, canceled: bool) {
        unsafe {
            ffi::gtk_cell_area_stop_editing(self.as_ref().to_glib_none().0, canceled.into_glib());
        }
    }

    #[doc(alias = "add-editable")]
    fn connect_add_editable<
        F: Fn(&Self, &CellRenderer, &CellEditable, &gdk::Rectangle, TreePath) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_editable_trampoline<
            P,
            F: Fn(&P, &CellRenderer, &CellEditable, &gdk::Rectangle, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            editable: *mut ffi::GtkCellEditable,
            cell_area: *mut gdk::ffi::GdkRectangle,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                &CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                &from_glib_borrow(editable),
                &from_glib_borrow(cell_area),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    add_editable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "apply-attributes")]
    fn connect_apply_attributes<F: Fn(&Self, &TreeModel, &TreeIter, bool, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn apply_attributes_trampoline<
            P,
            F: Fn(&P, &TreeModel, &TreeIter, bool, bool) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            is_expander: glib::ffi::gboolean,
            is_expanded: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            f(
                &CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(model),
                &from_glib_borrow(iter),
                from_glib(is_expander),
                from_glib(is_expanded),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-changed")]
    fn connect_focus_changed<F: Fn(&Self, &CellRenderer, TreePath) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_changed_trampoline<
            P,
            F: Fn(&P, &CellRenderer, TreePath) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            let path = from_glib_full(crate::ffi::gtk_tree_path_new_from_string(path));
            f(
                &CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                path,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "remove-editable")]
    fn connect_remove_editable<F: Fn(&Self, &CellRenderer, &CellEditable) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn remove_editable_trampoline<
            P,
            F: Fn(&P, &CellRenderer, &CellEditable) + 'static,
        >(
            this: *mut ffi::GtkCellArea,
            renderer: *mut ffi::GtkCellRenderer,
            editable: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            f(
                &CellArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(renderer),
                &from_glib_borrow(editable),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-editable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_editable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "edit-widget")]
    fn connect_edit_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_edit_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            f(&CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::edit-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_edit_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "edited-cell")]
    fn connect_edited_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_edited_cell_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            f(&CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::edited-cell\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_edited_cell_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-cell")]
    fn connect_focus_cell_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_cell_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<CellArea>,
        {
            let f: &F = &*(f as *const F);
            f(&CellArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-cell\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_cell_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellArea")
    }
}
