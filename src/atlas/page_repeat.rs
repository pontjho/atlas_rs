use strum_macros::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum PageRepeat
{
    None,
    X,
    Y,
    XY
}
