//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCompareFunction {
        MTLCompareFunctionNever = 0,
        MTLCompareFunctionLess = 1,
        MTLCompareFunctionEqual = 2,
        MTLCompareFunctionLessEqual = 3,
        MTLCompareFunctionGreater = 4,
        MTLCompareFunctionNotEqual = 5,
        MTLCompareFunctionGreaterEqual = 6,
        MTLCompareFunctionAlways = 7,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLStencilOperation {
        MTLStencilOperationKeep = 0,
        MTLStencilOperationZero = 1,
        MTLStencilOperationReplace = 2,
        MTLStencilOperationIncrementClamp = 3,
        MTLStencilOperationDecrementClamp = 4,
        MTLStencilOperationInvert = 5,
        MTLStencilOperationIncrementWrap = 6,
        MTLStencilOperationDecrementWrap = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    pub struct MTLStencilDescriptor;

    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl ClassType for MTLStencilDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl MTLStencilDescriptor {
        #[method(stencilCompareFunction)]
        pub fn stencilCompareFunction(&self) -> MTLCompareFunction;

        #[method(setStencilCompareFunction:)]
        pub fn setStencilCompareFunction(&self, stencil_compare_function: MTLCompareFunction);

        #[method(stencilFailureOperation)]
        pub fn stencilFailureOperation(&self) -> MTLStencilOperation;

        #[method(setStencilFailureOperation:)]
        pub fn setStencilFailureOperation(&self, stencil_failure_operation: MTLStencilOperation);

        #[method(depthFailureOperation)]
        pub fn depthFailureOperation(&self) -> MTLStencilOperation;

        #[method(setDepthFailureOperation:)]
        pub fn setDepthFailureOperation(&self, depth_failure_operation: MTLStencilOperation);

        #[method(depthStencilPassOperation)]
        pub fn depthStencilPassOperation(&self) -> MTLStencilOperation;

        #[method(setDepthStencilPassOperation:)]
        pub fn setDepthStencilPassOperation(
            &self,
            depth_stencil_pass_operation: MTLStencilOperation,
        );

        #[method(readMask)]
        pub fn readMask(&self) -> u32;

        #[method(setReadMask:)]
        pub fn setReadMask(&self, read_mask: u32);

        #[method(writeMask)]
        pub fn writeMask(&self) -> u32;

        #[method(setWriteMask:)]
        pub fn setWriteMask(&self, write_mask: u32);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    pub struct MTLDepthStencilDescriptor;

    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl ClassType for MTLDepthStencilDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl MTLDepthStencilDescriptor {
        #[method(depthCompareFunction)]
        pub fn depthCompareFunction(&self) -> MTLCompareFunction;

        #[method(setDepthCompareFunction:)]
        pub fn setDepthCompareFunction(&self, depth_compare_function: MTLCompareFunction);

        #[method(isDepthWriteEnabled)]
        pub fn isDepthWriteEnabled(&self) -> bool;

        #[method(setDepthWriteEnabled:)]
        pub fn setDepthWriteEnabled(&self, depth_write_enabled: bool);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other frontFaceStencil)]
        pub fn frontFaceStencil(&self) -> Id<MTLStencilDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setFrontFaceStencil:)]
        pub fn setFrontFaceStencil(&self, front_face_stencil: Option<&MTLStencilDescriptor>);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other backFaceStencil)]
        pub fn backFaceStencil(&self) -> Id<MTLStencilDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setBackFaceStencil:)]
        pub fn setBackFaceStencil(&self, back_face_stencil: Option<&MTLStencilDescriptor>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait MTLDepthStencilState: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>, Shared>;
    }

    unsafe impl ProtocolType for dyn MTLDepthStencilState {}
);
