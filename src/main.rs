use clap::{Arg, Command};

fn main() {
    let cmd = clap::Command::new("bvm")
        .bin_name("bvm")
        .about("Bun Version Manager - Install and manage Bun versions")
        .styles(CLAP_STYLING)
        .subcommand_required(true)
        .subcommand(
            Command::new("install")
                .about("Install a specific version of Bun")
                .arg(
                    Arg::new("version")
                        .help("Version to install (e.g. 1.0.0)")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("use")
                .about("Switch to a specific Bun version")
                .arg(
                    Arg::new("version")
                        .help("Version to use (e.g. 1.0.0)")
                        .required(true),
                ),
        )
        .subcommand(Command::new("list").about("List installed Bun versions"))
        .subcommand(Command::new("current").about("Show current active Bun version"))
        .subcommand(
            Command::new("uninstall")
                .about("Uninstall a specific version of Bun")
                .arg(
                    Arg::new("version")
                        .help("Version to uninstall (e.g. 1.0.0)")
                        .required(true),
                ),
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let version = sub_matches.get_one::<String>("version").unwrap();
            println!("Installing Bun version {}", version);
        }
        Some(("use", sub_matches)) => {
            let version = sub_matches.get_one::<String>("version").unwrap();
            println!("Switching to Bun version {}", version);
        }
        Some(("list", _)) => {
            println!("Listing installed Bun versions...");
        }
        Some(("current", _)) => {
            println!("Current Bun version:");
        }
        Some(("uninstall", sub_matches)) => {
            let version = sub_matches.get_one::<String>("version").unwrap();
            println!("Uninstalling Bun version {}", version);
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}

// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
