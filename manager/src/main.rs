mod window;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, gio};
use window::Window;

const APP_ID: &str = "net.dworv.snarvei";

fn main() {
    // Register and include resources
    gio::resources_register_include!("snarvei.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}