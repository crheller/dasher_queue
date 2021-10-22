use crate::{utils::spawn, Event, FieldEntity};
use async_channel::Sender;
use glib::{clone, SignalHandlerId};
use gtk::prelude::*;
use gtk::*;

// Do I need a new struct for each "Field"?
pub struct StringField {
    pub entry: gtk::Entry,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
    pub label: gtk::Label,
}

impl StringField {
    pub fn new(row: i32, fieldname: String, value: String) -> Self {
        Self {
            entry: cascade! {
                gtk::Entry::new();
                ..set_text(&value);
                ..set_hexpand(true);
                ..show();
            },
            entry_signal: None,
            row,
            label: cascade! {
                gtk::Label::new(Some(&fieldname));
                ..set_text(&fieldname);
                ..set_hexpand(true);
                ..show();
            },
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

pub struct DropdownField {
    pub options: gtk::ComboBoxText,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
    pub label: gtk::Label,
}

impl DropdownField {
    pub fn new(row: i32, fieldname: String) -> Self {
        Self {
            options: cascade! {
                gtk::ComboBoxText::with_entry(); // with entry allows typing too
                ..insert_text(0, "test");
                ..set_hexpand(true);
                ..set_has_default(false);
                ..show();
            },
            entry_signal: None,
            row,
            label: cascade! {
                gtk::Label::new(Some(&fieldname));
                ..set_text(&fieldname);
                ..set_hexpand(true);
                ..show();
            },
        }
    }

    pub fn connect(&mut self, tx: Sender<Event>, entity: FieldEntity) {
        let signal = self.options.connect_changed(clone!(@strong tx => move |_| {
            let tx = tx.clone();
            spawn(async move {
                let _ = tx.send(Event::EntryUpdate).await;
            });
        }));
        self.entry_signal = Some(signal);
    }
    
    pub fn set_text(&mut self, text: &str) {
        let signal = self.entry_signal.as_ref().unwrap();
        self.options.block_signal(signal);
        //self.options.set_text(text);
        self.options.unblock_signal(signal);
    }
}