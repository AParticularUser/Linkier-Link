use crate::imports::*;
use crate::common::funcs::*;
use crate::link::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


//change motion depending on aerial used out of up-special
unsafe extern "C" fn landing_fall_special_status_init(agent: &mut L2CFighterCommon) -> L2CValue {
    let air_motion = MotionModule::motion_kind(agent.module_accessor);
    WorkModule::set_int64(agent.module_accessor, air_motion as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    smashline::original_status(Init, agent, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(agent)
}
unsafe extern "C" fn landing_fall_special_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(agent);
    if VarModule::is_flag(agent.module_accessor, instance::LINK_FLAG_SPECIAL_HI_CANCEL_LANDING) {
        let air_motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
        let landing_motion;
        if air_motion == smash::hash40("attack_air_n") {
            landing_motion = "landing_air_n";
        }else if air_motion == smash::hash40("attack_air_f") {
            landing_motion = "landing_air_f";
        }else if air_motion == smash::hash40("attack_air_b") {
            landing_motion = "landing_air_b";
        }else if air_motion == smash::hash40("attack_air_hi") {
            landing_motion = "landing_air_hi";
        }else if air_motion == smash::hash40("attack_air_lw") {
            landing_motion = "landing_air_lw";
        }else {
            landing_motion = "landing_fall_special";
        }
        let rate = MotionModule::rate(agent.module_accessor);
        MotionModule::change_motion(agent.module_accessor, Hash40::new(landing_motion), 0.0, rate, false, 0.0, false, false);
    }
    false.into()
}
//sudo hit-grab for ending-blow
unsafe extern "C" fn landing_fall_special_check_attack(agent: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash_rs::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
    let opponent_boma = sv_battle_object::module_accessor(object_id);
    let opponent_status = StatusModule::status_kind(opponent_boma);
    if MotionModule::motion_kind(agent.module_accessor) == hash40("landing_air_lw")
    && category == *BATTLE_OBJECT_CATEGORY_FIGHTER 
    && collision_kind == *COLLISION_KIND_HIT 
    && DamageModule::damage(opponent_boma, 0) >= param::LINK_FLOAT_FINISHING_BLOW_DAMAGE_MIN
    && [
        *FIGHTER_STATUS_KIND_DOWN,
        *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_WAIT,
        *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
        *FIGHTER_STATUS_KIND_DOWN_EAT,
        *FIGHTER_STATUS_KIND_DOWN_SPOT,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_REFLECT_LR//,
        // *FIGHTER_STATUS_KIND_DOWN_STAND,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        // *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK
    ].contains(&opponent_status) {
        let is_facing_up = WorkModule::is_flag(opponent_boma, *FIGHTER_STATUS_DOWN_FLAG_UP);
        let opponent_pos = PostureModule::pos(opponent_boma);
        let attacker_pos = PostureModule::pos(agent.module_accessor);
        PostureModule::set_pos(agent.module_accessor, &Vector3f{x:((*attacker_pos).x +(*opponent_pos).x)/2.0, y:(*attacker_pos).y, z:(*attacker_pos).z});
        LinkModule::link(opponent_boma,  *LINK_NO_CAPTURE, agent.battle_object_id);
        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_THROWN, false);
        agent.change_status(FIGHTER_LINK_STATUS_KIND_FINISHING_BLOW.into(), false.into());
        let motion;
        if is_facing_up {
            motion = hash40("down_wait_u");
        }else {
            motion = hash40("down_wait_d");
        }
        WorkModule::set_int64(opponent_boma, motion as i64, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND);
        LinkModule::set_model_constraint_pos_ort(
            opponent_boma, 
            *LINK_NO_CAPTURE, 
            Hash40::new("top"), 
            Hash40::new("throw"), 
            (*CONSTRAINT_FLAG_NO_FLIP|*CONSTRAINT_FLAG_ORIENTATION|*CONSTRAINT_FLAG_POSITION|*CONSTRAINT_FLAG_OFFSET_ROT|*CONSTRAINT_FLAG_OFFSET_TRANSLATE) as u32, 
            true
        );
        if PostureModule::lr(agent.module_accessor) == PostureModule::lr(opponent_boma) {
            LinkModule::set_constraint_rot_offset(opponent_boma, &Vector3f{x:0.0, y:0.0, z:0.0});
        }else {
            LinkModule::set_constraint_rot_offset(opponent_boma, &Vector3f{x:0.0, y:180.0, z:0.0});
        }
    }
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_status_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_status_main);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_check_attack);
}