#![allow(dead_code)]

use std::{fmt, mem};

/// Copied from https://github.com/flier/tokio-kafka
/// The error code from Kafka server.
///
/// We use numeric codes to indicate what problem occurred on the server.
pub type ErrorCode = i16;

/// Various errors reported by a remote Kafka server.
///
/// We use numeric codes to indicate what problem occurred on the server.
/// These can be translated by the client into exceptions or
/// whatever the appropriate error handling mechanism in the client language.
///
/// See also [Kafka Errors](http://kafka.apache.org/protocol.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i16)]
pub enum KafkaCode {
    Unknown = -1,
    None = 0,

    OffsetOutOfRange = 1,

    CorruptMessage = 2,

    UnknownTopicOrPartition = 3,

    InvalidMessageSize = 4,

    LeaderNotAvailable = 5,

    NotLeaderForPartition = 6,

    RequestTimedOut = 7,

    BrokerNotAvailable = 8,

    ReplicaNotAvailable = 9,

    MessageSizeTooLarge = 10,

    StaleControllerEpoch = 11,

    OffsetMetadataTooLarge = 12,

    NetworkException = 13,

    GroupLoadInProgress = 14,

    GroupCoordinatorNotAvailable = 15,

    NotCoordinatorForGroup = 16,

    InvalidTopic = 17,

    RecordListTooLarge = 18,

    NotEnoughReplicas = 19,

    NotEnoughReplicasAfterAppend = 20,

    InvalidRequiredAcks = 21,

    IllegalGeneration = 22,

    InconsistentGroupProtocol = 23,

    InvalidGroupId = 24,

    UnknownMemberId = 25,

    InvalidSessionTimeout = 26,

    RebalanceInProgress = 27,

    InvalidOffsetCommitSize = 28,

    TopicAuthorizationFailed = 29,

    GroupAuthorizationFailed = 30,

    ClusterAuthorizationFailed = 31,

    InvalidTimestamp = 32,

    UnsupportedSaslMechanism = 33,

    IllegalSaslState = 34,

    UnsupportedVersion = 35,

    TopicAlreadyExists = 36,

    InvalidPartitions = 37,

    InvalidReplicationFactor = 38,

    InvalidReplicaAssignment = 39,

    InvalidConfig = 40,

    NotController = 41,

    InvalidRequest = 42,

    UnsupportedForMessageFormat = 43,

    PolicyViolation = 44,

    OutOfOrderSequenceNumber = 45,

    DuplicateSequenceNumber = 46,

    InvalidProducerEpoch = 47,

    InvalidTxnState = 48,

    InvalidProducerIdMapper = 49,

    InvalidTransactionTimeout = 50,

    ConcurrentTransactions = 51,

    TransactionCoordinatorFenced = 52,

    TransactionalIdAuthorizationFailed = 53,

    ProducerIdAuthorizationFailed = 54,

    SecurityDisabled = 55,

    BrokerAuthorizationFailed = 56,
}

