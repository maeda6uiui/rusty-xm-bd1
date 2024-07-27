use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::io::{Read, Write};
use std::{cmp, fmt, fs, io, str};
use std::path::Path;
use nalgebra as na;

type Vector3f=na::Vector3<f32>;

#[derive(Debug,Clone)]
pub struct UV{
    pub u: f32,
    pub v: f32,
}

impl fmt::Display for UV{
    fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        write!(f,"({},{})",self.u,self.v)
    }
}

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

struct Reader{
    texture_filenames: HashMap<i32,String>,
    blocks: Vec<Block>,
}

impl Reader{
    fn new(path: &Path)->Result<Reader,Box<dyn Error>>{
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

struct Writer;

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

    fn write(path: &Path,blocks: &Vec<Block>,texture_filenames: &HashMap<i32,String>)->Result<(),Box<dyn Error>>{
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
