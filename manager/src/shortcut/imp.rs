use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/net/dworv/snarvei/shortcut.ui")]
pub struct Shortcut {
    #[template_child]
    pub trigger: TemplateChild<gtk::Box>,
    #[template_child]
    pub output: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for Shortcut {
    const NAME: &'static str = "Shortcut";
    type Type = super::Shortcut;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Shortcut {
    // #[template_callback]
    // fn handle_new_collection(&self, button: &Button) {
    //     println!("WWWWWW")
    // }
}

impl ObjectImpl for Shortcut {}

impl WidgetImpl for Shortcut {}

impl BoxImpl for Shortcut {}
