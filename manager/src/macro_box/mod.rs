mod imp;

use glib::Object;
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct Macro(ObjectSubclass<imp::Macro>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Macro {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
