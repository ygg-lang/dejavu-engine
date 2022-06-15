use super::*;

#[derive(Debug)]
pub struct TemplateDisplay<E, T>
where
    E: Escaper,
    T: Display,
{
    value: DisplayValue<T>,
    escaper: E,
}

impl<E, T> TemplateDisplay<E, T>
where
    E: Escaper,
    T: Display,
{
    pub fn new_unsafe(value: T, escaper: E) -> Self {
        Self { value: DisplayValue::Unsafe(value), escaper }
    }

    pub fn new_safe(value: T, escaper: E) -> Self {
        Self { value: DisplayValue::Safe(value), escaper }
    }

    #[must_use]
    pub fn mark_safe(mut self) -> TemplateDisplay<E, T> {
        self.value = match self.value {
            DisplayValue::Unsafe(t) => DisplayValue::Safe(t),
            _ => self.value,
        };
        self
    }
}

impl<E, T> Display for TemplateDisplay<E, T>
where
    E: Escaper,
    T: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self.value {
            DisplayValue::Unsafe(ref t) => write!(EscapeWriter { fmt, escaper: &self.escaper }, "{}", t),
            DisplayValue::Safe(ref t) => t.fmt(fmt),
        }
    }
}

#[derive(Debug)]
pub struct EscapeWriter<'a, E, W> {
    fmt: W,
    escaper: &'a E,
}

impl<E, W> Write for EscapeWriter<'_, E, W>
where
    W: Write,
    E: Escaper,
{
    fn write_str(&mut self, s: &str) -> Result {
        self.escaper.write_escaped(&mut self.fmt, s)
    }
}

pub fn escape<E>(string: &str, escaper: E) -> Escaped<'_, E>
where
    E: Escaper,
{
    Escaped { string, escaper }
}

#[derive(Debug)]
pub struct Escaped<'a, E>
where
    E: Escaper,
{
    string: &'a str,
    escaper: E,
}

impl<E> Display for Escaped<'_, E>
where
    E: Escaper,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
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
