use glib::Object;
use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
    pub struct Keybind(ObjectSubclass<imp::Keybind>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

mod imp {
    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/dworv/snarvei/keybind.ui")]
    pub struct Keybind {
        #[template_child]
        pub keybind_enter: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub control: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub shift: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub alt: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub meta: TemplateChild<gtk::CheckButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Keybind {
        const NAME: &'static str = "Keybind";
        type Type = super::Keybind;
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
    impl Keybind {
        // #[template_callback]
        // fn handle_new_collection(&self, button: &Button) {
        //     println!("WWWWWW")
        // }
    }

    impl ObjectImpl for Keybind {}
    impl WidgetImpl for Keybind {}
    impl BoxImpl for Keybind {}
}

impl Keybind {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
