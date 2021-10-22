use std::string;

use crate::{Event, FieldEntity};
use crate::background::BgEvent;
use crate::widgets::{StringField, DropdownField};
use crate::utils::*;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;


pub struct App {
    // Grid that organizes all the fields into the GUI
    pub container: gtk::Grid,
    //Do I need this thing to manage fields, or not?
    pub fields: SlotMap<FieldEntity, StringField>,
    pub dfields: SlotMap<FieldEntity, DropdownField>,
    // my own attempt set up the expt fields (can these go in pub fields somehow?)
    pub experiment_class: String,
    pub experiment_rig: String,
    pub fish_id: String,
    pub fish_genotype: String,
    pub fish_dpf: i32,
    pub chamber_id: String,
    pub imaging: bool,
    pub hardware_test: bool,
    pub addedby: String,
    pub protocol_file: String,

    // need these events
    pub tx: Sender<Event>,
    pub btx: Sender<BgEvent>,
}


impl App {
    pub fn new(app: &gtk::Application, tx: Sender<Event>, btx: Sender<BgEvent>) -> Self {
        // set up the app and fill in default values. sort of a beast. not doing anything complicate though
        let container = cascade! {
            gtk::Grid::new();
            ..set_column_spacing(10);
            ..set_row_spacing(4);
            ..set_border_width(4);
            ..show();
        };

        let _window = cascade! {
            gtk::ApplicationWindow::new(app);
            ..set_title("dasherQueue");
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

        gtk::Window::set_default_icon_name("dasherQueue");
       
        // set up default values incase query returns None
        let mut app = Self {
            container,
            fields: SlotMap::with_key(),
            dfields: SlotMap::with_key(),
            experiment_class: String::from("proline"),
            experiment_rig: String::from("RoLi-11"),
            fish_id: String::from("expert-parakeet"),
            fish_genotype: String::from("Tg(elavl3:H2B-GCaMP6s+/+)"),
            fish_dpf: 8i32,
            chamber_id: String::from("my_chamber"),
            imaging: true,
            hardware_test: false,
            addedby: String::from("charlie"),
            protocol_file: String::from("Load protocol file..."),
            tx, 
            btx,
        };
        println!("chamber id before query: {}", app.chamber_id);
        app.fill_defaults(); // db query to fill defaults based on db
        println!("chamber id after query: {}", app.chamber_id);

        //hijack in the insert_row function for making fields
        //app.insert_row(0);
        app.insert_str_field(0,String::from("fish_id"), String::from(&app.fish_id));
        let expt_ops = get_field_options(String::from("experiment_class"));
        app.insert_dropdown_field(1, String::from("experiment_class"), &expt_ops);
        
        app
    }

    fn insert_str_field(&mut self, row: i32, fieldname: String, value: String) -> FieldEntity {
        // Increment the row value of each Task is below the new row
        for field in self.fields.values_mut() {
            if field.row >= row {
                field.row += 1;
            }
        }

        self.container.insert_row(row);
        let field = StringField::new(row, fieldname, value);

        self.container.attach(&field.label, 0, row, 1, 1);
        self.container.attach(&field.entry, 1, row, 1, 1);

        field.entry.grab_focus();

        let entity = self.fields.insert(field);
        self.fields[entity].connect(self.tx.clone(), entity);

        return entity;
    }

    fn insert_dropdown_field(&mut self, row: i32, fieldname: String, value_options: &Vec<String>) -> FieldEntity {
        // Increment the row value of each Task is below the new row
        for field in self.fields.values_mut() {
            if field.row >= row {
                field.row += 1;
            }
        }
        for field in self.dfields.values_mut() {
            if field.row >= row {
                field.row += 1;
            }
        }

        self.container.insert_row(row);
        let field = DropdownField::new(row, fieldname);

        self.container.attach(&field.label, 0, row, 1, 1);
        self.container.attach(&field.options, 1, row, 1, 1);

        for option in value_options {
            field.options.append(Some(&option), &option);
        };
        //field.options.selection();
        let entity = self.dfields.insert(field);
        self.dfields[entity].connect(self.tx.clone(), entity);

        // dummy test
        // let mut stupid = gtk::Entry::new();
        // stupid.set_text("Some text");
        // stupid.get_buffer();

        return entity;
    }

    fn fill_defaults(&mut self) {
        // first, find the rig name to get default values
        // let local_ip = local_ip::get().unwrap(); 
        let local_ip = "10.48.10.11"; // hardcode for demo
        let vec: Vec<&str> = local_ip.split(".").collect();
        let rig = format!("RoLi-{}", vec[vec.len()-1]);
        // get user
        let userid = users::get_current_username().unwrap();
        println!("{:?} is running this code on rig {}", userid, rig);

        let res = get_last_data_entry(userid, rig);
        // update values of app (self) with most recent sql entry
        if let None = res {
            println!("No data was found matching this user / rig. Leaving all db fields as defaults")
        } else {
            let r = res.unwrap();
            self.experiment_class = r.experiment_class.clone();
            self.experiment_rig = r.experiment_rig.clone();
            self.fish_id = r.fish_id.clone();
            self.chamber_id = r.chamber_id.clone();
            self.imaging = r.imaging;
            self.hardware_test = r.hardware_test;
            self.addedby = r.addedby.clone();
        }
    }

    pub fn modified(&mut self) {
        let tx = self.tx.clone();
    }

    pub async fn closed(&mut self) {
        let _ = self.btx.send(BgEvent::Quit).await;
    }
}