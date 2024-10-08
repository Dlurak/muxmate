pub mod directory;
pub mod list;
pub mod project;
pub mod template;

use clap::{Parser, Subcommand};

/// A CLI for tmux session management
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize the config
    ///
    /// This command will initialize your config directories.
    Init,
    /// Manage directories in the context of moxide and tmux
    ///
    /// This command provides functionalities to interact with tmux sessions based on directories.
    #[command(alias = "dir", alias = "dirs", alias = "directories")]
    Directory(directory::DirectoryCli),
    /// Manage templates in the context of moxide and tmux
    ///
    /// This command provides functionalities to interact with tmux sessions based on templates
    #[command(alias = "temp", alias = "templ")]
    Template(template::TemplateCli),
    /// Manage projects in the context of moxide and tmux
    ///
    /// This command provides functionalities to interact with tmux sessions based on projects
    #[command(alias = "proj", alias = "projects")]
    Project(project::ProjectCli),
    /// List all moxide directories, templates and projecets
    #[command(alias = "ls")]
    List(list::ListCli),
}
