
pub enum Gain {
    High = 0b0000_1110,   // GN2 = 0, GN1 = 1, GN0 = 1 (2.5 Ga)
    Default = 0b0000_0010, // GN2 = 0, GN1 = 0, GN0 = 1 (1.3 Ga, oletus)
    Low = 0b0000_0000,     // GN2 = 0, GN1 = 0, GN0 = 0 (0.88 Ga)
}