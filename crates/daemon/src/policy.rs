//! Code generated by `policy-ifgen`. DO NOT EDIT.
#![allow(clippy::duplicated_attributes)]
#![allow(clippy::enum_variant_names)]
#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
extern crate alloc;
use alloc::{string::String, vec::Vec};
use policy_ifgen::{
    macros::{actions, effect, effects, value},
    ClientError, Id, Value,
};
/// KeyBundle policy struct.
#[value]
pub struct KeyBundle {
    pub ident_key: Vec<u8>,
    pub sign_key: Vec<u8>,
    pub enc_key: Vec<u8>,
}
/// Role policy enum.
#[value]
pub enum Role {
    Owner,
    Admin,
    Operator,
    Member,
}
/// ChanOp policy enum.
#[value]
pub enum ChanOp {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
/// Enum of policy effects that can occur in response to a policy action.
#[effects]
pub enum Effect {
    TeamCreated(TeamCreated),
    TeamTerminated(TeamTerminated),
    MemberAdded(MemberAdded),
    MemberRemoved(MemberRemoved),
    OwnerAssigned(OwnerAssigned),
    AdminAssigned(AdminAssigned),
    OperatorAssigned(OperatorAssigned),
    OwnerRevoked(OwnerRevoked),
    AdminRevoked(AdminRevoked),
    OperatorRevoked(OperatorRevoked),
    LabelDefined(LabelDefined),
    LabelUndefined(LabelUndefined),
    LabelAssigned(LabelAssigned),
    LabelRevoked(LabelRevoked),
    NetworkNameSet(NetworkNameSet),
    NetworkNameUnset(NetworkNameUnset),
    BidiChannelCreated(BidiChannelCreated),
    BidiChannelReceived(BidiChannelReceived),
    UniChannelCreated(UniChannelCreated),
    UniChannelReceived(UniChannelReceived),
}
/// TeamCreated policy effect.
#[effect]
pub struct TeamCreated {
    pub owner_id: Id,
}
/// TeamTerminated policy effect.
#[effect]
pub struct TeamTerminated {
    pub owner_id: Id,
}
/// MemberAdded policy effect.
#[effect]
pub struct MemberAdded {
    pub user_id: Id,
    pub user_keys: KeyBundle,
}
/// MemberRemoved policy effect.
#[effect]
pub struct MemberRemoved {
    pub user_id: Id,
}
/// OwnerAssigned policy effect.
#[effect]
pub struct OwnerAssigned {
    pub user_id: Id,
}
/// AdminAssigned policy effect.
#[effect]
pub struct AdminAssigned {
    pub user_id: Id,
}
/// OperatorAssigned policy effect.
#[effect]
pub struct OperatorAssigned {
    pub user_id: Id,
}
/// OwnerRevoked policy effect.
#[effect]
pub struct OwnerRevoked {
    pub user_id: Id,
}
/// AdminRevoked policy effect.
#[effect]
pub struct AdminRevoked {
    pub user_id: Id,
}
/// OperatorRevoked policy effect.
#[effect]
pub struct OperatorRevoked {
    pub user_id: Id,
}
/// LabelDefined policy effect.
#[effect]
pub struct LabelDefined {
    pub label: i64,
}
/// LabelUndefined policy effect.
#[effect]
pub struct LabelUndefined {
    pub label: i64,
}
/// LabelAssigned policy effect.
#[effect]
pub struct LabelAssigned {
    pub user_id: Id,
    pub label: i64,
    pub op: ChanOp,
}
/// LabelRevoked policy effect.
#[effect]
pub struct LabelRevoked {
    pub user_id: Id,
    pub label: i64,
}
/// NetworkNameSet policy effect.
#[effect]
pub struct NetworkNameSet {
    pub user_id: Id,
    pub net_identifier: String,
}
/// NetworkNameUnset policy effect.
#[effect]
pub struct NetworkNameUnset {
    pub user_id: Id,
}
/// BidiChannelCreated policy effect.
#[effect]
pub struct BidiChannelCreated {
    pub parent_cmd_id: Id,
    pub author_id: Id,
    pub author_enc_key_id: Id,
    pub peer_id: Id,
    pub peer_enc_pk: Vec<u8>,
    pub label: i64,
    pub channel_key_id: Id,
}
/// BidiChannelReceived policy effect.
#[effect]
pub struct BidiChannelReceived {
    pub parent_cmd_id: Id,
    pub author_id: Id,
    pub author_enc_pk: Vec<u8>,
    pub peer_id: Id,
    pub peer_enc_key_id: Id,
    pub label: i64,
    pub encap: Vec<u8>,
}
/// UniChannelCreated policy effect.
#[effect]
pub struct UniChannelCreated {
    pub parent_cmd_id: Id,
    pub author_id: Id,
    pub writer_id: Id,
    pub reader_id: Id,
    pub author_enc_key_id: Id,
    pub peer_enc_pk: Vec<u8>,
    pub label: i64,
    pub channel_key_id: Id,
}
/// UniChannelReceived policy effect.
#[effect]
pub struct UniChannelReceived {
    pub parent_cmd_id: Id,
    pub author_id: Id,
    pub writer_id: Id,
    pub reader_id: Id,
    pub author_enc_pk: Vec<u8>,
    pub peer_enc_key_id: Id,
    pub label: i64,
    pub encap: Vec<u8>,
}
/// Implements all supported policy actions.
#[actions]
pub trait ActorExt {
    fn create_team(
        &mut self,
        owner_keys: KeyBundle,
        nonce: Vec<u8>,
    ) -> Result<(), ClientError>;
    fn terminate_team(&mut self) -> Result<(), ClientError>;
    fn add_member(&mut self, user_keys: KeyBundle) -> Result<(), ClientError>;
    fn remove_member(&mut self, user_id: Id) -> Result<(), ClientError>;
    fn assign_role(&mut self, user_id: Id, role: Role) -> Result<(), ClientError>;
    fn revoke_role(&mut self, user_id: Id, role: Role) -> Result<(), ClientError>;
    fn define_label(&mut self, label: i64) -> Result<(), ClientError>;
    fn undefine_label(&mut self, label: i64) -> Result<(), ClientError>;
    fn assign_label(
        &mut self,
        user_id: Id,
        label: i64,
        op: ChanOp,
    ) -> Result<(), ClientError>;
    fn revoke_label(&mut self, user_id: Id, label: i64) -> Result<(), ClientError>;
    fn set_network_name(
        &mut self,
        user_id: Id,
        net_identifier: String,
    ) -> Result<(), ClientError>;
    fn unset_network_name(&mut self, user_id: Id) -> Result<(), ClientError>;
    fn create_bidi_channel(
        &mut self,
        peer_id: Id,
        label: i64,
    ) -> Result<(), ClientError>;
    fn create_uni_channel(
        &mut self,
        writer_id: Id,
        reader_id: Id,
        label: i64,
    ) -> Result<(), ClientError>;
}
