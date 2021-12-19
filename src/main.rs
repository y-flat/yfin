use install::{install, install_compiler, install_stdlib};
use uninstall::{uninstall};
use upgrade::{upgrade};
use structopt::StructOpt;

pub mod install;
pub mod package;
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

    /// Install stdlib
    #[structopt(name = "install-stdlib")]
    /// yfin install-stdlib
    InstallStdlib {},

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
}

fn main() {
    match Command::from_args() {
        Command::Install { url } => install(&url),
        Command::InstallCompiler {} => install_compiler(),
        Command::InstallStdlib {} => install_stdlib(),
        Command::Uninstall { package } => uninstall(&package),
        Command::Upgrade { package } => upgrade(&package),
    }
}
