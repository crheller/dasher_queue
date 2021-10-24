use crate::Event;
use async_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};
use crate::app::App;


pub enum BgEvent {
    // Signals that we should collect up the information from all
    // fields and send to database
    //SendToDB,

    // Signals that the process has saved to disk and it is safe to exit
    Quit,

}

pub async fn run(tx: Sender<Event>, rx: Receiver<BgEvent>) {

    while let Ok(event) = rx.recv().await {
        match event {
            // what is supposed to happen in here?? How to I actually edit my app
            // with one of these functions?
            // safe to return to the main UI and quit application
            BgEvent::Quit => break
        }
    }

    let _ = tx.send(Event::Quit).await;
}

//BgEvent::SendToDB => app.sync_to_db().await,