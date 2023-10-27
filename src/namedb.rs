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

// rustc didn't want me to name this file `リムル.rs`. :(

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Context;

#[derive(Clone, Default)]
pub struct NameDb {
    names: Vec<Option<String>>,
}

impl NameDb {
    pub fn new(base_path: &Path, my_name: &str) -> anyhow::Result<NameDb> {
        let target_path = base_path.join(my_name);
        let f = match File::open(&target_path) {
            Ok(f) => f,
            Err(x) if x.kind() == std::io::ErrorKind::NotFound => {
                return Ok(NameDb::default())
            }
            Err(x) => {
                return Err(x).with_context(|| {
                    format!("unable to open {:?}", target_path)
                })?
            }
        };
        let f = BufReader::new(f);
        let names = f
            .lines()
            .map(|line| {
                let line = line?;
                let line = line.trim();
                if line.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(line.to_string()))
                }
            })
            .collect::<anyhow::Result<Vec<Option<String>>>>()?;
        Ok(NameDb { names })
    }
    pub fn identify<T>(&self, index: T) -> serde_json::Value
    where
        usize: TryFrom<T>,
    {
        let Ok(index): Result<usize, _> = index.try_into() else { unreachable!() };
        match self.names.get(index).and_then(Option::as_ref) {
            Some(str) => {
                assert!(!str.is_empty());
                serde_json::Value::String(str.to_string())
            }
            None => serde_json::Value::Number(index.into()),
        }
    }
}

#[derive(Clone)]
pub struct NameDbs {
    pub monster_class_names: NameDb,
    pub monster_names: NameDb,
    pub projectile_names: NameDb,
    pub weapon_names: NameDb,
    pub item_names: NameDb,
    pub effect_names: NameDb,
    pub damage_type_names: NameDb,
    pub collection_names: NameDb,
    pub sound_names: NameDb,
    pub weapon_class_names: NameDb,
}

impl NameDbs {
    pub fn new(namedb_path: Option<&Path>) -> anyhow::Result<NameDbs> {
        match namedb_path {
            None => Ok(NameDbs::default()),
            Some(namedb_path) => Ok(NameDbs {
                monster_class_names: NameDb::new(
                    namedb_path,
                    "monster_class_names.txt",
                )?,
                monster_names: NameDb::new(namedb_path, "monster_names.txt")?,
                projectile_names: NameDb::new(
                    namedb_path,
                    "projectile_names.txt",
                )?,
                weapon_names: NameDb::new(namedb_path, "weapon_names.txt")?,
                item_names: NameDb::new(namedb_path, "item_names.txt")?,
                effect_names: NameDb::new(namedb_path, "effect_names.txt")?,
                damage_type_names: NameDb::new(
                    namedb_path,
                    "damage_type_names.txt",
                )?,
                collection_names: NameDb::new(
                    namedb_path,
                    "collection_names.txt",
                )?,
                sound_names: NameDb::new(namedb_path, "sound_names.txt")?,
                ..Default::default()
            }),
        }
    }
}

impl Default for NameDbs {
    fn default() -> Self {
        Self {
            monster_class_names: Default::default(),
            monster_names: Default::default(),
            projectile_names: Default::default(),
            weapon_names: Default::default(),
            item_names: Default::default(),
            effect_names: Default::default(),
            damage_type_names: Default::default(),
            collection_names: Default::default(),
            sound_names: Default::default(),
            weapon_class_names: NameDb {
                names: vec![
                    Some("melee".to_string()),
                    Some("normal".to_string()),
                    Some("dual function".to_string()),
                    Some("dual wield".to_string()),
                    Some("multipurpose".to_string()),
                ],
            },
        }
    }
}
