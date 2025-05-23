// use crate::imports::*;

pub unsafe fn get_table_value(table: *mut smash_rs::lib::L2CTable, key: &str) -> smash_rs::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash_rs::phx::Hash40::from_hex_str(key).unwrap()
    } else {
        smash_rs::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}
extern "C" {
    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn is_one_on_one() -> bool;
}

//// Checks if your current status is one where you're being damaged.
// pub unsafe fn is_damage_check(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
//     let status = StatusModule::status_kind(module_accessor);
//     if FighterStopModuleImpl::is_damage_stop(module_accessor)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
//     || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DOWN..=*FIGHTER_STATUS_KIND_LAY_DOWN).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DOWN_DAMAGE..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
//     || (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
//     || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
//     || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
//     || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
//     || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
//         return true;
//     }
//     false
// }
//used in conjunction with 
// WorkModule::set_int64(fighter.module_accessor, hash40("<motion>") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
// WorkModule::set_int64(fighter.module_accessor, hash40("<motion>") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
//for statuses where the grounded and aerial versions are the same 
// pub unsafe fn air_to_ground_transition_status_func(agent: &mut L2CFighterCommon, inherit_frame: bool) {
//     if agent.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
//         KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//         agent.set_situation(SITUATION_KIND_GROUND.into());
//         GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
//         let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
//         if inherit_frame {
//             MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
//             agent.clear_lua_stack();
//             lua_args!(agent, MA_MSC_CMD_EFFECT_LANDING_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, false, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
//             sv_module_access::effect(agent.lua_state_agent);
//         }else {
//             MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
//         }
//     }else {
//         KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
//         agent.set_situation(SITUATION_KIND_AIR.into());
//         GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//         let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
//         if inherit_frame {
//             MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
//         }else {
//             MotionModule::change_motion(agent.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
//         }
//     }
// }
