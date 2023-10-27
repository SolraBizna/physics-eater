/*
    This file is part of physics-eater, copyright 2023 Solra Bizna.

    physics-eater is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    physics-eater is distributed in the hope that it will be useful, but
    WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along
    with physics-eater. If not, see <https://www.gnu.org/licenses/>.
*/

use std::path::PathBuf;

#[macro_use]
mod util;
use util::*;

mod namedb;
use namedb::*;
mod physics;
use physics::*;
mod wad;
use wad::*;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
#[allow(clippy::large_enum_variant)]
enum Command {
    /// Parse the header and directory of a Marathon 2 WAD, and display
    /// information about it.
    ShowWad {},
    /// Parse some bare M1 chunks (like a Marathon 1 physics file) and display
    /// information about them.
    ShowChunks {},
    /// Convert a Marathon 1 physics file into JSON on stdout.
    ConvertM1Physics {
        /// Path to a directory containing files like "monster_names.txt",
        /// "projectile_names.txt", etc. These files contain one name per line
        /// (with blank lines indicating gaps in the naming).
        #[arg(long)]
        namedb: Option<PathBuf>,
    },
    /// Convert a Marathon 2 physics file into JSON on stdout.
    ConvertM2Physics {
        /// Path to a directory containing files like "monster_names.txt",
        /// "projectile_names.txt", etc. These files contain one name per line
        /// (with blank lines indicating gaps in the naming).
        #[arg(long)]
        namedb: Option<PathBuf>,
    },
}

#[derive(Parser, Debug)]
#[clap(
    author = "Solra Bizna <solra@bizna.name>",
    version,
    about = "A tool for turning Marathon physics files into JSON"
)]
struct Invocation {
    /// The path to the physics model to work on.
    physics_path: PathBuf,
    /// What command to run.
    #[command(subcommand)]
    command: Command,
}

fn inner_main() -> anyhow::Result<()> {
    let Invocation {
        physics_path,
        command,
    } = Invocation::parse();
    match command {
        Command::ShowWad {} => show_wad(physics_path),
        Command::ShowChunks {} => show_chunks(physics_path),
        Command::ConvertM1Physics { namedb } => {
            let namedbs = NameDbs::new(namedb.as_deref())?;
            m1::convert_physics(physics_path, namedbs)
        }
        Command::ConvertM2Physics { namedb } => {
            let namedbs = NameDbs::new(namedb.as_deref())?;
            m2::convert_physics(physics_path, namedbs)
        }
    }
}

fn main() {
    let result = inner_main();
    match result {
        Ok(()) => (),
        Err(x) => {
            eprintln!("\nUnhandled error!\n{x:?}");
            std::process::exit(1)
        }
    }
}
