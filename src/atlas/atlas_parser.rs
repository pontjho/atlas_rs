use crate::atlas::page::Page;

pub trait AtlasParser
{
    fn parse(&self, lines: std::slice::Iter<&str>) -> Vec<Page>;
}
