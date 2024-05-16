use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "A utility to make Hyprland dropdowns.",
    subcommand_required = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // TODO: implement this when you actually need it lol you deffo do not need any autostart like
    // this yet
    //
    // #[command(about = "Initialize with a config file", arg_required_else_help = true)]
    // Init { config: PathBuf },
    #[command(
        about = "Register windows to a special workspace",
        arg_required_else_help = true
    )]
    Register {
        #[command(subcommand)]
        register_command: RegisterCommand,
    },
    #[command(about = "Remove special workspaces", arg_required_else_help = true)]
    Unregister { name: String },
    #[command(
        about = "Toggle a managed special workspace",
        arg_required_else_help = true
    )]
    Toggle { name: String },

    #[command(about = "Debug printouts")]
    Debug {
        #[command(subcommand)]
        command: Option<DebugCommand>,
    },
}

#[derive(Subcommand)]
pub enum RegisterCommand {
    #[command(arg_required_else_help = true)]
    Command {
        #[arg(help = "Name of the special workspace.")]
        name: String,
        #[arg(
            help = "Command to execute. Any windows spawned will be moved into a special workspace"
        )]
        command: String,
    },
    #[command(
        about = "Move all windows with the specified class into a special workspace.",
        arg_required_else_help = true
    )]
    Class {
        #[arg(help = "Name of the special workspace")]
        name: String,
        #[arg(help = "Class of the windows to be moved")]
        class: String,
    },
    #[command(
        about = "Move the active window into a special workspace. For ad hoc workspaces, specify the keybind argument in Hyprland format to attach a keybind to the workspace"
    )]
    Active {
        #[arg(help = "Name of the special workspace.")]
        name: String,
        #[arg(
            short,
            long,
            help = "Keybind to dynamically assign to Hyprland in the format '<MOD>,<KEY>'. Used for adhoc workspaces"
        )]
        keybind: Option<String>,
        #[arg(help = "Override existing keybind", short = 'f', long = "force")]
        force_keybind: bool,
    },
}

#[derive(Subcommand)]
pub enum DebugCommand {
    #[command(about = "List all clients and workspaces")]
    All,
    #[command(about = "List all workspaces")]
    Workspaces,
    #[command(about = "List all clients")]
    Clients,
    #[command(about = "Get the active client")]
    ActiveClient,
    #[command(about = "List all binds")]
    Binds,
    ExecActive {
        command: String,
    },
}
