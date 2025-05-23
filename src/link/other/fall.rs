use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::{
    status_kind_ex::*,
    *
};


unsafe extern "C" fn fall_special_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let status_kind_prev = agent.global_table[global_table::PREV_STATUS_KIND].get_i32();
    //allow for easer down-air input out of up-special
    if agent.global_table[global_table::STATUS_FRAME].get_i32() <= param::LINK_INT_SPECIAL_HI_CANCEL_FORGIVENESS 
    && ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
    && status_kind_prev != FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL 
    && status_kind_prev != FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2 {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL.into(), false.into());
    }
    //flick stick up to briefly use glider again
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    let flick_frame = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("jump_flick_y"));
    let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
    if status_kind_prev != FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2
    && stick_y >= stick_y_tilt
    && flick_count <= flick_frame {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
        return true.into()
    }     
    smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_FALL_SPECIAL)(agent)
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_FALL_SPECIAL, fall_special_status_exec);
}