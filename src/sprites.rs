pub mod ui {
    pub mod buttons {
        pub mod active {
            pub const STOP: i32 = 64;
            pub const NORMAL: i32 = 66;
            pub const FAST: i32 = 68;
        }

        pub mod highlighted {
            pub const STOP: i32 = 32;
            pub const NORMAL: i32 = 34;
            pub const FAST: i32 = 36;
        }

        pub mod inactive {
            pub const STOP: i32 = 0;
            pub const NORMAL: i32 = 2;
            pub const FAST: i32 = 4;
        }
    }
}
