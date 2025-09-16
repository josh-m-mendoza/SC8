use canparse::pgn::{PgnLibrary, SpnDefinition, ParseMessage};

fn main() {

    // Parse dbc file into PgnLibrary
    let lib = PgnLibrary::from_dbc_file("./example.dbc").unwrap();

    // Pull signal definition for engine speed
    let enginespeed_def: &SpnDefinition = lib
        .get_spn("Engine_Speed").unwrap();

    // Parse frame containing engine speed
    let msg: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let engine_speed: f32 = enginespeed_def.parse_message(&msg).unwrap();

    println!("Engine speed: {}", engine_speed);
}