use crate::{
    game::{actor::get_first_enemy, events::ActorEventFlowMgr, file_manager, flag_managers::{DungeonflagManager, ItemflagManager, SceneflagManager, StoryflagManager}, player, reloader::{self, get_spawn_master, get_spawn_slave}}, menus::main_menu, system::button::*, utils::{console::Console, menu::SimpleMenu, practice_saves::load_practice_save}
};

use core::fmt::Write;
use core::ffi::c_void;
use core::option::*;

pub struct Trick {
    name:   &'static str,
    description: &'static str,
    associated_enum: ActiveTrick,
    on_select: Option<fn()>,
}

const TRICKS: [Trick; 15] = [
    Trick {
        name:   "Wing Ceremony Cutscene Skip",
        description: "Practice WCCS Save Prompt sidehop (Kills Link for faster reloads).",
        associated_enum: ActiveTrick::Wccs,
        on_select: Some(reload_wccs_prompt),
    },
    Trick {
        name:   "Guay Deathwarp",
        description: "Practice the guay deathwarp after Sky RBW.",
        associated_enum: ActiveTrick::Guay,
        on_select: Some(reload_guay),
    },
    Trick {
        name:   "Keese Yeet",
        description: "Practice in Earth Temple positioned for Keese Yeet.",
        associated_enum: ActiveTrick::KeeseYeet,
        on_select: Some(reload_keese_yeet),
    },
    Trick {
        name:   "Extending Blow",
        description: "Practice the Extending Blow in Deep Woods.",
        associated_enum: ActiveTrick::ExtendingBlow,
        on_select: Some(reload_eb),
    },
    Trick {
        name:   "Ghirahim 1",
        description: "Practice fighting Ghirahim in Skyview Temple (with Goddess Sword).",
        associated_enum: ActiveTrick::Ghirahim1,
        on_select: Some(reload_g1),
    },
    Trick {
        name:   "Scaldera",
        description: "Practice fighting Scaldera in Earth Temple (with Goddess Sword).",
        associated_enum: ActiveTrick::Scaldera,
        on_select: Some(reload_scaldera),
    },
    Trick {
        name:   "Moldarach",
        description: "Practice fighting Moldarach in Lanayru Mining Facility.",
        associated_enum: ActiveTrick::Moldarach,
        on_select: Some(reload_moldarach),
    },
    /*
    Trick {
        name:   "Imprisoned 1",
        description: "Practice fighting The Imprisoned after learning Ballad of the Goddess.",
        associated_enum: ActiveTrick::Imp1,
        on_select: Some(reload_imp1),
    },
    */
    Trick {
        name:   "Koloktos",
        description: "Practice fighting Koloktos in Ancient Cistern (with Goddess Sword).",
        associated_enum: ActiveTrick::Koloktos,
        on_select: Some(reload_koloktos),
    },
    Trick {
        name:   "Tentalus",
        description: "Practice fighting Tentalus in Sandship.",
        associated_enum: ActiveTrick::Tentalus,
        on_select: Some(reload_tentalus),
    },
    Trick {
        name:   "Ghirahim 2",
        description: "Practice fighting Ghirahim in Fire Sanctuary.",
        associated_enum: ActiveTrick::Ghirahim2(G2State::Phase1),
        on_select: Some(reload_g2),
    },
    /*
    Trick {
        name:   "Imprisoned 2",
        description: "Practice fighting The Imprisoned before opening the Gate of Time.",
        associated_enum: ActiveTrick::Imp2,
        on_select: Some(reload_imp2),
    },
    Trick {
        name:   "Imprisoned 3",
        description: "Practice fighting The Imprisoned during the Faron SotH quest.",
        associated_enum: ActiveTrick::Imp3,
        on_select: Some(reload_imp3),
    },
    */
    Trick {
        name:   "Bilocyte",
        description: "Practice fighting Bilocyte in the Thunderhead (currently at the main entrance).",
        associated_enum: ActiveTrick::Bilocyte,
        on_select: Some(reload_bilocyte),
    },
    Trick {
        name:   "Horde",
        description: "Practice fighting the Horde Battle in Hylia's Realm.",
        associated_enum: ActiveTrick::Horde,
        on_select: Some(reload_horde),
    },
    Trick {
        name:   "Ghirahim 3",
        description: "Practice fighting Ghirahim in Hylia's Realm.",
        associated_enum: ActiveTrick::Ghirahim3,
        on_select: Some(reload_g3),
    },
    Trick {
        name:   "Demise",
        description: "Practice fighting Demise at the end of the game.",
        associated_enum: ActiveTrick::Demise,
        on_select: Some(reload_demise),
    },
    Trick {
        name:   "Cutscene Skip Wrong Warp",
        description: "Practice Death Trick & CSWW (and file dupe).",
        associated_enum: ActiveTrick::Csww(CswwState::DoingReset),
        on_select: Some(reload_csww),
    }
];

