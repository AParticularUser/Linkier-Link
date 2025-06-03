#![allow(non_snake_case)]
use crate::imports::*;


//special thanks the Wuboy/Wubor Patch for porting HDR's VarModule
#[skyline::hook(offset = 0x3af300)]
pub unsafe fn battleobjectmoduleaccessor__initialize_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: *const u64) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    if object_id != 0x50000000 {
        CustomVarManager::reset_var_module(module_accessor, false);
    }
}
#[skyline::hook(offset = 0x3afa10)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    VarModule::start(module_accessor);
}
#[skyline::hook(offset = 0x3afe00)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    CustomVarManager::reset_var_module(module_accessor, true);
    original!()(module_accessor, param_1)
}
#[skyline::hook(offset = 0x3af720)]
pub unsafe fn battleobjectmoduleaccessor__finalize_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    CustomVarManager::remove_var_module(module_accessor);
    original!()(module_accessor)
}
//hook for enabling paraglider if not using C#'s param_config
// #[skyline::hook(offset = 0x3a6670)]
// unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
//     if weapon_kind == *WEAPON_KIND_LINK_PARASAIL {
//         return 1;
//     }
//     call_original!(weapon_kind, entry_id)
// }


pub fn install() {
    skyline::install_hooks!(
        battleobjectmoduleaccessor__initialize_modules,
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules,
        battleobjectmoduleaccessor__finalize_modules//,

        // get_article_use_type_mask
    );
}