// This file defines a bunch of style enums
#[derive(Clone, PartialEq)]
pub enum Colors {
    Primary,
    Secondary,
    Success,
    Standard,
    Danger,
    Warning,
    Info,
    Dark,
    Light,
}
pub fn get_color(palette: &Colors) -> String {
    match palette {
        Colors::Primary => String::from("primary"),
        Colors::Secondary => String::from("secondary"),
        Colors::Success => String::from("success"),
        Colors::Standard => String::from("standard"),
        Colors::Danger => String::from("danger"),
        Colors::Warning => String::from("warning"),
        Colors::Info => String::from("info"),
        Colors::Dark => String::from("dark"),
        Colors::Light => String::from("light"),
    }
}
#[derive(Clone, PartialEq)]
pub enum Shapes {
    Rounded,
    RoundedTop,
    RoundedEnd,
    RoundedBottom,
    RoundedStart,
    RoundedCircle,
    RoundedPill,
    RoundedZero,
    RoundedOne,
    RoundedTwo,
    RoundedThree,
}
pub fn get_shape(shape: &Shapes) -> String {
    match shape {
        Shapes::Rounded => String::from("rounded"),
        Shapes::RoundedTop => String::from("rounded-top"),
        Shapes::RoundedEnd => String::from("rounded-end"),
        Shapes::RoundedBottom => String::from("rounded-bottom"),
        Shapes::RoundedStart => String::from("rounded-start"),
        Shapes::RoundedCircle => String::from("rounded-circle"),
        Shapes::RoundedPill => String::from("rounded-pill"),
        Shapes::RoundedZero => String::from("rounded-0"),
        Shapes::RoundedOne => String::from("rounded-1"),
        Shapes::RoundedTwo => String::from("rounded-2"),
        Shapes::RoundedThree => String::from("rounded-3"),
    }
}
/** Component Sizes */
#[derive(Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}
/** Styles for inputs */
#[derive(Clone, PartialEq)]
pub enum Style {
    Standard,
    Outline,
}

/** Returns a string to append to classes based on the color palette enum */

/** Returns a string to append to classes based on the size enum */
pub fn get_size(size: &Size) -> String {
    match size {
        Size::Small => String::from("small"),
        Size::Medium => String::from("medium"),
        Size::Large => String::from("large"),
    }
}
/** Returns a string to append to classes based on the style enum */
pub fn get_style(style: &Style) -> String {
    match style {
        Style::Standard => String::from("standard"),
        Style::Outline => String::from("outline"),
    }
}
