use std::process;
use std::convert;

use bakkesmod::prelude::*;
use bakkesmod::prelude::cvar::CVar;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::game;
use bakkesmod::console;

use rand::prelude::*;

// This is for better code clarity 
// Boost type from BoostPickupWrapper.get_boost_type()
//   1 => Big boost
//   2 => Pads
enum BoostType {
    BIG,
    PAD,
}

impl convert::From<u8> for BoostType {
    fn from(val: u8) -> BoostType {
        match val {
            1 => BoostType::BIG,
            2 => BoostType::PAD,
            _ => process::abort(),
        }
    }
}

const NUM_PADS: usize = 28;
const NUM_BIG_BOOSTS: usize = 6;

#[plugin_init]
pub fn on_load() {
    // Enable button 
    // TODO look into when and why you would want to save to cfg
    let is_enabled = console::register_cvar("boost_roulette_is_enabled", "0", "Status of Boost Roulette Plugin (0 = Disabled | 1 = Enables)", true, true, 0.0, true, 1.0, false);
    let big_boost_boom_chance = console::register_cvar("boost_roulette_big_boost_chance", &(100/NUM_BIG_BOOSTS).to_string(), "The percent chance a big boost demos on pickup", true, true, 0.0, true, 100.0, false);
    let pad_boom_chance = console::register_cvar("boost_roulette_pad_chance", &(100/NUM_PADS).to_string(), "The percent chance a pad demos on pickup", true, true, 0.0, true, 100.0, false);
    
    console::add_on_value_changed(&is_enabled, Box::new(is_enabled_changed));
}

fn is_enabled_changed(_: String, is_enabled: CVar) {
    log_console!("'is_enabled' => {}", is_enabled.get_bool_value());

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
    let boost_type = BoostType::from(boost.get_boost_type());

    let is_unlucky = match boost_type {
        BoostType::BIG => roll_big_boost(),
        BoostType::PAD => roll_pad(),
    };

    if is_unlucky {
        // Since CarWrapper implements RBactor (RBactor is a component of CarWrapper),
        // we can just pass the adress of the CarWrapper as the RBactor 
        car.demolish2(car.addr(), 0);
    }
} 

fn get_roll() -> f32 {
    thread_rng().gen_range(0.0 .. 100.0)
}

fn roll_big_boost() -> bool {
    let roll = get_roll();
    match console::get_cvar("boost_roulette_big_boost_chance") {
        Some(chance) => roll < chance.get_float_value(),
        None => {
            log_console!("'boost_roulette_big_boost_chance' cvar not found!");
            false
        },
    }
}

fn roll_pad() -> bool {
    let roll = get_roll();
    match console::get_cvar("boost_roulette_pad_chance") {
        Some(chance) => roll < chance.get_float_value(),
        None => {
            log_console!("'boost_roulette_pad_chance' cvar not found!");
            false
        },
    }
}