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

use std::{fs::File, io::Read};

use anyhow::anyhow;
use serde::Serialize;
use serde_json::Value;

pub const MONSTER_PHYSICS_TAG: [u8; 4] = *b"mons";
pub const EFFECT_PHYSICS_TAG: [u8; 4] = *b"effe";
pub const PROJECTILE_PHYSICS_TAG: [u8; 4] = *b"proj";
pub const PHYSICS_PHYSICS_TAG: [u8; 4] = *b"phys";
pub const WEAPON_PHYSICS_TAG: [u8; 4] = *b"weap";

#[derive(Serialize)]
struct MonsterFlags {
    omniscient: bool,
    flies: bool,
    is_alien: bool,
    major: bool,
    minor: bool,
    cannot_skip: bool,
    floats: bool,
    cannot_attack: bool,
    uses_sniper_ledges: bool,
    is_invisible: bool,
    is_subtly_invisible: bool,
    kamikaze: bool, // misspelled grievously in the original source
    berserker: bool,
    enlarged: bool,
    delayed_hard_death: bool,
    fires_symmetrically: bool,
    nuclear_hard_death: bool,
    cannot_fire_backwards: bool,
    can_die_in_flames: bool,
    waits_with_clear_shot: bool,
    tiny: bool,
    attacks_immediately: bool,
    not_afraid_of_water: bool,
    not_afraid_of_sewage: bool,
    not_afraid_of_lava: bool,
    not_afraid_of_goo: bool,
    can_teleport_under_media: bool,
    chooses_weapons_randomly: bool,
}

impl MonsterFlags {
    pub fn read(input: impl Read) -> anyhow::Result<MonsterFlags> {
        Ok(decode_flags!(
            read32(input)? => MonsterFlags {
                omniscient,
                flies,
                is_alien,
                major,
                minor,
                cannot_skip,
                floats,
                cannot_attack,
                uses_sniper_ledges,
                is_invisible,
                is_subtly_invisible,
                kamikaze,
                berserker,
                enlarged,
                delayed_hard_death,
                fires_symmetrically,
                nuclear_hard_death,
                cannot_fire_backwards,
                can_die_in_flames,
                waits_with_clear_shot,
                tiny,
                attacks_immediately,
                not_afraid_of_water,
                not_afraid_of_sewage,
                not_afraid_of_lava,
                not_afraid_of_goo,
                can_teleport_under_media,
                chooses_weapons_randomly,
            }
        ))
    }
}

#[derive(Serialize)]
struct DamageDefinitionFlags {
    alien_damage: bool,
}

impl DamageDefinitionFlags {
    pub fn read(input: impl Read) -> anyhow::Result<DamageDefinitionFlags> {
        Ok(
            decode_flags!(read16(input)? => DamageDefinitionFlags { alien_damage }),
        )
    }
}

#[derive(Serialize)]
struct DamageDefinition {
    damage_type: Option<Value>,
    flags: DamageDefinitionFlags,
    base: i16,
    random: i16,
    scale: f32,
}

impl DamageDefinition {
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
    ) -> anyhow::Result<DamageDefinition> {
        let damage_type = read_optional_16(&mut input)?
            .map(|x| namedbs.damage_type_names.identify(x));
        let flags = DamageDefinitionFlags::read(&mut input)?;
        let base = read16(&mut input)? as i16;
        let random = read16(&mut input)? as i16;
        let scale = read_fx_16_16(&mut input)?;
        Ok(DamageDefinition {
            damage_type,
            flags,
            base,
            random,
            scale,
        })
    }
}

#[derive(Serialize)]
struct AttackDefinition {
    pub projectile_type: Value,
    pub repetitions: Option<u16>,
    pub error: f32,
    pub range: f32,
    pub attack_sequence: Option<u16>,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

impl AttackDefinition {
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
    ) -> anyhow::Result<Option<AttackDefinition>> {
        let projectile_type = read_optional_16(&mut input)?
            .map(|x| namedbs.projectile_names.identify(x));
        let repetitions = read_optional_16(&mut input)?;
        let error = read_angle(&mut input)?;
        let range = read_world_distance(&mut input)?;
        let attack_sequence = read_optional_16(&mut input)?;
        let dx = read_world_distance(&mut input)?;
        let dy = read_world_distance(&mut input)?;
        let dz = read_world_distance(&mut input)?;
        Ok(projectile_type.map(|projectile_type| AttackDefinition {
            projectile_type,
            repetitions,
            error,
            range,
            attack_sequence,
            dx,
            dy,
            dz,
        }))
    }
}

