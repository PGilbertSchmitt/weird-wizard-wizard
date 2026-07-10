use std::collections::HashSet;

use tauri::{AppHandle, Wry};

use crate::{
    db,
    import::{pipe_separate, ProgressPayload},
    ipc::{emit, EmitChannel},
    store::{get_app_data_state, get_database},
    WWError, WWResult,
};

use super::ImportEvent;

pub async fn run_seed_import(app: &AppHandle<Wry>) -> WWResult<()> {
    let import_state = get_app_data_state(&app)?;
    let import_state = import_state.lock().await;

    let db_state = get_database(&app)?;
    let db_state = db_state.lock().await;

    if let Some(import_data) = &import_state.import_data {
        let summary = import_data.summary();
        let total_record_count = summary.total_records();
        let mut processed_records = 0;
        let pool = &db_state.pool;
        let mut tx = pool.begin().await?;

        let language_map =
            db::languages::Language::insert_all(&mut tx, &import_data.languages).await?;
        processed_records += summary.languages;
        emit_progress(&app, processed_records, total_record_count)?;

        let speed_trait_map =
            db::speed_traits::SpeedTrait::insert_all(&mut tx, &import_data.speed_traits).await?;
        processed_records += summary.speed_traits;
        emit_progress(&app, processed_records, total_record_count)?;

        let sense_map = db::senses::Sense::insert_all(&mut tx, &import_data.senses).await?;
        processed_records += summary.senses;
        emit_progress(&app, processed_records, total_record_count)?;

        let immunities: HashSet<String> = import_data
            .ancestries
            .iter()
            .flat_map(|anc| pipe_separate(&anc.immunities))
            .collect();
        let immunity_map = db::immunities::Immunity::insert_all(&mut tx, &immunities).await?;

        let ancestry_map = db::ancestries::Ancestry::insert_all(
            &mut tx,
            &import_data.ancestries,
            &language_map,
            &speed_trait_map,
            &sense_map,
            &immunity_map,
        )
        .await?;
        processed_records += summary.ancestries;
        emit_progress(&app, processed_records, total_record_count)?;

        db::professions::Profession::insert_all(&mut tx, &import_data.professions).await?;
        processed_records += summary.professions;
        emit_progress(&app, processed_records, total_record_count)?;

        db::professions::ProfessionCategory::insert_all(
            &mut tx,
            &import_data.profession_categories,
        )
        .await?;
        processed_records += summary.profession_categories;
        emit_progress(&app, processed_records, total_record_count)?;

        let options_map =
            db::option_blocks::OptionBlock::insert_all(&mut tx, &import_data.options).await?;
        processed_records += summary.options;
        emit_progress(&app, processed_records, total_record_count)?;

        let table_map =
            db::info_tables::InfoTable::insert_all(&mut tx, &import_data.tables).await?;
        processed_records += summary.tables;
        emit_progress(&app, processed_records, total_record_count)?;

        let trad_map =
            db::traditions::Tradition::insert_all(&mut tx, &import_data.traditions, &table_map)
                .await?;
        processed_records += summary.traditions;
        emit_progress(&app, processed_records, total_record_count)?;

        db::spells::Spell::insert_all(
            &mut tx,
            &import_data.magic_spells,
            &trad_map,
            &table_map,
            &options_map,
        )
        .await?;
        processed_records += summary.magic_spells;
        emit_progress(&app, processed_records, total_record_count)?;

        db::magic_talents::MagicTalent::insert_all(
            &mut tx,
            &import_data.magic_talents,
            &trad_map,
            &table_map,
            &options_map,
        )
        .await?;
        processed_records += summary.magic_talents;
        emit_progress(&app, processed_records, total_record_count)?;

        let novice_path_map =
            db::paths::Path::insert_all_novice(&mut tx, &import_data.novice_paths, &ancestry_map)
                .await?;
        processed_records += summary.novice_paths;
        emit_progress(&app, processed_records, total_record_count)?;

        let expert_path_map = db::paths::Path::insert_all_expert_or_master(
            &mut tx,
            &import_data.expert_paths,
            db::etc::PathKind::Expert,
        )
        .await?;
        processed_records += summary.expert_paths;
        emit_progress(&app, processed_records, total_record_count)?;

        let master_path_map = db::paths::Path::insert_all_expert_or_master(
            &mut tx,
            &import_data.master_paths,
            db::etc::PathKind::Master,
        )
        .await?;
        processed_records += summary.master_paths;
        emit_progress(&app, processed_records, total_record_count)?;

        db::levels::Level::insert_all(
            &mut tx,
            &import_data.novice_levels,
            &novice_path_map,
            &trad_map,
            &language_map,
            &speed_trait_map,
        )
        .await?;
        processed_records += summary.novice_levels;
        emit_progress(&app, processed_records, total_record_count)?;

        db::levels::Level::insert_all(
            &mut tx,
            &import_data.expert_levels,
            &expert_path_map,
            &trad_map,
            &language_map,
            &speed_trait_map,
        )
        .await?;
        processed_records += summary.expert_levels;
        emit_progress(&app, processed_records, total_record_count)?;

        db::levels::Level::insert_all(
            &mut tx,
            &import_data.master_levels,
            &master_path_map,
            &trad_map,
            &language_map,
            &speed_trait_map,
        )
        .await?;
        processed_records += summary.master_levels;
        emit_progress(&app, processed_records, total_record_count)?;

        tx.commit().await?;

        emit(&app, EmitChannel::IMPORT, &Ok(ImportEvent::Done).into())?;

        Ok(())
    } else {
        Err(WWError::Generic("No".to_owned()))
    }
}

fn emit_progress(
    app: &AppHandle<Wry>,
    processed_records: usize,
    total_record_count: usize,
) -> WWResult<()> {
    emit(
        &app,
        EmitChannel::IMPORT,
        &Ok(ImportEvent::Progress(ProgressPayload(
            processed_records,
            total_record_count,
        )))
        .into(),
    )?;
    Ok(())
}
