use crate::imports::*;
use crate::common::consts::*;
use crate::link::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


////status
//start
unsafe extern "C" fn special_hi_start_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_start_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_start_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(agent.module_accessor) {
        if MotionModule::is_end(agent.module_accessor) {
            if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(), false.into());
            }else {
                agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
            }
            return true.into()
        }
        if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into()
        }
    }
    false.into()
}
//hold
unsafe extern "C" fn special_hi_hold_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    let hold_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame"));
    WorkModule::set_int(agent.module_accessor, hold_frame, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_hold_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_hold_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //jump-cancel
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        agent.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        return true.into()
    }
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND 
    || ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    || WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) <= 0 {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return true.into()
    }
    WorkModule::dec_int(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    false.into()
}
//end
unsafe extern "C" fn special_hi_end_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        motion = "special_hi";
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }else {
        //note: need to fix shieldb bone desyncing for certain animations
        // let status_prev = agent.global_table[global_table::PREV_STATUS_KIND].get_i32();
        // if status_prev == FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_START
        // || status_prev == FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_LOOP
        // || status_prev == FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_JUMP
        // || status_prev == FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_ATTACK
        // // || status_prev == FIGHTER_LINK_STATUS_KIND_SHIELD_SURF_END 
        // {
        //     motion = "special_air_hi_shield";
        // }else {
            motion = "special_air_hi";
        // }
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    MotionModule::change_motion(agent.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_RISE);
    VarModule::set_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_END_RISE_COUNT, 0);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_ENABLE_CANCEL);
    VarModule::set_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME, 0);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_end_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_end_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return true.into()
    }
    //ledge-grab
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_ENABLE_CANCEL) {
        //attack-cancel
        if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL.into(), false.into());
            return true.into()
        }
        //flick stick down to end up-special
        let flick_frame = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
        let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
        let stick_y = ControlModule::get_stick_y(agent.module_accessor);
        let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("dive_cont_value"));
        if flick_frame >= flick_count 
        && stick_y <= stick_y_tilt {
            agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return true.into()
        }
        //bomb drop
        if VarModule::get_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME) <= 0 {
            let stick_y = ControlModule::get_stick_y(agent.module_accessor);
            let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
            if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            && stick_y <= stick_y_tilt 
            {
                special_hi_bomb_drop(agent);
            }
        }else {
            VarModule::dec_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME);
        }
    }
    //rise
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_RISE) {
        //start
        if VarModule::get_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_END_RISE_COUNT) <= 0 {
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_RISE_STABLE_Y);
            sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_RISE_STABLE_Y);
            sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_RISE_ACCEL_Y);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, param::LINK_FLOAT_SPECIAL_HI_RISE_BRAKE_X, 0.0);
        }else {
            //end
            if VarModule::get_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_END_RISE_COUNT) >= param::LINK_INT_SPECIAL_HI_RISE_FRAME {
                VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_RISE);
                sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
                sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
                sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_ACCEL_Y);
                sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_BRAKE_Y);
            }
        }
        //timer
        VarModule::inc_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_END_RISE_COUNT);
    }
    false.into()
}
unsafe extern "C" fn special_hi_end_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_FALL {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    false.into()
}
pub unsafe fn special_hi_bomb_drop(agent: &mut L2CFighterCommon) -> bool {
    if ArticleModule::is_generatable(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB) {
        //spawn and drop bomb
        VarModule::set_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME, param::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE);
        macros::PLAY_SE(agent, Hash40::new("se_link_special_l01"));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB, false, 0);
        let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        LinkModule::link(article_boma,*WEAPON_LINK_NO_CONSTRAINT, agent.global_table[global_table::OBJECT_ID].get_u32());
        LinkModule::set_model_constraint_pos_ort(article_boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("top"), (*CONSTRAINT_FLAG_NO_FLIP|*CONSTRAINT_FLAG_ORIENTATION|*CONSTRAINT_FLAG_POSITION|*CONSTRAINT_FLAG_OFFSET_ROT|*CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, true);
        LinkModule::unlink(article_boma,*WEAPON_LINK_NO_CONSTRAINT);
        let pos_x = PostureModule::pos_x(agent.module_accessor);
        let pos_y = PostureModule::pos_y(agent.module_accessor);
        PostureModule::set_pos_2d(article_boma, &Vector2f{x:pos_x, y:pos_y +param::LINK_FLOAT_SPECIAL_HI_BOMBDROP_OFFSET_Y});
        //makes bomb act like it was thrown
        let team_no = TeamModule::team_no(agent.module_accessor) as i32;
        let team_owner_id = TeamModule::team_owner_id(agent.module_accessor) as u32;
        TeamModule::set_team(article_boma, team_no, true);
        TeamModule::set_team_owner_id(article_boma, team_owner_id);
        StatusModule::change_status_force(article_boma, *ITEM_STATUS_KIND_FALL, false);
        return true
    }
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB) {
        let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        if StatusModule::status_kind(article_boma) == *ITEM_STATUS_KIND_THROW 
        || StatusModule::status_kind(article_boma) == *ITEM_STATUS_KIND_FALL
        || StatusModule::status_kind(article_boma) == *ITEM_STATUS_KIND_LANDING {
            //start bomb timer
            VarModule::set_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME, param::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE);
            WorkModule::set_int(article_boma,param::LINK_INT_SPECIAL_HI_BOMBDROP_BLAST,  *ITEM_INSTANCE_WORK_INT_BOMB_COUNTER);
            WorkModule::on_flag(article_boma,  *ITEM_LINKBOMB_INSTANCE_WORK_FLAG_IS_BLAST);
            WorkModule::off_flag(article_boma,  *ITEM_INSTANCE_WORK_FLAG_EATABLE);
            MotionModule::change_motion(article_boma, Hash40::new("flash"), 0.0, 1.0, false, 0.0, false, false);
            return true
        }
        if ItemModule::is_have_item(agent.module_accessor, 0) {
            if ItemModule::get_have_item_id(agent.module_accessor, 0) == object_id as u64 {
                //drop bomb if holding
                VarModule::set_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME, param::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE);
                ItemModule::drop_item(agent.module_accessor, 0.0, 0.0, 0);
                let pos_x = PostureModule::pos_x(agent.module_accessor);
                let pos_y = PostureModule::pos_y(agent.module_accessor);
                PostureModule::set_pos_2d(article_boma, &Vector2f{x:pos_x, y:pos_y +param::LINK_FLOAT_SPECIAL_HI_BOMBDROP_OFFSET_Y});
                return true
            }
        }
    }
    return false
}
//fall
unsafe extern "C" fn special_hi_fall_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn special_hi_fall_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion_n;
    let motion_f;
    let motion_b;
    if MotionModule::motion_kind(agent.module_accessor) == hash40("special_air_hi_shield") {
        motion_n = "special_air_hi_shield_fall";
        motion_f = "special_air_hi_shield_fall_f";
        motion_b = "special_air_hi_shield_fall_b";
    }else {
        motion_n = "special_air_hi_fall";
        motion_f = "special_air_hi_fall_f";
        motion_b = "special_air_hi_fall_b";
    }
    WorkModule::set_int64(agent.module_accessor, hash40(motion_f) as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
    WorkModule::set_int64(agent.module_accessor, hash40(motion_b) as i64, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
    MotionModule::change_motion(agent.module_accessor, Hash40::new(motion_n), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::add_motion_2nd(agent.module_accessor, Hash40::new(motion_f), 0.0, 1.0, false, 1.0);
    //physics stuff
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let lr = PostureModule::lr(agent.module_accessor);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
    sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_ACCEL_Y);
    sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_BRAKE_Y);
    sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_X*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_X, 0.0);
    sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, param::LINK_FLOAT_SPECIAL_HI_FALL_ACCEL_X, 0.0);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_fall_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_fall_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //landing
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    //ledge-grab
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    //attack-cancel
    if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL.into(), false.into());
        return true.into()
    }
    //flick stick down to end up-special-fall early
    let flick_frame = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
    let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("dive_cont_value"));
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if flick_frame >= flick_count 
    && stick_y < stick_y_tilt {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    //bomb drop
    if VarModule::get_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME) <= 0 {
        if ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        {
            special_hi_bomb_drop(agent);
        }
    }else {
        VarModule::dec_int(agent.module_accessor, instance::LINK_INT_SPECIAL_HI_BOMBDROP_DISABLE_FRAME);
    }
    ////control-stick based motion-blending
    let blend_frames = 25.0;
    //invert weight
    let motion_f = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_F);
    let motion_b = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_FALL_WORK_INT_MOTION_KIND_B);
    let motion_2nd = MotionModule::motion_kind_2nd(agent.module_accessor);
    let weight_curr;
    if motion_2nd == motion_b {
        weight_curr = (1.0-MotionModule::weight(agent.module_accessor)) *-1.0;
    }else {
        weight_curr = 1.0-MotionModule::weight(agent.module_accessor);
    }
    //update weight 
    //change motion if drifting in opposite direction
    let lr = PostureModule::lr(agent.module_accessor);
    let stick_x = ControlModule::get_stick_x(agent.module_accessor)*lr;
    if stick_x != weight_curr {
        let weight_new;
        if stick_x > weight_curr {
            weight_new = (weight_curr + (1.0/blend_frames)).clamp(-1.0, stick_x);
            if weight_curr <= 0.0
            && weight_new > 0.0 {
                MotionModule::add_motion_2nd(agent.module_accessor, Hash40::new_raw(motion_f), 0.0, 1.0, false, 0.0);
            }
        }else {
            weight_new = (weight_curr - (1.0/blend_frames)).clamp(stick_x, 1.0);
            if weight_curr >= 0.0
            && weight_new < 0.0 {
                MotionModule::add_motion_2nd(agent.module_accessor, Hash40::new_raw(motion_b), 0.0, 1.0, false, 0.0);
            }
        }
        MotionModule::set_weight(agent.module_accessor, 1.0-(weight_new.abs()), false);
    }
    false.into()
}
unsafe extern "C" fn special_hi_fall_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    macros::EFFECT_OFF_KIND(agent, Hash40::new("link_entry"), false, false);
    false.into()
}
//cancel
unsafe extern "C" fn special_hi_cancel_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = *FIGHTER_STATUS_KIND_ATTACK_AIR;
    smashline::original_status(Pre, agent, status_kind)(agent)
}
unsafe extern "C" fn special_hi_cancel_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.sub_attack_air();
    WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW2_BLANK_TIME);
    WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_HIT_COUNT);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_cancel_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_cancel_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //down-air-bounce stuff
    let hit_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_HIT_COUNT);
    let max_hit = WorkModule::get_param_int(agent.module_accessor, hash40("param_private"), hash40("air_lw_max_hit"));
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK) 
    && hit_count <= max_hit {
        let blank_time = WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW2_BLANK_TIME);
        if blank_time <= 0.0 {
            if AttackModule::is_infliction(agent.module_accessor, *COLLISION_KIND_MASK_HIT) 
            || AttackModule::is_infliction(agent.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                let max_blank_time = WorkModule::get_param_float(agent.module_accessor, hash40("param_private"), hash40("air_lw_blank_time"));
                WorkModule::set_float(agent.module_accessor, max_blank_time, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW2_BLANK_TIME);
                WorkModule::inc_int(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_HIT_COUNT);
                //call twice to match normal down-air-bounce hight
                if KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0 {
                    MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attackairlw2bound"), -1);
                }
                MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attackairlw2bound"), -1);
            }
        }else if !agent.global_table[global_table::IS_STOP].get_bool() {
            let rate = MotionModule::rate(agent.module_accessor);
            WorkModule::sub_float(agent.module_accessor, rate, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW2_BLANK_TIME);
            let new_blank_time = blank_time-rate;
            if new_blank_time <= 0.0 {
                AttackModule::clear_all(agent.module_accessor);
                MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attackairlw2attack"), -1);
                MotionAnimcmdModule::call_script_single(agent.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_attackairlw2attack"), -1);
            }
        } 
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return true.into()
    }else if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
            VarModule::on_flag(agent.module_accessor, instance::LINK_FLAG_SPECIAL_HI_CANCEL_LANDING);
        }else {
            VarModule::off_flag(agent.module_accessor, instance::LINK_FLAG_SPECIAL_HI_CANCEL_LANDING);
        }
        agent.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }else if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    false.into()
}
unsafe extern "C" fn special_hi_cancel_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = *FIGHTER_STATUS_KIND_ATTACK_AIR;
    smashline::original_status(End, agent, status_kind)(agent)
}
//2nd
unsafe extern "C" fn special_hi_2_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn special_hi_2_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_hi_2"), 0.0, 1.0, false, 0.0, false, false);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_2_BRAKE);
    VarModule::set_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_2_BRAKE_COUNT, 0);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_2_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_2_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //landing
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    //ledge-grab
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return true.into()
    }
    //flick stick down to end up-special-2 early
    let flick_frame = WorkModule::get_param_int(agent.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
    let flick_count = ControlModule::get_flick_no_reset_y(agent.module_accessor);
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("dive_cont_value"));
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if flick_frame >= flick_count 
    && stick_y <= stick_y_tilt {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    //brake
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_2_BRAKE) {
        //start
        if VarModule::get_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_2_BRAKE_COUNT) <= 0 {
            let lr = PostureModule::lr(agent.module_accessor);
            sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
            sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_Y);
            sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_ACCEL_Y);
            sv_kinetic_energy!(set_brake, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::LINK_FLOAT_SPECIAL_HI_FALL_BRAKE_Y);
            sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_X*lr, 0.0);
            sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, param::LINK_FLOAT_SPECIAL_HI_FALL_STABLE_X, 0.0);
        //end
        }else if VarModule::get_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_2_BRAKE_COUNT) >= param::LINK_INT_SPECIAL_HI_BRAKE_FRAME {
            VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_2_BRAKE);
            agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return true.into()
        }
        //timer
        VarModule::inc_int(agent.module_accessor, status::LINK_INT_SPECIAL_HI_2_BRAKE_COUNT);
    }
    false.into()
}
unsafe extern "C" fn special_hi_2_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    false.into()
}
////motion
//start
unsafe extern "C" fn special_hi_start_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 90, 100, 30, 0, 30.0, 0.0, 15.0, 0.0, Some(0.0), Some(35.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6/*rehit-rate*/, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true/*friendly-fire*/, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}
unsafe extern "C" fn special_hi_start_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.4);
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, 1, 5, 0, 5, 5, 5, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        for _ in 0..5 {
            macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.4, 30, 16, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
}
unsafe extern "C" fn special_hi_start_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_squat"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_special_s04"));
    }
}
unsafe extern "C" fn special_hi_start_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0/*id*/, 15/*intervul*/, 45/*angle*/, 400/*strength*/, 0.6, 0/*x-offset*/, 15/*y-offset*/, 40/*x-length*/, 30/*y-hight*/, 40);
        AREA_WIND_2ND_arg10(agent, 1/*id*/, 15/*intervul*/, 135/*angle*/, 400/*strength*/, 0.6, 0/*x-offset*/, 15/*y-offset*/, 40/*x-length*/, 30/*y-hight*/, 40);
    }
}
//hold
unsafe extern "C" fn special_hi_hold_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 90, 100, 30, 0, 30.0, 0.0, 15.0, 0.0, Some(0.0), Some(35.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6/*rehit-rate*/, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true/*friendly-fire*/, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}
unsafe extern "C" fn special_hi_hold_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    loop {
        if macros::is_excute(agent) {
            for _ in 0..1 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 10, 0, -90, 0, 0, 3, 0,0,0,0,0,0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.3);
            macros::LAST_EFFECT_SET_RATE(agent, 0.2);
            for _ in 0..1 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.7, 1, 5, 0, 5, 5, 5, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            for _ in 0..1 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
    }
}
unsafe extern "C" fn special_hi_hold_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let sound = SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_common_spirits_wind_loop"), true, false, false);
        SoundModule::set_se_vol(agent.module_accessor, sound as i32, 0.25, 0);
    }
}
unsafe extern "C" fn special_hi_hold_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AREA_WIND_2ND_arg10(agent, 0/*id*/, 5/*intervul*/, 90/*angle*/, 300/*strength*/, 0.6, 0/*x-offset*/, 25/*y-offset*/, 40/*x-length*/, 50/*y-hight*/, 40);
    }
}
//end
unsafe extern "C" fn special_hi_end_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi"), false, 1.0);
        let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        LinkModule::set_model_constraint_pos_ort(article_boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("haver"), Hash40::new("haver"), (*CONSTRAINT_FLAG_NO_FLIP|*CONSTRAINT_FLAG_ORIENTATION|*CONSTRAINT_FLAG_POSITION) as u32, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 90, 100, 30, 0, 30.0, 0.0, 0.0, 0.0, Some(0.0), Some(25.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6/*rehit-rate*/, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true/*friendly-fire*/, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_ENABLE_CANCEL);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn special_hi_end_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 6, 0, 5, 0, 0, 0, 1.5, false);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.7, 1, 5, 0, 5, 5, 5, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 10, 0, -90, 0, 0, 3, 0,0,0, 0,0,0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.3);
            macros::LAST_EFFECT_SET_RATE(agent, 0.2);
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
    }
}
unsafe extern "C" fn special_hi_end_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_special_s04"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appear02"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_appear01"));
    }
}
unsafe extern "C" fn special_hi_end_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AREA_WIND_2ND_arg10(agent, 0/*id*/, 5/*intervul*/, 90/*angle*/, 300/*strength*/, 0.6, 0/*x-offset*/, 25/*y-offset*/, 40/*x-length*/, 50/*y-hight*/, 40);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }  
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
        AREA_WIND_2ND_arg10(agent, 0/*id*/, 5/*intervul*/, 90/*angle*/, 300/*strength*/, 0.6, 0/*x-offset*/, 0/*y-offset*/, 40/*x-length*/, 50/*y-hight*/, 40);
    }  
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}
//end-air
unsafe extern "C" fn special_air_hi_end_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi"), false, 1.0);
        let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        LinkModule::set_model_constraint_pos_ort(article_boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("haver"), Hash40::new("haver"), (*CONSTRAINT_FLAG_NO_FLIP|*CONSTRAINT_FLAG_ORIENTATION|*CONSTRAINT_FLAG_POSITION) as u32, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 90, 100, 30, 0, 30.0, 0.0, 0.0, 0.0, Some(0.0), Some(25.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6/*rehit-rate*/, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true/*friendly-fire*/, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_END_ENABLE_CANCEL);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn special_air_hi_end_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.5, 1, 5, 0, 5, 5, 5, false);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        for _ in 0..5 {
            macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.4, 30, 16, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 6, 0, 5, 0, 0, 0, 1.5, false);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.7, 1, 5, 0, 5, 5, 5, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5, 10, 0, -90, 0, 0, 3, 0,0,0, 0,0,0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.3);
            macros::LAST_EFFECT_SET_RATE(agent, 0.2);
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            for _ in 0..2 {
                macros::EFFECT(agent, Hash40::new("sys_damage_aura"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 0.3, 26, 16, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 5.0, 10.0);
                macros::LAST_EFFECT_SET_RATE(agent, 0.5);
            }
        }
        wait(agent.lua_state_agent, 6.0);
    }
}
unsafe extern "C" fn special_air_hi_end_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_special_s04"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appear02"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_link_appear01"));
    }
}
unsafe extern "C" fn special_air_hi_end_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0/*id*/, 5/*intervul*/, 90/*angle*/, 300/*strength*/, 0.6, 0/*x-offset*/, 5/*y-offset*/, 40/*x-length*/, 50/*y-hight*/, 40);
    }   
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}
//fall
unsafe extern "C" fn special_hi_fall_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi_loop"), false, 1.0);
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
// unsafe extern "C" fn special_hi_fall_eff(agent: &mut L2CAgentBase) {}
unsafe extern "C" fn special_hi_fall_snd(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            let sound = SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_link_appear01"), true, false, false);
            SoundModule::set_se_vol(agent.module_accessor, sound as i32, 1.2, 0);
        }
        wait(agent.lua_state_agent, 35.0);
    }
}
unsafe extern "C" fn special_hi_fall_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//2nd
unsafe extern "C" fn special_hi_2_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_SPECIAL_HI_2_BRAKE);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi"), false, 1.0);
        let article = ArticleModule::get_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        LinkModule::set_model_constraint_pos_ort(article_boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("haver"), Hash40::new("haver"), (*CONSTRAINT_FLAG_NO_FLIP|*CONSTRAINT_FLAG_ORIENTATION|*CONSTRAINT_FLAG_POSITION) as u32, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 25.0);
    macros::FT_MOTION_RATE(agent, 2.0);
}
unsafe extern "C" fn special_hi_2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 6, 0, 5, 0, 0, 0, 1.5, false);
    }
}
unsafe extern "C" fn special_hi_2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_appear02"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let sound = SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_link_appear01"), true, false, false);
        SoundModule::set_se_vol(agent.module_accessor, sound as i32, 1.2, 0);
    }
}
unsafe extern "C" fn special_hi_2_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
////parasail
unsafe extern "C" fn parasail_special_hi_loop_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    loop {
        if macros::is_excute(agent) {
            let fighter_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let fighter_boma = sv_battle_object::module_accessor(fighter_id);
            if MotionModule::weight(fighter_boma) < 0.2 {
                EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("rot"), 9.5, 0.04, 1.1, 0, 0, 0, 1, false);
                macros::LAST_EFFECT_SET_RATE(agent, 1.5);
                EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("rot"), -9.5, 0.04, 1.1, 0, 0, 0, 1, false);
                macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            }
        }
        wait(agent.lua_state_agent, 7.0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //start
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_start_status_main);
    //hold
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, special_hi_hold_status_main);
    //end
    agent.status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_status_main);
    agent.status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, special_hi_end_status_end);
    //fall
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_FALL, special_hi_fall_status_end);
    //cancel
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL, special_hi_cancel_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL, special_hi_cancel_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_CANCEL, special_hi_cancel_status_end);
    //2nd
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2, special_hi_2_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2, special_hi_2_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_2, special_hi_2_status_end);
    ////motion
    //start
    agent.game_acmd("game_specialhistart", special_hi_start_game, Priority::High);
    agent.effect_acmd("effect_specialhistart", special_hi_start_eff, Priority::High);
    agent.sound_acmd("sound_specialhistart", special_hi_start_snd, Priority::High);
    agent.expression_acmd("expression_specialhistart", special_hi_start_exp, Priority::High);
    //hold
    agent.game_acmd("game_specialhihold", special_hi_hold_game, Priority::High);
    agent.effect_acmd("effect_specialhihold", special_hi_hold_eff, Priority::High);
    agent.sound_acmd("sound_specialhihold", special_hi_hold_snd, Priority::High);
    agent.expression_acmd("expression_specialhihold", special_hi_hold_exp, Priority::High);
    //end
    agent.game_acmd("game_specialhi", special_hi_end_game, Priority::High);
    agent.effect_acmd("effect_specialhi", special_hi_end_eff, Priority::High);
    agent.sound_acmd("sound_specialhi", special_hi_end_snd, Priority::High);
    agent.expression_acmd("expression_specialhi", special_hi_end_exp, Priority::High);
    //end-air
    agent.game_acmd("game_specialairhi", special_air_hi_end_game, Priority::High);
    agent.effect_acmd("effect_specialairhi", special_air_hi_end_eff, Priority::High);
    agent.sound_acmd("sound_specialairhi", special_air_hi_end_snd, Priority::High);
    agent.expression_acmd("expression_specialairhi", special_air_hi_end_exp, Priority::High);
    //end-air-shield
    agent.game_acmd("game_specialairhishield", special_air_hi_end_game, Priority::High);
    agent.effect_acmd("effect_specialairhishield", special_air_hi_end_eff, Priority::High);
    agent.sound_acmd("sound_specialairhishield", special_air_hi_end_snd, Priority::High);
    agent.expression_acmd("expression_specialairhishield", special_air_hi_end_exp, Priority::High);
    //fall
    agent.game_acmd("game_specialairhifall", special_hi_fall_game, Priority::High);
    // agent.effect_acmd("effect_specialairhifall", special_hi_fall_eff, Priority::High);
    agent.sound_acmd("sound_specialairhifall", special_hi_fall_snd, Priority::High);
    agent.expression_acmd("expression_specialairhifall", special_hi_fall_exp, Priority::High);
    //fall-shield
    agent.game_acmd("game_specialairhishieldfall", special_hi_fall_game, Priority::High);
    // agent.effect_acmd("effect_specialairhishieldfall", special_hi_fall_eff, Priority::High);
    agent.sound_acmd("sound_specialairhishieldfall", special_hi_fall_snd, Priority::High);
    agent.expression_acmd("expression_specialairhishieldfall", special_hi_fall_exp, Priority::High);
    //2nd
    agent.game_acmd("game_specialairhi2", special_hi_2_game, Priority::High);
    agent.effect_acmd("effect_specialairhi2", special_hi_2_eff, Priority::High);
    agent.sound_acmd("sound_specialairhi2", special_hi_2_snd, Priority::High);
    agent.expression_acmd("expression_specialairhi2", special_hi_2_exp, Priority::High);
    ////parasail
    Agent::new("link_parasail")
    .effect_acmd("effect_specialhiloop", parasail_special_hi_loop_eff, Priority::High)
    .install();
}