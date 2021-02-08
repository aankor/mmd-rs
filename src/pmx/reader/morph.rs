use crate::{
  pmx::morph::*,
  reader::{helpers::ReadHelpers, BoneReader},
  Error, Index, Result, Settings, VertexIndex,
};
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::io::Read;
use std::marker::PhantomData;

pub struct MorphReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> MorphReader<R> {
  pub fn new(mut b: BoneReader<R>) -> Result<MorphReader<R>> {
    while b.remaining > 0 {
      b.next::<i32>()?;
    }
    let count = b.read.read_i32::<LE>()?;

    Ok(MorphReader {
      settings: b.settings,
      count,
      remaining: count,
      read: b.read,
    })
  }

  pub fn next<I, VI, BI, MI, RBI>(&mut self) -> Result<Option<Morph<I, VI, BI, MI, RBI>>>
  where
    I: Index,
    VI: VertexIndex,
    BI: Index,
    MI: Index,
    RBI: Index,
  {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    let local_name = self.read.read_text(self.settings.text_encoding)?;
    let universal_name = self.read.read_text(self.settings.text_encoding)?;
    let panel = Panel::from(self.read.read_u8()?);
    let morph_type = self.read.read_u8()?;
    let morph_count = self.read.read_u32::<LE>()?;

    let offsets = match morph_type {
      0 => Offsets::Group(self.next_morph_offsets(morph_count)?),
      1 => Offsets::Vertex(self.next_vertex_offsets(morph_count)?),
      2 => Offsets::Bone(self.next_bone_offsets(morph_count)?),
      3 => Offsets::UV(self.next_uv_offsets(morph_count)?),
      4 => Offsets::AdditionalUV1(self.next_uv_offsets(morph_count)?),
      5 => Offsets::AdditionalUV2(self.next_uv_offsets(morph_count)?),
      6 => Offsets::AdditionalUV3(self.next_uv_offsets(morph_count)?),
      7 => Offsets::AdditionalUV4(self.next_uv_offsets(morph_count)?),
      8 => Offsets::Material(self.next_material_offsets(morph_count)?),
      9 => Offsets::Flip(self.next_morph_offsets(morph_count)?),
      10 => Offsets::Impulse(self.next_impulse_offsets(morph_count)?),
      e => return Err(Error::InvalidMorphType(e)),
    };

    Ok(Some(Morph {
      local_name,
      universal_name,
      panel,
      offsets,
    }))
  }

  pub fn iter<I, VI, BI, MI, RBI>(&mut self) -> MorphIterator<R, I, VI, BI, MI, RBI> {
    MorphIterator {
      reader: self,
      phantom: PhantomData,
    }
  }

  fn next_morph_offsets<I: Index>(&mut self, count: u32) -> Result<Vec<MorphOffset<I>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(MorphOffset {
        morph: self.read.read_index(self.settings.morph_index_size)?,
        rate: self.read.read_f32::<LE>()?,
      })
    }

    Ok(offsets)
  }

  fn next_vertex_offsets<VI: VertexIndex>(&mut self, count: u32) -> Result<Vec<VertexOffset<VI>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(VertexOffset {
        vertex: self
          .read
          .read_vertex_index(self.settings.vertex_index_size)?,
        offset: self.read.read_vec3()?,
      })
    }

    Ok(offsets)
  }

  fn next_bone_offsets<BI: Index>(&mut self, count: u32) -> Result<Vec<BoneOffset<BI>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(BoneOffset {
        bone: self.read.read_index(self.settings.bone_index_size)?,
        translation: self.read.read_vec3()?,
        rotation: self.read.read_vec4()?,
      })
    }

    Ok(offsets)
  }

  fn next_uv_offsets<VI: VertexIndex>(&mut self, count: u32) -> Result<Vec<UVOffset<VI>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(UVOffset {
        vertex: self
          .read
          .read_vertex_index(self.settings.vertex_index_size)?,
        offset: self.read.read_vec4()?,
      })
    }

    Ok(offsets)
  }

  fn next_material_offsets<MI: Index>(&mut self, count: u32) -> Result<Vec<MaterialOffset<MI>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(MaterialOffset {
        material: self.read.read_index(self.settings.material_index_size)?,
        method: OffsetMethod::try_from(self.read.read_u8()?)?,
        diffuse_color: self.read.read_vec4()?,
        specular_color: self.read.read_vec3()?,
        specular_strength: self.read.read_f32::<LE>()?,
        ambient_color: self.read.read_vec3()?,
        edge_color: self.read.read_vec4()?,
        edge_scale: self.read.read_f32::<LE>()?,
        texture_tint: self.read.read_vec4()?,
        environment_tint: self.read.read_vec4()?,
        toon_tint: self.read.read_vec4()?,
      })
    }

    Ok(offsets)
  }

  fn next_impulse_offsets<RBI: Index>(&mut self, count: u32) -> Result<Vec<ImpulseOffset<RBI>>> {
    let mut offsets = Vec::with_capacity(count as usize);

    for _ in 0..count {
      offsets.push(ImpulseOffset {
        rigid_body: self.read.read_index(self.settings.rigidbody_index_size)?,
        local: self.read.read_u8()? != 0,
        velocity: self.read.read_vec3()?,
        torque: self.read.read_vec3()?,
      })
    }

    Ok(offsets)
  }
}

pub struct MorphIterator<'a, R, I = i32, VI = i32, BI = i32, MI = i32, RBI = i32> {
  reader: &'a mut MorphReader<R>,
  phantom: PhantomData<(I, VI, BI, MI, RBI)>,
}

impl<R, I, VI, BI, MI, RBI> Iterator for MorphIterator<'_, R, I, VI, BI, MI, RBI>
where
  R: Read,
  I: Index,
  VI: VertexIndex,
  BI: Index,
  MI: Index,
  RBI: Index,
{
  type Item = Result<Morph<I, VI, BI, MI, RBI>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
