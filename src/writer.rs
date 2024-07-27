use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io::Write;
use std::{cmp, fs, io};
use std::path::Path;
use crate::block::Block;

pub struct Writer;

impl Writer{
    fn add_texture_filenames_to_bin(bin: &mut Vec<u8>,texture_filenames: &HashMap<i32,String>){
        let mut sorted_texture_filenames=BTreeMap::<i32,String>::new();
        for (k,v) in texture_filenames.iter(){
            sorted_texture_filenames.insert(*k, v.to_string());
        }

        let mut texture_count=0;
        for (_,v) in sorted_texture_filenames.iter(){
            let texture_filename_bytes=v.as_bytes();
            let mut texture_filename_buffer=[0u8;31];
            for i in 0..cmp::min(texture_filename_bytes.len(), 30){
                texture_filename_buffer[i]=texture_filename_bytes[i];
            }

            for i in 0..31{
                bin.push(texture_filename_buffer[i]);
            }

            texture_count+=1;
            if texture_count==10{
                break;
            }
        }

        for _ in texture_count..10{
            for _ in 0..31{
                bin.push(0u8);
            }
        }
    }

    pub fn write(path: &Path,blocks: &Vec<Block>,texture_filenames: &HashMap<i32,String>)->Result<(),Box<dyn Error>>{
        let mut bin=vec![];

        //Texture filenames
        Writer::add_texture_filenames_to_bin(&mut bin, texture_filenames);

        //Number of blocks
        let num_blocks=blocks.len();
        (num_blocks as u16).to_le_bytes().iter().for_each(|b| bin.push(*b));
        
        //Blocks
        for i in 0..num_blocks{
            let block=&blocks[i];

            //Vertex positions
            let vertex_positions=&block.vertex_positions;
            for j in 0..8{
                vertex_positions[j].x.to_le_bytes().iter().for_each(|b| bin.push(*b));
            }
            for j in 0..8{
                vertex_positions[j].y.to_le_bytes().iter().for_each(|b| bin.push(*b));
            }
            for j in 0..8{
                vertex_positions[j].z.to_le_bytes().iter().for_each(|b| bin.push(*b));
            }

            //UVs
            let uvs=&block.uvs;
            for j in 0..24{
                uvs[j].u.to_le_bytes().iter().for_each(|b| bin.push(*b));
            }
            for j in 0..24{
                uvs[j].v.to_le_bytes().iter().for_each(|b| bin.push(*b));
            }

            //Texture IDs
            let texture_ids=&block.texture_ids;
            for j in 0..6{
                (texture_ids[j] as u8).to_le_bytes().iter().for_each(|b| bin.push(*b));
                for _ in 0..3{
                    bin.push(0u8);
                }
            }

            //Enabled flag
            if block.enabled{
                bin.push(1u8);
            }else{
                bin.push(0u8);
            }
            for _ in 0..3{
                bin.push(0u8);
            }
        }

        let mut buf_writer=io::BufWriter::new(fs::File::create(path)?);
        buf_writer.write_all(&mut bin)?;

        Ok(())
    }
}