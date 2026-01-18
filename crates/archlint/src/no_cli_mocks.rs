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
        #[must_use]
        pub const fn red(self) -> Self {
            self
        }
        #[must_use]
        pub const fn green(self) -> Self {
            self
        }
        #[must_use]
        pub const fn yellow(self) -> Self {
            self
        }
        #[must_use]
        pub const fn blue(self) -> Self {
            self
        }
        #[must_use]
        pub const fn magenta(self) -> Self {
            self
        }
        #[must_use]
        pub const fn cyan(self) -> Self {
            self
        }
        #[must_use]
        pub const fn white(self) -> Self {
            self
        }
        #[must_use]
        pub const fn attr(self, _: Attribute) -> Self {
            self
        }
        #[must_use]
        pub const fn fg(self, _: Color) -> Self {
            self
        }
        #[must_use]
        pub const fn for_stderr(self) -> Self {
            self
        }
        #[must_use]
        pub const fn dim(self) -> Self {
            self
        }
        #[must_use]
        pub const fn bold(self) -> Self {
            self
        }
        #[must_use]
        pub const fn underlined(self) -> Self {
            self
        }
    }
    pub const fn style<T>(t: T) -> MockStyled<T> {
        MockStyled(t)
    }

    pub struct Term;
    impl Term {
        #[must_use]
        pub const fn stdout() -> Self {
            Self
        }
        #[must_use]
        pub const fn is_term(&self) -> bool {
            false
        }
    }
}

#[cfg(not(feature = "cli"))]
pub mod indicatif {
    pub struct ProgressBar;
    impl ProgressBar {
        #[must_use]
        pub const fn new(_: u64) -> Self {
            Self
        }
        #[must_use]
        pub const fn new_spinner() -> Self {
            Self
        }
        pub const fn set_style(&self, _: ProgressStyle) {}
        pub const fn inc(&self, _: u64) {}
        pub const fn finish_and_clear(&self) {}
        pub fn set_message<S: Into<std::borrow::Cow<'static, str>>>(&self, _: S) {}
        pub fn println<T: std::fmt::Display>(&self, s: T) {
            log::info!("{s}");
        }
        pub const fn enable_steady_tick(&self, _: std::time::Duration) {}
    }
    pub struct ProgressStyle;
    impl ProgressStyle {
        #[must_use]
        pub const fn default_bar() -> Self {
            Self
        }
        #[must_use]
        pub const fn default_spinner() -> Self {
            Self
        }
        pub const fn template(self, _: &str) -> Result<Self, String> {
            Ok(self)
        }
        #[must_use]
        pub const fn progress_chars(self, _: &str) -> Self {
            self
        }
        #[must_use]
        pub const fn tick_chars(self, _: &str) -> Self {
            self
        }
        #[must_use]
        pub const fn unwrap(self) -> Self {
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
        #[must_use]
        pub const fn new() -> Self {
            Self
        }
        pub const fn load_preset(&mut self, (): ()) -> &mut Self {
            self
        }
        pub const fn apply_modifier(&mut self, (): ()) -> &mut Self {
            self
        }
        pub const fn set_content_arrangement(&mut self, _: ContentArrangement) -> &mut Self {
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
        #[must_use]
        pub fn new<T: std::fmt::Display>(_: T) -> Self {
            Self
        }
        #[must_use]
        pub const fn add_attribute(self, _: Attribute) -> Self {
            self
        }
        #[must_use]
        pub const fn fg(self, _: Color) -> Self {
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
