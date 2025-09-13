pub struct Obscura {
    prompt: String,
    maps: [String],
    current_map: u8,
    story: HaspMap<u8, String>, // story progression per level
}
