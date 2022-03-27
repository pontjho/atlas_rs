#[derive(Clone, Debug)]
pub struct Region
{
    pub name: String,
    pub index: Option<usize>,
    pub bounds: (usize, usize, usize, usize),
    pub offsets: (usize, usize, usize, usize),
    pub rotate: usize,
    pub split: Option<(usize, usize, usize, usize)>,
    pub pad: Option<(isize, isize, isize, isize)>
}

impl Default for Region
{
    fn default() -> Self
    {
        Region {
            name: "".to_string(),
            index: None,
            bounds: (0, 0, 0, 0),
            offsets: (0, 0, 0, 0),
            rotate: 0,
            split: None,
            pad: None
        }
    }
}

impl Region
{
    pub fn new(name: &str) -> Self
    {
        Region { name: name.to_string(), ..Region::default() }
    }
}
