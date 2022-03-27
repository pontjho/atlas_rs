use strum_macros::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
pub enum PageFilter
{
    Nearest,
    Linear,
    MipMap,
    MipMapNearestNearest,
    MipMapLinearNearest,
    MipMapNearestLinear,
    MipMapLinearLinear
}