#[derive(PartialEq, Eq)]
enum MenuState {
    Off,
    Main,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum G2State {
    Phase1,
    BetweenPhases,
    Phase2
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum CswwState {
    DoingReset,
    DeathTrick,
    Failed,
    FileDupe(bool),
    FileDupeEval(bool),
}


// Some tricks have an associated u8 value to track partial progress
#[derive(PartialEq, Eq, Copy, Clone)]
enum ActiveTrick {
    None,
    Wccs,
    Guay,
    KeeseYeet,
    ExtendingBlow,
    Ghirahim1,
    Scaldera,
    Moldarach,
    // Imp1,
    Koloktos,
    Tentalus,
    Ghirahim2(G2State),
    // Imp2,
    Bilocyte,
    //Imp3,
    Horde,
    Ghirahim3,
    Demise,
    Csww(CswwState),
}

pub struct TricksMenu {
    state:  MenuState,
    cursor: u32,
    active_trick: ActiveTrick,
}

#[no_mangle]
#[link_section = "data"]
static mut TRICKS_MENU: TricksMenu = TricksMenu {
    state:  MenuState::Off,
    cursor: 0,
    active_trick: ActiveTrick::None,
};

impl super::Menu for TricksMenu {
    fn enable() {
        unsafe { TRICKS_MENU.state = MenuState::Main };
    }

    fn disable() {
        unsafe { TRICKS_MENU.state = MenuState::Off };
    }

    fn input() {
        let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

        match tricks_menu.state {
            MenuState::Off => {},
            MenuState::Main => {
                if is_pressed(B) {
                    TricksMenu::disable();
                } else if is_pressed(A) {
                    let trick = &TRICKS[tricks_menu.cursor as usize];
                    let is_active = core::mem::discriminant(&trick.associated_enum) == core::mem::discriminant(&tricks_menu.active_trick);
                    if is_active {
                        tricks_menu.active_trick = ActiveTrick::None;
                    } else {
                        tricks_menu.active_trick = trick.associated_enum;
                        match trick.on_select {
                            None => {},
                            Some(f) => {
                                (f)();
                                TricksMenu::disable();
                                main_menu::MainMenu::disable();
                            }
                        }
                    }
                }
            },
        }
    }

    fn display() {
        let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

        let menu = crate::reset_menu();
        menu.set_heading("Practice a trick (reloads on success or pressing D-Pad Left).");
        for trick in &TRICKS {
            menu.add_entry_fmt(format_args!(
                "{} [{}]",
                trick.name,
                if core::mem::discriminant(&trick.associated_enum) == core::mem::discriminant(&tricks_menu.active_trick) { "x" } else { "" }
            ), trick.description);
        }
        menu.set_cursor(tricks_menu.cursor);
        menu.draw();
        tricks_menu.cursor = menu.move_cursor();
    }

    fn is_active() -> bool {
        unsafe { TRICKS_MENU.state != MenuState::Off }
    }
}

extern "C" {
    static mut FRAME_COUNT: u32;
    static mut TITLE_LOADER: u8; // probably wrong but AP reads this byte to check if on title screen
}

fn get_boss_health() -> Option<u32> {
    get_first_enemy().map(|e| unsafe { (e.add(0x10) as *mut u32).read() })
}

fn is_boss_dead() -> bool {
    match get_boss_health() {
        Some(hp) => hp == 0,
        None => false,
    }
}

// For WCCS, the buffer will stop accepting A presses on the frame that is 3 frames too late
// For CSWW, the buffer will stop accepting A presses when the frame counter starts up again
#[link_section = "data"]
pub static mut A_PRESS_BUFFER: u32 = 0;

#[link_section = "data"]
pub static mut TWO_HOLD_BUFFER: u32 = 0;


// Frames "-2" and "-1" are the good frames, but there is a 3 frame input delay
// So frame 5 is actually 3 frames late, and frames 1 and 2 are the good ones
const WCCS_THREE_FRAMES_LATE: u32 = 5;

const CSWW_TIMER_CONTINUE_FRAME: u32 = 161;
const CSWW_MAP_UPDATE: u32 = 178; // Map changes to File 1's map around this time
const CSWW_FILE_DUPE_END_FRAME: u32 = 32;

pub fn update_buffer() {
    // The buffer's bits store whether or not A was pressed in the last 8 frames
    unsafe {
        A_PRESS_BUFFER <<= 1;
        TWO_HOLD_BUFFER <<= 1;
        if is_pressed(A) {
            A_PRESS_BUFFER += 1;
        }
        if is_down(TWO) {
            TWO_HOLD_BUFFER += 1;
        }
    }
}

fn eval_wccs() {
    let buffer = unsafe {A_PRESS_BUFFER};
    let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    // We're checking inputs 3 frames after the window closed
    if buffer & 0x10 != 0 {
        // 4 frames ago
        console.set_font_color(0x00FF00FF);
        let _ = console.write_fmt(format_args!("got it (first frame)"));
    }
    else if buffer & 0x08 != 0 {
        // 3 frames ago
        console.set_font_color(0x00FF00FF);
        let _ =console.write_fmt(format_args!("got it (second frame)"));
    }
    else if buffer & 0x20 != 0 {
        // 5 frames ago
        console.set_font_color(0xFFFF00FF);
        let _ =console.write_fmt(format_args!("1 frame early"));
    }
    else if buffer & 0x04 != 0 {
        // 2 frames ago
        console.set_font_color(0xFFFF00FF);
        let _ = console.write_fmt(format_args!("1 frame late"));
    }
    else if buffer & 0x40 != 0 {
        // 6 frames ago
        console.set_font_color(0xFFC000FF);
        let _ = console.write_fmt(format_args!("2 frames early"));
    }
    else if buffer & 0x02 != 0 {
        // 1 frame ago
        console.set_font_color(0xFFC000FF);
        let _ = console.write_fmt(format_args!("2 frames late"));
    }
    else if buffer & 0x80 != 0 {
        // 7 frames ago
        console.set_font_color(0xFF4000FF);
        let _ = console.write_fmt(format_args!("3 frames early"));
    }
    else if buffer & 0x01 != 0 {
        // this frame
        console.set_font_color(0xFF4000FF);
        let _ = console.write_fmt(format_args!("3 frames late"));
    } else {
        console.set_font_color(0xFF0000FF);
        let _ = console.write_fmt(format_args!("more than 3 frames off"));
    }
    let _ = console.write_fmt(format_args!("\nTry again by pressing D-Pad Left."));
    console.draw(false);
}

fn display_boss_health(name: &'static str) {
    if let Some(hp) = get_boss_health() {
        let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
        console.set_bg_color(0x0000007F);
        console.set_font_size(0.5f32);
        console.set_dynamic_size(true);
        console.set_font_color(0xFFFFFFFF);
        let _ = console.write_fmt(format_args!("{} health: {}", name, hp));
        console.draw(false);
    }
}

fn check_wccs() {
    let count = unsafe {FRAME_COUNT};
    if count < WCCS_THREE_FRAMES_LATE {
        update_buffer();
    }
    // kinda hacky but prevents eye-blinding reloads from the display
    if count >= WCCS_THREE_FRAMES_LATE && count & 0x80000000 == 0 {
        eval_wccs();
        // Kill Link for faster reloads
        file_manager::set_current_health(0);
    }
}

fn reload_wccs_prompt() {
    // kinda hacky but prevents eye-blinding reloads from the display
    unsafe { FRAME_COUNT = 0x80000000; }
    reloader::set_save_prompt_flag();
    reloader::trigger_entrance(
        b"F000\0".as_ptr(),
        0,
        3, // Layer 3
        0,
        2,
        2,
        1,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(8);
}

fn reload_guay() {
    // Flag 24 is having seen the Fi text near Faron Pillar, must be unset
    // 364 is spiral charge, should also be unset
    StoryflagManager::set_to_value(24, 0);
    StoryflagManager::set_to_value(364, 0);
    StoryflagManager::do_commit();
    reloader::trigger_entrance(
        b"F020\0".as_ptr(),
        0,
        2, // Layer 2
        20, // Entrance 20
        2,
        2,
        1,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(24);
}

fn reload_keese_yeet() {
    SceneflagManager::unset_global(14, 29); // ET keese yeet rope cut
    SceneflagManager::unset_global(14, 24); // ET drawbridge down
    StoryflagManager::do_commit();
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for Keese Yeet
    current_file.pos_t1.x = 512.0;
    current_file.pos_t1.y = 0.0;
    current_file.pos_t1.z = 6600.0;
    current_file.angle_t1 = 0;
    reloader::trigger_entrance(
        b"D200\0".as_ptr(),
        1,
        0,
        2, // Entrance 2 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}

fn reload_g1() {
    SceneflagManager::set_global(13, 102); // Heart Container obtained
    StoryflagManager::set_to_value(466, 0); // Unset intro cutscene flag
    StoryflagManager::do_commit();
    set_sword_to_goddess();
    reloader::trigger_entrance(
        b"B100\0".as_ptr(),
        0,
        1, // Layer 1
        2, // Entrance 2 (after cs)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(8);
}

fn reload_scaldera() {
    SceneflagManager::set_global(14, 47); // Boulder rolling cutscene
    SceneflagManager::set_global(14, 37); // Fi Text in Room
    SceneflagManager::set_global(14, 56); // Heart Container obtained
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    // StoryflagManager::set_to_value(7, 0); // Unset ET Beaten
    // StoryflagManager::set_to_value(189, 0); // Unset flag after Scaldera CS
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(92, 1); // Give Bomb Bag
    ItemflagManager::increase_counter(2, 10); // Refill Bombs
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for Scaldera cutscene trigger
    current_file.pos_t1.x = 407.0;
    current_file.pos_t1.y = 7700.0;
    current_file.pos_t1.z = -21166.0;
    current_file.angle_t1 = 16384;
    current_file.equipped_b_item = 0; // Bomb Bag
    reloader::trigger_entrance(
        b"B200\0".as_ptr(),
        10, // Room 10 (actual boss area)
        2, // Layer 2
        1, // Entrance 1 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(24);
}

fn reload_moldarach() {
    SceneflagManager::set_global(17, 126); // Heart Container obtained
    SceneflagManager::unset_global(17, 120); // Related to boss defeat
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::set_to_value(30, 1); // Give Pouch Storyflag
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(52, 1); // Give Slingshot
    ItemflagManager::set_to_value(49, 1); // Give Gust Bellows
    ItemflagManager::set_to_value(20, 1); // Give Clawshots
    ItemflagManager::set_to_value(112, 1); // Give Pouch itemflag
    ItemflagManager::increase_counter(4, 20); // Refill Seeds
    ItemflagManager::set_to_value(11, 1); // Give Goddess Sword (unless player already has a higher sword)
    let current_file = file_manager::get_file_A();
    current_file.pouch_items[0] = 0x100074; // Wooden Shield
    current_file.shield_pouch_slot = 0;
    current_file.lastUsedPouchItemSlot = 0;
    reloader::trigger_entrance(
        b"B300\0".as_ptr(),
        0,
        1, // Layer 1
        1,
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(24);
}

fn reload_koloktos() {
    SceneflagManager::set_global(12, 77); // Heart Container obtained
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(137, 1); // Give Whip
    ItemflagManager::set_to_value(52, 1); // Give Slingshot
    ItemflagManager::increase_counter(4, 20); // Refill Seeds
    ItemflagManager::set_to_value(92, 1); // Give Bomb Bag
    ItemflagManager::increase_counter(2, 10); // Refill Bombs
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    current_file.equipped_b_item = 6; // Whip
    reloader::trigger_entrance(
        b"B101\0".as_ptr(),
        0,
        1, // Layer 1
        2, // Entrance 2
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(16);
}

fn reload_tentalus() {
    SceneflagManager::unset_global(18, 82); // Crest rises
    SceneflagManager::unset_global(18, 84); // Crest struck
    SceneflagManager::set_global(18, 85); // Heart Container obtained
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(19, 1); // Give Bow
    ItemflagManager::increase_counter(1, 20); // Refill Arrows
    ItemflagManager::set_to_value(12, 1); // Give Goddess Longsword (unless player already has a higher sword)
    let current_file = file_manager::get_file_A();
    current_file.equipped_b_item = 1; // Bow
    reloader::trigger_entrance(
        b"B301\0".as_ptr(),
        0,
        1, // Layer 1
        0, // Entrance 0
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(16);
}

fn reload_g2() {
    SceneflagManager::set_global(15, 124); // Heart Container obtained
    StoryflagManager::set_to_value(84, 0); // Unset defeated G2 storyflag
    StoryflagManager::set_to_value(464, 0); // Unset intro cutscene flag
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(9, 1); // Give Goddess White Sword (unless player already has a higher sword)
    reloader::trigger_entrance(
        b"B201\0".as_ptr(),
        0,
        1, // Layer 1
        1, // Entrance 1 (after cs)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(8);
}

fn reload_horde() {
    StoryflagManager::set_to_value(134, 0); // Unset horde defeated
    StoryflagManager::set_to_value(347, 0); // Unset horde cutscene
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(13, 1); // Give Master Sword (unless player already has a higher sword)
    reloader::trigger_entrance(
        b"F403\0".as_ptr(),
        1,
        13, // Layer 13 (horde cutscene)
        0, // Entrance 0
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(80); // Full refill, whatever the file's max health happens to be
}

fn reload_g3() {
    StoryflagManager::set_to_value(347, 1); // Set horde cutscene (for barriers)
    StoryflagManager::set_to_value(225, 0); // Unset G3 defeated
    StoryflagManager::set_to_value(348, 0); // Unset G3 cutscene
    StoryflagManager::set_to_value(30, 1); // Give Pouch Storyflag
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(112, 1); // Give Pouch itemflag
    ItemflagManager::set_to_value(11, 1); // Give Goddess Sword (unless player already has a higher sword)
    let current_file = file_manager::get_file_A();
    current_file.pouch_items[0] = 0x100074; // Wooden Shield
    current_file.shield_pouch_slot = 0;
    current_file.lastUsedPouchItemSlot = 0;
    reloader::trigger_entrance(
        b"F403\0".as_ptr(),
        1,
        14, // Layer 14 (G3 cutscene)
        2, // Entrance 2
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(80); // Full refill, whatever the file's max health happens to be
}

fn reload_demise() {
    let current_file = file_manager::get_file_A();
    StoryflagManager::set_to_value(30, 1); // Give Pouch Storyflag
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(112, 1); // Give Pouch itemflag
    ItemflagManager::set_to_value(20, 1); // Give Clawshots :)
    ItemflagManager::set_to_value(11, 1); // Give Goddess Sword (unless player already has a higher sword)
    current_file.pouch_items[0] = 0x100074; // Wooden Shield
    current_file.shield_pouch_slot = 0;
    current_file.lastUsedPouchItemSlot = 0;
    reloader::trigger_entrance(
        b"B400\0".as_ptr(),
        0,
        1, // Layer 1 (boss fight)
        0,
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(80); // Full refill, whatever the file's max health happens to be
}
/*
fn reload_imprisoned(fight: u16) {
    // Set flag for this imp fight, unset other flags
    const BASE_TRIGGER: u16 = 143;
    const BASE_DEFEATED: u16 = 131;
    const BASE_SCENE_TRIGGER: u16 = 10;
    for idx in 1..=3 {
        // StoryflagManager::set_to_value(BASE_TRIGGER + idx - 1, (idx == fight) as u16);
        // StoryflagManager::set_to_value(BASE_DEFEATED + idx - 1, (idx < fight) as u16); // Boss defeated
        /*
        if idx == fight {
            SceneflagManager::set_global(10, BASE_SCENE_TRIGGER + idx - 1);
        } else {
            SceneflagManager::unset_global(10, BASE_SCENE_TRIGGER + idx - 1);
        }
        */
    }
    StoryflagManager::set_to_value(11, (fight != 1) as u16); // Groosenator tracks
    SceneflagManager::set_global(10, 7); // Air vents
    if fight != 1 {
        SceneflagManager::set_global(10, 91); // Another flag
    }
    if fight == 3 {
       //  SceneflagManager::set_global(10, 108); // Another flag
    }
    SceneflagManager::unset_global(10, 9); // Big air vent

    // Give at least the appropriate sword, and set/unset certain flags
    match fight {
        1 => {
            ItemflagManager::set_to_value(11, 1);
            // Setting story flags set in boss rush
            StoryflagManager::set_to_value(703, 1);
            StoryflagManager::set_to_value(13, 0);
            StoryflagManager::set_to_value(131, 0);
            StoryflagManager::set_to_value(136, 1);
            StoryflagManager::set_to_value(71, 0);
            StoryflagManager::set_to_value(149, 0);
        },
        2 => {
            ItemflagManager::set_to_value(13, 1);
            // Setting story flags set in boss rush
            StoryflagManager::set_to_value(704, 1);
            StoryflagManager::set_to_value(132, 0);
            StoryflagManager::set_to_value(136, 1);
            // these were set by the flow
            /*
            SceneflagManager::unset_global(10, 26);
            StoryflagManager::set_to_value(796, 1);
            SceneflagManager::set_global(10, 36);
            SceneflagManager::unset_global(10, 30);
            SceneflagManager::set_global(10, 81);
            StoryflagManager::set_to_value(795, 0);
            */
            // StoryflagManager::set_to_value(144, 1);
        },
        3 => {
            ItemflagManager::set_to_value(14, 1);
            // Setting story flags set in boss rush
            StoryflagManager::set_to_value(705, 1);
            StoryflagManager::set_to_value(136, 1);
            // StoryflagManager::set_to_value(145, 1);
            // these were set by the flow
            /*
            SceneflagManager::unset_global(10, 108);
            StoryflagManager::set_to_value(796, 1);
            SceneflagManager::set_global(10, 36);
            StoryflagManager::set_to_value(145, 1);
            */
        },
        _ => {},
    }

    StoryflagManager::do_commit();
    let current_file = file_manager::get_file_A();
    // Positioned for the cutscene trigger
    // current_file.pos_t1.x = -772.0;
    // current_file.pos_t1.y = 1.0;
    // current_file.pos_t1.z = -153.0;
    reloader::trigger_entrance(
        b"F401\0".as_ptr(),
        1,
        0, // 1 + (fight as u8), // 2 for imp1, 3 for imp2, 4 for imp3
        7, // Entrance 7
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    // reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
    file_manager::set_current_health(80); // Full refill, whatever the file's max health happens to be
}

fn reload_imp1() {
    reload_imprisoned(1);
}
fn reload_imp2() {
    reload_imprisoned(2);
}
fn reload_imp3() {
    reload_imprisoned(3);
}
*/

fn reload_bilocyte() {
    StoryflagManager::set_to_value(364, 1); // Spiral Charge
    StoryflagManager::set_to_value(366, 1);
    StoryflagManager::set_to_value(368, 1);
    // StoryflagManager::set_to_value(288, 1); // Triggered Bilocyte fight
    StoryflagManager::set_to_value(200, 0); // Defeated Bilocyte fight
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(19, 1); // Give Bow
    ItemflagManager::increase_counter(1, 20); // Refill Arrows
    ItemflagManager::set_to_value(14, 1); // Give True Master Sword
    /*
    let current_file = file_manager::get_file_A();
    current_file.pos_t1.x = -111223.0;
    current_file.pos_t1.y = -1206.0;
    current_file.pos_t1.z = -89997.0;
    current_file.angle_t1 = 0;
    */
    reloader::trigger_entrance(
        b"F023\0".as_ptr(),
        0,
        1,
        1,
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    // reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}


fn set_sword_to_goddess() {
    ItemflagManager::set_to_value(11, 1); // Give Goddess Sword
    // Remove higher-level swords
    ItemflagManager::set_to_value(12, 0);
    ItemflagManager::set_to_value(9, 0);
    ItemflagManager::set_to_value(13, 0);
    ItemflagManager::set_to_value(14, 0);
    ItemflagManager::do_commit();
}

fn reload_eb() {
    StoryflagManager::set_to_value(58, 1); // Give B-Wheel
    StoryflagManager::do_commit();
    ItemflagManager::set_to_value(52, 1); // Give Slingshot
    ItemflagManager::increase_counter(4, 20); // Refill Seeds
    set_sword_to_goddess();
    let current_file = file_manager::get_file_A();
    // Positioned for EB
    current_file.pos_t1.x = -450.0;
    current_file.pos_t1.y = 2405.0;
    current_file.pos_t1.z = 15000.0;
    current_file.angle_t1 = 32000;
    reloader::trigger_entrance(
        b"F101\0".as_ptr(),
        0,
        1,
        2, // Entrance 2 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    reloader::set_reloader_type(1);
    reloader::set_reload_trigger(5);
}

const CSWW_SAVE: &str = "/saves/All Dungeons/Gate of Time Skip";

fn reload_csww() {
    load_practice_save(CSWW_SAVE);
}

fn load_csww_entrance() {
    reloader::trigger_entrance(
        b"F402\0".as_ptr(),
        2,
        2,
        2, // Entrance 2 (for no entrance animation)
        0,
        0,
        0,
        0xF,
        0xFF,
    );
    file_manager::set_current_health(0);

    reloader::set_reload_trigger(5);
}

fn eval_death_trick_fail() {
    let buffer = unsafe {A_PRESS_BUFFER};
    let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    // let _ = console.write_fmt(format_args!("{:08X} - ", buffer));
    if buffer >= 0x10000 {
        // early
        let frames = 16 - buffer.leading_zeros();
        match frames {
            1 => console.set_font_color(0xFFFF00FF),
            2 => console.set_font_color(0xFFC000FF),
            _ => console.set_font_color(0xFF0000FF),
        };
        let _ = console.write_fmt(format_args!("{} {} early", frames, if frames == 1 {"frame"} else {"frames"}));
    } else {
        // late
        let frames = buffer.leading_zeros() - 17;
        match frames {
            1 => console.set_font_color(0xFFFF00FF),
            2 => console.set_font_color(0xFFC000FF),
            _ => console.set_font_color(0xFF0000FF),
        };
        let _ = console.write_fmt(format_args!("{} {} late", frames, if frames == 1 {"frame"} else {"frames"}));
    }
    let _ = console.write_fmt(format_args!("\nTry again by pressing D-Pad Left."));
    console.draw(false);
}

fn eval_death_trick_success(first_frame: bool) {
    let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    console.set_font_color(0x00FF00FF);
    // let _ = console.write_fmt(format_args!("{:08X} - ", buffer));
    let _ = console.write_fmt(format_args!("got death trick ({} frame)", if first_frame {"first"} else {"second"}));
    // let _ = console.write_fmt(format_args!("\nTry again by pressing D-Pad Left."));
    console.draw(false);
}

fn eval_file_dupe(first_frame: bool) {
    let mut console = Console::with_pos_and_size(0f32, 378f32, 120f32, 60f32);
    console.set_bg_color(0x0000007F);
    console.set_font_size(0.5f32);
    console.set_dynamic_size(true);
    console.set_font_color(0x00FF00FF);
    // let _ = console.write_fmt(format_args!("{:08X} - ", buffer));
    let _ = console.write_fmt(format_args!("got death trick ({} frame)\n", if first_frame {"first"} else {"second"}));
    let a_buffer = unsafe {A_PRESS_BUFFER};
    let two_buffer = unsafe {TWO_HOLD_BUFFER};

    let two_hold_start = two_buffer.leading_zeros() as i32;
    let relevant_a_mask = (1 << 13) - 1;
    let a_press = a_buffer.leading_zeros() as i32;
    let relevant_a_press = (a_buffer & relevant_a_mask).leading_zeros() as i32;
    let frame_diff = relevant_a_press - two_hold_start;
    // let _ = console.write_fmt(format_args!("{:08X} ({}) - ", a_buffer, relevant_a_press));
    // let _ = console.write_fmt(format_args!("{:08X} ({}) - ", two_buffer, two_hold_start));
    if two_hold_start > 22 {
        let frames = two_hold_start - 22;
        match frames {
            1 => console.set_font_color(0xFFFF00FF),
            2 => console.set_font_color(0xFFC000FF),
            _ => console.set_font_color(0xFF0000FF),
        };
        let _ = console.write_fmt(format_args!("2 press was {} {} late", frames, if frames == 1 {"frame"} else {"frames"}));
    } else if frame_diff > 2 {
        if a_press - two_hold_start <= 2 {
            // it's possible the 2 press was also too early for this to be possible at all, but the A press definitely was too
            let frames = 19 - a_press;
            match frames {
                1 => console.set_font_color(0xFFFF00FF),
                2 => console.set_font_color(0xFFC000FF),
                _ => console.set_font_color(0xFF0000FF),
            };
            let _ = console.write_fmt(format_args!("A press was {} {} early", frames, if frames == 1 {"frame"} else {"frames"}));
        } else {
            let frames = frame_diff - 2;
            match frames {
                1 => console.set_font_color(0xFFFF00FF),
                2 => console.set_font_color(0xFFC000FF),
                _ => console.set_font_color(0xFF0000FF),
            };
            let _ = console.write_fmt(format_args!("A press was {} {} too late relative to the 2 press", frames, if frames == 1 {"frame"} else {"frames"}));
        }
    } else {
        let _ = console.write_fmt(format_args!("got file dupe"));
    }
    let _ = console.write_fmt(format_args!("\nTry again by pressing D-Pad Left."));
    console.draw(false);
}

pub fn update_tricks() {
    let tricks_menu: &mut TricksMenu = unsafe { &mut TRICKS_MENU };

    match tricks_menu.active_trick {
        ActiveTrick::None => {},
        ActiveTrick::Wccs => {
            check_wccs();
            if ButtonBuffer::check_combo_down_up(DPAD_LEFT, C) {
                reload_wccs_prompt();
            } else if let Some(link) = player::as_mut() {
                if link.pos.z < 5205f32 {
                    // Link dies and falls over after successful WCCS, reload
                    link.pos.z = 5300f32;
                    reload_wccs_prompt();
                }
            }
        },
        ActiveTrick::Guay => {
            let health = file_manager::get_current_health();
            // Auto-reload on successful deathwarp
            if ButtonBuffer::check_combo_down_up(DPAD_LEFT, C) || health == 0 {
                reload_guay();
            }
        },
        ActiveTrick::KeeseYeet => {
            // Auto-reload on successful Keese Yeet
            if ButtonBuffer::check_combo_down_up(DPAD_LEFT, C) || SceneflagManager::check_global(14, 29) {
                reload_keese_yeet();
            } else if let Some(link) = player::as_mut() {
                if link.pos.x >= 4999f32 && link.pos.z <= 3451f32 && link.angle.y == -16384 {
                    // Position failed to load somehow, so reload again
                    link.pos.x = 4900f32;
                    reload_keese_yeet();
                }
            }
        },
        ActiveTrick::ExtendingBlow => {
            if ButtonBuffer::check_combo_down_up(DPAD_LEFT, C) {
                reload_eb();
            } else if let Some(link) = player::as_mut() {
                if link.pos.z < 2500f32 {
                    // Successfully got EB
                    link.pos.z = 4000f32;
                    reload_eb();
                }
            }
        },
        ActiveTrick::Ghirahim1 => {
            if is_pressed(DPAD_LEFT) || is_boss_dead() {
                reload_g1();
            }
            DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
            display_boss_health("Ghirahim");
        },
        ActiveTrick::Scaldera => {
            if is_pressed(DPAD_LEFT) || is_boss_dead() {
                reload_scaldera();
            }
    
            if let Some(link) = player::as_mut() {
                // Bounding box near cutscene trigger
                let should_set_zoneflags = link.pos.x > 0f32 && link.pos.y > 7400f32 && link.pos.z < -20000f32 && link.pos.y < 7600f32;

                if should_set_zoneflags {
                    // No idea why, but setting these zoneflags allows skipping Ghirahim's text
                    SceneflagManager::set_zone_flag(10, 193, true);
                    SceneflagManager::set_zone_flag(10, 194, true);
                    SceneflagManager::set_zone_flag(10, 195, true);
                }

                if SceneflagManager::check_zone_flag(10, 192) && !SceneflagManager::check_zone_flag(10, 195) {
                    // We need to unset this flag if it's already set on load (from a post-Scaldera file), but 
                    // NOT if we manually triggered the fight already.
                    SceneflagManager::set_zone_flag(10, 192, false);
                }

                DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
                display_boss_health("Scaldera");
            }
        },
        ActiveTrick::Moldarach => {
            DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
            if is_pressed(DPAD_LEFT) || SceneflagManager::check_global(17, 120) {
                reload_moldarach();
            }
        },
        ActiveTrick::Koloktos => {
            if is_pressed(DPAD_LEFT) || is_boss_dead() {
                reload_koloktos();
            }
            // Setting these zoneflags skips straight to the Koloktos spawning cutscene
            SceneflagManager::set_zone_flag(0, 193, true);
            SceneflagManager::set_zone_flag(0, 210, true);
            DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
        },
        ActiveTrick::Tentalus => {
            // This scene flag sets super late though :(
            if is_pressed(DPAD_LEFT) || SceneflagManager::check_global(18, 82) {
                reload_tentalus();
            }

            DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
        },
        ActiveTrick::Ghirahim2(state) => {
            if is_pressed(DPAD_LEFT) {
                tricks_menu.active_trick = ActiveTrick::Ghirahim2(G2State::Phase1);
                reload_g2();
            }
            // State transitions
            match state {
                G2State::Phase1 => {
                    if is_boss_dead() {
                        // Health at 0, move to next state
                        tricks_menu.active_trick = ActiveTrick::Ghirahim2(G2State::BetweenPhases);
                    }
                },
                G2State::BetweenPhases => {
                    if !is_boss_dead() {
                        // Health is nonzero, second phase has begun
                        tricks_menu.active_trick = ActiveTrick::Ghirahim2(G2State::Phase2);
                    }
                },
                G2State::Phase2 => {
                    if is_boss_dead() {
                        // Health is 0 again, boss defeated
                        tricks_menu.active_trick = ActiveTrick::Ghirahim2(G2State::Phase1);
                        reload_g2();
                    }
                },
            };
            
            DungeonflagManager::set_to_value(3, 0); // Unset boss beaten dungeonflag
            display_boss_health("Ghirahim");
        },
        ActiveTrick::Bilocyte => {
            // This story flag sets super late though :(
            if is_pressed(DPAD_LEFT) || StoryflagManager::check(200) {
                reload_bilocyte();
            }
        },
        ActiveTrick::Horde => {
            if is_pressed(DPAD_LEFT) || StoryflagManager::check(134) {
                reload_horde();
            }
        },
        ActiveTrick::Ghirahim3 => {
            // Hylia's Realm layer 15 = post-G3 cutscene
            if is_pressed(DPAD_LEFT) || reloader::get_spawn_slave().layer == 15 {
                reload_g3();
            }
        },
        ActiveTrick::Demise => {
            // Demise arena layer 14 = post-Demise cutscene
            if is_pressed(DPAD_LEFT) || reloader::get_spawn_slave().layer == 14 {
                reload_demise();
            }
        },
        ActiveTrick::Csww(state) => {
            let frame_count = unsafe { FRAME_COUNT };
            match state {
                CswwState::DoingReset => {
                    if frame_count == 1 {
                        tricks_menu.active_trick = ActiveTrick::Csww(CswwState::DeathTrick);
                        load_csww_entrance();
                    }
                },
                CswwState::DeathTrick => {
                    if is_pressed(DPAD_LEFT) {
                        tricks_menu.active_trick = ActiveTrick::Csww(CswwState::DoingReset);
                        reloader::soft_reset();
                    } else if frame_count < CSWW_TIMER_CONTINUE_FRAME {
                        update_buffer();
                    } else {
                        let spawn_master = get_spawn_master();
                        if spawn_master.name[1] == b'2' { // F200 = Eldin, the stage it's momentarily using during death trick
                            let buffer = unsafe {&mut A_PRESS_BUFFER};
                            tricks_menu.active_trick = ActiveTrick::Csww(CswwState::FileDupe(*buffer & 0x8000 != 0));
                            *buffer = 0;
                        } else {
                            tricks_menu.active_trick = ActiveTrick::Csww(CswwState::Failed);
                        }
                    }
                },
                CswwState::FileDupe(first_frame) => {
                    eval_death_trick_success(first_frame);

                    if frame_count == CSWW_FILE_DUPE_END_FRAME {
                        tricks_menu.active_trick = ActiveTrick::Csww(CswwState::FileDupeEval(first_frame));
                    } else {
                        update_buffer();
                    }
                },
                CswwState::Failed => {
                    if is_pressed(DPAD_LEFT) {
                        tricks_menu.active_trick = ActiveTrick::Csww(CswwState::DoingReset);
                        reloader::soft_reset();
                    }

                    eval_death_trick_fail();
                },
                CswwState::FileDupeEval(first_frame) => {
                    if is_pressed(DPAD_LEFT) {
                        tricks_menu.active_trick = ActiveTrick::Csww(CswwState::DoingReset);
                        reloader::soft_reset();
                    }

                    eval_file_dupe(first_frame);
                }

            }
        },
    }
}