#[derive(Serialize)]
struct MonsterDefinition {
    #[serde(skip_serializing_if = "serde_json::Value::is_number")]
    pub name: Value,
    pub collection: Option<Value>,
    pub clut: Option<u16>,
    pub vitality: Option<u16>,
    pub immunities: Vec<Value>,
    pub weaknesses: Vec<Value>,
    pub flags: MonsterFlags,
    pub class: Option<Value>,
    pub friends: Vec<Value>,
    pub enemies: Vec<Value>,
    pub activation_sound: Option<Value>,
    pub conversation_sound: Option<Value>,
    pub flaming_sound: Option<Value>,
    pub random_sound: Option<Value>,
    pub random_sound_mask: Option<u16>,
    pub carrying_item_type: Option<Value>,
    pub radius: f32,
    pub height: f32,
    pub preferred_hover_height: f32,
    pub minimum_ledge_delta: f32,
    pub maximum_ledge_delta: f32,
    pub external_velocity_scale: f32,
    pub impact_effect: Option<Value>,
    pub melee_impact_effect: Option<Value>,
    pub half_visual_arc: f32,
    pub half_vertical_visual_arc: f32,
    pub visual_range: f32,
    pub dark_visual_range: f32,
    pub intelligence: Option<u16>,
    pub speed: f32,
    pub gravity: f32,
    pub terminal_velocity: f32,
    pub door_retry_mask: Option<u16>,
    pub shrapnel_radius: Option<f32>,
    pub shrapnel_damage: DamageDefinition,
    // these are marked as shape descriptors in the code, but they're actually
    // sequences
    pub hit_sequence: Option<u16>,
    pub hard_dying_sequence: Option<u16>,
    pub soft_dying_sequence: Option<u16>,
    pub hard_dead_sequence: Option<u16>,
    pub soft_dead_sequence: Option<u16>,
    pub stationary_sequence: Option<u16>,
    pub moving_sequence: Option<u16>,
    pub attack_frequency: Option<u16>,
    pub melee_attack: Option<AttackDefinition>,
    pub ranged_attack: Option<AttackDefinition>,
}

