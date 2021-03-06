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

    pub fn finalise(self) -> Self
    {
        let (_, _, ow, oh) = self.offsets;
        let (_, _, bw, bh) = self.bounds;

        let offsets = if ow == 0 && oh == 0 { (0, 0, bw, bh) } else { self.offsets };
        Region {
            offsets,
            ..self
        }
    }
}
