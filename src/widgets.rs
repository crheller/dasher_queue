use crate::{utils::spawn, Event, FieldEntity};
use async_channel::Sender;
use glib::{clone, SignalHandlerId};
use gtk::prelude::*;

// Do I need a new struct for each "Field"?
pub struct Field {
    pub entry: gtk::Entry,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
}

impl Field {
    pub fn new(row: i32) -> Self {
        Self {
            entry: cascade! {
                gtk::Entry::new();
                ..set_hexpand(true);
                ..show();
            },
            entry_signal: None,
            row,
        }
    }

    pub fn connect(&mut self, tx: Sender<Event>, entity: FieldEntity) {
        let signal = self.entry.connect_changed(clone!(@strong tx => move |_| {
            let tx = tx.clone();
            spawn(async move {
                let _ = tx.send(Event::EntryUpdate).await;
            });
        }));
        self.entry_signal = Some(signal);
    }
    
    pub fn set_text(&mut self, text: &str) {
        let signal = self.entry_signal.as_ref().unwrap();
        self.entry.block_signal(signal);
        self.entry.set_text(text);
        self.entry.unblock_signal(signal);
    }

}
