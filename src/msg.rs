use std::{
    fmt,
    fmt::{Debug, Formatter}
};

#[derive(Clone)]
pub struct Message {
    pub kind: Kind,
    pub importance: Importance,
    pub hidden: bool,
    pub contents: Vec<Text>
}

#[derive(Copy, Clone)]
pub enum Kind {
    Display,
    Debug,
    Warning,
    Error
}

#[derive(Copy, Clone)]
pub enum Importance {
    Hidden,
    Verbose,
    Low,
    Normal,
    High,
    VeryHigh
}

#[derive(Clone)]
pub struct Text {
    pub bold: bool,
    pub italic: bool,
    pub color: Color,
    pub background_color: Color,
    pub text: String
}

#[derive(Copy, Clone)]
pub enum Color {
    Default,
    White,
    Gray,
    Black,
    Red,
    Orange,
    Yellow,
    Green,
    Pink,
    Blue,
    Rgb(u8, u8, u8)
}

/// Some shortcuts for initializing common types of messages.
impl Message {
    /// A "normal" message is intended for displaying to the user, has normal importance, and is not
    /// hidden.
    pub fn normal(contents: Vec<Text>) -> Self {
        Message {
            kind: Kind::Display,
            importance: Importance::Normal,
            hidden: false,
            contents
        }
    }
}

/// Some shortcuts for initializing common formats for text within messages.
impl Text {
    /// TODO: document
    pub fn normal(text: &str) -> Self {
        Text {
            bold: false,
            italic: false,
            color: Color::Default,
            background_color: Color::Default,
            text: String::from(text)
        }
    }

    /// TODO: document
    pub fn bold(text: &str) -> Self {
        Text {
            bold: true,
            italic: false,
            color: Color::Default,
            background_color: Color::Default,
            text: String::from(text)
        }
    }

    /// TODO: document
    pub fn italic(text: &str) -> Self {
        Text {
            bold: false,
            italic: true,
            color: Color::Default,
            background_color: Color::Default,
            text: String::from(text)
        }
    }
}

// Debug implementations for Message and Text - for testing/debugging

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for text in &self.contents {
            f.write_str(&format!("{:?} ", text))?;
        }

        Ok(())
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}