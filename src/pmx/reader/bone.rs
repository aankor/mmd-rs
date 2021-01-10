use crate::{Error, Settings, reader::MaterialReader, reader::helpers::ReadHelpers, pmx::bone::*};
use std::io::Read;
use fallible_iterator::{FallibleIterator, convert};
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::marker::PhantomData;
use enumflags2::BitFlags;

pub struct BoneReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R
}

impl<R: Read> BoneReader<R> {
  pub fn new(mut v: MaterialReader<R>) -> Result<BoneReader<R>, Error> {
    while v.remaining > 0 {
      v.next::<i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(BoneReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next<I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>>(&mut self) -> Result<Option<Bone<I>>, Error> {
    if self.remaining <= 0 {
      return Ok(None)
    }

    self.remaining -= 1;
  
  
    let local_name = self.read.read_text(self.settings.text_encoding)?;
    let universal_name = self.read.read_text(self.settings.text_encoding)?;
    let position = self.read.read_vec3()?;
    let parent = self.read.read_index(self.settings.bone_index_size)?;
    let transform_level = self.read.read_i32::<LE>()?;
    let bone_flags = BitFlags::from_bits(self.read.read_u16::<LE>()?).unwrap();
  
    let connection = bone_flags.contains(BoneFlags::Connection)
                               .then(|| Ok::<_, Error>(
                                 Connection::Index(self.read.read_index(self.settings.bone_index_size)?)
                               ))
                               .unwrap_or_else(|| Ok::<_, Error>(
                                 Connection::Position(self.read.read_vec3()?)
                               ))?;
    
    let additional = bone_flags.intersects(BoneFlags::AddRotation | BoneFlags::AddMovement)
                               .then(|| Ok::<_, Error>(Additional {
                                 parent: self.read.read_index(self.settings.bone_index_size)?,
                                 rate: self.read.read_f32::<LE>()?,
                               }))
                               .transpose()?;
    
    let fixed_axis = bone_flags.contains(BoneFlags::FixedAxis)
                               .then(|| self.read.read_vec3())
                               .transpose()?;
    
    let local_axis = bone_flags.contains(BoneFlags::LocalAxis)
                               .then(|| Ok::<_, Error>(LocalAxis {
                                 x: self.read.read_vec3()?,
                                 z: self.read.read_vec3()?,
                               }))
                               .transpose()?;
    
    let external_parent_transform = bone_flags.contains(BoneFlags::ExternalParentTransform)
                                              .then(|| self.read.read_i32::<LE>())
                                              .transpose()?;
    
    let inverse_kinematics = bone_flags.contains(BoneFlags::InverseKinematics)
                                       .then(|| Ok::<_, Error>(InverseKinematics {
                                         ik_bone: self.read.read_index(self.settings.bone_index_size)?,
                                         iterations: self.read.read_u32::<LE>()?,
                                         limit_angle: self.read.read_f32::<LE>()?,
                                         links: convert((0 .. self.read.read_u32::<LE>()?).map(Ok::<_, Error>))
                                             .map(|_| Ok(IKLink {
                                               ik_bone: self.read.read_index(self.settings.bone_index_size)?,
                                               limits: (self.read.read_u8()? != 0).then(|| Ok::<_, Error>((self.read.read_vec3()?, self.read.read_vec3()?)))
                                                                                  .transpose()?,
                                             }))
                                             .collect()?
                                       }))
                                       .transpose()?;
    
    Ok(Some(Bone {
      local_name,
      universal_name,
      position,
      parent,
      transform_level,
      bone_flags,
      connection,
      additional,
      fixed_axis,
      local_axis,
      external_parent_transform,
      inverse_kinematics,
    }))
  }

  pub fn iter<I>(&mut self) -> BoneIterator<R, I> {
    BoneIterator {
      reader: self,
      phantom: PhantomData
    }
  }
}

pub struct BoneIterator<'a, R, I = i32> {
  reader: &'a mut BoneReader<R>,
  phantom: PhantomData<I>
}

impl<R: Read, I: TryFrom<i8> + TryFrom<i16> + TryFrom<i32>> FallibleIterator for BoneIterator<'_, R, I> {
  type Item = Bone<I>;
  type Error = Error;

  fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
    self.reader.next()
  }
}