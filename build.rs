fn main() {
    #[cfg(feature = "ui")]
    slint_build::compile("src/ui/calendar.slint").unwrap();
}
