use bakkesmod::prelude::*;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::game;

#[plugin_init]
pub fn on_load() {
    // Hook event on_boost_pickup() only works on freeplay/LAN

    // hook_event_with_caller() - callback is invoked when the event occurs 
    // hook_event_with_caller_post() - callback is invoked after the event occurs (event is completed)
    game::hook_event_with_caller_param("Function TAGame.VehiclePickup_Boost_TA.Pickup", Box::new(on_boost_pickup));
    game::hook_event_with_caller("Function TAGame.CarComponent_Boost_TA.GiveBoost", Box::new(give_boost));
}

// The parameter for the callback is the object we hooked our event to 
fn on_boost_pickup(boost: Box<BoostPickupWrapper>, param: Box<CarWrapper>) {
    log_console!("on_boost_pickup() invoked!");
    // ADDRESS WAS tied to the boost we picked up
    // Boost type: 
    //   1 => Big boost
    //   2 => Pads
    log_console!("by: {} type {}", boost.addr(), boost.get_boost_type());
    log_console!("param {}", param.addr());
    param.demolish();

    // log_console!("location of param {:?}", param.get_location());

    // match param.get_car() {
    //     Some(car) => {
    //         log_console!("param: {}", car.addr());
    //     },
    //     None => log_console!("param does not contain car"),
    // }
} 

fn give_boost(car: Box<CarComponentWrapper>) {
    // address was tied to the car 
    log_console!("give_boost() {}", car.addr());

    // Destroys car if it exists
    match car.get_car() {
        Some(car) => {
            log_console!("CarWrapper.addr = {}", car.addr());
            // car.demolish();
        },
        None => log_console!("CarComponent does not contain car"),
    }
}