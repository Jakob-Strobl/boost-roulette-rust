use bakkesmod::prelude::*;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::game;

#[plugin_init]
pub fn on_load() {
    // Hook event on_boost_pickup() only works on freeplay/LAN

    // hook_event_with_caller() - callback is invoked when the event occurs 
    // hook_event_with_caller_post() - callback is invoked after the event occurs (event is completed)
    game::hook_event_with_caller_param("Function TAGame.VehiclePickup_Boost_TA.Pickup", Box::new(on_boost_pickup));
}

// The parameter for the callback is the object we hooked our event to 
fn on_boost_pickup(boost: Box<BoostPickupWrapper>, car: Box<CarWrapper>) {
    log_console!("on_boost_pickup() invoked!");
    // ADDRESS WAS tied to the boost we picked up
    // Boost type: 
    //   1 => Big boost
    //   2 => Pads
    log_console!("by: {} type {}", boost.addr(), boost.get_boost_type());
    log_console!("param {}", param.addr());
} 