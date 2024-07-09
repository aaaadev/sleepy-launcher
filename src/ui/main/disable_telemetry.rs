use std::path::PathBuf;
use std::process::Command;

use relm4::prelude::*;

use privilege::runas::Command as RootCommand;

use crate::*;

use super::{App, AppMsg};

pub fn disable_telemetry(sender: ComponentSender<App>) {
    sender.input(AppMsg::DisableButtons(true));

    let config = Config::get().unwrap();

    std::thread::spawn(move || {
        let telemetry = config
            .launcher
            .edition
            .telemetry_servers()
            .iter()
            .map(|server| format!("echo '0.0.0.0 {server}' >> /etc/hosts"))
            .collect::<Vec<String>>()
            .join(" ; ");

        match RootCommand::new("zsh")
            .arg("-c")
            .arg(format!(
                "echo '' >> /etc/hosts ; {telemetry} ; echo '' >> /etc/hosts"
            ))
            .gui(true)
            .run()
        {
            Ok(status) => {
                if !status.success() {
                    tracing::error!("Failed to update /etc/hosts file");

                    sender.input(AppMsg::Toast {
                        title: tr!("telemetry-servers-disabling-error"),
                        description: None, // stdout/err is empty
                    });
                }
            }

            Err(err) => {
                tracing::error!("Failed to update /etc/hosts file");

                sender.input(AppMsg::Toast {
                    title: tr!("telemetry-servers-disabling-error"),
                    description: Some(err.to_string()),
                });
            }
        }

        sender.input(AppMsg::DisableButtons(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed: false,
            show_status_page: true,
        });
    });
}
