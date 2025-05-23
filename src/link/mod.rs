use crate::imports::*;
// use crate::common::funcs::*;
use crate::common::consts::*;
use crate::link::consts::vars::*;


unsafe extern "C" fn link_main(agent: &mut L2CFighterCommon) {
    println!("============== new frame: main ==============");
    // println!("l_diff: {}", SlopeModule::floor_diff_l(agent.module_accessor));
    // println!("r_diff: {}", SlopeModule::floor_diff_r(agent.module_accessor));
    // let touch_normal = GroundModule::get_touch_normal(agent.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    // println!("touch_normal_x: {}", touch_normal.x);
    // println!("touch_normal_y: {}", touch_normal.y);
    
    //reset flag if KO'ed
    if VarModule::is_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED) {
        if StatusModule::status_kind(agent.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
            VarModule::off_flag(agent.module_accessor, instance::LINK_FLAG_SKYWARD_STRIKE_IS_CHARGED);
        }
        // let effect = VarModule::get_int(agent.module_accessor, instance::LINK_INT_SKYWARD_STRIKE_EFFECT_HANDLE);
        // if !sword_is_visible 
        // && effect_is_visible {
        //     EffectModule::set_visible(agent.module_accessor, effect as u32, false);
        // }
        // if sword_is_visible 
        // && !effect_is_visible {
        //     EffectModule::set_visible(agent.module_accessor, effect as u32, true);
        // }
    }

    //is actionable
    if (
        WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) 
        || WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) 
        || WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) 
    ) && ItemModule::is_have_item(agent.module_accessor, 0) {
        //special-lw input
        let stick_x = ControlModule::get_stick_x(agent.module_accessor);
        let stick_y = ControlModule::get_stick_y(agent.module_accessor);
        let special_stick_x = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
        let special_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
        let squat_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("squat_stick_y"));
        let special_button = ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        // let special_n = agent.global_table[global_table::CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0;
        let special_s = agent.global_table[global_table::CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0;
        let special_lw = agent.global_table[global_table::CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
        let is_squat = agent.global_table[global_table::STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_WAIT;
        // println!("stick_x: {}", stick_x);
        // println!("stick_y: {}", stick_y);
        // println!("special_stick_x: {}", special_stick_x);
        // println!("special_stick_y: {}", special_stick_y);
        // println!("squat_stick_y: {}", squat_stick_y);
        // println!("BUTTON_SPECIAL: {}", special_button);
        // // println!("special_n: {}", special_n);
        // println!("special_s: {}", special_s);
        // println!("special_lw: {}", special_lw);
        // println!("is_squat: {}", is_squat);
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
            //drop item & change status
            if ArticleModule::is_generatable(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB) 
            && !WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
                ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
                agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
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