mod imp;
mod keybind;

use glib::Object;
use gtk::{glib, prelude::*, subclass::prelude::*};
use keybind::Keybind;

glib::wrapper! {
    pub struct Shortcut(ObjectSubclass<imp::Shortcut>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Shortcut {
    pub fn new() -> Self {
        let shortcut: Self = Object::builder().build();
        shortcut.imp().trigger.append(&Keybind::new());
        shortcut.imp().output.append(&Keybind::new());
        shortcut
    }
}
