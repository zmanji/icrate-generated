// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `GameKit` framework

#[cfg_attr(feature = "apple", link(name = "GameKit", kind = "framework"))]
extern "C" {}

#[path = "GKAccessPoint.rs"]
mod __GKAccessPoint;
#[path = "GKAchievement.rs"]
mod __GKAchievement;
#[path = "GKAchievementDescription.rs"]
mod __GKAchievementDescription;
#[path = "GKAchievementViewController.rs"]
mod __GKAchievementViewController;
#[path = "GKBasePlayer.rs"]
mod __GKBasePlayer;
#[path = "GKChallenge.rs"]
mod __GKChallenge;
#[path = "GKChallengeEventHandler.rs"]
mod __GKChallengeEventHandler;
#[path = "GKChallengesViewController.rs"]
mod __GKChallengesViewController;
#[path = "GKCloudPlayer.rs"]
mod __GKCloudPlayer;
#[path = "GKDefines.rs"]
mod __GKDefines;
#[path = "GKDialogController.rs"]
mod __GKDialogController;
#[path = "GKError.rs"]
mod __GKError;
#[path = "GKEventListener.rs"]
mod __GKEventListener;
#[path = "GKFriendRequestComposeViewController.rs"]
mod __GKFriendRequestComposeViewController;
#[path = "GKGameCenterViewController.rs"]
mod __GKGameCenterViewController;
#[path = "GKGameSession.rs"]
mod __GKGameSession;
#[path = "GKGameSessionError.rs"]
mod __GKGameSessionError;
#[path = "GKGameSessionEventListener.rs"]
mod __GKGameSessionEventListener;
#[path = "GKGameSessionSharingViewController.rs"]
mod __GKGameSessionSharingViewController;
#[path = "GKLeaderboard.rs"]
mod __GKLeaderboard;
#[path = "GKLeaderboardEntry.rs"]
mod __GKLeaderboardEntry;
#[path = "GKLeaderboardScore.rs"]
mod __GKLeaderboardScore;
#[path = "GKLeaderboardSet.rs"]
mod __GKLeaderboardSet;
#[path = "GKLeaderboardViewController.rs"]
mod __GKLeaderboardViewController;
#[path = "GKLocalPlayer.rs"]
mod __GKLocalPlayer;
#[path = "GKMatch.rs"]
mod __GKMatch;
#[path = "GKMatchmaker.rs"]
mod __GKMatchmaker;
#[path = "GKMatchmakerViewController.rs"]
mod __GKMatchmakerViewController;
#[path = "GKNotificationBanner.rs"]
mod __GKNotificationBanner;
#[path = "GKPlayer.rs"]
mod __GKPlayer;
#[path = "GKPublicConstants.rs"]
mod __GKPublicConstants;
#[path = "GKPublicProtocols.rs"]
mod __GKPublicProtocols;
#[path = "GKSavedGame.rs"]
mod __GKSavedGame;
#[path = "GKSavedGameListener.rs"]
mod __GKSavedGameListener;
#[path = "GKScore.rs"]
mod __GKScore;
#[path = "GKSession.rs"]
mod __GKSession;
#[path = "GKSessionError.rs"]
mod __GKSessionError;
#[path = "GKTurnBasedMatch.rs"]
mod __GKTurnBasedMatch;
#[path = "GKTurnBasedMatchmakerViewController.rs"]
mod __GKTurnBasedMatchmakerViewController;
#[path = "GKVoiceChat.rs"]
mod __GKVoiceChat;
#[path = "GKVoiceChatService.rs"]
mod __GKVoiceChatService;

