use crate::imports::*;
use crate::link::consts::status_kind_ex::*;


//shield-surf input
unsafe extern "C" fn guard_on_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    //shield-surf input
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START.into(), false.into());
        0.into()
    }else {
        smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_GUARD_ON)(agent)
    }
}
unsafe extern "C" fn guard_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    //shield-surf input
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START.into(), false.into());
        0.into()
    }else {
        smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_GUARD)(agent)
    }
}
//parry
unsafe extern "C" fn guard_off_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    }
}
unsafe extern "C" fn just_shield_off_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_shieldguard"));
    }
}


pub fn install(agent: &mut smashline::Agent) {
    //shield
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_status_exec);
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD, guard_status_exec);
    //parry
    agent.game_acmd("game_guardoff", guard_off_game, Priority::High);
    agent.sound_acmd("sound_justshieldoff", just_shield_off_snd, Priority::High);
}