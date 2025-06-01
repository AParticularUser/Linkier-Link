use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


////status
//air-dodge
//fixing shield visibility not switching when grabbing an item
unsafe extern "C" fn escape_air_status_init(agent: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(agent.module_accessor, 0) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
    }else {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
    }
    smashline::original_status(Init, agent, *FIGHTER_STATUS_KIND_ESCAPE_AIR)(agent)
}
unsafe extern "C" fn escape_air_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    //shield-surf input
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) 
    && agent.global_table[global_table::STATUS_FRAME].get_i32() <= param::LINK_INT_SHIELD_SURF_FORGIVENESS 
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START.into(), false.into());
    }
    //fix shield visibility
    if ItemModule::is_have_item(agent.module_accessor, 0) 
    && !VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //air-dodge
    agent.status(Init, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_status_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ESCAPE_AIR, escape_air_status_exec);
}