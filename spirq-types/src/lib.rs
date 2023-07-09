mod reg;
mod ty;
mod walk;

pub use reg::TypeRegistry;
pub use ty::{
    AccessType, ArrayType, CombinedImageSamplerType, DescriptorType, ImageType, MatrixAxisOrder,
    MatrixType, PointerType, SampledImageType, ScalarType, StorageImageType, StructMember,
    StructType, SubpassDataType, Type, VectorType,
};
pub use walk::Walk;
