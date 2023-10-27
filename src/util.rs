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

use std::io::Read;

pub fn read16(mut input: impl Read) -> anyhow::Result<u16> {
    let mut buf = [0; 2];
    input.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

pub fn read32(mut input: impl Read) -> anyhow::Result<u32> {
    let mut buf = [0; 4];
    input.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

pub fn read_fx_16_16(input: impl Read) -> anyhow::Result<f32> {
    Ok(read32(input)? as i32 as f32 / 65536.0)
}

pub fn read_fx_6_10(input: impl Read) -> anyhow::Result<f32> {
    Ok(read16(input)? as i16 as f32 / 1024.0)
}

pub use read_fx_6_10 as read_world_distance;
pub use read_fx_6_10 as read_world_speed;
pub use read_fx_6_10 as read_world_accel;

pub fn read_optional_fx_6_10(input: impl Read) -> anyhow::Result<Option<f32>> {
    read_optional_16(input).map(|x| x.map(|x| x as i16 as f32 / 1024.0))
}

pub fn read_angle(input: impl Read) -> anyhow::Result<f32> {
    Ok(read16(input)? as i16 as f32 * 360.0 / 512.0)
}

pub fn read_optional_16(input: impl Read) -> anyhow::Result<Option<u16>> {
    let ret = read16(input)?;
    if ret & 0x8000 != 0 {
        Ok(None)
    } else {
        Ok(Some(ret))
    }
}

pub fn read_optional_32(input: impl Read) -> anyhow::Result<Option<u32>> {
    let ret = read32(input)?;
    if ret & 0x8000 != 0 {
        Ok(None)
    } else {
        Ok(Some(ret))
    }
}

pub fn read_generic_bitfield32(input: impl Read) -> anyhow::Result<Vec<u32>> {
    let ret = read32(input)?;
    Ok((0..32).filter(|x| ret & (1 << x) != 0).collect())
}

macro_rules! extract_flags {
    ($flags:ident, $flagbit:ident, $nextflag:ident, $($restflags:ident),+) => {
        extract_flags!($flags, $flagbit, $nextflag);
        extract_flags!($flags, $flagbit, $($restflags),*);
    };
    ($flags:ident, $flagbit:ident, $nextflag:ident) => {
        let $nextflag = $flags & $flagbit != 0;
        $flagbit <<= 1;
    };
}

macro_rules! decode_flags {
    ($input:expr => $Flags:ident { $($flagname:ident),+ $(,)? }) => {
        { #[allow(unused)] {
            let flags = $input;
            let mut flagbit = 1;
            extract_flags!(flags, flagbit, $($flagname),+);
            $Flags {
                $($flagname),+
            }
        }}
    };
}