impl MonsterDefinition {
    pub fn read_definitions(
        input: &[u8],
        namedbs: &NameDbs,
    ) -> anyhow::Result<Vec<MonsterDefinition>> {
        const SIZE_OF_MONSTER_DEFINITION: usize = 138;
        if input.len() % SIZE_OF_MONSTER_DEFINITION != 0 {
            return Err(anyhow!("non-integer number of monster definitions, or corrupted/misdetected physics file"));
        }
        input
            .chunks_exact(SIZE_OF_MONSTER_DEFINITION)
            .enumerate()
            .map(|(i, x)| MonsterDefinition::read(x, namedbs, i))
            .collect()
    }
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
        index: usize,
    ) -> anyhow::Result<MonsterDefinition> {
        let collection_and_clut = read_optional_16(&mut input)?;
        let collection = collection_and_clut
            .map(|x| namedbs.collection_names.identify(x % 32));
        let clut = collection_and_clut.map(|x| x / 32);
        Ok(MonsterDefinition {
            name: namedbs.monster_names.identify(index),
            collection,
            clut,
            vitality: read_optional_16(&mut input)?,
            immunities: read_generic_bitfield32(&mut input)?
                .into_iter()
                .map(|x| namedbs.damage_type_names.identify(x))
                .collect(),
            weaknesses: read_generic_bitfield32(&mut input)?
                .into_iter()
                .map(|x| namedbs.damage_type_names.identify(x))
                .collect(),
            flags: MonsterFlags::read(&mut input)?,
            class: read_optional_32(&mut input)?
                .map(|x| namedbs.monster_class_names.identify(x)),
            friends: read_generic_bitfield32(&mut input)?
                .into_iter()
                .map(|x| namedbs.monster_class_names.identify(x))
                .collect(),
            enemies: read_generic_bitfield32(&mut input)?
                .into_iter()
                .map(|x| namedbs.monster_class_names.identify(x))
                .collect(),
            activation_sound: read_optional_16(&mut input)?
                .map(|x| namedbs.sound_names.identify(x)),
            conversation_sound: read_optional_16(&mut input)?
                .map(|x| namedbs.sound_names.identify(x)),
            flaming_sound: read_optional_16(&mut input)?
                .map(|x| namedbs.sound_names.identify(x)),
            random_sound: read_optional_16(&mut input)?
                .map(|x| namedbs.sound_names.identify(x)),
            random_sound_mask: read_optional_16(&mut input)?,
            carrying_item_type: read_optional_16(&mut input)?
                .map(|x| namedbs.item_names.identify(x)),
            radius: read_world_distance(&mut input)?,
            height: read_world_distance(&mut input)?,
            preferred_hover_height: read_world_distance(&mut input)?,
            minimum_ledge_delta: read_world_distance(&mut input)?,
            maximum_ledge_delta: read_world_distance(&mut input)?,
            external_velocity_scale: read_fx_16_16(&mut input)?,
            impact_effect: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            melee_impact_effect: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            half_visual_arc: read_angle(&mut input)?,
            half_vertical_visual_arc: read_angle(&mut input)?,
            visual_range: read_world_distance(&mut input)?,
            dark_visual_range: read_world_distance(&mut input)?,
            intelligence: read_optional_16(&mut input)?,
            speed: read_world_speed(&mut input)?,
            gravity: read_world_accel(&mut input)?,
            terminal_velocity: read_world_speed(&mut input)?,
            door_retry_mask: read_optional_16(&mut input)?,
            shrapnel_radius: read_optional_fx_6_10(&mut input)?,
            shrapnel_damage: DamageDefinition::read(&mut input, namedbs)?,
            hit_sequence: read_optional_16(&mut input)?,
            hard_dying_sequence: read_optional_16(&mut input)?,
            soft_dying_sequence: read_optional_16(&mut input)?,
            hard_dead_sequence: read_optional_16(&mut input)?,
            soft_dead_sequence: read_optional_16(&mut input)?,
            stationary_sequence: read_optional_16(&mut input)?,
            moving_sequence: read_optional_16(&mut input)?,
            attack_frequency: read_optional_16(&mut input)?,
            melee_attack: AttackDefinition::read(&mut input, namedbs)?,
            ranged_attack: AttackDefinition::read(&mut input, namedbs)?,
        })
    }
}

#[derive(Serialize)]
struct EffectFlags {
    pub end_when_animation_loops: bool,
    pub end_when_transfer_animation_loops: bool,
    pub sound_only: bool,
    pub make_twin_visible: bool, // ????
}

#[derive(Serialize)]
struct EffectDefinition {
    #[serde(skip_serializing_if = "serde_json::Value::is_number")]
    name: Value,
    collection: Option<Value>,
    clut: Option<u16>,
    sequence: Option<u16>,
    flags: EffectFlags,
}

impl EffectDefinition {
    pub fn read_definitions(
        input: &[u8],
        namedbs: &NameDbs,
    ) -> anyhow::Result<Vec<EffectDefinition>> {
        const SIZE_OF_EFFECT_DEFINITION: usize = 6;
        if input.len() % SIZE_OF_EFFECT_DEFINITION != 0 {
            return Err(anyhow!("non-integer number of effect definitions, or corrupted/misdetected physics file"));
        }
        input
            .chunks_exact(SIZE_OF_EFFECT_DEFINITION)
            .enumerate()
            .map(|(i, x)| EffectDefinition::read(x, namedbs, i))
            .collect()
    }
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
        index: usize,
    ) -> anyhow::Result<EffectDefinition> {
        let collection_and_clut = read_optional_16(&mut input)?;
        let collection = collection_and_clut
            .map(|x| namedbs.collection_names.identify(x % 32));
        let clut = collection_and_clut.map(|x| x / 32);
        Ok(EffectDefinition {
            name: namedbs.effect_names.identify(index),
            collection,
            clut,
            sequence: read_optional_16(&mut input)?,
            flags: decode_flags!(read16(&mut input)? => EffectFlags {
                end_when_animation_loops,
                end_when_transfer_animation_loops,
                sound_only,
                make_twin_visible,
            }),
        })
    }
}