#[cfg(feature = "GameKit_GKAccessPoint")]
pub use self::__GKAccessPoint::GKAccessPoint;
pub use self::__GKAccessPoint::{
    GKAccessPointLocation, GKAccessPointLocationBottomLeading, GKAccessPointLocationBottomTrailing,
    GKAccessPointLocationTopLeading, GKAccessPointLocationTopTrailing,
};
#[cfg(feature = "GameKit_GKAchievement")]
pub use self::__GKAchievement::GKAchievement;
#[cfg(feature = "GameKit_GKAchievementDescription")]
pub use self::__GKAchievementDescription::GKAchievementDescription;
#[cfg(feature = "GameKit_GKAchievementViewController")]
pub use self::__GKAchievementViewController::GKAchievementViewController;
pub use self::__GKAchievementViewController::GKAchievementViewControllerDelegate;
#[cfg(feature = "GameKit_GKBasePlayer")]
pub use self::__GKBasePlayer::GKBasePlayer;
#[cfg(feature = "GameKit_GKAchievementChallenge")]
pub use self::__GKChallenge::GKAchievementChallenge;
#[cfg(feature = "GameKit_GKChallenge")]
pub use self::__GKChallenge::GKChallenge;
pub use self::__GKChallenge::GKChallengeComposeCompletionBlock;
#[cfg(feature = "GameKit_GKScoreChallenge")]
pub use self::__GKChallenge::GKScoreChallenge;
pub use self::__GKChallenge::{
    GKChallengeState, GKChallengeStateCompleted, GKChallengeStateDeclined, GKChallengeStateInvalid,
    GKChallengeStatePending,
};
#[cfg(feature = "GameKit_GKChallengeEventHandler")]
pub use self::__GKChallengeEventHandler::GKChallengeEventHandler;
pub use self::__GKChallengeEventHandler::GKChallengeEventHandlerDelegate;
#[cfg(feature = "GameKit_GKChallengesViewController")]
pub use self::__GKChallengesViewController::GKChallengesViewController;
pub use self::__GKChallengesViewController::GKChallengesViewControllerDelegate;
#[cfg(feature = "GameKit_GKCloudPlayer")]
pub use self::__GKCloudPlayer::GKCloudPlayer;
#[cfg(feature = "GameKit_GKDialogController")]
pub use self::__GKDialogController::GKDialogController;
pub use self::__GKDialogController::GKViewController;
pub use self::__GKError::GKErrorDomain;
pub use self::__GKError::{
    GKErrorAPINotAvailable, GKErrorAPIObsolete, GKErrorAuthenticationInProgress, GKErrorCancelled,
    GKErrorChallengeInvalid, GKErrorCode, GKErrorCommunicationsFailure, GKErrorConnectionTimeout,
    GKErrorFriendListDenied, GKErrorFriendListDescriptionMissing, GKErrorFriendListRestricted,
    GKErrorFriendRequestNotAvailable, GKErrorGameSessionRequestInvalid, GKErrorGameUnrecognized,
    GKErrorInvalidCredentials, GKErrorInvalidParameter, GKErrorInvalidPlayer,
    GKErrorInvitationsDisabled, GKErrorMatchNotConnected, GKErrorMatchRequestInvalid,
    GKErrorNotAuthenticated, GKErrorNotAuthorized, GKErrorNotSupported,
    GKErrorParentalControlsBlocked, GKErrorPlayerPhotoFailure,
    GKErrorPlayerStatusExceedsMaximumLength, GKErrorPlayerStatusInvalid,
    GKErrorRestrictedToAutomatch, GKErrorScoreNotSet, GKErrorTurnBasedInvalidParticipant,
    GKErrorTurnBasedInvalidState, GKErrorTurnBasedInvalidTurn, GKErrorTurnBasedMatchDataTooLarge,
    GKErrorTurnBasedTooManySessions, GKErrorUbiquityContainerUnavailable, GKErrorUnderage,
    GKErrorUnexpectedConnection, GKErrorUnknown, GKErrorUserDenied,
};
pub use self::__GKEventListener::GKChallengeListener;
#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
pub use self::__GKFriendRequestComposeViewController::GKFriendRequestComposeViewController;
pub use self::__GKFriendRequestComposeViewController::GKFriendRequestComposeViewControllerDelegate;
pub use self::__GKGameCenterViewController::GKGameCenterControllerDelegate;
#[cfg(feature = "GameKit_GKGameCenterViewController")]
pub use self::__GKGameCenterViewController::GKGameCenterViewController;
pub use self::__GKGameCenterViewController::{
    GKGameCenterViewControllerState, GKGameCenterViewControllerStateAchievements,
    GKGameCenterViewControllerStateChallenges, GKGameCenterViewControllerStateDashboard,
    GKGameCenterViewControllerStateDefault, GKGameCenterViewControllerStateLeaderboards,
    GKGameCenterViewControllerStateLocalPlayerFriendsList,
    GKGameCenterViewControllerStateLocalPlayerProfile,
};
#[cfg(feature = "GameKit_GKGameSession")]
pub use self::__GKGameSession::GKGameSession;
pub use self::__GKGameSession::{
    GKConnectionState, GKConnectionStateConnected, GKConnectionStateNotConnected,
};
pub use self::__GKGameSession::{
    GKTransportType, GKTransportTypeReliable, GKTransportTypeUnreliable,
};
pub use self::__GKGameSessionError::GKGameSessionErrorDomain;
pub use self::__GKGameSessionError::{
    GKGameSessionErrorBadContainer, GKGameSessionErrorCloudDriveDisabled,
    GKGameSessionErrorCloudQuotaExceeded, GKGameSessionErrorCode,
    GKGameSessionErrorConnectionCancelledByUser, GKGameSessionErrorConnectionFailed,
    GKGameSessionErrorInvalidSession, GKGameSessionErrorNetworkFailure,
    GKGameSessionErrorNotAuthenticated, GKGameSessionErrorSendDataNoRecipients,
    GKGameSessionErrorSendDataNotConnected, GKGameSessionErrorSendDataNotReachable,
    GKGameSessionErrorSendRateLimitReached, GKGameSessionErrorSessionConflict,
    GKGameSessionErrorSessionHasMaxConnectedPlayers, GKGameSessionErrorSessionNotShared,
    GKGameSessionErrorUnknown,
};
pub use self::__GKGameSessionEventListener::GKGameSessionEventListener;
#[cfg(feature = "GameKit_GKLeaderboard")]
pub use self::__GKLeaderboard::GKLeaderboard;
pub use self::__GKLeaderboard::{
    GKLeaderboardPlayerScope, GKLeaderboardPlayerScopeFriendsOnly, GKLeaderboardPlayerScopeGlobal,
};
pub use self::__GKLeaderboard::{
    GKLeaderboardTimeScope, GKLeaderboardTimeScopeAllTime, GKLeaderboardTimeScopeToday,
    GKLeaderboardTimeScopeWeek,
};
pub use self::__GKLeaderboard::{
    GKLeaderboardType, GKLeaderboardTypeClassic, GKLeaderboardTypeRecurring,
};
#[cfg(feature = "GameKit_GKLeaderboardEntry")]
pub use self::__GKLeaderboardEntry::GKLeaderboardEntry;
#[cfg(feature = "GameKit_GKLeaderboardScore")]
pub use self::__GKLeaderboardScore::GKLeaderboardScore;
#[cfg(feature = "GameKit_GKLeaderboardSet")]
pub use self::__GKLeaderboardSet::GKLeaderboardSet;
#[cfg(feature = "GameKit_GKLeaderboardViewController")]
pub use self::__GKLeaderboardViewController::GKLeaderboardViewController;
pub use self::__GKLeaderboardViewController::GKLeaderboardViewControllerDelegate;
#[cfg(feature = "GameKit_GKLocalPlayer")]
pub use self::__GKLocalPlayer::GKLocalPlayer;
pub use self::__GKLocalPlayer::GKLocalPlayerListener;
pub use self::__GKLocalPlayer::GKPlayerAuthenticationDidChangeNotificationName;
pub use self::__GKLocalPlayer::{
    GKFriendsAuthorizationStatus, GKFriendsAuthorizationStatusAuthorized,
    GKFriendsAuthorizationStatusDenied, GKFriendsAuthorizationStatusNotDetermined,
    GKFriendsAuthorizationStatusRestricted,
};
#[cfg(feature = "GameKit_GKMatch")]
pub use self::__GKMatch::GKMatch;
pub use self::__GKMatch::GKMatchDelegate;
pub use self::__GKMatch::{
    GKMatchSendDataMode, GKMatchSendDataReliable, GKMatchSendDataUnreliable,
};
pub use self::__GKMatch::{
    GKPlayerConnectionState, GKPlayerStateConnected, GKPlayerStateDisconnected,
    GKPlayerStateUnknown,
};
#[cfg(feature = "GameKit_GKInvite")]
pub use self::__GKMatchmaker::GKInvite;
pub use self::__GKMatchmaker::GKInviteEventListener;
pub use self::__GKMatchmaker::GKInviteeResponse;
#[cfg(feature = "GameKit_GKMatchRequest")]
pub use self::__GKMatchmaker::GKMatchRequest;
#[cfg(feature = "GameKit_GKMatchmaker")]
pub use self::__GKMatchmaker::GKMatchmaker;
pub use self::__GKMatchmaker::{
    GKInviteRecipientResponse, GKInviteRecipientResponseAccepted,
    GKInviteRecipientResponseDeclined, GKInviteRecipientResponseFailed,
    GKInviteRecipientResponseIncompatible, GKInviteRecipientResponseNoAnswer,
    GKInviteRecipientResponseUnableToConnect, GKInviteeResponseAccepted, GKInviteeResponseDeclined,
    GKInviteeResponseFailed, GKInviteeResponseIncompatible, GKInviteeResponseNoAnswer,
    GKInviteeResponseUnableToConnect,
};
pub use self::__GKMatchmaker::{
    GKMatchType, GKMatchTypeHosted, GKMatchTypePeerToPeer, GKMatchTypeTurnBased,
};
#[cfg(feature = "GameKit_GKMatchmakerViewController")]
pub use self::__GKMatchmakerViewController::GKMatchmakerViewController;
pub use self::__GKMatchmakerViewController::GKMatchmakerViewControllerDelegate;
pub use self::__GKMatchmakerViewController::{
    GKMatchmakingMode, GKMatchmakingModeAutomatchOnly, GKMatchmakingModeDefault,
    GKMatchmakingModeInviteOnly, GKMatchmakingModeNearbyOnly,
};
#[cfg(feature = "GameKit_GKNotificationBanner")]
pub use self::__GKNotificationBanner::GKNotificationBanner;
#[cfg(feature = "GameKit_GKPlayer")]
pub use self::__GKPlayer::GKPlayer;
pub use self::__GKPlayer::GKPlayerDidChangeNotificationName;
pub use self::__GKPlayer::GKPlayerIDNoLongerAvailable;
pub use self::__GKPlayer::{GKPhotoSize, GKPhotoSizeNormal, GKPhotoSizeSmall};
pub use self::__GKPublicConstants::GKVoiceChatServiceErrorDomain;
pub use self::__GKPublicConstants::{
    GKPeerConnectionState, GKPeerStateAvailable, GKPeerStateConnected, GKPeerStateConnectedRelay,
    GKPeerStateConnecting, GKPeerStateDisconnected, GKPeerStateUnavailable,
};
pub use self::__GKPublicConstants::{GKSendDataMode, GKSendDataReliable, GKSendDataUnreliable};
pub use self::__GKPublicConstants::{
    GKSessionMode, GKSessionModeClient, GKSessionModePeer, GKSessionModeServer,
};
pub use self::__GKPublicConstants::{
    GKVoiceChatServiceAudioUnavailableError, GKVoiceChatServiceClientMissingRequiredMethodsError,
    GKVoiceChatServiceError, GKVoiceChatServiceInternalError, GKVoiceChatServiceInvalidCallIDError,
    GKVoiceChatServiceInvalidParameterError, GKVoiceChatServiceMethodCurrentlyInvalidError,
    GKVoiceChatServiceNetworkConfigurationError, GKVoiceChatServiceNoRemotePacketsError,
    GKVoiceChatServiceOutOfMemoryError, GKVoiceChatServiceRemoteParticipantBusyError,
    GKVoiceChatServiceRemoteParticipantCancelledError,
    GKVoiceChatServiceRemoteParticipantDeclinedInviteError,
    GKVoiceChatServiceRemoteParticipantHangupError,
    GKVoiceChatServiceRemoteParticipantResponseInvalidError,
    GKVoiceChatServiceUnableToConnectError, GKVoiceChatServiceUninitializedClientError,
    GKVoiceChatServiceUnsupportedRemoteVersionError,
};
pub use self::__GKPublicProtocols::GKSessionDelegate;
pub use self::__GKPublicProtocols::GKVoiceChatClient;
#[cfg(feature = "GameKit_GKSavedGame")]
pub use self::__GKSavedGame::GKSavedGame;
pub use self::__GKSavedGameListener::GKSavedGameListener;
#[cfg(feature = "GameKit_GKScore")]
pub use self::__GKScore::GKScore;
#[cfg(feature = "GameKit_GKSession")]
pub use self::__GKSession::GKSession;
pub use self::__GKSessionError::GKSessionErrorDomain;
pub use self::__GKSessionError::{
    GKSessionCancelledError, GKSessionCannotEnableError, GKSessionConnectionClosedError,
    GKSessionConnectionFailedError, GKSessionConnectivityError, GKSessionDataTooBigError,
    GKSessionDeclinedError, GKSessionError, GKSessionInProgressError, GKSessionInternalError,
    GKSessionInvalidParameterError, GKSessionNotConnectedError, GKSessionPeerNotFoundError,
    GKSessionSystemError, GKSessionTimedOutError, GKSessionTransportError, GKSessionUnknownError,
};
pub use self::__GKTurnBasedMatch::GKExchangeTimeoutDefault;
pub use self::__GKTurnBasedMatch::GKExchangeTimeoutNone;
#[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
pub use self::__GKTurnBasedMatch::GKTurnBasedEventHandler;
pub use self::__GKTurnBasedMatch::GKTurnBasedEventHandlerDelegate;
pub use self::__GKTurnBasedMatch::GKTurnBasedEventListener;
#[cfg(feature = "GameKit_GKTurnBasedExchange")]
pub use self::__GKTurnBasedMatch::GKTurnBasedExchange;
#[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeReply;
#[cfg(feature = "GameKit_GKTurnBasedMatch")]
pub use self::__GKTurnBasedMatch::GKTurnBasedMatch;
#[cfg(feature = "GameKit_GKTurnBasedParticipant")]
pub use self::__GKTurnBasedMatch::GKTurnBasedParticipant;
pub use self::__GKTurnBasedMatch::GKTurnTimeoutDefault;
pub use self::__GKTurnBasedMatch::GKTurnTimeoutNone;
pub use self::__GKTurnBasedMatch::{
    GKTurnBasedExchangeStatus, GKTurnBasedExchangeStatusActive, GKTurnBasedExchangeStatusCanceled,
    GKTurnBasedExchangeStatusComplete, GKTurnBasedExchangeStatusResolved,
    GKTurnBasedExchangeStatusUnknown,
};
pub use self::__GKTurnBasedMatch::{
    GKTurnBasedMatchOutcome, GKTurnBasedMatchOutcomeCustomRange, GKTurnBasedMatchOutcomeFirst,
    GKTurnBasedMatchOutcomeFourth, GKTurnBasedMatchOutcomeLost, GKTurnBasedMatchOutcomeNone,
    GKTurnBasedMatchOutcomeQuit, GKTurnBasedMatchOutcomeSecond, GKTurnBasedMatchOutcomeThird,
    GKTurnBasedMatchOutcomeTied, GKTurnBasedMatchOutcomeTimeExpired, GKTurnBasedMatchOutcomeWon,
};
pub use self::__GKTurnBasedMatch::{
    GKTurnBasedMatchStatus, GKTurnBasedMatchStatusEnded, GKTurnBasedMatchStatusMatching,
    GKTurnBasedMatchStatusOpen, GKTurnBasedMatchStatusUnknown,
};
pub use self::__GKTurnBasedMatch::{
    GKTurnBasedParticipantStatus, GKTurnBasedParticipantStatusActive,
    GKTurnBasedParticipantStatusDeclined, GKTurnBasedParticipantStatusDone,
    GKTurnBasedParticipantStatusInvited, GKTurnBasedParticipantStatusMatching,
    GKTurnBasedParticipantStatusUnknown,
};
#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
pub use self::__GKTurnBasedMatchmakerViewController::GKTurnBasedMatchmakerViewController;
pub use self::__GKTurnBasedMatchmakerViewController::GKTurnBasedMatchmakerViewControllerDelegate;
#[cfg(feature = "GameKit_GKVoiceChat")]
pub use self::__GKVoiceChat::GKVoiceChat;
pub use self::__GKVoiceChat::{
    GKVoiceChatPlayerConnected, GKVoiceChatPlayerConnecting, GKVoiceChatPlayerDisconnected,
    GKVoiceChatPlayerSilent, GKVoiceChatPlayerSpeaking, GKVoiceChatPlayerState,
};
#[cfg(feature = "GameKit_GKVoiceChatService")]
pub use self::__GKVoiceChatService::GKVoiceChatService;
