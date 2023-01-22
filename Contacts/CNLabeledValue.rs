//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNLabeledValue")]
    pub struct CNLabeledValue<ValueType: Message = Object, ValueTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ValueType, ValueTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Contacts_CNLabeledValue")]
    unsafe impl<ValueType: Message, ValueTypeOwnership: Ownership> ClassType
        for CNLabeledValue<ValueType, ValueTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Contacts_CNLabeledValue")]
    unsafe impl<ValueType: Message, ValueTypeOwnership: Ownership>
        CNLabeledValue<ValueType, ValueTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<ValueType, ValueTypeOwnership>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labeledValueWithLabel:value:)]
        pub unsafe fn labeledValueWithLabel_value(
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLabel:value:)]
        pub unsafe fn initWithLabel_value(
            this: Option<Allocated<Self>>,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:)]
        pub unsafe fn labeledValueBySettingLabel(
            &self,
            label: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other labeledValueBySettingValue:)]
        pub unsafe fn labeledValueBySettingValue(&self, value: &ValueType) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labeledValueBySettingLabel:value:)]
        pub unsafe fn labeledValueBySettingLabel_value(
            &self,
            label: Option<&NSString>,
            value: &ValueType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForLabel:)]
        pub unsafe fn localizedStringForLabel(label: &NSString) -> Id<NSString, Shared>;
    }
);

extern_static!(CNLabelHome: &'static NSString);

extern_static!(CNLabelWork: &'static NSString);

extern_static!(CNLabelSchool: &'static NSString);

extern_static!(CNLabelOther: &'static NSString);

extern_static!(CNLabelEmailiCloud: &'static NSString);

extern_static!(CNLabelURLAddressHomePage: &'static NSString);

extern_static!(CNLabelDateAnniversary: &'static NSString);
