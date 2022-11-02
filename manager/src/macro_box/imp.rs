use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/net/dworv/snarvei/macro.ui")]
pub struct Macro {
    #[template_child]
    pub macro_settings: TemplateChild<gtk::Box>,
    #[template_child]
    pub macro_trigger: TemplateChild<gtk::Box>,
    #[template_child]
    pub macro_action: TemplateChild<gtk::Box>,
    #[template_child]
    pub macro_name: TemplateChild<gtk::Text>,
    #[template_child]
    pub macro_shortcut_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub macro_shortcut_record: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Macro {
    const NAME: &'static str = "MacroBox";
    type Type = super::Macro;
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
impl Macro {
    // #[template_callback]
    // fn handle_new_collection(&self, button: &Button) {
    //     println!("WWWWWW")
    // }
}

impl ObjectImpl for Macro {}

impl WidgetImpl for Macro {}

impl BoxImpl for Macro {}
