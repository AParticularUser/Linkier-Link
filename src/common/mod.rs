use crate::imports::*;
use crate::common::consts::vars::*;


unsafe extern "C" fn common_frame(agent: &mut L2CFighterCommon) {
    if VarModule::is_flag(agent.module_accessor, status::FIGHTER_FLAG_LINK_FINISHING_BLOW_DEAD) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KNOCKOUT_FRAME) == 0 
        || StatusModule::status_kind(agent.module_accessor) != *FIGHTER_STATUS_KIND_THROWN 
        || GroundModule::is_attach(agent.module_accessor)
        // || agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND 
        {
            VarModule::off_flag(agent.module_accessor, status::FIGHTER_FLAG_LINK_FINISHING_BLOW_DEAD);
            StatusModule::change_status_force(agent.module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
        }else {
            WorkModule::dec_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KNOCKOUT_FRAME);
            if MotionModule::is_end(agent.module_accessor) {
                if WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROWN_WORK_INT_MOTION_KIND) == hash40("down_wait_u") {
                    MotionModule::change_motion(agent.module_accessor, Hash40::new("down_wait_u"), 0.0, 1.0, false, 0.0, false, false);
                }else {
                    MotionModule::change_motion(agent.module_accessor, Hash40::new("down_wait_d"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            let touch_normal_x = GroundModule::get_touch_normal(agent.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32).x;
            let slope_angle = touch_normal_x.to_degrees()*PostureModule::lr(agent.module_accessor);
            ModelModule::set_joint_rotate(
                agent.module_accessor, 
                Hash40::new("top"), 
                &Vector3f{x:slope_angle, y:90.0*PostureModule::lr(agent.module_accessor), z:0.0}, 
                MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, 
                MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8}
            );
        }
    }
}


pub mod funcs;
pub mod consts;

pub fn install() {
    // funcs::install();
    Agent::new("fighter")
	.on_line(Main, common_frame)
	.install();
}