#[derive(Serialize)]
struct ProjectileFlags {
    pub guided: bool,
    pub stop_when_animation_loops: bool,
    pub persistent: bool,
    pub alien: bool,
    pub affected_by_gravity: bool,
    pub no_horizontal_error: bool,
    pub no_vertical_error: bool,
    pub can_toggle_control_panels: bool,
    pub positive_vertical_error: bool,
    pub melee: bool,
    pub persistent_and_virulent: bool,
    pub usually_pass_transparent_side: bool,
    pub sometimes_pass_transparent_side: bool,
    pub doubly_affected_by_gravity: bool,
}

#[derive(Serialize)]
struct ProjectileDefinition {
    #[serde(skip_serializing_if = "serde_json::Value::is_number")]
    name: Value,
    collection: Option<Value>,
    clut: Option<u16>,
    sequence: Option<u16>,
    detonation_effect: Option<Value>,
    contrail_effect: Option<Value>,
    ticks_between_contrails: Option<Value>,
    maximum_contrails: Option<Value>,
    radius: f32,
    area_of_effect: f32,
    damage: DamageDefinition,
    flags: ProjectileFlags,
    speed: f32,
    maximum_range: f32,
    flyby_sound: Option<Value>,
}

impl ProjectileDefinition {
    pub fn read_definitions(
        input: &[u8],
        namedbs: &NameDbs,
    ) -> anyhow::Result<Vec<ProjectileDefinition>> {
        const SIZE_OF_PROJECTILE_DEFINITION: usize = 36;
        if input.len() % SIZE_OF_PROJECTILE_DEFINITION != 0 {
            return Err(anyhow!("non-integer number of projectile definitions, or corrupted/misdetected physics file"));
        }
        input
            .chunks_exact(SIZE_OF_PROJECTILE_DEFINITION)
            .enumerate()
            .map(|(i, x)| ProjectileDefinition::read(x, namedbs, i))
            .collect()
    }
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
        index: usize,
    ) -> anyhow::Result<ProjectileDefinition> {
        let collection_and_clut = read_optional_16(&mut input)?;
        let collection = collection_and_clut
            .map(|x| namedbs.collection_names.identify(x % 32));
        let clut = collection_and_clut.map(|x| x / 32);
        Ok(ProjectileDefinition {
            name: namedbs.projectile_names.identify(index),
            collection,
            clut,
            sequence: read_optional_16(&mut input)?,
            detonation_effect: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            contrail_effect: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            ticks_between_contrails: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            maximum_contrails: read_optional_16(&mut input)?
                .map(|x| namedbs.effect_names.identify(x)),
            radius: read_world_distance(&mut input)?,
            area_of_effect: read_world_distance(&mut input)?,
            damage: DamageDefinition::read(&mut input, namedbs)?,
            flags: decode_flags!(read16(&mut input)? => ProjectileFlags {
                guided,
                stop_when_animation_loops,
                persistent,
                alien,
                affected_by_gravity,
                no_horizontal_error,
                no_vertical_error,
                can_toggle_control_panels,
                positive_vertical_error,
                melee,
                persistent_and_virulent,
                usually_pass_transparent_side,
                sometimes_pass_transparent_side,
                doubly_affected_by_gravity,
                //rebounds_from_floor,?
                // don't know how many of these are valid,
                // but the list definitely stops here
            }),
            speed: read_world_speed(&mut input)?,
            maximum_range: read_world_distance(&mut input)?,
            flyby_sound: read_optional_16(&mut input)?
                .map(|x| namedbs.sound_names.identify(x)),
        })
    }
}

