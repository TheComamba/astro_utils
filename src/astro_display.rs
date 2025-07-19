use astro_coords::direction::Direction;

pub trait AstroDisplay {
    fn astro_display(&self) -> String;
}

impl AstroDisplay for f64 {
    fn astro_display(&self) -> String {
        format!("{:.2}", self)
    }
}

impl AstroDisplay for String {
    fn astro_display(&self) -> String {
        self.clone()
    }
}

impl AstroDisplay for Direction {
    fn astro_display(&self) -> String {
        self.to_string()
    }
}

impl<T> AstroDisplay for Option<T>
where
    T: AstroDisplay,
{
    fn astro_display(&self) -> String {
        match self {
            Some(value) => value.astro_display(),
            None => String::new(),
        }
    }
}

impl<T> AstroDisplay for &T
where
    T: AstroDisplay,
{
    fn astro_display(&self) -> String {
        (*self).astro_display()
    }
}
