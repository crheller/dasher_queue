#[macro_use]
extern crate cascade;

mod app;
mod background;
mod widgets;
mod utils;
mod dbsettings;

use self::app::App;
use gio::prelude::*;

/// The name that we will register to the system to identify our application
pub const APP_ID: &str = "info.heller.charles";

// Create a key type to identify the keys that we'll use for the Task SlotMap.
// I am not sure I will need this...?
slotmap::new_key_type! {
    pub struct FieldEntity;
}

pub enum Event {
    // Update the information stored in this "entry" so that 
    // we can send it to the DB
    EntryUpdate(FieldEntity),

    ToggleUpdate(FieldEntity),

    LoadFile(FieldEntity),

    Save,

    Closed,

    Quit
}

fn main() {
    let app_name = "dasherQueue";

    glib::set_program_name(Some(app_name));
    glib::set_application_name(app_name);

    // Initializes GTK and registers our application. gtk::Application helps us
    // set up an application with less work
    let app = gtk::Application::new(
        Some(APP_ID),
        Default::default()
    ); //.expect("failed to init application");

    // After the application has been registered, it will trigger an activate
    // signal, which will give us the okay to construct our application and set
    // up our application logic. We're going to use `app` to create the
    // application window in the future.
    app.connect_activate(|app| {
        // Channel for UI events in the main thread (click events etc.)
        let (tx, rx) = async_channel::unbounded();
        // Channel for background events to the background thread (SQL queries, queuing dasher, writing to db)
        let (btx, brx) = async_channel::unbounded();

        // Take ownership of a copy of the UI event sender (tx),
        // and the background event receiver (brx).
        std::thread::spawn(glib::clone!(@strong tx => move || {
            // Fetch the executor registered for this thread
            utils::thread_context()
                // Block this thread on an event loop future
                .block_on(background::run(tx, brx));
        }));
        
        let mut app = App::new(app, tx, btx);

        let event_handler = async move {
            while let Ok(event) = rx.recv().await {
                match event {
                    Event::EntryUpdate(entity) => app.modified(entity),
                    Event::ToggleUpdate(entity) => app.toggle(entity),
                    Event::LoadFile(entity) => app.open_file_browser(entity),
                    Event::Save => app.save_to_db(),
                    Event::Closed => app.closed().await,
                    Event::Quit => gtk::main_quit(),
                }
            } 
        };

        utils::spawn(event_handler);
    });
    
    // This last step performs the same duty as gtk::main()
    //app.run(&[]);
    app.run();
}
