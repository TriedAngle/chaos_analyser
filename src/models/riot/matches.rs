use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchDTO {
    pub game_id: i64,
    pub participant_identities: Vec<ParticipantIdentityDTO>,
    pub queue_id: i32,
    pub game_type: String,
    pub game_duration: i64,
    pub teams: Vec<TeamStatsDTO>,
    pub platform_id: String,
    pub game_creation: i64,
    pub season_id: i64,
    pub game_version: String,
    pub map_id: i32,
    pub game_mode: String,
    pub participants: Vec<ParticipantDTO>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentityDTO {
    pub participant_id: i32,
    pub player: PlayerDTO,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDTO {
    pub profile_icon: i32,
    pub account_id: String,
    pub match_history_uri: String,
    pub current_account_id: String,
    pub current_platform_id: String,
    pub summoner_name: String,
    pub summoner_id: String,
    pub platform_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatsDTO {
    pub tower_kills: i32,
    pub rift_herald_kills: i32,
    pub first_blood: bool,
    pub inhibitor_kills: i32,
    pub bans: Vec<TeamBansDTO>,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub baron_kills: i32,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub vilemaw_kills: i32,
    pub first_rift_herald: bool,
    pub team_id: i32,
    pub win: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamBansDTO {
    pub champion_id: i32,
    pub pick_turn: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDTO {
    pub participant_id: i32,
    pub champion_id: i32,
    pub runes: Vec<RuneDTO>,
    pub stats: ParticipantStatsDTO,
    pub team_id: i32,
    pub timeline: ParticipantTimelineDTO,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub highest_achieved_season_tier: String,
    pub masteries: Vec<MasteryDTO>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuneDTO {
    pub rune_id: i32,
    pub rank: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStatsDTO {
    pub item0: i32,
    pub item2: i32,
    pub total_units_healed: i32,
    pub item1: i32,
    pub largest_multi_kill: i32,
    pub gold_earned: i32,
    pub first_inhibitor_kill: bool,
    pub physical_damage_taken: i64,
    pub node_neutralize_assist: i32,
    pub total_player_score: i32,
    pub champ_level: i32,
    pub damage_dealt_to_objectives: i64,
    pub total_damage_taken: i64,
    pub neutral_minions_killed: i64,
    pub deaths: i32,
    pub triple_kills: i32,
    pub magic_damage_dealt_to_champions: i64,
    pub wards_killed: i32,
    pub penta_kills: i32,
    pub damage_self_mitigated: i64,
    pub largest_critical_strike: i32,
    pub node_neutralize: i32,
    pub total_time_crowd_control_dealt: i32,
    pub first_tower_kill: bool,
    pub magic_damage_dealt: i64,
    pub total_score_rank: i32,
    pub node_capture: i32,
    pub wards_placed: i32,
    pub total_damage_dealt: i64,
    pub time_ccing_others: i64,
    pub magical_damage_taken: i64,
    pub largest_killing_spree: i32,
    pub total_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub neutral_minions_killed_team_jungle: i32,
    pub total_minions_killed: i32,
    pub first_inhibitor_assist: bool,
    pub vision_wards_bought_in_game: i32,
    pub objective_player_score: i32,
    pub kills: i32,
    pub first_tower_assist: bool,
    pub combat_player_score: i32,
    pub inhibitor_kills: i32,
    pub turret_kills: i32,
    pub participant_id: i32,
    pub true_damage_taken: i32,
    pub first_blood_assist: bool,
    pub node_capture_assist: i32,
    pub assists: i32,
    pub team_objective: i32,
    pub alters_neutralized: i32,
    pub gold_spent: i32,
    pub damage_dealt_to_turrets: i64,
    pub alters_captured: i32,
    pub win: bool,
    pub total_heal: i64,
    pub unreal_kills: i64,
    pub vision_score: i64,
    pub physical_damage_dealt: i64,
    pub first_blood_kill: bool,
    pub longest_time_spent_living: i32,
    pub killing_sprees: i32,
    pub sight_wards_bought_in_game: i32,
    pub true_damage_dealt_to_champions: i64,
    pub neutral_minions_killed_enemy_jungle: i32,
    pub double_kills: i32,
    pub true_damage_dealt: i32,
    pub quadra_kills: i32,
    pub item4: i32,
    pub item3: i32,
    pub item6: i32,
    pub item5: i32,
    pub player_score_0: i32,
    pub player_score_1: i32,
    pub player_score_2: i32,
    pub player_score_3: i32,
    pub player_score_4: i32,
    pub player_score_5: i32,
    pub player_score_6: i32,
    pub player_score_7: i32,
    pub player_score_8: i32,
    pub perk0: i32,
    pub perk0_var1: i32,
    pub perk0_var2: i32,
    pub perk0_var3: i32,
    pub perk1: i32,
    pub perk1_var1: i32,
    pub perk1_var2: i32,
    pub perk1_var3: i32,
    pub perk2: i32,
    pub perk2_var1: i32,
    pub perk2_var2: i32,
    pub perk2_var3: i32,
    pub perk3: i32,
    pub perk3_var1: i32,
    pub perk3_var2: i32,
    pub perk3_var3: i32,
    pub perk4: i32,
    pub perk4_var1: i32,
    pub perk4_var2: i32,
    pub perk4_var3: i32,
    pub perk5: i32,
    pub perk5_var1: i32,
    pub perk5_var2: i32,
    pub perk5_var3: i32,
    pub perk_primary_style: i32,
    pub perk_sub_style: i32,
    pub stat_perk0: i32,
    pub stat_perk1: i32,
    pub stat_perk2: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimelineDTO {
    pub participant_id: i32,
    pub cs_diff_per_min_deltas: HashMap<String, f32>,
    pub damage_taken_per_mid_deltas: HashMap<String, f32>,
    pub role: String,
    pub damage_taken_diff_per_min_deltas: HashMap<String, f32>,
    pub xp_per_mid_deltas: HashMap<String, f32>,
    pub xp_diff_per_min_deltas: HashMap<String, f32>,
    pub lane: String,
    pub creeps_per_min_deltas: HashMap<String, f32>,
    pub gold_per_mid_deltas: HashMap<String, f32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MasteryDTO {
    pub rank: i32,
    pub mastery_id: i32,
}
