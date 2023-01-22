//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactType {
        CNContactTypePerson = 0,
        CNContactTypeOrganization = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNContactSortOrder {
        CNContactSortOrderNone = 0,
        CNContactSortOrderUserDefault = 1,
        CNContactSortOrderGivenName = 2,
        CNContactSortOrderFamilyName = 3,
    }
);

extern_protocol!(
    pub struct CNKeyDescriptor;

    unsafe impl ProtocolType for CNKeyDescriptor {}
);

extern_methods!(
    /// Contacts
    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSString {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNContact")]
    pub struct CNContact;

    #[cfg(feature = "Contacts_CNContact")]
    unsafe impl ClassType for CNContact {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Contacts_CNContact")]
    unsafe impl CNContact {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[method(contactType)]
        pub unsafe fn contactType(&self) -> CNContactType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namePrefix)]
        pub unsafe fn namePrefix(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other givenName)]
        pub unsafe fn givenName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other middleName)]
        pub unsafe fn middleName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other previousFamilyName)]
        pub unsafe fn previousFamilyName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nameSuffix)]
        pub unsafe fn nameSuffix(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nickname)]
        pub unsafe fn nickname(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other organizationName)]
        pub unsafe fn organizationName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other departmentName)]
        pub unsafe fn departmentName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticGivenName)]
        pub unsafe fn phoneticGivenName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticMiddleName)]
        pub unsafe fn phoneticMiddleName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticFamilyName)]
        pub unsafe fn phoneticFamilyName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other phoneticOrganizationName)]
        pub unsafe fn phoneticOrganizationName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other note)]
        pub unsafe fn note(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageData)]
        pub unsafe fn imageData(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other thumbnailImageData)]
        pub unsafe fn thumbnailImageData(&self) -> Option<Id<NSData, Shared>>;

        #[method(imageDataAvailable)]
        pub unsafe fn imageDataAvailable(&self) -> bool;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNPhoneNumber",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other phoneNumbers)]
        pub unsafe fn phoneNumbers(&self) -> Id<NSArray<CNLabeledValue<CNPhoneNumber>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other emailAddresses)]
        pub unsafe fn emailAddresses(&self) -> Id<NSArray<CNLabeledValue<NSString>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNPostalAddress",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other postalAddresses)]
        pub unsafe fn postalAddresses(
            &self,
        ) -> Id<NSArray<CNLabeledValue<CNPostalAddress>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other urlAddresses)]
        pub unsafe fn urlAddresses(&self) -> Id<NSArray<CNLabeledValue<NSString>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNContactRelation",
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other contactRelations)]
        pub unsafe fn contactRelations(
            &self,
        ) -> Id<NSArray<CNLabeledValue<CNContactRelation>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Contacts_CNSocialProfile",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other socialProfiles)]
        pub unsafe fn socialProfiles(&self)
            -> Id<NSArray<CNLabeledValue<CNSocialProfile>>, Shared>;

        #[cfg(all(
            feature = "Contacts_CNInstantMessageAddress",
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other instantMessageAddresses)]
        pub unsafe fn instantMessageAddresses(
            &self,
        ) -> Id<NSArray<CNLabeledValue<CNInstantMessageAddress>>, Shared>;

        #[cfg(feature = "Foundation_NSDateComponents")]
        #[method_id(@__retain_semantics Other birthday)]
        pub unsafe fn birthday(&self) -> Option<Id<NSDateComponents, Shared>>;

        #[cfg(feature = "Foundation_NSDateComponents")]
        #[method_id(@__retain_semantics Other nonGregorianBirthday)]
        pub unsafe fn nonGregorianBirthday(&self) -> Option<Id<NSDateComponents, Shared>>;

        #[cfg(all(
            feature = "Contacts_CNLabeledValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDateComponents"
        ))]
        #[method_id(@__retain_semantics Other dates)]
        pub unsafe fn dates(&self) -> Id<NSArray<CNLabeledValue<NSDateComponents>>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isKeyAvailable:)]
        pub unsafe fn isKeyAvailable(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(areKeysAvailable:)]
        pub unsafe fn areKeysAvailable(&self, key_descriptors: &NSArray<CNKeyDescriptor>) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString, Shared>;

        #[method(comparatorForNameSortOrder:)]
        pub unsafe fn comparatorForNameSortOrder(sort_order: CNContactSortOrder) -> NSComparator;

        #[method_id(@__retain_semantics Other descriptorForAllComparatorKeys)]
        pub unsafe fn descriptorForAllComparatorKeys() -> Id<CNKeyDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isUnifiedWithContactWithIdentifier:)]
        pub unsafe fn isUnifiedWithContactWithIdentifier(
            &self,
            contact_identifier: &NSString,
        ) -> bool;
    }
);

extern_static!(CNContactPropertyNotFetchedExceptionName: &'static NSString);

extern_static!(CNContactIdentifierKey: &'static NSString);

extern_static!(CNContactNamePrefixKey: &'static NSString);

extern_static!(CNContactGivenNameKey: &'static NSString);

extern_static!(CNContactMiddleNameKey: &'static NSString);

extern_static!(CNContactFamilyNameKey: &'static NSString);

extern_static!(CNContactPreviousFamilyNameKey: &'static NSString);

extern_static!(CNContactNameSuffixKey: &'static NSString);

extern_static!(CNContactNicknameKey: &'static NSString);

extern_static!(CNContactOrganizationNameKey: &'static NSString);

extern_static!(CNContactDepartmentNameKey: &'static NSString);

extern_static!(CNContactJobTitleKey: &'static NSString);

extern_static!(CNContactPhoneticGivenNameKey: &'static NSString);

extern_static!(CNContactPhoneticMiddleNameKey: &'static NSString);

extern_static!(CNContactPhoneticFamilyNameKey: &'static NSString);

extern_static!(CNContactPhoneticOrganizationNameKey: &'static NSString);

extern_static!(CNContactBirthdayKey: &'static NSString);

extern_static!(CNContactNonGregorianBirthdayKey: &'static NSString);

extern_static!(CNContactNoteKey: &'static NSString);

extern_static!(CNContactImageDataKey: &'static NSString);

extern_static!(CNContactThumbnailImageDataKey: &'static NSString);

extern_static!(CNContactImageDataAvailableKey: &'static NSString);

extern_static!(CNContactTypeKey: &'static NSString);

extern_static!(CNContactPhoneNumbersKey: &'static NSString);

extern_static!(CNContactEmailAddressesKey: &'static NSString);

extern_static!(CNContactPostalAddressesKey: &'static NSString);

extern_static!(CNContactDatesKey: &'static NSString);

extern_static!(CNContactUrlAddressesKey: &'static NSString);

extern_static!(CNContactRelationsKey: &'static NSString);

extern_static!(CNContactSocialProfilesKey: &'static NSString);

extern_static!(CNContactInstantMessageAddressesKey: &'static NSString);
