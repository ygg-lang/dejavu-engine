use core::fmt::{Display, Formatter, Write};

pub trait Escaper {
    fn write_escaped<W>(&self, fmt: W, string: &str) -> core::fmt::Result
        where
            W: Write;
}


#[derive(Debug)]
pub struct EscapeDisplay<E, T>
    where
        E: Escaper,
        T: Display,
{
    value: DisplayValue<T>,
    escaper: E,
}

#[derive(Debug)]
pub struct EscapeWriter<'a, E, W> {
    fmt: W,
    escaper: &'a E,
}

#[derive(Debug)]
pub struct Escaped<'a, E>
    where
        E: Escaper,
{
    string: &'a str,
    escaper: E,
}

impl<E, T> EscapeDisplay<E, T>
    where
        E: Escaper,
        T: Display,
{
    pub fn dangerous(value: T, escaper: E) -> Self {
        Self {
            value: DisplayValue::Unsafe(value),
            escaper,
        }
    }

    pub fn safe(value: T, escaper: E) -> Self {
        Self {
            value: DisplayValue::Safe(value),
            escaper,
        }
    }

    #[must_use]
    pub fn mark_safe(mut self) -> EscapeDisplay<E, T> {
        self.value = match self.value {
            DisplayValue::Unsafe(t) => DisplayValue::Safe(t),
            _ => self.value,
        };
        self
    }
}

impl<E, T> Display for EscapeDisplay<E, T>
    where
        E: Escaper,
        T: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
        match self.value {
            DisplayValue::Unsafe(ref t) => write!(
                EscapeWriter {
                    fmt,
                    escaper: &self.escaper,
                },
                "{t}"
            ),
            DisplayValue::Safe(ref t) => t.fmt(fmt),
        }
    }
}

impl<E, W> Write for EscapeWriter<'_, E, W>
    where
        W: Write,
        E: Escaper,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.escaper.write_escaped(&mut self.fmt, s)
    }
}

pub fn escape<E>(string: &str, escaper: E) -> Escaped<'_, E>
    where
        E: Escaper,
{
    Escaped { string, escaper }
}


impl<E> Display for Escaped<'_, E>
    where
        E: Escaper,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
        self.escaper.write_escaped(fmt, self.string)
    }
}


#[derive(Debug, PartialEq)]
enum DisplayValue<T>
    where
        T: Display,
{
    Safe(T),
    Unsafe(T),
}

