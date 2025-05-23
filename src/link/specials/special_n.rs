use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::{
    status_kind_ex::*,
    // vars::*,
    *
};


////status
//shoot
unsafe extern "C" fn special_n_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let status_step = WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    //special-n cancel
    if status_step != *FIGHTER_LINK_STATUS_BOW_STEP_END 
    && WorkModule::is_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
        0.into()
    //shield-surf input
    }else if agent.global_table[global_table::STATUS_FRAME].get_i32() <= param::LINK_INT_SHIELD_SURF_FORGIVENESS 
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START.into(), false.into());
        0.into()
    }else {
        smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_SPECIAL_N)(agent)
    }
}
unsafe extern "C" fn special_n_status_exit(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL {
        smashline::original_status(Exit, agent, *FIGHTER_STATUS_KIND_SPECIAL_N)(agent);
    }
    0.into()
}
unsafe extern "C" fn special_n_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL {
        smashline::original_status(End, agent, *FIGHTER_STATUS_KIND_SPECIAL_N)(agent);
    }
    0.into()
}
//cancel
unsafe extern "C" fn special_n_cancel_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_cancel_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_FRONT));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
        }
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
            if ItemModule::is_have_item(agent.module_accessor, 0) {
                ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
            }
            ItemModule::have_item(agent.module_accessor,ItemKind(*ITEM_KIND_LINKARROW), 0, 0, true, true);
        }
        agent.sub_shift_status_main(L2CValue::Ptr(special_n_cancel_status_main_loop as *const () as _))
}
pub unsafe fn special_n_cancel_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        let motion;
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_FRONT));
            motion = Hash40::new("special_n_cancel");
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            motion = Hash40::new("special_air_n_cancel");
        }
        MotionModule::change_motion_inherit_frame(agent.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    false.into()

}
unsafe extern "C" fn special_n_cancel_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Exit, agent, *FIGHTER_STATUS_KIND_SPECIAL_N)(agent);
    0.into()
}
////motion
//cancel
// unsafe extern "C" fn special_n_cancel_game(agent: &mut L2CAgentBase) {}
// unsafe extern "C" fn special_n_cancel_eff(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn special_n_cancel_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_link_special_n01"));
    }
}
unsafe extern "C" fn special_n_cancel_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), false, 18.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, -1.7);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_status_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_status_exit);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_status_end);
    //cancel
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_N_CANCEL, special_n_cancel_status_end);
    ////motion
    // agent.game_acmd("game_specialncancel", special_n_cancel_game, Priority::High);
    // agent.effect_acmd("effect_specialncancel", special_n_cancel_eff, Priority::High);
    agent.sound_acmd("sound_specialncancel", special_n_cancel_snd, Priority::High);
    agent.expression_acmd("expression_specialncancel", special_n_cancel_exp, Priority::High);
    // agent.game_acmd("game_specialairncancel", special_n_cancel_game, Priority::High);
    // agent.effect_acmd("effect_specialairncancel", special_n_cancel_eff, Priority::High);
    agent.sound_acmd("sound_specialairncancel", special_n_cancel_snd, Priority::High);
    agent.expression_acmd("expression_specialairncancel", special_n_cancel_exp, Priority::High);
}