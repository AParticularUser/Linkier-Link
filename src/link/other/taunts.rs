use crate::imports::*;
use crate::common::{
    consts::*,
    funcs::*
};
use crate::link::{
    consts::{
        status_kind_ex::*,
        vars::*
    }
};


////status
//added mortal-draw and skyward-charge
unsafe extern "C" fn appeal_status_main(agent: &mut L2CFighterCommon) -> L2CValue { 
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_MORTAL_DRAW);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_SKYWARD_CHARGE);
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_APPEAL)(agent)
}
unsafe extern "C" fn appeal_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(agent.module_accessor);
    if motion == hash40("appeal_hi_l") 
    || motion == hash40("appeal_hi_r") {
        if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_MORTAL_DRAW) 
        && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)  
        && ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) == false {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW.into(), false.into());
            return true.into()
        }
    }
    if motion == hash40("appeal_lw_l") 
    || motion == hash40("appeal_lw_r") {
        if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_SKYWARD_CHARGE)
        && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)  
        && !ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_SKYWARD_CHARGE.into(), false.into());
            return true.into()
        }
    }
    false.into()
}
//mortal-draw
//hold taunt to ready mortal-draw
//press attack to perform mortal-draw
//release button to cancel
unsafe extern "C" fn mortal_draw_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn mortal_draw_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(agent.module_accessor);
    VarModule::set_float(agent.module_accessor, status::LINK_FLOAT_APPEAL_PREV_MOTION_FRAME, frame);
    MotionModule::change_motion(agent.module_accessor, Hash40::new("mortal_draw_loop"), 0.0, 1.0, false, 0.0, false, false);
    agent.sub_shift_status_main(L2CValue::Ptr(mortal_draw_status_main_loop as *const () as _))
}
pub unsafe fn mortal_draw_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(agent.module_accessor) == hash40("mortal_draw_loop") {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("mortal_draw"), 0.0, 1.0, false, 0.0, false, false);
        }else if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            let frame = VarModule::get_float(agent.module_accessor, status::LINK_FLOAT_APPEAL_PREV_MOTION_FRAME);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_hi_r"), frame, 1.0, false, 5.0, true, false);
        }
    }else if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
unsafe extern "C" fn mortal_draw_check_attack(agent: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let opponent_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
    let opponent_boma = sv_battle_object::module_accessor(opponent_id);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER 
    && collision_kind == *COLLISION_KIND_HIT {
        let opponent_pos = PostureModule::pos_2d(opponent_boma);
        FighterUtil::request_critical_hit_cut_in_force(
            agent.module_accessor, //boma
            opponent_id, //target id
            &Vector2f{x: opponent_pos.x, y: opponent_pos.y}, //background pos global
            -1, //fighter_kind, -1 for default
            Hash40::new("param_critical"), //param hash
            0, //unused???
            true, //use offset params
            0,
            true
        );
    }
    0.into()
}
unsafe extern "C" fn mortal_draw_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//skyward-charge
//hold taunt to charge sword-beam
//release button to cancel
unsafe extern "C" fn skyward_charge_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn skyward_charge_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(agent.module_accessor);
    VarModule::set_float(agent.module_accessor, status::LINK_FLOAT_APPEAL_PREV_MOTION_FRAME, frame);
    MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
    agent.sub_shift_status_main(L2CValue::Ptr(skyward_charge_status_main_loop as *const () as _))
}
pub unsafe fn skyward_charge_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(agent.module_accessor) == hash40("appeal_lw_charge"){
        if ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
        || MotionModule::is_end(agent.module_accessor) {
            if MotionModule::is_end(agent.module_accessor) {
                VarModule::on_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED);
                macros::PLAY_SE(agent, Hash40::new("se_link_appeal_l01"));
                macros::PLAY_SE(agent, Hash40::new("se_link_special_n04"));
                let effect = VarModule::get_int(agent.module_accessor, instance::LINK_INT_SKYWARD_STRIKE_EFFECT_HANDLE);
                EffectModule::set_rate(agent.module_accessor, effect as u32, 2.0);
            }else {
                EffectModule::kill_kind(agent.module_accessor, Hash40::new("link_sword_appeal"), false, true);
                EffectModule::detach_kind(agent.module_accessor, Hash40::new("link_kaiten_hold"), 0);
                EffectModule::kill_kind(agent.module_accessor, Hash40::new("link_kaiten_hold"), true, true);
            }
            SoundModule::stop_se(agent.module_accessor, Hash40::new("se_link_special_l02"), 0);
            let frame = VarModule::get_float(agent.module_accessor, status::LINK_FLOAT_APPEAL_PREV_MOTION_FRAME);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("appeal_lw_r"), frame, 1.0, false, 5.0, true, false);
        }
    }else if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
