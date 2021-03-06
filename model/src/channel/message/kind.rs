#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MessageType {
    Regular = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelMessagePinned = 6,
    GuildMemberJoin = 7,
    UserPremiumSub = 8,
    UserPremiumSubTier1 = 9,
    UserPremiumSubTier2 = 10,
    UserPremiumSubTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
}
