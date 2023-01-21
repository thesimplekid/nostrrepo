#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod comms;
mod db;
mod errors;
mod globals;
mod issues;
mod nostr;
mod overlord;
mod people;
mod repositories;
mod ui;

use comms::ToOverlordMessage;
use errors::Error;
use globals::GLOBALS;

use std::ops::DerefMut;
use std::{env, mem, thread};

//#[tokio::main]
// When compiling natively:
fn main() -> Result<(), Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // We create and enter the runtime on the main thread so that
    // non-async code can have a runtime context within which to spawn
    // async tasks.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _main_rt = rt.enter(); // <-- this allows it.

    // We run our main async code on a separate thread, not just a
    // separate task. This leave the main thread for UI work only.
    // egui is most portable when it is on the main thread.
    let async_thread = thread::spawn(move || {
        if let Err(e) = rt.block_on(tokio_main()) {
            tracing::error!("{}", e);
        }
    });

    if let Err(e) = ui::run() {
        tracing::error!("{}", e);
    }

    // Wait for the async thread to complete
    async_thread.join().unwrap();
    Ok(())
}

async fn tokio_main() -> Result<(), Error> {
    // Set up portan globally
    // Set up database
    db::setup_database().await?;
    nostr::setup_nostr().await?;

    // Steal `tmp_overlord_receiver` from the GLOBALS, and give it to a new Overlord
    let overlord_receiver = {
        let mut mutex_option = GLOBALS.tmp_overlord_receiver.lock().await;
        mem::replace(mutex_option.deref_mut(), None)
    }
    .unwrap();

    // Run the overlord
    let mut overlord = crate::overlord::Overlord::new(overlord_receiver);
    overlord.run().await;
    Ok(())
}

// Any task can call this to shutdown
pub fn initiate_shutdown() -> Result<(), Error> {
    let to_overlord = GLOBALS.to_overlord.clone();
    let _ = to_overlord.send(ToOverlordMessage::Shutdown); // ignore errors
    Ok(())
}
