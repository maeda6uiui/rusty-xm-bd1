use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::{fs, io, str};
use std::path::Path;
use crate::block::Block;

pub struct Reader{
    pub texture_filenames: HashMap<i32,String>,
    pub blocks: Vec<Block>,
}

impl Reader{
    pub fn new(path: &Path)->Result<Reader,Box<dyn Error>>{
        let mut reader=Reader{
            texture_filenames: HashMap::new(),
            blocks: Vec::new(),
        };

        let mut buf_reader=io::BufReader::new(fs::File::open(path)?);
        let mut bin=vec![];
        buf_reader.read_to_end(&mut bin)?;

        let mut pos=0;

        //Texture filenames
        //Note that handling of texture filenames does not take non-ASCII characters into account
        for i in 0..10{
            let mut texture_filename_buffer=[0u8;31];

            for j in 0..31{
                texture_filename_buffer[j]=bin[pos];
                pos+=1;
            }

            
            let raw_texture_filename=str::from_utf8(&texture_filename_buffer)?;
            
            let mut first_null_pos=30;
            for j in 0..30{
                if raw_texture_filename.chars().nth(j).unwrap()=='\0'{
                    first_null_pos=j;
                    break;
                }
            }

            let mut texture_filename=raw_texture_filename[0..first_null_pos].to_string();
            texture_filename=texture_filename.replace("\\", "/");
            reader.texture_filenames.insert(i, texture_filename);
        }

        //Number of blocks
        let num_blocks=u16::from_be_bytes(bin[pos..pos+2].try_into()?);
        pos+=2;

        //Blocks
        for _ in 0..num_blocks{
            let mut block=Block::new();

            //Vertex positions
            for i in 0..8{
                block.vertex_positions[i].x=f32::from_le_bytes(bin[pos..pos+4].try_into()?);
                pos+=4;
            }
            for i in 0..8{
                block.vertex_positions[i].y=f32::from_le_bytes(bin[pos..pos+4].try_into()?);
                pos+=4;
            }
            for i in 0..8{
                block.vertex_positions[i].z=f32::from_le_bytes(bin[pos..pos+4].try_into()?);
                pos+=4;
            }

            //UVs
            for i in 0..24{
                block.uvs[i].u=f32::from_le_bytes(bin[pos..pos+4].try_into()?);
                pos+=4;
            }
            for i in 0..24{
                block.uvs[i].v=f32::from_le_bytes(bin[pos..pos+4].try_into()?);
                pos+=4;
            }

            //Texture IDs
            for i in 0..6{
                block.texture_ids[i]=u8::from_le_bytes(bin[pos..pos+1].try_into()?) as i32;
                pos+=4;
            }

            //Enabled flag
            let enabled=u8::from_le_bytes(bin[pos..pos+1].try_into()?);
            block.enabled=enabled!=0;
            pos+=4;

            reader.blocks.push(block);
        }

        Ok(reader)
    }
}