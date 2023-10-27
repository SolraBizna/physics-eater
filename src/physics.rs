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

use super::*;

use std::io::{Read, Seek, SeekFrom};

pub mod m1;
pub mod m2;

pub fn is_m1_physics(mut input: impl Read + Seek) -> anyhow::Result<bool> {
    let mut buf = [0; 4];
    input.read_exact(&mut buf)?;
    input.seek(SeekFrom::Current(-4))?;
    Ok(buf == m1::MONSTER_PHYSICS_TAG
        || buf == m1::EFFECT_PHYSICS_TAG
        || buf == m1::PROJECTILE_PHYSICS_TAG
        || buf == m1::PHYSICS_PHYSICS_TAG
        || buf == m1::WEAPON_PHYSICS_TAG)
}

// Neat. The copyright notice was longer than the file.