unsafe extern "C" fn skyward_charge_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED) {
        EffectModule::detach_kind(agent.module_accessor, Hash40::new("link_kaiten_hold"), 0);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("link_kaiten_hold"), false, true);
    }
    EffectModule::kill_kind(agent.module_accessor, Hash40::new("link_sword_appeal"), false, true);
    SoundModule::stop_se(agent.module_accessor, Hash40::new("se_link_special_l02"), 0);
    0.into()
}
////motion
//up-taunt
unsafe extern "C" fn appeal_hi_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_MORTAL_DRAW);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_MORTAL_DRAW);
    }
}
unsafe extern "C" fn appeal_hi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn appeal_hi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appeal_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appeal_h02"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appeal_h03"));
    }
}
unsafe extern "C" fn appeal_hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 105.0);
    if macros::is_excute(agent) {
        if ItemModule::is_have_item(agent.module_accessor, 0) {
            ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        }else {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}
//mortal-draw-loop
// unsafe extern "C" fn mortal_draw_loop_game(agent: &mut L2CAgentBase) {}
// unsafe extern "C" fn mortal_draw_loop_eff(agent: &mut L2CAgentBase) {}
// unsafe extern "C" fn mortal_draw_loop_snd(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn mortal_draw_loop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}
//mortal-draw
unsafe extern "C" fn mortal_draw_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 100.0, 361, 10, 0, 75, 3.2, 8.6, 0.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -95.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 100.0, 361, 10, 0, 75, 3.5, 3.0, 0.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -95.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 100.0, 361, 10, 0, 75, 3.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -95.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 0.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 0.1);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 100.0, 361, 10, 0, 75, 2.5, 0.0, 14.0, 14.0, Some(0.0), Some(14.0), Some(7.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -95.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 100.0, 361, 10, 0, 75, 2.5, 0.0, 2.5, 16.0, Some(0.0), Some(2.5), Some(7.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -95.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 0.1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.5);
}
unsafe extern "C" fn mortal_draw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
}
unsafe extern "C" fn mortal_draw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_ll"));
        macros::PLAY_SE(agent, Hash40::new("vc_link_attack06"));
    }
}
unsafe extern "C" fn mortal_draw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}
//side-taunt
unsafe extern "C" fn appeal_s_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        //fixing shield visibility not switching when grabbing an item
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 105.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 10);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}
//down-taunt
unsafe extern "C" fn appeal_lw_game(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_SKYWARD_CHARGE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_APPEAL_ENABLE_SKYWARD_CHARGE);
    }
}
unsafe extern "C" fn appeal_lw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 13, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}
unsafe extern "C" fn appeal_lw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_squat"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_link_attack03"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appeal_s03"));
    }
}
unsafe extern "C" fn appeal_lw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//skyward-charge
// unsafe extern "C" fn appeal_lw_charge_game(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn appeal_lw_charge_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_kaiten_hold"), Hash40::new("sword2"), 1, 0, 0, 0, 0, 0, 0.3, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.07);
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("link_sword_appeal"), Hash40::new("sword1"), &Vector3f{x:0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rate(agent.module_accessor, effect as u32, 0.18);
        VarModule::set_int(agent.module_accessor, instance::LINK_INT_SKYWARD_STRIKE_EFFECT_HANDLE, effect as i32);
    }
}
unsafe extern "C" fn appeal_lw_charge_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        let sound = SoundModule::play_se(agent.module_accessor, Hash40::new("se_link_special_l02"), false, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, sound as i32, 3.0, 0);
    }
}
unsafe extern "C" fn appeal_lw_charge_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //up-taunt
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, appeal_status_exec);
    //mortal-draw
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW, mortal_draw_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW, mortal_draw_status_main);
    agent.status(CheckAttack, FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW, mortal_draw_check_attack);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW, mortal_draw_status_end);
    //skyward-charge
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SKYWARD_CHARGE, skyward_charge_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SKYWARD_CHARGE, skyward_charge_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SKYWARD_CHARGE, skyward_charge_status_end);
    ////motion
    //up-taunt
    agent.game_acmd("game_appealhil", appeal_hi_game, Priority::High);
    agent.game_acmd("game_appealhir", appeal_hi_game, Priority::High);
    agent.effect_acmd("effect_appealhil", appeal_hi_eff, Priority::High);
    agent.effect_acmd("effect_appealhir", appeal_hi_eff, Priority::High);
    agent.sound_acmd("sound_appealhil", appeal_hi_snd, Priority::High);
    agent.sound_acmd("sound_appealhir", appeal_hi_snd, Priority::High);
    agent.expression_acmd("expression_appealhil", appeal_hi_exp, Priority::High);
    agent.expression_acmd("expression_appealhir", appeal_hi_exp, Priority::High);
    //mortal-draw-loop
    // agent.game_acmd("game_mortaldrawloop", mortal_draw_loop_game, Priority::High);
    // agent.effect_acmd("effect_mortaldrawloop", mortal_draw_loop_eff, Priority::High);
    // agent.sound_acmd("sound_mortaldrawloop", mortal_draw_loop_snd, Priority::High);
    agent.expression_acmd("expression_mortaldrawloop", mortal_draw_loop_exp, Priority::High);
    //mortal-draw
    agent.game_acmd("game_mortaldraw", mortal_draw_game, Priority::High);
    agent.effect_acmd("effect_mortaldraw", mortal_draw_eff, Priority::High);
    agent.sound_acmd("sound_mortaldraw", mortal_draw_snd, Priority::High);
    agent.expression_acmd("expression_mortaldraw", mortal_draw_exp, Priority::High);
    //side-taunt
    agent.expression_acmd("expression_appealsl", appeal_s_exp, Priority::High);
    agent.expression_acmd("expression_appealsr", appeal_s_exp, Priority::High);
    //down-taunt
    agent.game_acmd("game_appeallwl", appeal_lw_game, Priority::High);
    agent.game_acmd("game_appeallwr", appeal_lw_game, Priority::High);
    agent.effect_acmd("effect_appeallwl", appeal_lw_eff, Priority::High);
    agent.effect_acmd("effect_appeallwr", appeal_lw_eff, Priority::High);
    agent.sound_acmd("sound_appeallwl", appeal_lw_snd, Priority::High);
    agent.sound_acmd("sound_appeallwr", appeal_lw_snd, Priority::High);
    agent.expression_acmd("expression_appeallwl", appeal_lw_exp, Priority::High);
    agent.expression_acmd("expression_appeallwr", appeal_lw_exp, Priority::High);
    //skyward-charge
    // agent.game_acmd("game_appeallwcharge", appeal_lw_charge_game, Priority::High);
    agent.effect_acmd("effect_appeallwcharge", appeal_lw_charge_eff, Priority::High);
    agent.sound_acmd("sound_appeallwcharge", appeal_lw_charge_snd, Priority::High);
    agent.expression_acmd("expression_appeallwcharge", appeal_lw_charge_exp, Priority::High);
}