impl KafkaCode {
    pub fn key(self) -> ErrorCode {
        unsafe { mem::transmute(self) }
    }
    pub fn reason(self) -> &'static str {
        match self {
            KafkaCode::Unknown => {
                "The server experienced an unexpected error when processing the request"
            }
            KafkaCode::None => "Ok",
            KafkaCode::OffsetOutOfRange => {
                "The requested offset is not within the range of offsets maintained by the server."
            }
            KafkaCode::CorruptMessage => {
                "This message has failed its CRC checksum, exceeds the valid size, or is otherwise corrupt."
            }
            KafkaCode::UnknownTopicOrPartition => "This server does not host this topic-partition.",
            KafkaCode::InvalidMessageSize => "The requested fetch size is invalid.",
            KafkaCode::LeaderNotAvailable => {
                "There is no leader for this topic-partition as we are in the middle of a leadership election."
            }
            KafkaCode::NotLeaderForPartition => {
                "This server is not the leader for that topic-partition."
            }
            KafkaCode::RequestTimedOut => "The request timed out.",
            KafkaCode::BrokerNotAvailable => "The broker is not available.",
            KafkaCode::ReplicaNotAvailable => {
                "The replica is not available for the requested topic-partition"
            }
            KafkaCode::MessageSizeTooLarge => {
                "The request included a message larger than the max message size the server will accept."
            }
            KafkaCode::StaleControllerEpoch => "The controller moved to another broker.",
            KafkaCode::OffsetMetadataTooLarge => {
                "The metadata field of the offset request was too large."
            }
            KafkaCode::NetworkException => {
                "The server disconnected before a response was received."
            }
            KafkaCode::GroupLoadInProgress => {
                "The coordinator is loading and hence can't process requests."
            }
            KafkaCode::GroupCoordinatorNotAvailable => "The coordinator is not available.",
            KafkaCode::NotCoordinatorForGroup => "This is not the correct coordinator.",
            KafkaCode::InvalidTopic => {
                "The request attempted to perform an operation on an invalid topic."
            }
            KafkaCode::RecordListTooLarge => {
                "The request included message batch larger than the configured segment size on the server."
            }
            KafkaCode::NotEnoughReplicas => {
                "Messages are rejected since there are fewer in-sync replicas than required."
            }
            KafkaCode::NotEnoughReplicasAfterAppend => {
                "Messages are written to the log, but to fewer in-sync replicas than required."
            }
            KafkaCode::InvalidRequiredAcks => {
                "Produce request specified an invalid value for required acks."
            }
            KafkaCode::IllegalGeneration => "Specified group generation id is not valid.",
            KafkaCode::InconsistentGroupProtocol => {
                "The group member's supported protocols are incompatible with those of existing members."
            }
            KafkaCode::InvalidGroupId => "The configured groupId is invalid",
            KafkaCode::UnknownMemberId => "The coordinator is not aware of this member.",
            KafkaCode::InvalidSessionTimeout => {
                "The session timeout is not within the range allowed by the broker"
            }
            KafkaCode::RebalanceInProgress => "The group is rebalancing, so a rejoin is needed.",
            KafkaCode::InvalidOffsetCommitSize => "The committing offset data size is not valid",
            KafkaCode::TopicAuthorizationFailed => "Topic authorization failed.",
            KafkaCode::GroupAuthorizationFailed => "Group authorization failed.",
            KafkaCode::ClusterAuthorizationFailed => "Cluster authorization failed.",
            KafkaCode::InvalidTimestamp => {
                "The timestamp of the message is out of acceptable range."
            }
            KafkaCode::UnsupportedSaslMechanism => {
                "The broker does not support the requested SASL mechanism."
            }
            KafkaCode::IllegalSaslState => "Request is not valid given the current SASL state.",
            KafkaCode::UnsupportedVersion => "The version of API is not supported.",
            KafkaCode::TopicAlreadyExists => "Topic with this name already exists.",
            KafkaCode::InvalidPartitions => "Number of partitions is invalid.",
            KafkaCode::InvalidReplicationFactor => "Replication-factor is invalid.",
            KafkaCode::InvalidReplicaAssignment => "Replica assignment is invalid.",
            KafkaCode::InvalidConfig => "Configuration is invalid.",
            KafkaCode::NotController => "This is not the correct controller for this cluster.",
            KafkaCode::InvalidRequest => {
                "This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker."
            }
            KafkaCode::UnsupportedForMessageFormat => {
                "The message format version on the broker does not support the request."
            }
            KafkaCode::PolicyViolation => {
                "Request parameters do not satisfy the configured policy."
            }
            KafkaCode::OutOfOrderSequenceNumber => {
                "The broker received an out of order sequence number"
            }
            KafkaCode::DuplicateSequenceNumber => "The broker received a duplicate sequence number",
            KafkaCode::InvalidProducerEpoch => "Producer attempted an operation with an old epoch",
            KafkaCode::InvalidTxnState => {
                "The producer attempted a transactional operation in an invalid state"
            }
            KafkaCode::InvalidProducerIdMapper => {
                "The producer attempted to use a producer id which is not currently assigned to its transactional id"
            }
            KafkaCode::InvalidTransactionTimeout => {
                "The transaction timeout is larger than the maximum value allowed by the broker"
            }
            KafkaCode::ConcurrentTransactions => {
                "The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing"
            }
            KafkaCode::TransactionCoordinatorFenced => {
                "Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer"
            }
            KafkaCode::TransactionalIdAuthorizationFailed => {
                "Transactional Id authorization failed"
            }
            KafkaCode::ProducerIdAuthorizationFailed => {
                "Producer is not authorized to use producer Ids, which is required to write idempotent data."
            }
            KafkaCode::SecurityDisabled => "Security features are disabled.",
            KafkaCode::BrokerAuthorizationFailed => "Broker authorization failed",
        }
    }
}

impl From<ErrorCode> for KafkaCode {
    fn from(v: ErrorCode) -> Self {
        if v < -1 || v > 56 {
            KafkaCode::Unknown
        } else {
            unsafe { mem::transmute(v) }
        }
    }
}

impl fmt::Display for KafkaCode {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:?}", self)
    }
}
