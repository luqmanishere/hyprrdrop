use std::cmp::Ordering;

use anyhow::{bail, Context, Result};
use clap::Parser;
use hyprland::{
    data::{Binds, Client, Clients, Workspace, Workspaces},
    dispatch::{Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial},
    keyword::Keyword,
    shared::{HyprData, HyprDataActive, HyprDataActiveOptional, HyprDataVec},
};

use crate::{
    cli::{Cli, Commands, DebugCommand, RegisterCommand},
    utils::{check_if_bound, prepend_workspace_prefix},
};

mod cli;
mod utils;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // TODO: logging for debug purposes
    match cli.command {
        Some(Commands::Register { register_command }) => {
            match register_command {
                RegisterCommand::Command {
                    name: _,
                    command: _,
                } => {
                    // TODO: allow registering a command into a workspace

                    todo!("not implemented yet")
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
                    // check for keybind option, early abort if keybind exists and force not
                    // specified
                    let workspace_name = prepend_workspace_prefix(&name);

                    // INFO: there was an idiot here who forgot to do the register the workspace
                    // without the keybind flag
                    if let Some(keybind) = keybind {
                        if check_if_bound(&keybind)? && !force_keybind {
                            bail!(format!("Key {keybind} is bound. Use -f to override"));
                        } else {
                            register_active(&workspace_name)?;

                            let (mods, key) = keybind.split_once(',').expect("correct pattern");
                            // unbind whatever is set. quite destructive
                            Keyword::set("unbind", format!("{},{}", mods, key))?;
                            Keyword::set(
                                "bind",
                                format!(
                                    "{},{},togglespecialworkspace,{}",
                                    mods, key, workspace_name
                                ),
                            )?;
                            println!(
                                "bound special workspace {workspace_name} to keybind {keybind}"
                            );
                        }
                    } else {
                        register_active(&workspace_name)?;
                    }
                    // TODO: support taking the name from stdin
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
                    .find(|e| e.name.contains(&sp_workspace_name))
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
            let workspace_name = prepend_workspace_prefix(&name);
            match Workspaces::get()?
                .into_iter()
                .find(|s| s.name.contains(&workspace_name))
            {
                Some(_) => {
                    Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(workspace_name)))?;
                }
                None => {
                    eprintln!("No workspace with name {workspace_name} found.");
                    std::process::exit(1)
                }
            }
        }

        Some(Commands::Debug { command }) => match command {
            // TODO: prettier print
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
            Some(DebugCommand::Binds) => {
                let binds = Binds::get()?;
                println!("{binds:#?}");
            }
            Some(DebugCommand::ExecActive { command: _ }) => {
                todo!();
            }
        },
        None => {}
    }

    Ok(())
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

/// Register the currently active client to the given special workspace name
fn register_active(workspace_name: &str) -> Result<()> {
    // get the active client
    let client = Client::get_active()?.expect("an active window exists");

    Dispatch::call(DispatchType::MoveToWorkspaceSilent(
        WorkspaceIdentifierWithSpecial::Special(Some(workspace_name)),
        Some(WindowIdentifier::Address(client.address)),
    ))?;
    println!(
        "moved client {} to workspace {workspace_name}",
        client.title
    );
    Ok(())
}
