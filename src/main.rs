use std::cmp::Ordering;

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use hyprland::{
    data::{Binds, Client, Clients, Workspace, Workspaces},
    dispatch::{Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial},
    keyword::Keyword,
    shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprDataVec},
};

use crate::cli::{Cli, Commands, DebugCommand, RegisterCommand};

/// all workspace names are prefixed by this
const WORKSPACE_PREFIX: &str = "hyprrdrop";

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Register { register_command }) => {
            match register_command {
                RegisterCommand::Command { name, command } => {
                    // TODO: allow registering a command into a workspace
                    let workspace_name = prepend_workspace_prefix(&name);

                    todo!()
                    // Dispatch::call(DispatchType::Exec(command.as_str()))?;
                }
                RegisterCommand::Class { name, class } => {
                    println!("got command register, with name: {name}, class: {class:?}");

                    let workspace_name = prepend_workspace_prefix(&name);

                    let clients = Clients::get()?
                        .to_vec()
                        .into_iter()
                        .filter(|e| e.class == class)
                        .collect::<Vec<_>>();

                    match clients.len().cmp(&1) {
                        Ordering::Equal | Ordering::Greater => {
                            // move these clients to the special workspace
                            for client in clients {
                                Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                                    WorkspaceIdentifierWithSpecial::Special(Some(&workspace_name)),
                                    Some(WindowIdentifier::Address(client.address)),
                                ))?;
                                println!("moved {} to workspace {}", client.title, workspace_name);
                            }
                        }
                        Ordering::Less => {
                            bail!("no clients with class {class} found");
                        }
                    }
                }
                RegisterCommand::Active {
                    name,
                    keybind,
                    force_keybind,
                } => {
                    let workspace_name = prepend_workspace_prefix(&name);
                    // get the active client
                    let client = Client::get_active()?.expect("an active window exists");

                    Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                        WorkspaceIdentifierWithSpecial::Special(Some(&workspace_name)),
                        Some(WindowIdentifier::Address(client.address)),
                    ))?;
                    println!(
                        "moved client {} to workspace {workspace_name}",
                        client.title
                    );

                    if let Some(keybind) = keybind {
                        let (mods, key) = keybind.split_once(',').expect("correct pattern");
                        // unbind whatever is set. quite destructive
                        Keyword::set("unbind", format!("{},{}", mods, key))?;
                        Keyword::set(
                            "bind",
                            format!("{},{},togglespecialworkspace,{}", mods, key, workspace_name),
                        )?;
                        println!("bound special workspace {workspace_name} to keybind {keybind}");
                        // TODO: how to check keybinds
                        todo!();
                        check_bind(keybind)?;
                    }
                }
            }
        }

        Some(Commands::Unregister { name }) => {
            println!("got command register, with name: {name}");
            let sp_workspace_name = prepend_workspace_prefix(name.as_str());
            let active_workspace = Workspace::get_active()?;
            if !active_workspace.name.contains("special") {
                // move the window to current workspace if not special
                let sp_workspace = Workspaces::get()?
                    .into_iter()
                    .find(|e| e.name == sp_workspace_name)
                    .context(format!(
                        "finding special workplace with given name {sp_workspace_name}"
                    ))?;
                let sp_windows = Clients::get()?
                    .into_iter()
                    .filter(|e| {
                        e.workspace.name == sp_workspace.name || e.workspace.id == sp_workspace.id
                    })
                    .collect::<Vec<_>>();

                // there must be a window if the workspace exists
                for window in sp_windows {
                    Dispatch::call(DispatchType::MoveToWorkspace(
                        WorkspaceIdentifierWithSpecial::Id(active_workspace.id),
                        Some(WindowIdentifier::Address(window.address)),
                    ))?;
                }
                // workspace gets auto destroyed unless a persist option is set i think
            } else {
                // TODO: how tf to find a non special workspace ensuring the last order
                eprintln!("in active workspace, noop");
            }
        }

        Some(Commands::Toggle { name }) => {
            println!("got command toggle, with name: {name}");

            Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(
                prepend_workspace_prefix(name.as_str()),
            )))?;
        }

        Some(Commands::Debug { command }) => match command {
            Some(DebugCommand::All) | None => {
                list_all_workspaces()?;
                list_all_clients()?;
            }
            Some(DebugCommand::Clients) => {
                list_all_clients()?;
            }
            Some(DebugCommand::Workspaces) => {
                list_all_workspaces()?;
            }
            Some(DebugCommand::ActiveClient) => {
                let active = Client::get_active()?;
                println!("{active:#?}");
            }
            Some(DebugCommand::ExecActive { command: _ }) => {
                todo!();
            }
        },
        None => {}
    }

    Ok(())
}

fn check_bind(keybind: String) -> Result<bool> {
    let binds = Binds::get()?;
    todo!()
}

fn list_all_workspaces() -> Result<()> {
    let workspaces = Workspaces::get()?.to_vec();

    println!("{workspaces:#?}");
    Ok(())
}

fn list_all_clients() -> Result<()> {
    let clients = Clients::get()?.to_vec();
    println!("{clients:#?}");
    Ok(())
}

fn prepend_workspace_prefix(name: &str) -> String {
    format!("{WORKSPACE_PREFIX}-{name}")
}
