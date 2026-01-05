#[cfg(not(feature = "cli"))]
pub mod console {
    pub struct MockStyled<T>(pub T);
    impl<T: std::fmt::Display> std::fmt::Display for MockStyled<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
    pub enum Attribute {
        Bold,
        Italic,
        Underlined,
        Dim,
    }
    pub enum Color {
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
        Black,
    }

    impl<T> MockStyled<T> {
        pub fn red(self) -> Self {
            self
        }
        pub fn green(self) -> Self {
            self
        }
        pub fn yellow(self) -> Self {
            self
        }
        pub fn blue(self) -> Self {
            self
        }
        pub fn magenta(self) -> Self {
            self
        }
        pub fn cyan(self) -> Self {
            self
        }
        pub fn white(self) -> Self {
            self
        }
        pub fn attr(self, _: Attribute) -> Self {
            self
        }
        pub fn fg(self, _: Color) -> Self {
            self
        }
        pub fn for_stderr(self) -> Self {
            self
        }
        pub fn dim(self) -> Self {
            self
        }
        pub fn bold(self) -> Self {
            self
        }
        pub fn underlined(self) -> Self {
            self
        }
    }
    pub fn style<T>(t: T) -> MockStyled<T> {
        MockStyled(t)
    }

    pub struct Term;
    impl Term {
        pub fn stdout() -> Self {
            Self
        }
        pub fn is_term(&self) -> bool {
            false
        }
    }
}

#[cfg(not(feature = "cli"))]
pub mod indicatif {
    pub struct ProgressBar;
    impl ProgressBar {
        pub fn new(_: u64) -> Self {
            Self
        }
        pub fn new_spinner() -> Self {
            Self
        }
        pub fn set_style(&self, _: ProgressStyle) -> &Self {
            self
        }
        pub fn inc(&self, _: u64) {}
        pub fn finish_and_clear(&self) {}
        pub fn set_message<S: Into<std::borrow::Cow<'static, str>>>(&self, _: S) {}
        pub fn println<T: std::fmt::Display>(&self, s: T) {
            log::info!("{}", s);
        }
        pub fn enable_steady_tick(&self, _: std::time::Duration) {}
    }
    pub struct ProgressStyle;
    impl ProgressStyle {
        pub fn default_bar() -> Self {
            Self
        }
        pub fn default_spinner() -> Self {
            Self
        }
        pub fn template(self, _: &str) -> Result<Self, String> {
            Ok(self)
        }
        pub fn progress_chars(self, _: &str) -> Self {
            self
        }
        pub fn tick_chars(self, _: &str) -> Self {
            self
        }
        pub fn unwrap(self) -> Self {
            self
        }
    }
}

#[cfg(not(feature = "cli"))]
pub mod comfy_table {
    pub struct Table;
    impl Default for Table {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Table {
        pub fn new() -> Self {
            Self
        }
        pub fn load_preset(&mut self, _: ()) -> &mut Self {
            self
        }
        pub fn apply_modifier(&mut self, _: ()) -> &mut Self {
            self
        }
        pub fn set_content_arrangement(&mut self, _: ContentArrangement) -> &mut Self {
            self
        }
        pub fn set_header<T>(&mut self, _: T) -> &mut Self {
            self
        }
        pub fn add_row<T>(&mut self, _: T) -> &mut Self {
            self
        }
    }
    impl std::fmt::Display for Table {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[Table output disabled in library build]")
        }
    }
    pub mod presets {
        pub const UTF8_FULL: () = ();
    }
    pub mod modifiers {
        pub const UTF8_ROUND_CORNERS: () = ();
    }
    pub struct Cell;
    impl Cell {
        pub fn new<T: std::fmt::Display>(_: T) -> Self {
            Self
        }
        pub fn add_attribute(self, _: Attribute) -> Self {
            self
        }
        pub fn fg(self, _: Color) -> Self {
            self
        }
    }
    pub enum Attribute {
        Bold,
    }
    pub enum Color {
        Cyan,
        Green,
        DarkGrey,
        Red,
        Yellow,
        White,
    }
    pub enum ContentArrangement {
        Dynamic,
    }
}
