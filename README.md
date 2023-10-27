This is a tool for converting Marathon physics files into JSON form.

# Compiling

You will need to [install Rust](https://www.rust-lang.org/learn/get-started). Installation is usually fairly quick. Once installed, all you need is:

```sh
cd physics-eater
cargo install --path .
```

# Usage

You will either need to compile physics-eater yourself (see above) or obtain a binary from someone.

## Names

Legibility of the converted result can be improved using a names list. A names list is a text file containing one name per line. The first line names index 0, the second line names index 1, etc. If the line is blank, or the name list is not provided, a number will be used instead of a name.

physics-eater comes with a directory named `infinity_names`, which contains name lists appropriate for use with Marathon 2 or Infinity.

## Examples

(Change paths as appropriate.)

Converting a Marathon 2 physics file with names appropriate for Marathon Infinity (provided in this repository):

```sh
physics-eater /path/to/Marathon\ 2/Physics\ Models/Standard.phyA convert-m2-physics --namedb /path/to/infinity_names > ~/Desktop/Marathon2.json
```

Converting a Marathon 1 physics file with names appropriate for Marathon 1 (NOT provided):

```sh
physics-eater /path/to/Marathon/Physics.phys convert-m1-physics --namedb /path/to/m1_names > ~/Desktop/Marathon1.json
```

## A word on Infinity

Marathon Infinity shipped with a so-called "standard" physics file. This is *not* a Marathon Infinity physics file, this is a Marathon 2 physics file. No information relating to vacuum BOBs or the SMG is present in this file. If you actually want Marathon Infinity's physics, you'll have to get it from somewhere else.

# Units

- Angle: Degrees
- Distance: World Units (1 WU = 2m = 1024 IU)
- Time: Tick (*not* second!)
- Speed: World Units per Tick (*not* per second!)
- Acceleration: World Units per Tick per Tick (*not* per second per second!)

# Legalese

physics-eater is copyright 2023 Solra Bizna.

physics-eater is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

physics-eater is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received [a copy of the GNU General Public License](COPYING.md) along with physics-eater. If not, see <https://www.gnu.org/licenses/>.

Substantial portions of physics-eater are based on Aleph One. Aleph One is published under the GNU General Public License, which permits this usage.
