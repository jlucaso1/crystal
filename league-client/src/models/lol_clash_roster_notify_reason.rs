/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolClashRosterNotifyReason {
    #[serde(rename = "ROSTER_SET_TICKET")]
    ROSTERSETTICKET,
    #[serde(rename = "ROSTER_OFFER_TICKET")]
    ROSTEROFFERTICKET,
    #[serde(rename = "ROSTER_ACCEPT_TICKET")]
    ROSTERACCEPTTICKET,
    #[serde(rename = "ROSTER_DECLINE_TICKET")]
    ROSTERDECLINETICKET,
    #[serde(rename = "ROSTER_REVOKED_TICKET")]
    ROSTERREVOKEDTICKET,
    #[serde(rename = "BYE_AUTO_WIN")]
    BYEAUTOWIN,
    #[serde(rename = "BRACKET_READY")]
    BRACKETREADY,
    #[serde(rename = "CHANGE_POSITION")]
    CHANGEPOSITION,
    #[serde(rename = "EOG_PLAYER_UPDATE")]
    EOGPLAYERUPDATE,
    #[serde(rename = "REGISTERED")]
    REGISTERED,
    #[serde(rename = "RESTRICTION_AUTO_WIN")]
    RESTRICTIONAUTOWIN,
    #[serde(rename = "PHASE_UNREADY")]
    PHASEUNREADY,
    #[serde(rename = "PHASE_READY")]
    PHASEREADY,
    #[serde(rename = "PHASE_CHECKIN")]
    PHASECHECKIN,
    #[serde(rename = "PHASE_BACKOUT")]
    PHASEBACKOUT,
    #[serde(rename = "PERIOD_CANCEL")]
    PERIODCANCEL,
    #[serde(rename = "PERIOD_SPLIT")]
    PERIODSPLIT,
    #[serde(rename = "GAME_COMPLETED")]
    GAMECOMPLETED,
    #[serde(rename = "GAME_SCHEDULED")]
    GAMESCHEDULED,
    #[serde(rename = "GAME_STARTED")]
    GAMESTARTED,
    #[serde(rename = "GAME_STARTED_ERROR")]
    GAMESTARTEDERROR,
    #[serde(rename = "GAME_END_ERROR")]
    GAMEENDERROR,
    #[serde(rename = "QUEUE_DODGE")]
    QUEUEDODGE,
    #[serde(rename = "OWNER_TRANSFER")]
    OWNERTRANSFER,
    #[serde(rename = "SUB_INVITE")]
    SUBINVITE,
    #[serde(rename = "SUB_ACCEPT")]
    SUBACCEPT,
    #[serde(rename = "SUB_DECLINE")]
    SUBDECLINE,
    #[serde(rename = "SUB_REVOKE")]
    SUBREVOKE,
    #[serde(rename = "SUB_SUGGEST")]
    SUBSUGGEST,
    #[serde(rename = "SUB_ACCEPTSUGGEST")]
    SUBACCEPTSUGGEST,
    #[serde(rename = "SUB_DECLINESUGGEST")]
    SUBDECLINESUGGEST,
    #[serde(rename = "SUB_RECLAIM")]
    SUBRECLAIM,
    #[serde(rename = "SUB_LEAVE")]
    SUBLEAVE,
    #[serde(rename = "MEMBER_SUBBED")]
    MEMBERSUBBED,
    #[serde(rename = "MEMBER_SUB_REVOKE")]
    MEMBERSUBREVOKE,
    #[serde(rename = "MEMBER_SUB_RECLAIM")]
    MEMBERSUBRECLAIM,
    #[serde(rename = "VOTE_WITHDRAW_UPDATE")]
    VOTEWITHDRAWUPDATE,
    #[serde(rename = "VOTE_WITHDRAW_DISMISS")]
    VOTEWITHDRAWDISMISS,
    #[serde(rename = "WITHDRAW")]
    WITHDRAW,
    #[serde(rename = "ROUND_COMPLETE")]
    ROUNDCOMPLETE,
    #[serde(rename = "NO_SHOW_PING")]
    NOSHOWPING,
    #[serde(rename = "TIER_CHANGED")]
    TIERCHANGED,
    #[serde(rename = "BRACKET_ROSTER_REMOVED")]
    BRACKETROSTERREMOVED,
    #[serde(rename = "BRACKET_ROSTER_REPLACED")]
    BRACKETROSTERREPLACED,
    #[serde(rename = "CANNOT_FIND_MATCH")]
    CANNOTFINDMATCH,
    #[serde(rename = "BANNED_SMURF")]
    BANNEDSMURF,
    #[serde(rename = "BANNED_SMURF_TEAMMATE")]
    BANNEDSMURFTEAMMATE,
    #[serde(rename = "BANNED_SMURF_OPPONENT")]
    BANNEDSMURFOPPONENT,
    #[serde(rename = "TICKET_CHARGED")]
    TICKETCHARGED,
    #[serde(rename = "TICKET_REFUNDED")]
    TICKETREFUNDED,
    #[serde(rename = "TICKET_COULD_NOT_BE_CHARGED")]
    TICKETCOULDNOTBECHARGED,
    #[serde(rename = "SUB_INVITE_SELF")]
    SUBINVITESELF,
    #[serde(rename = "GAME_START_RETRY")]
    GAMESTARTRETRY,
    #[serde(rename = "GAME_START_RETRY_SUMMONERS")]
    GAMESTARTRETRYSUMMONERS,
    #[serde(rename = "GAME_START_RETRY_OPPONENT")]
    GAMESTARTRETRYOPPONENT,
    #[serde(rename = "GAME_START_FAILED")]
    GAMESTARTFAILED,
    #[serde(rename = "GAME_START_FAILED_SUMMONERS")]
    GAMESTARTFAILEDSUMMONERS,
    #[serde(rename = "GAME_START_FAILED_OPPONENT")]
    GAMESTARTFAILEDOPPONENT,
    #[serde(rename = "GAME_RESCHEDULED")]
    GAMERESCHEDULED,

}




