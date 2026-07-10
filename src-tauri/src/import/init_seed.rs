use std::{fs::File, path::PathBuf};

use crate::{
    import::ImportData,
    ipc::{emit, EmitChannel},
    store::get_app_data_state,
    WWResult,
};

use super::{extract_rows, ImportEvent};

use tauri::{AppHandle, Wry};

pub async fn initialize_seed_import(app: &AppHandle<Wry>, filepath: String) -> WWResult<()> {
    let file = File::open(filepath)?;
    let state = get_app_data_state(&app)?;
    let mut state = state.lock().await;
    let tmp_dir_path = state.add_import_dir()?;
    let mut zipfile = zip::ZipArchive::new(file)?;
    zipfile.extract(&tmp_dir_path).unwrap();
    println!(
        "|> Extracted zip contents to tmp directory: {:?}",
        tmp_dir_path
    );
    println!("---------------------------------");

    let import_data = ImportData {
        ancestries: extract_rows(join(&tmp_dir_path, "ancestries.csv"))?,
        languages: extract_rows(join(&tmp_dir_path, "languages.csv"))?,
        speed_traits: extract_rows(join(&tmp_dir_path, "speed_traits.csv"))?,
        senses: extract_rows(join(&tmp_dir_path, "senses.csv"))?,
        profession_categories: extract_rows(join(&tmp_dir_path, "profession_categories.csv"))?,
        professions: extract_rows(join(&tmp_dir_path, "professions.csv"))?,
        traditions: extract_rows(join(&tmp_dir_path, "magic_traditions.csv"))?,
        magic_talents: extract_rows(join(&tmp_dir_path, "magic_talents.csv"))?,
        magic_spells: extract_rows(join(&tmp_dir_path, "magic_spells.csv"))?,
        novice_paths: extract_rows(join(&tmp_dir_path, "novice_paths.csv"))?,
        novice_levels: extract_rows(join(&tmp_dir_path, "novice_levels.csv"))?,
        expert_paths: extract_rows(join(&tmp_dir_path, "expert_paths.csv"))?,
        expert_levels: extract_rows(join(&tmp_dir_path, "expert_levels.csv"))?,
        master_paths: extract_rows(join(&tmp_dir_path, "master_paths.csv"))?,
        master_levels: extract_rows(join(&tmp_dir_path, "master_levels.csv"))?,
        path_talents: extract_rows(join(&tmp_dir_path, "path_talents.csv"))?,
        options: extract_rows(join(&tmp_dir_path, "options.csv"))?,
        tables: extract_rows(join(&tmp_dir_path, "tables.csv"))?,
    };
    let import_summary = import_data.summary();
    state.import_data = Some(import_data);

    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Ready(import_summary)).into(),
    )?;

    Ok(())
}

fn join(path: &PathBuf, filename: &str) -> PathBuf {
    let mut next_path = path.clone();
    next_path.push(filename);
    next_path
}
