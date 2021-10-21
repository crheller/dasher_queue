use crate::Event;
use async_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};
use crate::app::App;


pub enum BgEvent {
    // Look up all fields from sql table to fill them
    FillFields,

    // Fill fish field with the name of a new fish. Need to query db for this
    NewFish,

    // Signals that we should collect up the information from all
    // fields and send to database
    //SendToDB,

    // Signals that the window has been closed, so we should save hdf5 file and
    // start up dasher
    Closed,

    // Signals that the process has saved to disk and it is safe to exit
    Quit,

}

pub async fn run(tx: Sender<Event>, rx: Receiver<BgEvent>) {

    while let Ok(event) = rx.recv().await {
        match event {
            // what is supposed to happen in here?? How to I actually edit my app
            // with one of these functions?
            BgEvent::FillFields => {
                println!("Here")
            },
            BgEvent::NewFish => {
                println!("Here")
            },
            BgEvent::Closed => {
                println!("Here")
            },
            BgEvent::Quit => break
        }
    }

    let _ = tx.send(Event::Quit).await;
}

//BgEvent::SendToDB => app.sync_to_db().await,