use std::vec;

use bakkesmod::prelude::*;
use bakkesmod::prelude::cvar::CVar;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::game;
use bakkesmod::console;

#[plugin_init]
pub fn on_load() {
    // Enable button 
    // TODO look into when and why you would want to save to cfg
    let is_enabled = console::register_cvar("boost_roulette_is_enabled", "0", "Status of Boost Roulette Plugin (0 = Disabled | 1 = Enables)", true, true, 0.0, true, 1.0, false);
    console::add_on_value_changed(&is_enabled, Box::new(is_enabled_changed));


}

fn is_enabled_changed(_: String, is_enabled: CVar) {
    log_console!("is_enabled changed to {}", is_enabled.get_bool_value());
    
    if is_enabled.get_bool_value() {
        // Hook event on_boost_pickup() only works on freeplay/LAN
        // hook_event_with_caller() - callback is invoked when the event occurs 
        // hook_event_with_caller_post() - callback is invoked after the event occurs (event is completed)
        game::hook_event_with_caller_param("Function TAGame.VehiclePickup_Boost_TA.Pickup", Box::new(on_boost_pickup));
    } else {
        game::unhook_event("Function TAGame.VehiclePickup_Boost_TA.Pickup");
    }
}


// The parameter for the callback is the object we hooked our event to 
fn on_boost_pickup(boost: Box<BoostPickupWrapper>, car: Box<CarWrapper>) {
    // This seems wrong 
    log_console!("on_boost_pickup() invoked!");
    // ADDRESS WAS tied to the boost we picked up
    // Boost type: 
    //   1 => Big boost
    //   2 => Pads
    log_console!("by: {} type {}", boost.addr(), boost.get_boost_type());
    log_console!("car addr: {}", car.addr());

    // Since CarWrapper implements RBactor (RBactor is a component of CarWrapper),
    // we can just pass the adress of the CarWrapper as the RBactor 
    car.demolish2(car.addr(), 0);
} 