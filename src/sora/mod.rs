use smashline::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
pub static mut CHECK : [bool; 8] = [false; 8];



#[fighter_frame( agent = FIGHTER_KIND_TRAIL)]
fn low(fighter: &mut L2CFighterCommon) {
    unsafe{
        let damage = DamageModule::damage(fighter.module_accessor, 0); 
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if damage >=100.0
        && !CHECK[entry_id] {
            smash_script::macros::PLAY_SE(fighter,Hash40::new("se_trail_hp"));
            CHECK[entry_id] = true;
        }
        else{
        if damage <=99.9
            && CHECK[entry_id] == true
            {
            CHECK[entry_id] = false;
            smash_script::macros::STOP_SE(fighter,Hash40::new("se_trail_hp"));
            }
        }
    }
  }

pub fn install() {
    smashline::install_agent_frames!(
        low
    );
}
