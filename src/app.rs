use crate::{Event, FieldEntity};
use crate::background::BgEvent;
use crate::widgets::Field;
use crate::utils::spawn;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;

pub struct App {
    pub container: gtk::Grid,
    pub fields: SlotMap<FieldEntity, Field>,
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
}

impl App {
    pub fn new(app: &gtk::Application, tx: Sender<Event>, btx: Sender<BgEvent>) -> Self {
        let container = cascade! {
            gtk::Grid::new();
            ..set_column_spacing(4);
            ..set_row_spacing(4);
            ..set_border_width(4);
            ..show();
        };

        let _window = cascade! {
            gtk::ApplicationWindow::new(app);
            ..set_title("Todo");
            ..set_default_size(400, 600);
            ..add(&container);
            ..connect_delete_event(clone!(@strong tx, @strong container => move |win, _| {
                // Detach to preserve widgets after destruction of window
                win.remove(&container);
        
                let tx = tx.clone();
                spawn(async move {
                    let _ = tx.send(Event::Closed).await;
                });
                gtk::Inhibit(false)
            }));
            ..show_all();
        };

        gtk::Window::set_default_icon_name("icon-name-here");

        let mut app = Self {
            container,
            fields: SlotMap::with_key(),
            tx,
            btx,
        };

        app.insert_row(0);

        app
    }

    fn insert_row(&mut self, row: i32) -> FieldEntity {
        // Increment the row value of each Task is below the new row
        for field in self.fields.values_mut() {
            if field.row >= row {
                field.row += 1;
            }
        }

        self.container.insert_row(row);
        let field = Field::new(row);

        self.container.attach(&field.entry, 0, row, 1, 1);

        field.entry.grab_focus();

        let entity = self.fields.insert(field);
        self.fields[entity].connect(self.tx.clone(), entity);
        return entity;
    }

    pub fn modified(&mut self) {
        let tx = self.tx.clone();
    }

    pub async fn closed(&mut self) {
        let _ = self.btx.send(BgEvent::Quit).await;
    }
}