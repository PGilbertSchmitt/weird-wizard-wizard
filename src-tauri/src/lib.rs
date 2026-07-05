use std::sync::Mutex;
use tauri::{Manager, async_runtime::Mutex as AsyncMutex};

mod db;
mod import;
mod ipc;
mod result;
mod store;

pub use result::{WWError, WWResult};

use crate::store::WWAppData;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(WWAppData::new()))
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let handle = app.handle();
                let database = db::Database::new(&handle)
                    .await
                    .expect("Failed to open database");

                app.manage(AsyncMutex::new(db::DatabaseState {
                    pool: database.pool,
                    path: database.path,
                }));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ipc::init_seed])
        .build(tauri::generate_context!())
        .expect("Unexpected error while running tauri application")
        .run_return(|app, ev| {
            match ev {
                // The tmp import dir is deleted on drop, so this ensures this happens on app close.
                tauri::RunEvent::Exit => {
                    app.state::<Mutex<WWAppData>>().lock().unwrap().drop_import();
                }
                _ => {}
            }
        });
}
