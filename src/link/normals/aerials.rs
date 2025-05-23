use crate::imports::*;
use crate::common::{
    consts::{
        vars::status::*,
        *
    },
    funcs::*
};
use crate::link::{
    consts::{
        status_kind_ex::*,
        vars::*,
        *
    }//,
    // funcs::*
};


////status
//attack
unsafe extern "C" fn attack_air_status_init(agent: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(agent.module_accessor, 0) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
    }else {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
    }
    smashline::original_status(Init, agent, *FIGHTER_STATUS_KIND_ATTACK_AIR)(agent)
}
unsafe extern "C" fn attack_air_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(agent.module_accessor, 0) 
    && !VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_HAS_ITEM_ANIM);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    smashline::original_status(Exec, agent, *FIGHTER_STATUS_KIND_ATTACK_AIR)(agent)
}
//landing
unsafe extern "C" fn landing_attack_air_check_attack(agent: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
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
        let touch_normal_x = GroundModule::get_touch_normal(agent.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32).x;
        let slope_angle = touch_normal_x.to_degrees()*PostureModule::lr(agent.module_accessor);
        if PostureModule::lr(agent.module_accessor) == PostureModule::lr(opponent_boma) {
            LinkModule::set_constraint_rot_offset(opponent_boma, &Vector3f{x:slope_angle, y:0.0, z:0.0});
        }else {
            LinkModule::set_constraint_rot_offset(opponent_boma, &Vector3f{x:slope_angle, y:180.0, z:0.0});
        }
    }
    0.into()
}
//ending-blow
unsafe extern "C" fn finishing_blow_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
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
        *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32,
        0
    );
    false.into()
}
unsafe extern "C" fn finishing_blow_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(agent.module_accessor, Hash40::new("ending_blow"), 0.0, 1.0, false, 0.0, false, false);
    let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE) as u32;
    let pos = PostureModule::pos_2d(agent.module_accessor);
    FighterUtil::request_critical_hit_cut_in_force(
        agent.module_accessor,
        capture_id,
        &Vector2f{x:pos.x, y:pos.y},
        -1,
        Hash40::new("param_critical"),
        0,
        false,
        0,
        false
    );
    VarModule::set_flag(agent.module_accessor, status::LINK_FLAG_FINISHING_BLOW_1V1, is_one_on_one());
    let capture_boma = sv_battle_object::module_accessor(capture_id);
    // StopModule::cancel_hit_stop(capture_boma);
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(0));
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(1));
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(2));
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(3));
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(4));
    // StopModule::cancel_other_stop(capture_boma, StopOtherKind(5));
    // StopModule::end_stop(capture_boma);
    // SlowModule::clear(capture_boma);
    ShakeModule::stop(capture_boma);
    ShakeModule::stop(agent.module_accessor);
    if WorkModule::get_int64(capture_boma, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND) == hash40("down_wait_u") {
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_u3"), 30.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d3"), 30.0, 1.0, false, 0.0, false, false);
    }
    let capture_lr = PostureModule::lr(capture_boma);
    let attacker_joint = &mut Vector3f{x:0.0, y:0.0, z:0.0};
    let capture_joint = &mut Vector3f{x:0.0, y:0.0, z:0.0};
    ModelModule::joint_global_position(agent.module_accessor, Hash40::new("sword1"), attacker_joint, false);
    ModelModule::joint_global_position(capture_boma, Hash40::new("waist"), capture_joint, false);
    LinkModule::set_constraint_translate_offset(capture_boma, &Vector3f{x:-capture_joint.z, y:0.0, z:(attacker_joint.x-capture_joint.x)*capture_lr});
    
    HitModule::set_whole(capture_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_FINISHING_BLOW_THROW);
    agent.sub_shift_status_main(L2CValue::Ptr(finishing_blow_status_main_loop as *const () as _))
}
pub unsafe fn finishing_blow_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    if VarModule::is_flag(agent.module_accessor, status::LINK_FLAG_FINISHING_BLOW_THROW) {
        VarModule::off_flag(agent.module_accessor, status::LINK_FLAG_FINISHING_BLOW_THROW);
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(capture_id);
        if WorkModule::get_int64(capture_boma, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND) == hash40("down_wait_u") {
            MotionModule::change_motion(capture_boma, Hash40::new("down_damage_u"), 0.0, 0.6, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d"), 0.0, 0.6, false, 0.0, false, false);
        }
        LinkModule::unlink_node(agent.module_accessor, *LINK_NO_CAPTURE);
        VarModule::on_flag(capture_boma, FIGHTER_FLAG_LINK_FINISHING_BLOW_DEAD);
        WorkModule::set_int(capture_boma, 30, *FIGHTER_INSTANCE_WORK_ID_INT_KNOCKOUT_FRAME);
        
        KineticModule::change_kinetic(capture_boma, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::attach_ground(capture_boma, true);
        GroundModule::set_collidable(capture_boma, true);
    }
    false.into()
}
unsafe extern "C" fn finishing_blow_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(agent.module_accessor, *LINK_NO_CAPTURE) {
        let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(capture_id);
        LinkModule::unlink_node(agent.module_accessor, *LINK_NO_CAPTURE);
        VarModule::on_flag(capture_boma, FIGHTER_FLAG_LINK_FINISHING_BLOW_DEAD);
        WorkModule::set_int(capture_boma, 30, *FIGHTER_INSTANCE_WORK_ID_INT_KNOCKOUT_FRAME);
    }
    0.into()
}
////motion
//neutral-air
unsafe extern "C" fn attack_air_n_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 40, 100, 0, 25, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 7.0, 40, 100, 0, 25, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 6.0, 40, 100, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 11.0, 40, 100, 0, 25, 4.5, 7.5, 0.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 8.0, 40, 100, 0, 25, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 8.0, 40, 100, 0, 25, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 7.0, 40, 100, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 15.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 40, 100, 0, 25, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 7.0, 40, 100, 0, 25, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 6.0, 40, 100, 0, 25, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_n_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
}
unsafe extern "C" fn attack_air_n_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_ll"));
    }
}
unsafe extern "C" fn attack_air_n_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

