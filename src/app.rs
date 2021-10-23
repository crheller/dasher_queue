use std::string;

use crate::{Event, FieldEntity};
use crate::background::BgEvent;
use crate::widgets::{DropdownField, CheckboxField, FileBrowserField};
use crate::utils::*;

use async_channel::Sender;
use glib::clone;
use glib::SourceId;
use gtk::prelude::*;
use slotmap::SlotMap;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use gtk::ResponseType::{Accept, Cancel};


pub struct App {
    // Grid that organizes all the fields into the GUI
    pub container: gtk::Grid,
    pub window: gtk::ApplicationWindow,
    //Do I need this thing to manage fields, or not?
    pub fields: SlotMap<FieldEntity, FileBrowserField>,
    pub dfields: SlotMap<FieldEntity, DropdownField>,
    pub cfields: SlotMap<FieldEntity, CheckboxField>,
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
            window: _window,
            fields: SlotMap::with_key(),
            dfields: SlotMap::with_key(),
            cfields: SlotMap::with_key(),
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
        //app.insert_str_field(0,String::from("fish_id"), String::from(&app.fish_id));

        // fill drop downs
        let user_ops = get_field_options("username".to_string(), "users".to_string());
        app.insert_dropdown_field(0, "username".to_string(), &user_ops, &app.addedby.to_string());
        
        let rig_ops = get_field_options("experiment_rig".to_string(), "data".to_string());
        app.insert_dropdown_field(1, "experiment_rig".to_string(), &rig_ops, &app.experiment_rig.to_string());

        let expt_ops = get_field_options("experiment_class".to_string(), "data".to_string());
        app.insert_dropdown_field(2, String::from("experiment_class"), &expt_ops, &app.experiment_class.to_string());

        let chamber_ops = get_field_options("chamber_id".to_string(), "chambers".to_string());
        app.insert_dropdown_field(3, "chamber_id".to_string(), &chamber_ops, &app.chamber_id.to_string());
        
        let fish_ops = get_field_options("fish_id".to_string(), "data".to_string());
        app.insert_dropdown_field(4, "fish_id".to_string(), &fish_ops, &"new".to_string());

        let geno_ops = get_field_options("genotype".to_string(), "fish".to_string());
        app.insert_dropdown_field(5, "fish_genotype".to_string(), &geno_ops, &app.fish_genotype.to_string());

        let dpf_ops = get_field_options("dpf".to_string(), "fish".to_string());
        app.insert_dropdown_field(6, "fish_dpf".to_string(), &dpf_ops, &app.fish_dpf.to_string());

        // checkbox for imaging
        app.insert_checkbox(7, "imaging".to_string(), app.imaging.clone());

        // checkbox for hardware test
        app.insert_checkbox(8, "hardware_test".to_string(), app.hardware_test.clone());

        // browse for protocol file
        app.insert_filebrowser(9);

        app
    }

    fn insert_dropdown_field(&mut self, row: i32, fieldname: String, value_options: &Vec<String>, default: &String) -> FieldEntity {
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
        let field = DropdownField::new(row, fieldname, default);

        self.container.attach(&field.label, 0, row, 1, 1);
        self.container.attach(&field.options, 1, row, 1, 1);

        for option in value_options {
            field.options.append(Some(&option), &option);
        };
        //field.options.set_text("Arial");
        let entity = self.dfields.insert(field);
        self.dfields[entity].connect(self.tx.clone(), entity);

        // dummy test
        // let mut stupid = gtk::Entry::new();
        // stupid.set_text("Some text");
        // stupid.get_buffer();

        return entity;
    }

    fn insert_checkbox(&mut self, row: i32, fieldname: String, default: bool) -> FieldEntity {
        self.container.insert_row(row);
        let field = CheckboxField::new(row, fieldname, default);

        self.container.attach(&field.label, 0, row, 1, 1);
        self.container.attach(&field.checkbox, 1, row, 1, 1);

        let entity = self.cfields.insert(field);
        //self.cfields[entity].connect(self.tx.clone(), entity);

        return entity;
    }

    fn insert_filebrowser(&mut self, row: i32) -> FieldEntity {
        self.container.insert_row(row);
        let field = FileBrowserField::new(row);

        self.container.attach(&field.button, 0, row, 1, 1);
        self.container.attach(&field.file_selection, 1, row, 1, 1);

        let entity = self.fields.insert(field);
        self.fields[entity].connect(self.tx.clone(), entity);

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

    pub fn open_file_browser(&mut self, entity: FieldEntity) {
        let tx = self.tx.clone();
        let mut file = None;
        let dialog = FileChooserDialog::new(Some("Select file"), 
        Some(&self.window), FileChooserAction::Open);
        dialog.add_button("Cancel", Cancel);
        dialog.add_button("Accept", Accept);
        let result = dialog.run();
        if result == Accept {
            file = dialog.filename();
        }
        let mut field = self.fields.get(entity);
        let fstring = file.unwrap().into_os_string().into_string().unwrap();
        field.unwrap().file_selection.set_text(&fstring);
        //file.unwrap();
        unsafe {
            dialog.destroy();
        }
        
    }

    pub async fn closed(&mut self) {
        let _ = self.btx.send(BgEvent::Quit).await;
    }
}
