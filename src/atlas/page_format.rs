use strum_macros::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
pub enum PageFormat
{
    Alpha,
    Intensity,
    LuminanceAlpha,
    RGB565,
    RGBA4444,
    RGB888,
    RGBA8888
}