fn is_false(x: &bool) -> bool {
    !*x
}

#[derive(Serialize)]
struct WeaponFlags {
    pub is_automatic: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub unknown: bool,
    pub disappears_after_use: bool,
}

#[derive(Serialize, Default)]
struct TriggerDefinition {
    pub rounds_per_magazine: Option<u16>,
    pub ammunition_type: Option<Value>,
    pub ticks_per_round: Option<u16>,
    pub recovery_ticks: Option<u16>,
    pub charging_ticks: Option<u16>,
    pub recoil_magnitude: f32,
    pub firing_sound: Option<Value>,
    pub click_sound: Option<Value>,
    pub charging_sound: Option<Value>,
    pub shell_casing_sound: Option<Value>,
    pub reloading_sound: Option<Value>,
    pub sound_activation_range: f32,
    pub projectile_type: Option<Value>,
    pub theta_error: f32,
    pub dx: f32,
    pub dz: f32,
    pub burst_count: Option<u16>,
}

#[derive(Serialize)]
struct WeaponDefinition {
    #[serde(skip_serializing_if = "serde_json::Value::is_number")]
    name: Value,
    item_type: Option<Value>,
    weapon_class: Option<Value>,
    flags: WeaponFlags,
    firing_light_intensity: f32,
    firing_intensity_decay_ticks: Option<u16>,
    idle_height: f32,
    bob_amplitude: f32,
    kick_height: f32,
    reload_height: f32,
    idle_width: f32,
    horizontal_amplitude: f32,
    collection: Option<u16>,
    idle_sequence: Option<u16>,
    firing_sequence: Option<u16>,
    reloading_sequence: Option<u16>,
    #[serde(skip)]
    _unused: u16,
    charging_sequence: Option<u16>,
    charged_sequence: Option<u16>,
    ready_ticks: Option<u16>,
    await_reload_ticks: Option<u16>,
    triggers: [TriggerDefinition; 2],
}

