use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use nalgebra as na;

use crate::block::Block;
use crate::reader::Reader;

type Vector3f=na::Vector3<f32>;
type Matrix4f=na::Matrix4<f32>;

pub struct BD1Manipulator{
    pub texture_filenames: HashMap<i32,String>,
    pub blocks: Vec<Block>,
    pub transformation_mat: Matrix4f,
}

impl BD1Manipulator{
    ///Creates an empty BD1 manipulator.
    pub fn new()->Self{
        BD1Manipulator{
            texture_filenames: HashMap::new(),
            blocks: Vec::new(),
            transformation_mat: Matrix4f::identity(),
        }
    }

    ///Loads a BD1 file and creates a BD1 manipulator.
    pub fn new_from_file(path: &Path)->Result<Self,Box<dyn Error>>{
        let reader=Reader::new(path)?;
        let manipulator=BD1Manipulator{
            texture_filenames: reader.texture_filenames,
            blocks: reader.blocks,
            transformation_mat: Matrix4f::identity(),
        };
        Ok(manipulator)
    }

    ///Applies transformation.
    pub fn apply_transformation(&mut self){
        self.blocks
            .iter_mut()
            .for_each(|block| {
                let mut new_vertex_positions=vec![];
                block.vertex_positions
                    .iter()
                    .for_each(|p| {
                        let new_vertex_position=self.transformation_mat.transform_vector(p);
                        new_vertex_positions.push(new_vertex_position);
                    });

                block.vertex_positions=new_vertex_positions;
            });
    }

    ///Resets transformation.
    pub fn reset_transformation(&mut self){
        self.transformation_mat=Matrix4f::identity();
    }

    ///Transforms the blocks with a matrix.
    pub fn transform(&mut self,mat: &Matrix4f)->&Self{
        self.transformation_mat=self.transformation_mat*mat;
        self
    }

    ///Translates the blocks.
    pub fn translate(&mut self,translation_x: f32,translation_y: f32,translation_z: f32)->&Self{
        let translation_mat=Matrix4f::new_translation(&Vector3f::new(translation_x, translation_y, translation_z));
        self.transform(&translation_mat)
    }

    /// Rotates the blocks around the X-axis.
    /// 
    /// # Arguments
    /// 
    /// * `th` - Rotaion angle in radian
    pub fn rot_x(&mut self,th: f32)->&Self{
        let rot_mat=Matrix4f::new_rotation(Vector3f::x()*th);
        self.transform(&rot_mat)
    }

    /// Rotates the blocks around the Y-axis.
    /// 
    /// # Arguments
    /// 
    /// * `th` - Rotation angle in radian
    pub fn rot_y(&mut self,th: f32)->&Self{
        let rot_mat=Matrix4f::new_rotation(Vector3f::y()*th);
        self.transform(&rot_mat)
    }

    /// Rotates the blocks around the Z-axis.
    /// 
    /// # Arguments
    /// 
    /// * `th` - Rotation angle in radian
    pub fn rot_z(&mut self,th: f32)->&Self{
        let rot_mat=Matrix4f::new_rotation(Vector3f::z()*th);
        self.transform(&rot_mat)
    }

    /// Rotates the blocks around an arbitrary axis.
    /// 
    /// # Arguments
    /// 
    /// * `th` - Rotation angle in radian
    /// * `axis_x` - X-component of the axis
    /// * `axis_y` - Y-component of the axis
    /// * `axis_z` - Z-component of the axis
    pub fn rot(&mut self,th: f32,axis_x: f32,axis_y: f32,axis_z: f32)->&Self{
        let rot_mat=Matrix4f::new_rotation(Vector3f::new(axis_x, axis_y, axis_z)*th);
        self.transform(&rot_mat)
    }

    /// Rescales the blocks.
    /// 
    /// # Arguments
    /// 
    /// * `scale_x` - X-axis scale
    /// * `scale_y` - Y-axis scale
    /// * `scale_z` - Z-axis scale
    pub fn rescale(&mut self,scale_x: f32,scale_y: f32,scale_z: f32)->&Self{
        let scale_mat=Matrix4f::new_nonuniform_scaling(&Vector3f::new(scale_x, scale_y, scale_z));
        self.transform(&scale_mat)
    }
}
