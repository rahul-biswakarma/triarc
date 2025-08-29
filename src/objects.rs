pub enum PlatformTile {
    Solid,
    Disappear,
}

pub enum PlaformTileTrigger {
    Step,
}

pub fn renderer(tile: PlatformTile) -> &'static str {
    match tile {
        PlatformTile::Solid => "████",
        PlatformTile::Disappear => "     ",
    }
}
