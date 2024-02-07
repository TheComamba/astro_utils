pub trait AstroDisplay {
    fn astro_display(&self) -> String;
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
