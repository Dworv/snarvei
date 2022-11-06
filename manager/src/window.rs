#[allow(unused_imports)]
use gtk::{gio, Application, subclass::prelude::*};
use glib::Object;

use crate::shortcut::Shortcut;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

mod imp {
    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Button, CompositeTemplate};

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/dworv/snarvei/window.ui")]
    pub struct Window {
        #[template_child]
        pub new_collection: TemplateChild<gtk::Button>,
        #[template_child]
        pub collections: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub shortcuts: TemplateChild<gtk::ListBox>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "SnarveiWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl Window {
        #[template_callback]
        fn handle_new_collection(&self, button: &Button) {
            println!("WWWWWW")
        }
    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder()
            .property("application", app)
            .build()
    }

    pub fn add_shortcut(&mut self, m: &Shortcut) {
        self.imp().shortcuts.append(m);
    }
}
