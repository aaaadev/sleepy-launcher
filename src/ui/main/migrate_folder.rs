use std::path::PathBuf;

use relm4::prelude::*;

use crate::*;

use super::{App, AppMsg};

pub fn migrate_folder(
    sender: ComponentSender<App>,
    from: PathBuf,
    to: PathBuf,
    cleanup_folder: Option<PathBuf>,
) {
    sender.input(AppMsg::DisableButtons(true));

    std::thread::spawn(move || {
        move_files::move_files(&from, &to).expect("Failed to perform migration");

        if let Some(cleanup_folder) = cleanup_folder {
            std::fs::remove_dir_all(cleanup_folder).expect("Failed to remove cleanup folder");
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true,
        });
    });
}