// unsafe extern "C" fn attack_air_n_game(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 4.0);
//     if macros::is_excute(agent) {
//         WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
//     }
//     frame(agent.lua_state_agent, 5.0);
//     if macros::is_excute(agent) {
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 80, 0, 60, 5.0, 0.0, 12.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 60, 95, 0, 60, 3.2, 8.5, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 7.0, 60, 95, 0, 60, 3.5, 3.0, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 6.0, 65, 80, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         AttackModule::set_lr_check_front(agent.module_accessor, 0);
//         macros::ATTACK(agent, 4, 1, Hash40::new("top"), 9.0, 65, 80, 0, 60, 5.0, 0.0, 12.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 5, 1, Hash40::new("sword2"), 10.0, 60, 95, 0, 60, 3.2, 8.5, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 6, 1, Hash40::new("sword2"), 10.0, 60, 95, 0, 60, 3.5, 3.0, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         macros::ATTACK(agent, 7, 1, Hash40::new("armr"), 9.0, 65, 80, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         AttackModule::set_lr_check_back(agent.module_accessor, 1);
//     }
//     frame(agent.lua_state_agent, 7.0);
//     if macros::is_excute(agent) {
//         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 80, 0, 60, 5.0, 0.0, 13.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
//         AttackModule::set_lr_check_default(agent.module_accessor,0);
//         AttackModule::clear(agent.module_accessor, 4, false);
//         AttackModule::clear(agent.module_accessor, 5, false);
//         AttackModule::clear(agent.module_accessor, 6, false);
//         AttackModule::clear(agent.module_accessor, 7, false);
//     }
//     frame(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         AttackModule::clear_all(agent.module_accessor);
//     }
//     frame(agent.lua_state_agent, 25.0);
//     if macros::is_excute(agent) {
//         WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
//     }
// }
// unsafe extern "C" fn attack_air_n_eff(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 5.0);
//     if macros::is_excute(agent) {
//         macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 4, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
//         macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
//     }
//     frame(agent.lua_state_agent, 10.0);
//     if macros::is_excute(agent) {
//         macros::AFTER_IMAGE_OFF(agent, 3);
//     }
//     frame(agent.lua_state_agent, 14.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
//     }
// }
// unsafe extern "C" fn attack_air_n_snd(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 5.0);
//     if macros::is_excute(agent) {
//         macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
//         macros::PLAY_SE(agent, Hash40::new("se_link_swing_m"));
//     }
// }
// unsafe extern "C" fn attack_air_n_exp(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 4.0);
//     if macros::is_excute(agent) {
//         ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
//     }
//     frame(agent.lua_state_agent, 6.0);
//     if macros::is_excute(agent) {
//         macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
//     }
// }

