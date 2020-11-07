use crate::ProvinceData;
use std::ops::Index;

#[derive(Debug)]
pub struct Province {
    pub name: String,
    pub population: u32,
    pub density_per_square_km: u16,
    pub connected_provinces: Vec<usize>
}

#[derive(Debug)]
pub struct ProvinceGraph {
    nodes: Vec<Province>
}

impl ProvinceGraph {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl Index<usize> for ProvinceGraph {
    type Output = Province;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl From<Vec<ProvinceData>> for ProvinceGraph {
    fn from(provinces: Vec<ProvinceData>) -> Self {
        let mut graph = Self {
            nodes: vec![]
        };

        for province in &provinces {
            let mut node = Province {
                name: province.name.clone(),
                population: province.population,
                density_per_square_km: province.density_per_square_km,
                connected_provinces: vec![]
            };
            graph.nodes.push(node);
        }

        for province in &provinces {
            for connected in &province.connected_provinces {
                let connected_idx = graph.nodes
                    .iter()
                    .enumerate()
                    .find(|(i, p)| p.name == *connected)
                    .map(|(i,p)| i)
                    .unwrap();
                let mut node = graph.nodes
                    .iter_mut()
                    .find(|p|p.name == province.name)
                    .unwrap();
                node.connected_provinces.push(connected_idx);
            }
        }

        graph
    }
}

impl IntoIterator for ProvinceGraph {
    type Item = Province;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<'a> IntoIterator for &'a ProvinceGraph {
    type Item = &'a Province;
    type IntoIter = std::slice::Iter<'a, Province>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter()
    }
}

impl<'a> IntoIterator for &'a mut ProvinceGraph {
    type Item = &'a mut Province;
    type IntoIter = std::slice::IterMut<'a, Province>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter_mut()
    }
}