impl WeaponDefinition {
    pub fn read_definitions(
        input: &[u8],
        namedbs: &NameDbs,
    ) -> anyhow::Result<Vec<WeaponDefinition>> {
        const SIZE_OF_WEAPON_DEFINITION: usize = 120;
        if input.len() % SIZE_OF_WEAPON_DEFINITION != 0 {
            return Err(anyhow!("non-integer number of weapon definitions, or corrupted/misdetected physics file"));
        }
        input
            .chunks_exact(SIZE_OF_WEAPON_DEFINITION)
            .enumerate()
            .map(|(i, x)| WeaponDefinition::read(x, namedbs, i))
            .collect()
    }
    pub fn read(
        mut input: impl Read,
        namedbs: &NameDbs,
        index: usize,
    ) -> anyhow::Result<WeaponDefinition> {
        let name = namedbs.weapon_names.identify(index);
        let item_type = read_optional_16(&mut input)?
            .map(|x| namedbs.item_names.identify(x));
        let weapon_class = read_optional_16(&mut input)?
            .map(|x| namedbs.weapon_class_names.identify(x));
        let flags = decode_flags!(read16(&mut input)? => WeaponFlags {
            is_automatic,
            unknown,
            disappears_after_use,
        });
        let mut triggers =
            [TriggerDefinition::default(), TriggerDefinition::default()];
        triggers[0].ammunition_type = read_optional_16(&mut input)?
            .map(|x| namedbs.item_names.identify(x));
        triggers[0].rounds_per_magazine = read_optional_16(&mut input)?;
        triggers[1].ammunition_type = read_optional_16(&mut input)?
            .map(|x| namedbs.item_names.identify(x));
        triggers[1].rounds_per_magazine = read_optional_16(&mut input)?;
        let firing_light_intensity = read_fx_16_16(&mut input)?;
        let firing_intensity_decay_ticks = read_optional_16(&mut input)?;
        let idle_height = read_fx_16_16(&mut input)?;
        let bob_amplitude = read_fx_16_16(&mut input)?;
        let kick_height = read_fx_16_16(&mut input)?;
        let reload_height = read_fx_16_16(&mut input)?;
        let idle_width = read_fx_16_16(&mut input)?;
        let horizontal_amplitude = read_fx_16_16(&mut input)?;
        let collection = read_optional_16(&mut input)?;
        let idle_sequence = read_optional_16(&mut input)?;
        let firing_sequence = read_optional_16(&mut input)?;
        let reloading_sequence = read_optional_16(&mut input)?;
        let _unused = read16(&mut input)?;
        let charging_sequence = read_optional_16(&mut input)?;
        let charged_sequence = read_optional_16(&mut input)?;
        triggers[0].ticks_per_round = read_optional_16(&mut input)?;
        triggers[1].ticks_per_round = read_optional_16(&mut input)?;
        let await_reload_ticks = read_optional_16(&mut input)?;
        let ready_ticks = read_optional_16(&mut input)?;
        triggers[0].recovery_ticks = read_optional_16(&mut input)?;
        triggers[1].recovery_ticks = read_optional_16(&mut input)?;
        triggers[0].charging_ticks = read_optional_16(&mut input)?;
        triggers[1].charging_ticks = read_optional_16(&mut input)?;
        triggers[0].recoil_magnitude = read_world_distance(&mut input)?;
        triggers[1].recoil_magnitude = read_world_distance(&mut input)?;
        triggers[0].firing_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[1].firing_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[0].click_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[1].click_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[0].reloading_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[1].reloading_sound = None;
        triggers[0].charging_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[1].charging_sound = triggers[0].charging_sound.clone();
        triggers[0].shell_casing_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[1].shell_casing_sound = read_optional_16(&mut input)?
            .map(|x| namedbs.sound_names.identify(x));
        triggers[0].sound_activation_range = read_world_distance(&mut input)?;
        triggers[1].sound_activation_range = read_world_distance(&mut input)?;
        triggers[0].projectile_type = read_optional_16(&mut input)?
            .map(|x| namedbs.projectile_names.identify(x));
        triggers[1].projectile_type = read_optional_16(&mut input)?
            .map(|x| namedbs.projectile_names.identify(x));
        triggers[0].theta_error = read_angle(&mut input)?;
        triggers[1].theta_error = read_angle(&mut input)?;
        triggers[0].dx = read_world_distance(&mut input)?;
        triggers[0].dz = read_world_distance(&mut input)?;
        triggers[1].dx = read_world_distance(&mut input)?;
        triggers[1].dz = read_world_distance(&mut input)?;
        triggers[0].burst_count = read_optional_16(&mut input)?;
        triggers[1].burst_count = read_optional_16(&mut input)?;
        let _unused2 = read16(&mut input)?;
        Ok(WeaponDefinition {
            name,
            item_type,
            weapon_class,
            flags,
            firing_light_intensity,
            firing_intensity_decay_ticks,
            idle_height,
            bob_amplitude,
            kick_height,
            reload_height,
            idle_width,
            horizontal_amplitude,
            collection,
            idle_sequence,
            firing_sequence,
            reloading_sequence,
            _unused,
            charging_sequence,
            charged_sequence,
            ready_ticks,
            await_reload_ticks,
            triggers,
        })
    }
}

#[derive(Serialize)]
struct PhysicsDefinition {
    pub maximum_forward_velocity: f32,
    pub maximum_backward_velocity: f32,
    pub maximum_perpendicular_velocity: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub airborne_deceleration: f32,
    pub gravitational_acceleration: f32,
    pub climbing_acceleration: f32,
    pub terminal_velocity: f32,
    pub external_deceleration: f32,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub maximum_angular_velocity: f32,
    pub angular_recentering_velocity: f32,
    pub fast_angular_velocity: f32,
    pub fast_angular_maximum: f32,
    pub maximum_elevation: f32,
    pub external_angular_deceleration: f32,
    pub step_delta: f32,
    pub step_amplitude: f32,
    pub radius: f32,
    pub height: f32,
    pub dead_height: f32,
    pub camera_height: f32,
    pub half_camera_separation: f32,
}

