use crate::{utils::spawn, Event, FieldEntity};
use async_channel::Sender;
use glib::{clone, SignalHandlerId};
use gtk::prelude::*;
use gtk::*;

pub struct DropdownField {
    pub options: gtk::ComboBoxText,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
    pub label: gtk::Label,
}

impl DropdownField {
    pub fn new(row: i32, fieldname: String, default: &String) -> Self {
        Self {
            options: cascade! {
                gtk::ComboBoxText::with_entry(); // with entry too
                ..insert_text(0, default); // is new is selected
                ..set_hexpand(true);
                ..set_has_default(true);
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

pub struct CheckboxField {
    pub checkbox: gtk::CheckButton,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
    pub label: gtk::Label,
}

impl CheckboxField {
    pub fn new(row: i32, fieldname: String, default: bool) -> Self {
        Self {
            checkbox: cascade! {
                gtk::CheckButton::new();
                ..set_active(default);
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

    // pub fn connect(&mut self, tx: Sender<Event>, entity: FieldEntity) {
    //     let signal = self.checkbox.connect_changed(clone!(@strong tx => move |_| {
    //         let tx = tx.clone();
    //         spawn(async move {
    //             let _ = tx.send(Event::EntryUpdate).await;
    //         });
    //     }));
    //     self.entry_signal = Some(signal);
    // }
}

pub struct FileBrowserField {
    pub file_selection: gtk::Entry,
    pub entry_signal: Option<SignalHandlerId>,
    // "index" of the field
    pub row: i32,
    pub button: gtk::Button,
}

impl FileBrowserField {
    pub fn new(row: i32) -> Self {
        Self {
            file_selection: cascade! {
                gtk::Entry::new();
                ..set_hexpand(true);
                ..show();
            },
            entry_signal: None,
            row,
            button: cascade! {
                gtk::Button::with_label("Choose protocol file");
                ..set_hexpand(true);
                ..show();
            },
        }
    }

    pub fn connect(&mut self, tx: Sender<Event>, entity: FieldEntity) {
        let signal = self.button.connect_clicked(clone!(@strong tx => move |_| {
            let tx = tx.clone();
            spawn(async move {
                let _ = tx.send(Event::LoadFile(entity)).await;
            });
        }));
        self.entry_signal = Some(signal);
    }
}