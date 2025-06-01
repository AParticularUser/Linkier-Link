use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::vars::*;


unsafe extern "C" fn link_main(agent: &mut L2CFighterCommon) {
    //reset flag if KO'ed
    if VarModule::is_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
            VarModule::off_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED);
        }
    }
    //spawn bomb even if holding item
    if !WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) 
    && ItemModule::is_have_item(agent.module_accessor, 0) {
        //is actionable
        let is_actionable;
        if WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) 
        || WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) 
        || WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) 
        {
            is_actionable = true;
        }else{
            is_actionable = false;
        }
        //is special-lw input
        let stick_x = ControlModule::get_stick_x(agent.module_accessor);
        let stick_y = ControlModule::get_stick_y(agent.module_accessor);
        let special_stick_x = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
        let special_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
        let squat_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("squat_stick_y"));
        let special_button = ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        let special_s = agent.global_table[global_table::CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0;
        let special_lw = agent.global_table[global_table::CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
        let is_squat = agent.global_table[global_table::STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_WAIT;
        let is_special_lw_input;
        if !special_s 
        && special_lw 
        || (
            special_button
            && is_squat 
            && stick_y <= squat_stick_y
        ) 
        || (
            special_button
            && !is_squat 
            && stick_x.abs() <= special_stick_x
            && stick_y <= special_stick_y*-1.0 
        ) {
            is_special_lw_input = true;
        }else {
            is_special_lw_input = false;
        }
        //change status
        if is_special_lw_input 
        && is_actionable {
            let bomb_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
            //drop item
            if ArticleModule::is_generatable(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB) {
                ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
                agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            //throw bomb
            }else if ItemModule::get_have_item_id(agent.module_accessor, 0) == bomb_id as u64  {
                agent.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            }
        }
    }
}


pub mod consts;
pub mod funcs;
mod normals;
mod specials;
mod other;

pub fn install() {
    let agent = &mut smashline::Agent::new("link");
    // agent.on_start(link_init);
    agent.on_line(Main, link_main);
    normals::install(agent);
    specials::install(agent);
    other::install(agent);
    agent.install();
}