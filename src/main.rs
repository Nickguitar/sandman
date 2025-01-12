mod args;
mod toggles;
mod container;

use crate::args::Args;
//use structopt::StructOpt;
use crate::container::Container;
use crate::container::ContainerConfig;
use home;
use std::path::{Path, Component};
//use std::ffi::OsStr;

/// Constants
const SANDMAN_DIR: &str = "sandman";

/// Loads and returns a container based on the TOML configuration
fn load_container(container_name: &String, absolute: bool) -> Container {
    let config_filename: String;
    let container_canonical_name: String;

    if absolute {
        config_filename = container_name.clone();
        //let path = Path::new(&config_filename);
        //let path_components = path.components().collect::<Vec<_>>();
        //let basename = path_components.last().unwrap();
        container_canonical_name = String::from("");
    }
    else {
        config_filename = format!("{}/{}/{}.toml", home::home_dir().unwrap().display(), SANDMAN_DIR, container_name);
        container_canonical_name = format!("sandman/{}", container_name.clone());
    }

    let config_raw = std::fs::read_to_string(&config_filename).unwrap();
    let config: ContainerConfig = toml::from_str(&config_raw).unwrap();

    Container {
        name: container_canonical_name,
        file: config_filename,
        config: config,
    }
}

/// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_args: Vec<String> = std::env::args().collect();
    let args: Args;
    let container: Container;

    let binpath = Path::new(&raw_args[0]);
    let binpath_components = binpath.components().collect::<Vec<_>>();
    let basename = binpath_components.last().unwrap();

    // Is our basename is "sandman" ?
    if basename == &Component::Normal("sandman".as_ref()) {
        // Running normally
        args = Args::cli_args();
        container = load_container(&args.container_name, false);
    }
    else {
        // Running as shebang, construct args as if we were calling "run"
        panic!("Running other than from the sandman binary is not yet implemented!")
        //let mut mock_args = vec![String::from("run")];
        //mock_args.extend(raw_args.clone());
        //args = Args::from_iter(&mock_args);
        //container = load_container(&args.container_name, true);
    }

    if args.verbose {
        dbg!(&binpath);
        dbg!(&raw_args);
        dbg!(&args);
        dbg!(&container);
    }

    if args.action == "run" {
        match container.run() {
            Err(status) => {
                println!("Failed to run container, {}", status);
            },
            _ => {},
        }
    }
    else if args.action == "run_or_exec" {
        panic!("run_or_exec not implemented yet!");
    }
    else if args.action == "build" {
        match container.build() {
            Err(status) => {
                println!("Failed to build container, {}", status);
            },
            _ => {},
        }
    }
    else if args.action == "args" {
        let arguments = container.running_args();
        let joined = arguments.join(" ");
        println!("{}", joined);
    }
    else {
        panic!("Action {} invalid", &args.action);
    };

    Ok(())
}
