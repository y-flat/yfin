use init::init;
use install::{install, install_compiler, install_yflib};
use local::{check_for_local, create_local};
use log::debug;
use structopt::StructOpt;
use uninstall::uninstall;
use upgrade::upgrade;

pub mod common;
pub mod error;
pub mod init;
pub mod install;
pub mod local;
pub mod package;
pub mod uninstall;
pub mod upgrade;
pub mod spinner;

#[derive(StructOpt)]
#[structopt(name = "yfin", about = "Y-Flat Installer")]
enum Command {
    /// Install from git repo url
    #[structopt(name = "install")]
    Install {
        /// yfin install <git-repo-url>
        #[structopt()]
        url: String,
    },

    /// Install compiler
    #[structopt(name = "install-compiler")]
    /// yfin install-compiler
    InstallCompiler {},

    /// Install yflib
    #[structopt(name = "install-yflib")]
    /// yfin install-yflib
    InstallYflib {},

    /// Uninstall package
    #[structopt(name = "uninstall")]
    Uninstall {
        /// yfin uninstall <package>
        #[structopt()]
        package: String,
    },

    /// Install newer version of package
    #[structopt(name = "upgrade")]
    Upgrade {
        /// yfin upgrade <package>
        #[structopt()]
        package: String,
    },

    /// Initialize a package
    #[structopt(name = "init")]
    Init {
        /// yfin init <name>
        #[structopt()]
        name: Option<String>,
    },
}

fn main() {
    env_logger::init();

    if !check_for_local() {
        create_local();
    }

    match Command::from_args() {
        Command::Install { url } => install(&url).unwrap(),
        Command::InstallCompiler {} => install_compiler(),
        Command::InstallYflib {} => install_yflib(),
        Command::Uninstall { package } => uninstall(&package).unwrap(),
        Command::Upgrade { package } => upgrade(&package).unwrap(),
        Command::Init { name } => init(name).unwrap(),
    }
}
