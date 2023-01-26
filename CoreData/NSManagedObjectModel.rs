//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSManagedObjectModel")]
    pub struct NSManagedObjectModel;

    #[cfg(feature = "CoreData_NSManagedObjectModel")]
    unsafe impl ClassType for NSManagedObjectModel {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSManagedObjectModel")]
unsafe impl NSCoding for NSManagedObjectModel {}

#[cfg(feature = "CoreData_NSManagedObjectModel")]
unsafe impl NSFastEnumeration for NSManagedObjectModel {}

extern_methods!(
    #[cfg(feature = "CoreData_NSManagedObjectModel")]
    unsafe impl NSManagedObjectModel {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSBundle"))]
        #[method_id(@__retain_semantics Other mergedModelFromBundles:)]
        pub unsafe fn mergedModelFromBundles(
            bundles: Option<&NSArray<NSBundle>>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other modelByMergingModels:)]
        pub unsafe fn modelByMergingModels(
            models: Option<&NSArray<NSManagedObjectModel>>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other entitiesByName)]
        pub unsafe fn entitiesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSEntityDescription>, Shared>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other entities)]
        pub unsafe fn entities(&self) -> Id<NSArray<NSEntityDescription>, Shared>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method(setEntities:)]
        pub unsafe fn setEntities(&self, entities: &NSArray<NSEntityDescription>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other configurations)]
        pub unsafe fn configurations(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other entitiesForConfiguration:)]
        pub unsafe fn entitiesForConfiguration(
            &self,
            configuration: Option<&NSString>,
        ) -> Option<Id<NSArray<NSEntityDescription>, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(setEntities:forConfiguration:)]
        pub unsafe fn setEntities_forConfiguration(
            &self,
            entities: &NSArray<NSEntityDescription>,
            configuration: &NSString,
        );

        #[cfg(all(feature = "CoreData_NSFetchRequest", feature = "Foundation_NSString"))]
        #[method(setFetchRequestTemplate:forName:)]
        pub unsafe fn setFetchRequestTemplate_forName(
            &self,
            fetch_request_template: Option<&NSFetchRequest>,
            name: &NSString,
        );

        #[cfg(all(feature = "CoreData_NSFetchRequest", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fetchRequestTemplateForName:)]
        pub unsafe fn fetchRequestTemplateForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSFetchRequest, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fetchRequestFromTemplateWithName:substitutionVariables:)]
        pub unsafe fn fetchRequestFromTemplateWithName_substitutionVariables(
            &self,
            name: &NSString,
            variables: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSFetchRequest, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizationDictionary)]
        pub unsafe fn localizationDictionary(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setLocalizationDictionary:)]
        pub unsafe fn setLocalizationDictionary(
            &self,
            localization_dictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other mergedModelFromBundles:forStoreMetadata:)]
        pub unsafe fn mergedModelFromBundles_forStoreMetadata(
            bundles: Option<&NSArray<NSBundle>>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other modelByMergingModels:forStoreMetadata:)]
        pub unsafe fn modelByMergingModels_forStoreMetadata(
            models: &NSArray<NSManagedObjectModel>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fetchRequestTemplatesByName)]
        pub unsafe fn fetchRequestTemplatesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSFetchRequest>, Shared>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other versionIdentifiers)]
        pub unsafe fn versionIdentifiers(&self) -> Id<NSSet, Shared>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method(setVersionIdentifiers:)]
        pub unsafe fn setVersionIdentifiers(&self, version_identifiers: &NSSet);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(isConfiguration:compatibleWithStoreMetadata:)]
        pub unsafe fn isConfiguration_compatibleWithStoreMetadata(
            &self,
            configuration: Option<&NSString>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other entityVersionHashesByName)]
        pub unsafe fn entityVersionHashesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSData>, Shared>;
    }
);
