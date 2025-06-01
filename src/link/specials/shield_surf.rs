use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


////status
//start
unsafe extern "C" fn shield_surf_start_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn shield_surf_start_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_start"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_air_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    //kinetic stuff
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let lr = PostureModule::lr(agent.module_accessor);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX, 0.0);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, (speed_x*lr).clamp(param::LINK_FLOAT_SHIELD_SURF_START_X, 999.0)*lr, 0.0);
    sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_ACCEL_MAX, 0.0);
    sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_AIR, 0.0);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
    agent.sub_shift_status_main(L2CValue::Ptr(shield_surf_start_status_main_loop as *const () as _))
}
pub unsafe fn shield_surf_start_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP.into(), false.into());
        return true.into()
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        let motion;
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.set_situation(SITUATION_KIND_GROUND.into());
            motion = Hash40::new("shield_surf_start");
        }else {
            agent.set_situation(SITUATION_KIND_AIR.into());
            motion = Hash40::new("shield_surf_air_start");
        }
        MotionModule::change_motion_inherit_frame(agent.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    //hop
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP) {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
        sv_kinetic_energy!(add_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SHIELD_SURF_START_Y);
    }
    false.into()
}
unsafe extern "C" fn shield_surf_start_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//loop
unsafe extern "C" fn shield_surf_loop_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn shield_surf_loop_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_loop"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(agent.module_accessor, Hash40::new("shield_surf_break"), 0.0, 1.0, false, 0.0);
        MotionModule::set_weight(agent.module_accessor, 1.0, false);
        macros::PLAY_STATUS(agent, Hash40::new("se_link_special_h03"));
        //kinetic stuff
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    }else {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_air_loop"), 0.0, 1.0, false, 0.0, false, false);
        //kinetic stuff
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_AIR, 0.0);
    }
    //kinetic stuff
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX*2.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    VarModule::set_float(agent.module_accessor, status::LINK_FLOAT_SHIELD_SURF_LOOP_SLOPE, 0.0);
    agent.sub_shift_status_main(L2CValue::Ptr(shield_surf_loop_status_main_loop as *const () as _))
}
pub unsafe fn shield_surf_loop_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(agent.module_accessor);
    //end
    if ( (
            KineticModule::get_sum_speed(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)*lr < 0.05
            || GroundModule::is_touch(agent.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) 
        ) && agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    ) || ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END.into(), false.into());
        return true.into()
    }
    //up-special cancel
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_y >= stick_y_tilt {
        agent.set_situation(SITUATION_KIND_AIR.into());
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return true.into()
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.set_situation(SITUATION_KIND_GROUND.into());
            MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("shield_surf_loop"), -1.0, 1.0, 0.0, false, false);
            MotionModule::add_motion_2nd(agent.module_accessor, Hash40::new("shield_surf_break"), 0.0, 1.0, false, 0.0);
            MotionModule::set_weight(agent.module_accessor, 1.0, false);
            macros::PLAY_STATUS(agent, Hash40::new("se_link_special_h03"));

            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        }else {
            agent.set_situation(SITUATION_KIND_AIR.into());
            MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("shield_surf_air_loop"), -1.0, 1.0, 0.0, false, false);
            macros::STOP_SE(agent, Hash40::new("se_link_special_h03"));

            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_AIR, 0.0);
        }
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX*2.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    }
    //attack 
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK.into(), false.into());
        return true.into()
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        //jump
        if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_JUMP.into(), false.into());
            return true.into()
        }

        //get slope
        let touch_normal_x = GroundModule::get_touch_normal(agent.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32).x;
        let slope_angle = touch_normal_x.to_degrees()*lr;
        let slope_max = param::LINK_FLOAT_SHIELD_SURF_LOOP_SLOPE_MAX;
        let slope_ratio = slope_angle.abs().clamp(0.0, slope_max)/slope_max;
        println!("slope_angle: {}", slope_angle);
        println!("slope_ratio: {}", slope_ratio);

        //set motion weight
        let stick_x = ControlModule::get_stick_x(agent.module_accessor)*lr;
        let stick_ratio = (stick_x+1.0)/2.0;
        println!("stick_ratio: {}", stick_ratio);
        let weight_prev = MotionModule::weight(agent.module_accessor);
        let weight_new = 1.0-(((1.0-weight_prev)+(stick_x*-1.0).clamp(0.0, 1.0))/2.0);
        MotionModule::set_weight(agent.module_accessor, weight_new, false);

        //rotate to match slope 
        let slope_prev = VarModule::get_float(agent.module_accessor, status::LINK_FLOAT_SHIELD_SURF_LOOP_SLOPE);
        let slope_new = slope_angle+(slope_prev-slope_angle)/2.0;
        VarModule::set_float(agent.module_accessor, status::LINK_FLOAT_SHIELD_SURF_LOOP_SLOPE, slope_new);
        ModelModule::set_joint_rotate(
            agent.module_accessor, 
            Hash40::new("rot"), 
            &Vector3f{x:slope_new, y:0.0, z:0.0}, 
            MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, 
            MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8}
        );

        ////kinetic stuff
        //calc stable speed
        let stable_speed;
        if slope_angle >= 0.0 {
            stable_speed= param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX*slope_ratio*stick_ratio;
        } else {
            stable_speed = 0.0;
        }
        println!("stable_speed: {}", stable_speed);

        //calc accel/brake
        let accel_add;
        let brake_add;
        if slope_angle >= 0.0 {
            let accel_range = param::LINK_FLOAT_SHIELD_SURF_LOOP_ACCEL_MAX-param::LINK_FLOAT_SHIELD_SURF_LOOP_ACCEL_MIN;
            let accel_mul = (slope_ratio+stick_ratio)/2.0;
            accel_add = param::LINK_FLOAT_SHIELD_SURF_LOOP_ACCEL_MIN+(accel_range*accel_mul);
            
            let brake_range = param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MAX-param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MIN;
            let brake_mul = (1.0-stick_ratio)/2.0;
            brake_add = param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MIN+(brake_range*brake_mul);
        }else {
            accel_add = 0.0;

            let brake_range = param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MAX-param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MIN;
            let brake_mul = (slope_ratio+stick_ratio)/2.0;
            brake_add = param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_GROUND_MIN+(brake_range*brake_mul);
        }
        println!("accel_add: {}", accel_add); 
        println!("brake_add: {}", brake_add);

        //set speed
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let mut speed_x = sv_kinetic_energy::get_speed_x(agent.lua_state_agent);
        speed_x *= lr;
        if speed_x > stable_speed {
            speed_x = (speed_x-brake_add).clamp(stable_speed, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX*2.0);
        }else if speed_x < stable_speed {
            speed_x = (speed_x+accel_add).clamp(-param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX*2.0, stable_speed);
        }
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x*lr, 0.0);
        println!("speed_x: {}", speed_x*lr);
    }
    false.into()
}
unsafe extern "C" fn shield_surf_loop_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//attack
unsafe extern "C" fn shield_surf_attack_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn shield_surf_attack_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_attack"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_air_attack"), 0.0, 1.0, false, 0.0, false, false);
    }
    //kinetic stuff
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX, 0.0);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_AIR, 0.0);
    agent.sub_shift_status_main(L2CValue::Ptr(shield_surf_attack_status_main_loop as *const () as _))
}
pub unsafe fn shield_surf_attack_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END.into(), false.into());
        }else {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP.into(), false.into());
        }
        return true.into()
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.set_situation(SITUATION_KIND_GROUND.into());
            MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("shield_surf_attack"), -1.0, 1.0, 0.0, false, false);
        }else {
            agent.set_situation(SITUATION_KIND_AIR.into());
            MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("shield_surf_air_attack"), -1.0, 1.0, 0.0, false, false);
        }
    }
    false.into()
}
unsafe extern "C" fn shield_surf_attack_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//jump
unsafe extern "C" fn shield_surf_jump_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn shield_surf_jump_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_jump"), 0.0, 1.0, false, 0.0, false, false);
    //kinetic stuff
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX, 0.0);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SHIELD_SURF_LOOP_BRAKE_AIR, 0.0);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
    agent.sub_shift_status_main(L2CValue::Ptr(shield_surf_jump_status_main_loop as *const () as _))
}
pub unsafe fn shield_surf_jump_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) 
    || (
        agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
        && agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    ) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP.into(), false.into());
        return true.into()
    }
    //cancel
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END.into(), false.into());
        return true.into()
    }
    //up-special cancel
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_y >= stick_y_tilt {
        agent.set_situation(SITUATION_KIND_AIR.into());
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return true.into()
    }
    //attack 
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK.into(), false.into());
        return true.into()
    }
    //hop
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP) {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SHIELD_SURF_JUMP_Y);
    }
    false.into()
}
unsafe extern "C" fn shield_surf_jump_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//end
unsafe extern "C" fn shield_surf_end_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn shield_surf_end_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_end"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("shield_surf_air_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    agent.sub_shift_status_main(L2CValue::Ptr(shield_surf_end_status_main_loop as *const () as _))
}
pub unsafe fn shield_surf_end_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
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
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            motion = Hash40::new("shield_surf_end");
        }else {
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            motion = Hash40::new("shield_surf_air_end");
        }
        MotionModule::change_motion_inherit_frame(agent.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    false.into()
}
unsafe extern "C" fn shield_surf_end_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
////motion
//start
unsafe extern "C" fn shield_surf_start_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
    }   
}
unsafe extern "C" fn shield_surf_start_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}
unsafe extern "C" fn shield_surf_start_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_jump"));
    }
}
unsafe extern "C" fn shield_surf_start_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}
//loop
// unsafe extern "C" fn shield_surf_loop_game(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn shield_surf_loop_eff(agent: &mut L2CAgentBase) {
    loop{
        if macros::is_excute(agent) {
            let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
            let size = (speed_x/param::LINK_FLOAT_SHIELD_SURF_LOOP_SPEED_STABLE_MAX)*1.9;
            macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, size, 0, 0, 0, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 7.0);
    }
}
// unsafe extern "C" fn shield_surf_loop_snd(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn shield_surf_loop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}
//attack
unsafe extern "C" fn shield_surf_attack_game(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 30, 95, 0, 75, 4.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SPIN);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 30, 95, 0, 70, 4.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SPIN);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 30, 95, 0, 65, 4.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SPIN);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn shield_surf_attack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn shield_surf_air_attack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 115, 0, 0.7, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
}
unsafe extern "C" fn shield_surf_attack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_jump"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_07"));
    }
}
unsafe extern "C" fn shield_surf_attack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}
//jump
unsafe extern "C" fn shield_surf_jump_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SHIELD_SURF_HOP);
    }
}
unsafe extern "C" fn shield_surf_jump_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}
unsafe extern "C" fn shield_surf_jump_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_jump"));
    }
}
unsafe extern "C" fn shield_surf_jump_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}
//end
unsafe extern "C" fn shield_surf_end_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}
unsafe extern "C" fn shield_surf_end_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}
unsafe extern "C" fn shield_surf_end_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_link_landing01"));
    }
}
unsafe extern "C" fn shield_surf_end_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //start
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START, shield_surf_start_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START, shield_surf_start_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START, shield_surf_start_status_end);
    //loop
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP, shield_surf_loop_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP, shield_surf_loop_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP, shield_surf_loop_status_end);
    //attack
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK, shield_surf_attack_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK, shield_surf_attack_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK, shield_surf_attack_status_end);
    //jump
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_JUMP, shield_surf_jump_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_JUMP, shield_surf_jump_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_JUMP, shield_surf_jump_status_end);
    //end
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END, shield_surf_end_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END, shield_surf_end_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END, shield_surf_end_status_end);
    ////motion
    //start
    agent.game_acmd("game_shieldsurfstart", shield_surf_start_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfstart", shield_surf_start_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfstart", shield_surf_start_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfstart", shield_surf_start_exp, Priority::High);
    // agent.game_acmd("game_shieldsurfairstart", shield_surf_start_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfairstart", shield_surf_start_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfstart", shield_surf_start_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfairstart", shield_surf_start_exp, Priority::High);
    //loop
    // agent.game_acmd("game_shieldsurfloop", shield_surf_loop_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfloop", shield_surf_loop_eff, Priority::High);
    // agent.sound_acmd("sound_shieldsurfloop", shield_surf_loop_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfloop", shield_surf_loop_exp, Priority::High);
    // agent.game_acmd("game_shieldsurfairloop", shield_surf_loop_game, Priority::High);
    // agent.effect_acmd("effect_shieldsurfairloop", shield_surf_loop_eff, Priority::High);
    // agent.sound_acmd("sound_shieldsurfairloop", shield_surf_loop_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfairloop", shield_surf_loop_exp, Priority::High);
    //attack
    agent.game_acmd("game_shieldsurfattack", shield_surf_attack_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfattack", shield_surf_attack_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfattack", shield_surf_attack_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfattack", shield_surf_attack_exp, Priority::High);
    agent.game_acmd("game_shieldsurfairattack", shield_surf_attack_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfairattack", shield_surf_air_attack_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfairattack", shield_surf_attack_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfairattack", shield_surf_attack_exp, Priority::High);
    //jump
    agent.game_acmd("game_shieldsurfjump", shield_surf_jump_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfjump", shield_surf_jump_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfjump", shield_surf_jump_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfjump", shield_surf_jump_exp, Priority::High);
    //end
    agent.game_acmd("game_shieldsurfend", shield_surf_end_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfend", shield_surf_end_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfend", shield_surf_end_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfend", shield_surf_end_exp, Priority::High);
    // agent.game_acmd("game_shieldsurfairend", shield_surf_end_game, Priority::High);
    agent.effect_acmd("effect_shieldsurfairend", shield_surf_end_eff, Priority::High);
    agent.sound_acmd("sound_shieldsurfairend", shield_surf_end_snd, Priority::High);
    agent.expression_acmd("expression_shieldsurfairend", shield_surf_end_exp, Priority::High);
}