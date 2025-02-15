// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `MetalKit` framework

#[cfg_attr(feature = "apple", link(name = "MetalKit", kind = "framework"))]
extern "C" {}

#[path = "MTKDefines.rs"]
mod __MTKDefines;
#[path = "MTKModel.rs"]
mod __MTKModel;
#[path = "MTKTextureLoader.rs"]
mod __MTKTextureLoader;
#[path = "MTKView.rs"]
mod __MTKView;

#[cfg(feature = "MetalKit_MTKMesh")]
pub use self::__MTKModel::MTKMesh;
#[cfg(feature = "MetalKit_MTKMeshBuffer")]
pub use self::__MTKModel::MTKMeshBuffer;
#[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
pub use self::__MTKModel::MTKMeshBufferAllocator;
#[cfg(all(
    feature = "Metal_MTLVertexDescriptor",
    feature = "ModelIO_MDLVertexDescriptor"
))]
pub use self::__MTKModel::MTKMetalVertexDescriptorFromModelIO;
#[cfg(all(
    feature = "Foundation_NSError",
    feature = "Metal_MTLVertexDescriptor",
    feature = "ModelIO_MDLVertexDescriptor"
))]
pub use self::__MTKModel::MTKMetalVertexDescriptorFromModelIOWithError;
pub use self::__MTKModel::MTKModelError;
pub use self::__MTKModel::MTKModelErrorDomain;
pub use self::__MTKModel::MTKModelErrorKey;
#[cfg(all(
    feature = "Metal_MTLVertexDescriptor",
    feature = "ModelIO_MDLVertexDescriptor"
))]
pub use self::__MTKModel::MTKModelIOVertexDescriptorFromMetal;
#[cfg(all(
    feature = "Foundation_NSError",
    feature = "Metal_MTLVertexDescriptor",
    feature = "ModelIO_MDLVertexDescriptor"
))]
pub use self::__MTKModel::MTKModelIOVertexDescriptorFromMetalWithError;
#[cfg(feature = "MetalKit_MTKSubmesh")]
pub use self::__MTKModel::MTKSubmesh;
#[cfg(feature = "MetalKit_MTKTextureLoader")]
pub use self::__MTKTextureLoader::MTKTextureLoader;
pub use self::__MTKTextureLoader::MTKTextureLoaderArrayCallback;
pub use self::__MTKTextureLoader::MTKTextureLoaderCallback;
pub use self::__MTKTextureLoader::MTKTextureLoaderCubeLayout;
pub use self::__MTKTextureLoader::MTKTextureLoaderCubeLayoutVertical;
pub use self::__MTKTextureLoader::MTKTextureLoaderError;
pub use self::__MTKTextureLoader::MTKTextureLoaderErrorDomain;
pub use self::__MTKTextureLoader::MTKTextureLoaderErrorKey;
pub use self::__MTKTextureLoader::MTKTextureLoaderOption;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionAllocateMipmaps;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionCubeLayout;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionGenerateMipmaps;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionOrigin;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionSRGB;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionTextureCPUCacheMode;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionTextureStorageMode;
pub use self::__MTKTextureLoader::MTKTextureLoaderOptionTextureUsage;
pub use self::__MTKTextureLoader::MTKTextureLoaderOrigin;
pub use self::__MTKTextureLoader::MTKTextureLoaderOriginBottomLeft;
pub use self::__MTKTextureLoader::MTKTextureLoaderOriginFlippedVertically;
pub use self::__MTKTextureLoader::MTKTextureLoaderOriginTopLeft;
#[cfg(feature = "MetalKit_MTKView")]
pub use self::__MTKView::MTKView;
pub use self::__MTKView::MTKViewDelegate;
