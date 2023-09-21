pub mod colors {

    use crate::Color; // if Color is defined in another module but in the same crate
    pub const RICH_BLACK: Color = Color::new(3, 3, 3, 1.0);
    pub const EERIE_BLACK: Color = Color::new(23, 23, 23, 1.0);
    pub const EERIE_BLACK_LIGHTEST: Color = Color::new(45, 45, 45, 1.0);
    pub const EERIE_BLACK_LIGHTEST_TRANSPARENT: Color = Color::new(45, 45, 45, 0.9);
    pub const EERIE_BLACK_LIGHTER: Color = Color::new(36, 36, 36, 1.0);
    pub const EERIE_BLACK_LIGHTER_TRANSPARENT: Color = Color::new(36, 36, 36, 0.9);
    pub const EERIE_BLACK_DARKER: Color = Color::new(18, 18, 18, 1.0);
    pub const EERIE_BLACK_DARKER_TRANSPARENT: Color = Color::new(18, 18, 18, 0.9);
    pub const CHARLESTON_GREEN: Color = Color::new(44, 44, 44, 1.0);
    pub const DARK_MEDIUM_GRAY: Color = Color::new(170, 170, 170, 1.0);
    pub const PLATINUM: Color = Color::new(233, 233, 233, 1.0);
    pub const MIDDLE_GREEN: Color = Color::new(82, 170, 94, 1.0);
    pub const TURQUOISE_GREEN: Color = Color::new(160, 208, 167, 1.0);
    pub const AMARANTH: Color = Color::new(239, 45, 86, 1.0);
}
