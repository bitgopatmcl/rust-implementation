mod configuration_file;
mod io;
mod lerna_manifest;
mod link;
mod make_depend;
mod opts;
mod package_manifest;
mod pin;
mod query;
mod typescript_config;

use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = opts::Opts::parse();

    match opts.subcommand {
        opts::ClapSubCommand::Link(args) => link::link_typescript_project_references(args),
        opts::ClapSubCommand::Pin(args) => pin::pin_version_numbers_in_internal_packages(args),
        opts::ClapSubCommand::MakeDepend(args) => make_depend::make_dependency_makefile(args),
        opts::ClapSubCommand::Query(args) => query::handle_subcommand(args),
    }
}