impl PhysicsDefinition {
    pub fn read(
        mut input: impl Read,
        _namedb: &NameDbs,
    ) -> anyhow::Result<PhysicsDefinition> {
        Ok(PhysicsDefinition {
            maximum_forward_velocity: read_fx_16_16(&mut input)?,
            maximum_backward_velocity: read_fx_16_16(&mut input)?,
            maximum_perpendicular_velocity: read_fx_16_16(&mut input)?,
            acceleration: read_fx_16_16(&mut input)?,
            deceleration: read_fx_16_16(&mut input)?,
            airborne_deceleration: read_fx_16_16(&mut input)?,
            gravitational_acceleration: read_fx_16_16(&mut input)?,
            climbing_acceleration: read_fx_16_16(&mut input)?,
            terminal_velocity: read_fx_16_16(&mut input)?,
            external_deceleration: read_fx_16_16(&mut input)?,
            angular_acceleration: read_fx_16_16(&mut input)?,
            angular_deceleration: read_fx_16_16(&mut input)?,
            maximum_angular_velocity: read_fx_16_16(&mut input)?,
            angular_recentering_velocity: read_fx_16_16(&mut input)?,
            fast_angular_velocity: read_fx_16_16(&mut input)?,
            fast_angular_maximum: read_fx_16_16(&mut input)?,
            maximum_elevation: read_fx_16_16(&mut input)?,
            external_angular_deceleration: read_fx_16_16(&mut input)?,
            step_delta: read_fx_16_16(&mut input)?,
            step_amplitude: read_fx_16_16(&mut input)?,
            radius: read_fx_16_16(&mut input)?,
            height: read_fx_16_16(&mut input)?,
            dead_height: read_fx_16_16(&mut input)?,
            camera_height: read_fx_16_16(&mut input)?,
            half_camera_separation: read_fx_16_16(&mut input)?,
        })
    }
}

#[derive(Serialize)]
struct PhysicsDefinitions {
    walking: PhysicsDefinition,
    running: PhysicsDefinition,
}

impl PhysicsDefinitions {
    pub fn read(
        mut input: impl Read,
        namedb: &NameDbs,
    ) -> anyhow::Result<PhysicsDefinitions> {
        Ok(PhysicsDefinitions {
            walking: PhysicsDefinition::read(&mut input, namedb)?,
            running: PhysicsDefinition::read(&mut input, namedb)?,
        })
    }
}

#[derive(Serialize)]
struct Physics {
    monster_definitions: Vec<MonsterDefinition>,
    effect_definitions: Vec<EffectDefinition>,
    projectile_definitions: Vec<ProjectileDefinition>,
    weapon_definitions: Vec<WeaponDefinition>,
    physics: PhysicsDefinitions,
}

pub fn convert_physics(
    physics_path: PathBuf,
    namedbs: NameDbs,
) -> anyhow::Result<()> {
    let chunks = Chunk::read_m1_chunks(File::open(physics_path)?)?;
    let monster_definitions = Chunk::find(&chunks, MONSTER_PHYSICS_TAG)
        .and_then(|x| MonsterDefinition::read_definitions(x, &namedbs))?;
    let effect_definitions = Chunk::find(&chunks, EFFECT_PHYSICS_TAG)
        .and_then(|x| EffectDefinition::read_definitions(x, &namedbs))?;
    let projectile_definitions = Chunk::find(&chunks, PROJECTILE_PHYSICS_TAG)
        .and_then(|x| ProjectileDefinition::read_definitions(x, &namedbs))?;
    let weapon_definitions = Chunk::find(&chunks, WEAPON_PHYSICS_TAG)
        .and_then(|x| WeaponDefinition::read_definitions(x, &namedbs))?;
    let physics_definitions = Chunk::find(&chunks, PHYSICS_PHYSICS_TAG)
        .and_then(|x| PhysicsDefinitions::read(x, &namedbs))?;
    let physics = Physics {
        monster_definitions,
        effect_definitions,
        projectile_definitions,
        weapon_definitions,
        physics: physics_definitions,
    };
    serde_json::to_writer_pretty(std::io::stdout(), &physics)?;
    Ok(())
}
