use bakkesmod::prelude::*;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::{game, console};

#[plugin_init]
pub fn on_load() {
    // Hook event on_boost_pickup()
    game::hook_event("Function TAGame.VehiclePickup_Boost_TA.Pickup", Box::new(on_boost_pickup));
}

fn on_boost_pickup() {
    log_console!("on_boost_pickup() invoked!");
}