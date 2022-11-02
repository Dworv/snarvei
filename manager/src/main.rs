mod macro_box;
mod window;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

const APP_ID: &str = "net.dworv.snarvei";

fn main() {
    // Register and include resources
    gio::resources_register_include!("snarvei.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let mut window = Window::new(app);
    window.add_macro(&macro_box::Macro::new());
    window.present();
}
