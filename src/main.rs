use init::init;
use install::{install, install_compiler, install_yflib};
use local::{check_for_local_bin, check_for_local_lib, create_local_bin, create_local_lib};
use log::debug;
use repository::update_repository_cache;
use structopt::StructOpt;
use uninstall::uninstall;
use upgrade::upgrade;

pub mod common;
pub mod display;
pub mod error;
pub mod init;
pub mod install;
pub mod local;
pub mod package;
pub mod repository;
pub mod spinner;
pub mod uninstall;
pub mod upgrade;

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

    /// Upgrade compiler
    #[structopt(name = "upgrade-compiler")]
    /// yfin install-compiler
    UpgradeCompiler {},

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

        #[structopt(short, long)]
        lib: bool,
    },

    #[structopt(name = "update")]
    Update {},
}

fn main() {
    env_logger::init();

    if !check_for_local_lib() {
        create_local_lib();
    }

    if !check_for_local_bin() {
        create_local_bin();
    }

    match Command::from_args() {
        Command::Install { url } => install(&url).unwrap(),
        Command::InstallCompiler {} => install_compiler(false),
        Command::UpgradeCompiler {} => install_compiler(true),
        Command::InstallYflib {} => install_yflib(),
        Command::Uninstall { package } => uninstall(&package).unwrap(),
        Command::Upgrade { package } => upgrade(&package).unwrap(),
        Command::Init { name, lib } => init(name, lib).unwrap(),
        Command::Update {} => update_repository_cache().unwrap(),
    }
}
