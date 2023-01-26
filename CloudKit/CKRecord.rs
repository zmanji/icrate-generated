//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CKRecordType = NSString;

pub type CKRecordFieldKey = NSString;

extern_static!(CKRecordTypeUserRecord: &'static CKRecordType);

extern_static!(CKRecordParentKey: &'static CKRecordFieldKey);

extern_static!(CKRecordShareKey: &'static CKRecordFieldKey);

extern_protocol!(
    pub unsafe trait CKRecordValue: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn CKRecordValue {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecord")]
    pub struct CKRecord;

    #[cfg(feature = "CloudKit_CKRecord")]
    unsafe impl ClassType for CKRecord {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecord")]
    unsafe impl CKRecord {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithRecordType:)]
        pub unsafe fn initWithRecordType(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordType:recordID:)]
        pub unsafe fn initWithRecordType_recordID(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            record_id: &CKRecordID,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithRecordType:zoneID:)]
        pub unsafe fn initWithRecordType_zoneID(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            zone_id: &CKRecordZoneID,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Id<CKRecordType, Shared>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other recordID)]
        pub unsafe fn recordID(&self) -> Id<CKRecordID, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other recordChangeTag)]
        pub unsafe fn recordChangeTag(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other creatorUserRecordID)]
        pub unsafe fn creatorUserRecordID(&self) -> Option<Id<CKRecordID, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other lastModifiedUserRecordID)]
        pub unsafe fn lastModifiedUserRecordID(&self) -> Option<Id<CKRecordID, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Id<ProtocolObject<dyn CKRecordValue>, Shared>>;

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allKeys)]
        pub unsafe fn allKeys(&self) -> Id<NSArray<CKRecordFieldKey>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allTokens)]
        pub unsafe fn allTokens(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Id<ProtocolObject<dyn CKRecordValue>, Shared>>;

        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other changedKeys)]
        pub unsafe fn changedKeys(&self) -> Id<NSArray<CKRecordFieldKey>, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method(encodeSystemFieldsWithCoder:)]
        pub unsafe fn encodeSystemFieldsWithCoder(&self, coder: &NSCoder);

        #[cfg(feature = "CloudKit_CKReference")]
        #[method_id(@__retain_semantics Other share)]
        pub unsafe fn share(&self) -> Option<Id<CKReference, Shared>>;

        #[cfg(feature = "CloudKit_CKReference")]
        #[method_id(@__retain_semantics Other parent)]
        pub unsafe fn parent(&self) -> Option<Id<CKReference, Shared>>;

        #[cfg(feature = "CloudKit_CKReference")]
        #[method(setParent:)]
        pub unsafe fn setParent(&self, parent: Option<&CKReference>);

        #[method(setParentReferenceFromRecord:)]
        pub unsafe fn setParentReferenceFromRecord(&self, parent_record: Option<&CKRecord>);

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(setParentReferenceFromRecordID:)]
        pub unsafe fn setParentReferenceFromRecordID(&self, parent_record_id: Option<&CKRecordID>);
    }
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSString {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "Foundation_NSNumber")]
    unsafe impl NSNumber {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl NSArray {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "Foundation_NSDate")]
    unsafe impl NSDate {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "CloudKit_CKReference")]
    unsafe impl CKReference {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "CloudKit_CKAsset")]
    unsafe impl CKAsset {}
);

extern_methods!(
    /// CKRecordValue
    #[cfg(feature = "CoreLocation_CLLocation")]
    unsafe impl CLLocation {}
);

extern_protocol!(
    pub unsafe trait CKRecordKeyValueSetting: NSObjectProtocol {
        #[method_id(@__retain_semantics Other objectForKey:)]
        unsafe fn objectForKey(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Id<ProtocolObject<dyn CKRecordValue>, Shared>>;

        #[method(setObject:forKey:)]
        unsafe fn setObject_forKey(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        unsafe fn objectForKeyedSubscript(
            &self,
            key: &CKRecordFieldKey,
        ) -> Option<Id<ProtocolObject<dyn CKRecordValue>, Shared>>;

        #[method(setObject:forKeyedSubscript:)]
        unsafe fn setObject_forKeyedSubscript(
            &self,
            object: Option<&ProtocolObject<dyn CKRecordValue>>,
            key: &CKRecordFieldKey,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allKeys)]
        unsafe fn allKeys(&self) -> Id<NSArray<CKRecordFieldKey>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other changedKeys)]
        unsafe fn changedKeys(&self) -> Id<NSArray<CKRecordFieldKey>, Shared>;
    }

    unsafe impl ProtocolType for dyn CKRecordKeyValueSetting {}
);

extern_methods!(
    /// CKRecordKeyValueSettingConformance
    #[cfg(feature = "CloudKit_CKRecord")]
    unsafe impl CKRecord {
        #[method_id(@__retain_semantics Other encryptedValues)]
        pub unsafe fn encryptedValues(
            &self,
        ) -> Id<ProtocolObject<dyn CKRecordKeyValueSetting>, Shared>;
    }
);
