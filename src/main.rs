#[macro_use]
extern crate cascade;

mod app;
mod background;
mod widgets;
mod utils;

use self::app::App;
use gio::prelude::*;

/// The name that we will register to the system to identify our application
pub const APP_ID: &str = "io.github.mmstick.ToDo";

fn main() {
    let app_name = "Todo";

    glib::set_program_name(Some(app_name));
    glib::set_application_name(app_name);

    // Initializes GTK and registers our application. gtk::Application helps us
    // set up an application with less work
    let app = gtk::Application::new(
        Some(APP_ID),
        Default::default()
    ).expect("failed to init application");

    // After the application has been registered, it will trigger an activate
    // signal, which will give us the okay to construct our application and set
    // up our application logic. We're going to use `app` to create the
    // application window in the future.
    app.connect_activate(|app| {
        let (tx, rx) = async_channel::unbounded();

        let mut app = App::new(app, tx);

        let event_handler = async move {
            while let Ok(event) = rx.recv().await {
                match event {

                }
            }
        };

        utils::spawn(event_handler);
    });

    // This last step performs the same duty as gtk::main()
    app.run(&[]);
}
