use nalgebra as na;
use crate::uv::UV;

type Vector3f=na::Vector3<f32>;

#[derive(Debug)]
pub struct Block{
    pub vertex_positions: Vec<Vector3f>,
    pub uvs: Vec<UV>,
    pub texture_ids: Vec<i32>,
    pub enabled: bool,
}

impl Block{
    pub fn new()->Self{
        let mut vertex_positions=vec![];
        for _ in 0..8{
            vertex_positions.push(Vector3f::new(0.0, 0.0, 0.0));
        }

        let mut uvs=vec![];
        for _ in 0..24{
            uvs.push(UV{u: 0.0,v:0.0});
        }

        let mut texture_ids=vec![];
        for _ in 0..6{
            texture_ids.push(0);
        }

        Block{
            vertex_positions: vertex_positions,
            uvs: uvs,
            texture_ids: texture_ids,
            enabled: true,
        }
    }
}

impl Clone for Block{
    fn clone(&self)->Self{
        let vertex_positions=self.vertex_positions
            .iter()
            .map(|v| Vector3f::new(v.x, v.y, v.z))
            .collect();
        let uvs=self.uvs
            .iter()
            .map(|val| UV{u: val.u,v: val.v})
            .collect();
        let texture_ids=self.texture_ids.clone();

        Block{
            vertex_positions: vertex_positions,
            uvs: uvs,
            texture_ids: texture_ids,
            enabled: self.enabled,
        }
    }
}
