//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAchievement")]
    pub struct GKAchievement;

    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl ClassType for GKAchievement {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKAchievement")]
unsafe impl NSCoding for GKAchievement {}

#[cfg(feature = "GameKit_GKAchievement")]
unsafe impl NSSecureCoding for GKAchievement {}

extern_methods!(
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadAchievementsWithCompletionHandler:)]
        pub unsafe fn loadAchievementsWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSArray<GKAchievement>, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(resetAchievementsWithCompletionHandler:)]
        pub unsafe fn resetAchievementsWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Init initWithIdentifier:player:)]
        pub unsafe fn initWithIdentifier_player(
            this: Option<Allocated<Self>>,
            identifier: Option<&NSString>,
            player: &GKPlayer,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(reportAchievements:withCompletionHandler:)]
        pub unsafe fn reportAchievements_withCompletionHandler(
            achievements: &NSArray<GKAchievement>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[method(percentComplete)]
        pub unsafe fn percentComplete(&self) -> c_double;

        #[method(setPercentComplete:)]
        pub unsafe fn setPercentComplete(&self, percent_complete: c_double);

        #[method(isCompleted)]
        pub unsafe fn isCompleted(&self) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastReportedDate)]
        pub unsafe fn lastReportedDate(&self) -> Id<NSDate, Shared>;

        #[method(showsCompletionBanner)]
        pub unsafe fn showsCompletionBanner(&self) -> bool;

        #[method(setShowsCompletionBanner:)]
        pub unsafe fn setShowsCompletionBanner(&self, shows_completion_banner: bool);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Option<Id<GKPlayer, Shared>>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "Use +reportAchievements:withCompletionHandler:"]
        #[method(reportAchievementWithCompletionHandler:)]
        pub unsafe fn reportAchievementWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[deprecated = "Use isHidden on the GKAchievementDescription class instead"]
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:forPlayer:)]
        pub unsafe fn initWithIdentifier_forPlayer(
            this: Option<Allocated<Self>>,
            identifier: Option<&NSString>,
            player_id: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Option<Id<NSString, Shared>>;
    }
);