//back-air
unsafe extern "C" fn attack_air_b_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 43, 28, 0, 31, 4.0, 0.0, 11.5, -18.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 367, 30, 0, 25, 4.0, 0.0, 11.5, -18.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 47, 30, 0, 33, 6.5, 0.0, 11.5, -11.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 367, 30, 0, 25, 6.5, 0.0, 11.5, -11.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 5.0, 47, 28, 0, 33, 7.0, 0.0, 11.5, -6.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 5.0, 38, 26, 0, 26, 7.0, 0.0, 11.5, -6.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 4, 4.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 5, 4.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 98, 0, 45, 4.5, 0.0, 11.0, -17.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 98, 0, 45, 6.5, 0.0, 11.0, -11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 361, 98, 0, 45, 6.8, 0.0, 11.0, -6.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_b_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 6, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 6, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}
unsafe extern "C" fn attack_air_b_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_m"));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_l"));
    }
}
unsafe extern "C" fn attack_air_b_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 13);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
//forward-air
unsafe extern "C" fn attack_air_f_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    // frame(agent.lua_state_agent, 7.0);
    // if macros::is_excute(agent) {
    //     let mul = 1.5;
    //     let air_accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("air_accel_y"), 0)*-1.0;
    //     sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y*mul);
    // }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 14.0, 45, 85, 0, 70, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 50, 85, 0, 85, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 55, 77, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.4);
        // AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 17.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 2.5);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 9.0, 45, 85, 0, 55, 3.0, 8.5, 0.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 9.0, 50, 85, 0, 65, 3.3, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 55, 72, 0, 80, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.4);
        // AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    // frame(agent.lua_state_agent, 36.0);
    // macros::FT_MOTION_RATE(agent, 1.0);
}
unsafe extern "C" fn attack_air_f_eff(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 4.0);
    // if macros::is_excute(agent) {
    //     macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    // }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 13, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    // frame(agent.lua_state_agent, 21.0);
    // if macros::is_excute(agent) {
    //     macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    // }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
}
unsafe extern "C" fn attack_air_f_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_link_swing_ll"));
    }
    // wait(agent.lua_state_agent, 4.0);
    // if macros::is_excute(agent) {
    //     macros::PLAY_LANDING_SE(agent, Hash40::new("se_link_landing02"));
    // }
}
unsafe extern "C" fn attack_air_f_exp(agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    // }
    // frame(agent.lua_state_agent, 3.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    // frame(agent.lua_state_agent, 21.0);
    // if macros::is_excute(agent) {
    //     slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 22.0);
    // if macros::is_excute(agent) {
    //     slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    // }
}
//forward-air-landing
unsafe extern "C" fn landing_air_f_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 14.0, 45, 85, 0, 70, 3.2, 8.5, 0.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 13.0, 50, 85, 0, 85, 3.5, 3.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.0, 55, 77, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 1.4);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn landing_air_f_eff(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 4.0);
    // if macros::is_excute(agent) {
    //     macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    // }
    // frame(agent.lua_state_agent, 11.0);
    // if macros::is_excute(agent) {
    //     macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 13, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    //     macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    // }
    // frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        // macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
}
unsafe extern "C" fn landing_air_f_snd(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 19.0);
    // if macros::is_excute(agent) {
    //     macros::PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
    //     macros::PLAY_SE(agent, Hash40::new("se_link_swing_ll"));
    // }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_link_landing02"));
        macros::PLAY_SE(agent, Hash40::new("se_link_attackhard_s01"));
    }
}
unsafe extern "C" fn landing_air_f_exp(agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    //     ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    //     slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    // }
    // frame(agent.lua_state_agent, 3.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 18.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}
//up-air
unsafe extern "C" fn attack_air_hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}
//up-air-landing
unsafe extern "C" fn landing_air_hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}
//down-air
unsafe extern "C" fn attack_air_lw_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 3.0, 7.0, 3.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 270, 80, 0, 30, 4.0, 1.5, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 60, 78, 0, 30, 4.0, 1.5, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 60, 80, 0, 30, 4.2, 1.0, 1.5, 0.0, Some(1.0), Some(4.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_PART_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_lw_exp(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    // }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}
//down-air-bounce
unsafe extern "C" fn attack_air_lw_2_attack_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 65, 80, 0, 50, 4.3, 1.0, 1.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}
//down-air-landing
unsafe extern "C" fn landing_air_lw_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 60, 80, 0, 30, 5.0, 0.0, 5.0, 2.0, Some(0.0), Some(5.0), Some(-2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn landing_air_lw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}
//ending-blow
unsafe extern "C" fn ending_blow_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::LINK_FLAG_FINISHING_BLOW_THROW);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        HitModule::set_whole(agent.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}
unsafe extern "C" fn ending_blow_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}
unsafe extern "C" fn ending_blow_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_link_special_h11"));
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::PLAY_LANDING_SE(agent, Hash40::new("se_link_landing01"));
    }
}
unsafe extern "C" fn ending_blow_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_status_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_status_exec);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, landing_attack_air_check_attack);
    //ending-blow
    agent.status(Pre, FIGHTER_LINK_STATUS_KIND_FINISHING_BLOW, finishing_blow_status_pre);
    agent.status(Main, FIGHTER_LINK_STATUS_KIND_FINISHING_BLOW, finishing_blow_status_main);
    agent.status(End, FIGHTER_LINK_STATUS_KIND_FINISHING_BLOW, finishing_blow_status_end);
    ////motion
    //neutral-air
    agent.game_acmd("game_attackairn", attack_air_n_game, Priority::High);
    agent.effect_acmd("effect_attackairn", attack_air_n_eff, Priority::High);
    agent.sound_acmd("sound_attackairn", attack_air_n_snd, Priority::High);
    agent.expression_acmd("expression_attackairn", attack_air_n_exp, Priority::High);
    //back-air
    agent.game_acmd("game_attackairb", attack_air_b_game, Priority::High);
    agent.effect_acmd("effect_attackairb", attack_air_b_eff, Priority::High);
    agent.sound_acmd("sound_attackairb", attack_air_b_snd, Priority::High);
    agent.expression_acmd("expression_attackairb", attack_air_b_exp, Priority::High);
    //forward-air
    agent.game_acmd("game_attackairf", attack_air_f_game, Priority::High);
    agent.effect_acmd("effect_attackairf", attack_air_f_eff, Priority::High);
    agent.sound_acmd("sound_attackairf", attack_air_f_snd, Priority::High);
    agent.expression_acmd("expression_attackairf", attack_air_f_exp, Priority::High);
    //forward-air-landing
    agent.game_acmd("game_landingairf", landing_air_f_game, Priority::High);
    agent.effect_acmd("effect_landingairf", landing_air_f_eff, Priority::High);
    agent.sound_acmd("sound_landingairf", landing_air_f_snd, Priority::High);
    agent.expression_acmd("expression_landingairf", landing_air_f_exp, Priority::High);
    //up-air
    agent.expression_acmd("expression_attackairhi", attack_air_hi_exp, Priority::High);
    //up-air-landing
    agent.expression_acmd("expression_landingairhi", landing_air_hi_exp, Priority::High);
    //down-air
    agent.game_acmd("game_attackairlw", attack_air_lw_game, Priority::High);
    agent.expression_acmd("expression_attackairlw", attack_air_lw_exp, Priority::High);
    //down-air-bounce
    agent.game_acmd("game_attackairlw2attack", attack_air_lw_2_attack_game, Priority::High);
    //up-down-landing
    agent.game_acmd("game_landingairlw", landing_air_lw_game, Priority::High);
    agent.expression_acmd("expression_landingairlw", landing_air_lw_exp, Priority::High);
    //ending-blow
    agent.game_acmd("game_endingblow", ending_blow_game, Priority::High);
    agent.effect_acmd("effect_endingblow", ending_blow_eff, Priority::High);
    agent.sound_acmd("sound_endingblow", ending_blow_snd, Priority::High);
    agent.expression_acmd("expression_endingblow", ending_blow_exp, Priority::High);
}