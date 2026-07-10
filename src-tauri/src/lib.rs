use tauri::{async_runtime::Mutex, Manager};

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

                app.manage(Mutex::new(db::DatabaseState {
                    pool: database.pool,
                    path: database.path,
                }));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::init_seed,
            ipc::run_seed,
            ipc::get_ancestry,
            ipc::get_full_ancestry,
            ipc::get_table,
            ipc::get_option_block,
            ipc::get_tradition,
            ipc::get_spell,
            ipc::get_magic_talent,
        ])
        .build(tauri::generate_context!())
        .expect("Unexpected error while running tauri application")
        .run_return(|_app, _ev| {
            // TODO: The WWAppData struct is stored via async mutex, so I can't safely acquire a lock
            // in this synchronous callback. I'll need to put the import_dir in a separately stored
            // sync mutex in order to safely drop it.
            // match ev {
            //     // The tmp import dir is deleted on drop, so this ensures this happens on app close.
            //     tauri::RunEvent::Exit => {
            //         if let Ok(data) = get_app_data_state(&app) {
            //             data.lock().await.drop_import();
            //         }
            //     }
            //     _ => {}
            // }
        });
}
