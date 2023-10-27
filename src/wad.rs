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

use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::{anyhow, Context};

use super::*;

#[allow(unused)]
const PRE_ENTRY_POINT_WADFILE_VERSION: u16 = 0;
const WADFILE_HAS_DIRECTORY_ENTRY: u16 = 1;
#[allow(unused)]
const WADFILE_SUPPORTS_OVERLAYS: u16 = 2;
#[allow(unused)]
const WADFILE_HAS_INFINITY_STUFF: u16 = 4;
const MAXIMUM_WADFILE_NAME_LENGTH: usize = 64;
const MAXIMUM_DIRECTORY_ENTRIES_PER_FILE: usize = 64;

pub struct Chunk {
    pub kind: [u8; 4],
    pub data: Vec<u8>,
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Chunk")
            .field("kind", &String::from_utf8_lossy(&self.kind))
            .field("bytes.len()", &self.data.len())
            .finish()
    }
}

impl Chunk {
    pub fn read_m2_chunks(
        mut input: impl Read + Seek,
    ) -> anyhow::Result<Vec<Chunk>> {
        let mut chunks = vec![];
        let mut next_offset = 0;
        loop {
            let offset = next_offset;
            if offset != 0 {
                input
                    .seek(SeekFrom::Start(offset as u64))
                    .context("unable to seek to a chunk of the WAD")?;
            }
            let mut kind = [0; 4];
            let Ok(()) = input.read_exact(&mut kind) else { break };
            next_offset = read32(&mut input)
                .context("unable to read a chunk of the WAD")?;
            let length = read32(&mut input)
                .context("unable to read a chunk of the WAD")?;
            let unknown = read32(&mut input)
                .context("unable to read a chunk of the WAD")?;
            if unknown != 0 {
                return Err(anyhow!("chunk #{} {:?}, located at {:08X} within the subfile, has a nonzero value in the unknown-purpose \"offset\" field", chunks.len(), String::from_utf8_lossy(&kind[..]), offset));
            }
            let mut chunk_data = vec![0; length as usize];
            input
                .read_exact(&mut chunk_data)
                .context("unable to read a chunk of the WAD")?;
            chunks.push(Chunk {
                kind,
                data: chunk_data,
            })
        }
        Ok(chunks)
    }
    pub fn read_m1_chunks(mut input: impl Read) -> anyhow::Result<Vec<Chunk>> {
        let mut chunks = vec![];
        loop {
            let mut kind = [0; 4];
            let Ok(()) = input.read_exact(&mut kind) else { break };
            let _ = read32(&mut input).context("unable to read a chunk")?;
            let count =
                read16(&mut input).context("unable to read a chunk")?;
            let size = read16(&mut input).context("unable to read a chunk")?;
            let length = count as usize * size as usize;
            let mut chunk_data = vec![0; length];
            input
                .read_exact(&mut chunk_data)
                .context("unable to read a chunk of a subfile of the WAD")?;
            chunks.push(Chunk {
                kind,
                data: chunk_data,
            })
        }
        Ok(chunks)
    }
    pub fn find(chunks: &[Chunk], kind: [u8; 4]) -> anyhow::Result<&[u8]> {
        for chunk in chunks.iter() {
            if chunk.kind == kind {
                return Ok(&chunk.data);
            }
        }
        Err(anyhow!(
            "Unable to find chunk of type {:?}",
            String::from_utf8_lossy(&kind)
        ))
    }
}

pub struct Wad {
    pub wad_version: u16,
    pub data_version: u16,
    pub file_name: [u8; MAXIMUM_WADFILE_NAME_LENGTH],
    pub checksum: u32,
    pub directory_offset: u32,
    pub wad_count: u16,
    pub application_specific_directory_data_size: u16,
    pub entry_header_size: u16,
    pub directory_entry_base_size: u16,
    pub parent_checksum: u32,
    pub files: Vec<Vec<Chunk>>,
}

impl Debug for Wad {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Wad")
            .field("wad_version", &self.wad_version)
            .field("data_version", &self.data_version)
            .field(
                "file_name",
                &String::from_utf8_lossy(
                    &self.file_name[..self
                        .file_name
                        .iter()
                        .position(|x| *x == 0)
                        .unwrap_or(MAXIMUM_WADFILE_NAME_LENGTH)],
                ),
            )
            .field("checksum", &self.checksum)
            .field("directory_offset", &self.directory_offset)
            .field("wad_count", &self.wad_count)
            .field(
                "application_specific_directory_data_size",
                &self.application_specific_directory_data_size,
            )
            .field("entry_header_size", &self.entry_header_size)
            .field(
                "directory_entry_base_size",
                &self.directory_entry_base_size,
            )
            .field("parent_checksum", &self.parent_checksum)
            .field("files", &self.files)
            .finish()
    }
}

impl Wad {
    pub fn read_wad(mut input: impl Read + Seek) -> anyhow::Result<Wad> {
        if is_m1_physics(&mut input)? {
            return Err(anyhow!(
                "this is a Marathon 1 physics file, not a WAD!"
            ));
        }
        let wad_version = read16(&mut input)?;
        let data_version = read16(&mut input)?;
        let mut file_name = [0; MAXIMUM_WADFILE_NAME_LENGTH];
        input.read_exact(&mut file_name)?;
        let checksum = read32(&mut input)?;
        let directory_offset = read32(&mut input)?;
        let wad_count = read16(&mut input)?;
        let application_specific_directory_data_size = read16(&mut input)?;
        let entry_header_size = read16(&mut input)?;
        let directory_entry_base_size = read16(&mut input)?;
        let parent_checksum = read32(&mut input)?;
        let directory_entry_base_size =
            if wad_version <= WADFILE_HAS_DIRECTORY_ENTRY {
                8
            } else {
                directory_entry_base_size
            };
        let unit_size = application_specific_directory_data_size as usize
            + directory_entry_base_size as usize;
        let mut files = vec![];
        for i in 0..MAXIMUM_DIRECTORY_ENTRIES_PER_FILE {
            let offset = directory_offset as u64 + unit_size as u64 * i as u64;
            input
                .seek(SeekFrom::Start(offset))
                .context("unable to seek to directory entry in WAD")?;
            let Ok(offset) = read32(&mut input) else { break };
            let length = read32(&mut input)?;
            input
                .seek(SeekFrom::Start(offset as u64))
                .context("unable to seek to a subfile in WAD")?;
            let mut data = vec![0; length as usize];
            input
                .read_exact(&mut data)
                .context("unable to read a subfile in WAD")?;
            let chunks = Chunk::read_m2_chunks(Cursor::new(&data))?;
            files.push(chunks);
        }
        Ok(Wad {
            wad_version,
            data_version,
            file_name,
            checksum,
            directory_offset,
            wad_count,
            application_specific_directory_data_size,
            entry_header_size,
            directory_entry_base_size,
            parent_checksum,
            files,
        })
    }
}

pub fn show_wad(wad_path: PathBuf) -> anyhow::Result<()> {
    let f = File::open(wad_path).context("unable to open file")?;
    let wad = Wad::read_wad(f).context("unable to read wad")?;
    dbg!(wad);
    Ok(())
}

pub fn show_chunks(wad_path: PathBuf) -> anyhow::Result<()> {
    let f = File::open(wad_path).context("unable to open file")?;
    let chunks = Chunk::read_m1_chunks(f).context("unable to read chunks")?;
    dbg!(chunks);
    Ok(())
}
