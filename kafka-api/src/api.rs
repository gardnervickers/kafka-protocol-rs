# ! [ allow ( dead_code ) ]use from_variants::FromVariants;
use kafka_protocol::KafkaRpcType;
use kafka_protocol_derive::KafkaRpc;
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AlterReplicaLogDirsRequest {
    #[doc = "The alterations to make for each directory."]
    #[kafka(added = 0i16)]
    pub dirs: Vec<AlterReplicaLogDir>,
}
impl crate::KafkaRequestBody for AlterReplicaLogDirsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        34i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterReplicaLogDirTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The partition indexes."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterReplicaLogDir {
    #[doc = "The absolute directory path."]
    #[kafka(added = 0i16)]
    pub path: String,
    #[doc = "The topics to add to the directory."]
    #[kafka(added = 0i16)]
    pub topics: Vec<AlterReplicaLogDirTopic>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AlterReplicaLogDirsResponse {
    #[doc = "Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each topic."]
    #[kafka(added = 0i16)]
    pub results: Vec<AlterReplicaLogDirTopicResult>,
}
impl crate::KafkaResponseBody for AlterReplicaLogDirsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        34i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterReplicaLogDirPartitionResult {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterReplicaLogDirTopicResult {
    #[doc = "The name of the topic."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The results for each partition."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16)]
pub struct ElectPreferredLeadersRequest {
    #[doc = "The topic partitions to elect the preferred leader of."]
    #[kafka(added = 0i16)]
    pub topic_partitions: Option<Vec<TopicPartitions>>,
    #[doc = "The time in ms to wait for the election to complete."]
    #[kafka(added = 0i16, default = "60000")]
    pub timeout_ms: i32,
}
impl crate::KafkaRequestBody for ElectPreferredLeadersRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        43i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TopicPartitions {
    #[doc = "The name of a topic."]
    #[kafka(added = 0i16)]
    pub topic: String,
    #[doc = "The partitions of this topic whose preferred leader should be elected"]
    #[kafka(added = 0i16)]
    pub partition_id: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16)]
pub struct ElectPreferredLeadersResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub replica_election_results: Vec<ReplicaElectionResult>,
}
impl crate::KafkaResponseBody for ElectPreferredLeadersResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        43i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct PartitionResult {
    #[doc = "The partition id"]
    #[kafka(added = 0i16)]
    pub partition_id: i32,
    #[doc = "The result error, or zero if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The result message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ReplicaElectionResult {
    #[doc = "The topic name"]
    #[kafka(added = 0i16)]
    pub topic: String,
    #[doc = "The results for each partition"]
    #[kafka(added = 0i16)]
    pub partition_result: Vec<PartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct InitProducerIdRequest {
    #[doc = "The transactional id, or null if the producer is not transactional."]
    #[kafka(added = 0i16)]
    pub transactional_id: Option<String>,
    #[doc = "The time in ms to wait for before aborting idle transactions sent by this producer."]
    #[kafka(added = 0i16)]
    pub transaction_timeout_ms: i32,
}
impl crate::KafkaRequestBody for InitProducerIdRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        22i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct InitProducerIdResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The current producer id."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "The current epoch associated with the producer id."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
}
impl crate::KafkaResponseBody for InitProducerIdResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        22i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 3i16)]
pub struct DeleteTopicsRequest {
    #[doc = "The names of the topics to delete"]
    #[kafka(added = 0i16)]
    pub topic_names: Vec<String>,
    #[doc = "The length of time in milliseconds to wait for the deletions to complete."]
    #[kafka(added = 0i16)]
    pub timeout_ms: i32,
}
impl crate::KafkaRequestBody for DeleteTopicsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        20i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 3i16)]
pub struct DeleteTopicsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each topic."]
    #[kafka(added = 0i16)]
    pub responses: Vec<DeletableTopicResult>,
}
impl crate::KafkaResponseBody for DeleteTopicsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        20i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeletableTopicResult {
    #[doc = "The topic name"]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The deletion error, or 0 if the deletion succeeded."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteAclsRequest {
    #[doc = "The filters to use when deleting ACLs."]
    #[kafka(added = 0i16)]
    pub filters: Vec<DeleteAclsFilter>,
}
impl crate::KafkaRequestBody for DeleteAclsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        31i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteAclsFilter {
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type_filter: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub resource_name_filter: Option<String>,
    #[doc = "The pattern type."]
    #[kafka(added = 1i16, default = "3")]
    pub pattern_type_filter: i8,
    #[doc = "The principal filter, or null to accept all principals."]
    #[kafka(added = 0i16)]
    pub principal_filter: Option<String>,
    #[doc = "The host filter, or null to accept all hosts."]
    #[kafka(added = 0i16)]
    pub host_filter: Option<String>,
    #[doc = "The ACL operation."]
    #[kafka(added = 0i16)]
    pub operation: i8,
    #[doc = "The permission type."]
    #[kafka(added = 0i16)]
    pub permission_type: i8,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteAclsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each filter."]
    #[kafka(added = 0i16)]
    pub filter_results: Vec<DeleteAclsFilterResult>,
}
impl crate::KafkaResponseBody for DeleteAclsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        31i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteAclsMatchingAcl {
    #[doc = "The deletion error code, or 0 if the deletion succeeded."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The deletion error message, or null if the deletion succeeded."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "The ACL resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The ACL resource name."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
    #[doc = "The ACL resource pattern type."]
    #[kafka(added = 1i16, default = "3")]
    pub pattern_type: i8,
    #[doc = "The ACL principal."]
    #[kafka(added = 0i16)]
    pub principal: String,
    #[doc = "The ACL host."]
    #[kafka(added = 0i16)]
    pub host: String,
    #[doc = "The ACL operation."]
    #[kafka(added = 0i16)]
    pub operation: i8,
    #[doc = "The ACL permission type."]
    #[kafka(added = 0i16)]
    pub permission_type: i8,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteAclsFilterResult {
    #[doc = "The error code, or 0 if the filter succeeded."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if the filter succeeded."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "The ACLs which matched this filter."]
    #[kafka(added = 0i16)]
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct DescribeConfigsRequest {
    #[doc = "The resources whose configurations we want to describe."]
    #[kafka(added = 0i16)]
    pub resources: Vec<DescribeConfigsResource>,
    #[doc = "True if we should include all synonyms."]
    #[kafka(added = 1i16, default = "false")]
    pub include_synoyms: bool,
}
impl crate::KafkaRequestBody for DescribeConfigsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        32i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeConfigsResource {
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
    #[doc = "The configuration keys to list, or null to list all configuration keys."]
    #[kafka(added = 0i16)]
    pub configuration_keys: Option<Vec<String>>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct DescribeConfigsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each resource."]
    #[kafka(added = 0i16)]
    pub results: Vec<DescribeConfigsResult>,
}
impl crate::KafkaResponseBody for DescribeConfigsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        32i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeConfigsSynonym {
    #[doc = "The synonym name."]
    #[kafka(added = 1i16)]
    pub name: String,
    #[doc = "The synonym value."]
    #[kafka(added = 1i16)]
    pub value: Option<String>,
    #[doc = "The synonym source."]
    #[kafka(added = 1i16)]
    pub source: i8,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeConfigsResourceResult {
    #[doc = "The configuration name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The configuration value."]
    #[kafka(added = 0i16)]
    pub value: Option<String>,
    #[doc = "True if the configuration is read-only."]
    #[kafka(added = 0i16)]
    pub read_only: bool,
    #[doc = "True if the configuration is not set."]
    #[kafka(added = 0i16)]
    pub is_default: bool,
    #[doc = "The configuration source."]
    #[kafka(added = 1i16, default = "-1")]
    pub config_source: i8,
    #[doc = "True if this configuration is sensitive."]
    #[kafka(added = 0i16)]
    pub is_sensitive: bool,
    #[doc = "The synonyms for this configuration key."]
    #[kafka(added = 1i16)]
    pub synonyms: Vec<DescribeConfigsSynonym>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeConfigsResult {
    #[doc = "The error code, or 0 if we were able to successfully describe the configurations."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if we were able to successfully describe the configurations."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
    #[doc = "Each listed configuration."]
    #[kafka(added = 0i16)]
    pub configs: Vec<DescribeConfigsResourceResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteRecordsRequest {
    #[doc = "Each topic that we want to delete records from."]
    #[kafka(added = 0i16)]
    pub topics: Vec<DeleteRecordsTopic>,
    #[doc = "How long to wait for the deletion to complete, in milliseconds."]
    #[kafka(added = 0i16)]
    pub timeout_ms: i32,
}
impl crate::KafkaRequestBody for DeleteRecordsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        21i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteRecordsPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The deletion offset."]
    #[kafka(added = 0i16)]
    pub offset: i64,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteRecordsTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition that we want to delete records from."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<DeleteRecordsPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteRecordsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "Each topic that we wanted to delete records from."]
    #[kafka(added = 0i16)]
    pub topics: Vec<DeleteRecordsTopicResult>,
}
impl crate::KafkaResponseBody for DeleteRecordsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        21i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteRecordsPartitionResult {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The partition low water mark."]
    #[kafka(added = 0i16)]
    pub low_watermark: i64,
    #[doc = "The deletion error code, or 0 if the deletion succeeded."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeleteRecordsTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition that we wanted to delete records from."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<DeleteRecordsPartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteGroupsRequest {
    #[doc = "The group names to delete."]
    #[kafka(added = 0i16)]
    pub groups_names: Vec<String>,
}
impl crate::KafkaRequestBody for DeleteGroupsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        42i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DeleteGroupsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The deletion results"]
    #[kafka(added = 0i16)]
    pub results: Vec<DeletableGroupResult>,
}
impl crate::KafkaResponseBody for DeleteGroupsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        42i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DeletableGroupResult {
    #[doc = "The group id"]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The deletion error, or 0 if the deletion succeeded."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct FindCoordinatorRequest {
    #[doc = "The coordinator key."]
    #[kafka(added = 0i16)]
    pub key: String,
    #[doc = "The coordinator key type.  (Group, transaction, etc.)"]
    #[kafka(added = 1i16, default = "0")]
    pub key_type: i8,
}
impl crate::KafkaRequestBody for FindCoordinatorRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        10i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct FindCoordinatorResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if there was no error."]
    #[kafka(added = 1i16)]
    pub error_message: Option<String>,
    #[doc = "The node id."]
    #[kafka(added = 0i16)]
    pub node_id: i32,
    #[doc = "The host name."]
    #[kafka(added = 0i16)]
    pub host: String,
    #[doc = "The port."]
    #[kafka(added = 0i16)]
    pub port: i32,
}
impl crate::KafkaResponseBody for FindCoordinatorResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        10i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AddOffsetsToTxnRequest {
    #[doc = "The transactional id corresponding to the transaction."]
    #[kafka(added = 0i16)]
    pub transactional_id: String,
    #[doc = "Current producer id in use by the transactional id."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "Current epoch associated with the producer id."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
    #[doc = "The unique group identifier."]
    #[kafka(added = 0i16)]
    pub group_id: String,
}
impl crate::KafkaRequestBody for AddOffsetsToTxnRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        25i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AddOffsetsToTxnResponse {
    #[doc = "Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The response error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for AddOffsetsToTxnResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        25i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct SaslAuthenticateRequest {
    #[doc = "The SASL authentication bytes from the client, as defined by the SASL mechanism."]
    #[kafka(added = 0i16)]
    pub auth_bytes: Vec<u8>,
}
impl crate::KafkaRequestBody for SaslAuthenticateRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        36i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct SaslAuthenticateResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "The SASL authentication bytes from the server, as defined by the SASL mechanism."]
    #[kafka(added = 0i16)]
    pub auth_bytes: Vec<u8>,
    #[doc = "The SASL authentication bytes from the server, as defined by the SASL mechanism."]
    #[kafka(added = 1i16, default = "0")]
    pub session_lifetime_ms: i64,
}
impl crate::KafkaResponseBody for SaslAuthenticateResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        36i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct ListOffsetRequest {
    #[doc = "The broker ID of the requestor, or -1 if this request is being made by a normal consumer."]
    #[kafka(added = 0i16)]
    pub replica_id: i32,
    #[doc = "This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records"]
    #[kafka(added = 2i16)]
    pub isolation_level: i8,
    #[doc = "Each topic in the request."]
    #[kafka(added = 0i16)]
    pub topics: Vec<ListOffsetTopic>,
}
impl crate::KafkaRequestBody for ListOffsetRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        2i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ListOffsetPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The current leader epoch."]
    #[kafka(added = 4i16)]
    pub current_leader_epoch: i32,
    #[doc = "The current timestamp."]
    #[kafka(added = 0i16)]
    pub timestamp: i64,
    #[doc = "The maximum number of offsets to report."]
    #[kafka(added = 0i16)]
    pub max_num_offsets: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ListOffsetTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition in the request."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<ListOffsetPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct ListOffsetResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 2i16)]
    pub throttle_time_ms: i32,
    #[doc = "Each topic in the response."]
    #[kafka(added = 0i16)]
    pub topics: Vec<ListOffsetTopicResponse>,
}
impl crate::KafkaResponseBody for ListOffsetResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        2i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ListOffsetPartitionResponse {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The partition error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The result offsets."]
    #[kafka(added = 0i16)]
    pub old_style_offsets: Vec<i64>,
    #[doc = "The timestamp associated with the returned offset."]
    #[kafka(added = 1i16, default = "-1")]
    pub timestamp: i64,
    #[doc = "The returned offset."]
    #[kafka(added = 1i16, default = "-1")]
    pub offset: i64,
    #[kafka(added = 4i16)]
    pub leader_epoch: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ListOffsetTopicResponse {
    #[doc = "The topic name"]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition in the response."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<ListOffsetPartitionResponse>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct DescribeGroupsRequest {
    #[doc = "The names of the groups to describe"]
    #[kafka(added = 0i16)]
    pub groups: Vec<String>,
}
impl crate::KafkaRequestBody for DescribeGroupsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        15i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct DescribeGroupsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "Each described group."]
    #[kafka(added = 0i16)]
    pub groups: Vec<DescribedGroup>,
}
impl crate::KafkaResponseBody for DescribeGroupsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        15i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribedGroupMember {
    #[doc = "The member ID assigned by the group coordinator."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[doc = "The client ID used in the member's latest join group request."]
    #[kafka(added = 0i16)]
    pub client_id: String,
    #[doc = "The client host."]
    #[kafka(added = 0i16)]
    pub client_host: String,
    #[doc = "The metadata corresponding to the current group protocol in use."]
    #[kafka(added = 0i16)]
    pub member_metadata: Vec<u8>,
    #[doc = "The current assignment provided by the group leader."]
    #[kafka(added = 0i16)]
    pub member_assignment: Vec<u8>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribedGroup {
    #[doc = "The describe error, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The group ID string."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The group state string, or the empty string."]
    #[kafka(added = 0i16)]
    pub group_state: String,
    #[doc = "The group protocol type, or the empty string."]
    #[kafka(added = 0i16)]
    pub protocol_type: String,
    #[doc = "The group protocol data, or the empty string."]
    #[kafka(added = 0i16)]
    pub protocol_data: String,
    #[doc = "The group members."]
    #[kafka(added = 0i16)]
    pub members: Vec<DescribedGroupMember>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 6i16)]
pub struct OffsetCommitRequest {
    #[doc = "The unique group identifier."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The generation of the group."]
    #[kafka(added = 1i16, default = "-1")]
    pub generation_id: i32,
    #[doc = "The member ID assigned by the group coordinator."]
    #[kafka(added = 1i16)]
    pub member_id: String,
    #[doc = "The time period in ms to retain the offset."]
    #[kafka(added = 2i16, removed = 4i16, default = "-1")]
    pub retention_time_ms: i64,
    #[doc = "The topics to commit offsets for."]
    #[kafka(added = 0i16)]
    pub topics: Vec<OffsetCommitRequestTopic>,
}
impl crate::KafkaRequestBody for OffsetCommitRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        8i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetCommitRequestPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The message offset to be committed."]
    #[kafka(added = 0i16)]
    pub committed_offset: i64,
    #[doc = "The leader epoch of this partition."]
    #[kafka(added = 6i16, default = "-1")]
    pub committed_leader_epoch: i32,
    #[doc = "The timestamp of the commit."]
    #[kafka(added = 1i16, default = "-1")]
    pub commit_timestamp: i64,
    #[doc = "Any associated metadata the client wants to keep."]
    #[kafka(added = 0i16)]
    pub committed_metadata: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetCommitRequestTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition to commit offsets for."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<OffsetCommitRequestPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 6i16)]
pub struct OffsetCommitResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 3i16)]
    pub throttle_time_ms: i32,
    #[doc = "The responses for each topic."]
    #[kafka(added = 0i16)]
    pub topics: Vec<OffsetCommitResponseTopic>,
}
impl crate::KafkaResponseBody for OffsetCommitResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        8i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetCommitResponsePartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetCommitResponseTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The responses for each partition in the topic."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<OffsetCommitResponsePartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ApiVersionsRequest {}
impl crate::KafkaRequestBody for ApiVersionsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        18i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ApiVersionsResponse {
    #[doc = "The top-level error code."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The APIs supported by the broker."]
    #[kafka(added = 0i16)]
    pub api_keys: Vec<ApiVersionsResponseKey>,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for ApiVersionsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        18i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ApiVersionsResponseKey {
    #[doc = "The API index."]
    #[kafka(added = 0i16)]
    pub index: i16,
    #[doc = "The minimum supported version, inclusive."]
    #[kafka(added = 0i16)]
    pub min_version: i16,
    #[doc = "The maximum supported version, inclusive."]
    #[kafka(added = 0i16)]
    pub max_version: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16)]
pub struct WriteTxnMarkersRequest {
    #[doc = "The transaction markers to be written."]
    #[kafka(added = 0i16)]
    pub markers: Vec<WritableTxnMarker>,
}
impl crate::KafkaRequestBody for WriteTxnMarkersRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        27i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct WritableTxnMarkerTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The indexes of the partitions to write transaction markers for."]
    #[kafka(added = 0i16)]
    pub partition_indexes: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct WritableTxnMarker {
    #[doc = "The current producer ID."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "The current epoch associated with the producer ID."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
    #[doc = "The result of the transaction to write to the partitions (false = ABORT, true = COMMIT)."]
    #[kafka(added = 0i16)]
    pub transaction_result: bool,
    #[doc = "Each topic that we want to write transaction marker(s) for."]
    #[kafka(added = 0i16)]
    pub topics: Vec<WritableTxnMarkerTopic>,
    #[doc = "Epoch associated with the transaction state partition hosted by this transaction coordinator"]
    #[kafka(added = 0i16)]
    pub coordinator_epoch: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16)]
pub struct WriteTxnMarkersResponse {
    #[doc = "The results for writing makers."]
    #[kafka(added = 0i16)]
    pub markers: Vec<WritableTxnMarkerResult>,
}
impl crate::KafkaResponseBody for WriteTxnMarkersResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        27i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct WritableTxnMarkerPartitionResult {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct WritableTxnMarkerTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The results by partition."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct WritableTxnMarkerResult {
    #[doc = "The current producer ID in use by the transactional ID."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "The results by topic."]
    #[kafka(added = 0i16)]
    pub topics: Vec<WritableTxnMarkerTopicResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeDelegationTokenRequest {
    #[doc = "Each owner that we want to describe delegation tokens for, or null to describe all tokens."]
    #[kafka(added = 0i16)]
    pub owners: Option<Vec<DescribeDelegationTokenOwner>>,
}
impl crate::KafkaRequestBody for DescribeDelegationTokenRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        41i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeDelegationTokenOwner {
    #[doc = "The owner principal type."]
    #[kafka(added = 0i16)]
    pub principal_type: String,
    #[doc = "The owner principal name."]
    #[kafka(added = 0i16)]
    pub principal_name: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeDelegationTokenResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The tokens."]
    #[kafka(added = 0i16)]
    pub tokens: Vec<DescribedDelegationToken>,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for DescribeDelegationTokenResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        41i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribedDelegationTokenRenewer {
    #[doc = "The renewer principal type"]
    #[kafka(added = 0i16)]
    pub principal_type: String,
    #[doc = "The renewer principal name"]
    #[kafka(added = 0i16)]
    pub principal_name: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribedDelegationToken {
    #[doc = "The token principal type."]
    #[kafka(added = 0i16)]
    pub principal_type: String,
    #[doc = "The token principal name."]
    #[kafka(added = 0i16)]
    pub principal_name: String,
    #[doc = "The token issue timestamp in milliseconds."]
    #[kafka(added = 0i16)]
    pub issue_timestamp: i64,
    #[doc = "The token expiry timestamp in milliseconds."]
    #[kafka(added = 0i16)]
    pub expiry_timestamp: i64,
    #[doc = "The token maximum timestamp length in milliseconds."]
    #[kafka(added = 0i16)]
    pub max_timestamp: i64,
    #[doc = "The token ID."]
    #[kafka(added = 0i16)]
    pub token_id: String,
    #[doc = "The token HMAC."]
    #[kafka(added = 0i16)]
    pub hmac: Vec<u8>,
    #[doc = "Those who are able to renew this token before it expires."]
    #[kafka(added = 0i16)]
    pub renewers: Vec<DescribedDelegationTokenRenewer>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 7i16)]
pub struct ProduceRequest {
    #[doc = "The transactional ID, or null if the producer is not transactional."]
    #[kafka(added = 3i16)]
    pub transactional_id: Option<String>,
    #[doc = "The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR."]
    #[kafka(added = 0i16)]
    pub acks: i16,
    #[doc = "The timeout to await a response in miliseconds."]
    #[kafka(added = 0i16)]
    pub timeout_ms: i32,
    #[doc = "Each topic to produce to."]
    #[kafka(added = 0i16)]
    pub topics: Vec<TopicProduceData>,
}
impl crate::KafkaRequestBody for ProduceRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        0i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct PartitionProduceData {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The record data to be produced."]
    #[kafka(added = 0i16)]
    pub records: Option<Vec<u8>>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TopicProduceData {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition to produce to."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<PartitionProduceData>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 7i16)]
pub struct ProduceResponse {
    #[doc = "Each produce response"]
    #[kafka(added = 0i16)]
    pub responses: Vec<TopicProduceResponse>,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for ProduceResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        0i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct PartitionProduceResponse {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The base offset."]
    #[kafka(added = 0i16)]
    pub base_offset: i64,
    #[doc = "The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended."]
    #[kafka(added = 2i16, default = "-1")]
    pub log_append_time_ms: i64,
    #[doc = "The log start offset."]
    #[kafka(added = 5i16, default = "-1")]
    pub log_start_offset: i64,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TopicProduceResponse {
    #[doc = "The topic name"]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition that we produced to within the topic."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<PartitionProduceResponse>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ListGroupsRequest {}
impl crate::KafkaRequestBody for ListGroupsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        16i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ListGroupsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "Each group in the response."]
    #[kafka(added = 0i16)]
    pub groups: Vec<ListedGroup>,
}
impl crate::KafkaResponseBody for ListGroupsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        16i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ListedGroup {
    #[doc = "The group ID."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The group protocol type."]
    #[kafka(added = 0i16)]
    pub protocol_type: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct OffsetForLeaderEpochRequest {
    #[doc = "Each topic to get offsets for."]
    #[kafka(added = 0i16)]
    pub topics: Vec<OffsetForLeaderTopic>,
}
impl crate::KafkaRequestBody for OffsetForLeaderEpochRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        23i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetForLeaderPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "An epoch used to fence consumers/replicas with old metadata.  If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned."]
    #[kafka(added = 2i16, default = "-1")]
    pub current_leader_epoch: i32,
    #[doc = "The epoch to look up an offset for."]
    #[kafka(added = 0i16)]
    pub leader_epoch: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetForLeaderTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition to get offsets for."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<OffsetForLeaderPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct OffsetForLeaderEpochResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 2i16)]
    pub throttle_time_ms: i32,
    #[doc = "Each topic we fetched offsets for."]
    #[kafka(added = 0i16)]
    pub topics: Vec<OffsetForLeaderTopicResult>,
}
impl crate::KafkaResponseBody for OffsetForLeaderEpochResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        23i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetForLeaderPartitionResult {
    #[doc = "The error code 0, or if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The leader epoch of the partition."]
    #[kafka(added = 1i16, default = "-1")]
    pub leader_epoch: i32,
    #[doc = "The end offset of the epoch."]
    #[kafka(added = 0i16)]
    pub end_offset: i64,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetForLeaderTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "Each partition in the topic we fetched offsets for."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<OffsetForLeaderPartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 7i16)]
pub struct MetadataRequest {
    #[doc = "The topics to fetch metadata for."]
    #[kafka(added = 0i16)]
    pub topics: Option<Vec<MetadataRequestTopic>>,
    #[doc = "If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so."]
    #[kafka(added = 4i16, default = "true")]
    pub allow_auto_topic_creation: bool,
}
impl crate::KafkaRequestBody for MetadataRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        3i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct MetadataRequestTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 7i16)]
pub struct MetadataResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 3i16)]
    pub throttle_time_ms: i32,
    #[doc = "Each broker in the response."]
    #[kafka(added = 0i16)]
    pub brokers: Vec<MetadataResponseBroker>,
    #[doc = "The cluster ID that responding broker belongs to."]
    #[kafka(added = 2i16)]
    pub cluster_id: Option<String>,
    #[doc = "The ID of the controller broker."]
    #[kafka(added = 1i16, default = "-1")]
    pub controller_id: i32,
    #[doc = "Each topic in the response."]
    #[kafka(added = 0i16)]
    pub topics: Vec<MetadataResponseTopic>,
}
impl crate::KafkaResponseBody for MetadataResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        3i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct MetadataResponseBroker {
    #[doc = "The broker ID."]
    #[kafka(added = 0i16)]
    pub node_id: i32,
    #[doc = "The broker hostname."]
    #[kafka(added = 0i16)]
    pub host: String,
    #[doc = "The broker port."]
    #[kafka(added = 0i16)]
    pub port: i32,
    #[doc = "The rack of the broker, or null if it has not been assigned to a rack."]
    #[kafka(added = 1i16)]
    pub rack: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct MetadataResponsePartition {
    #[doc = "The partition error, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The ID of the leader broker."]
    #[kafka(added = 0i16)]
    pub leader_id: i32,
    #[doc = "The leader epoch of this partition."]
    #[kafka(added = 7i16, default = "-1")]
    pub leader_epoch: i32,
    #[doc = "The set of all nodes that host this partition."]
    #[kafka(added = 0i16)]
    pub replica_nodes: Vec<i32>,
    #[doc = "The set of nodes that are in sync with the leader for this partition."]
    #[kafka(added = 0i16)]
    pub isr_nodes: Vec<i32>,
    #[doc = "The set of offline replicas of this partition."]
    #[kafka(added = 5i16)]
    pub offline_replicas: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct MetadataResponseTopic {
    #[doc = "The topic error, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "True if the topic is internal."]
    #[kafka(added = 1i16, default = "false")]
    pub is_internal: bool,
    #[doc = "Each partition in the topic."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<MetadataResponsePartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ControlledShutdownRequest {
    #[doc = "The id of the broker for which controlled shutdown has been requested."]
    #[kafka(added = 0i16)]
    pub broker_id: i32,
    #[doc = "The broker epoch."]
    #[kafka(added = 2i16, default = "-1")]
    pub broker_epoch: i64,
}
impl crate::KafkaRequestBody for ControlledShutdownRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        7i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct ControlledShutdownResponse {
    #[doc = "The top-level error code."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The partitions that the broker still leads."]
    #[kafka(added = 0i16)]
    pub remaining_partitions: Vec<RemainingPartition>,
}
impl crate::KafkaResponseBody for ControlledShutdownResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        7i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct RemainingPartition {
    #[doc = "The name of the topic."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The index of the partition."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct SyncGroupRequest {
    #[doc = "The unique group identifier."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The generation of the group."]
    #[kafka(added = 0i16)]
    pub generation_id: i32,
    #[doc = "The member ID assigned by the group."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[doc = "Each assignment."]
    #[kafka(added = 0i16)]
    pub assignments: Vec<SyncGroupRequestAssignment>,
}
impl crate::KafkaRequestBody for SyncGroupRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        14i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct SyncGroupRequestAssignment {
    #[doc = "The ID of the member to assign."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[doc = "The member assignment."]
    #[kafka(added = 0i16)]
    pub assignment: Vec<u8>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct SyncGroupResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The member assignment."]
    #[kafka(added = 0i16)]
    pub assignment: Vec<u8>,
}
impl crate::KafkaResponseBody for SyncGroupResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        14i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AlterConfigsRequest {
    #[doc = "The updates for each resource."]
    #[kafka(added = 0i16)]
    pub resources: Vec<AlterConfigsResource>,
    #[doc = "True if we should validate the request, but not change the configurations."]
    #[kafka(added = 0i16)]
    pub validate_only: bool,
}
impl crate::KafkaRequestBody for AlterConfigsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        33i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterableConfig {
    #[doc = "The configuration key name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The value to set for the configuration key."]
    #[kafka(added = 0i16)]
    pub value: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterConfigsResource {
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
    #[doc = "The configurations."]
    #[kafka(added = 0i16)]
    pub configs: Vec<AlterableConfig>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AlterConfigsResponse {
    #[doc = "Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The responses for each resource."]
    #[kafka(added = 0i16)]
    pub resources: Vec<AlterConfigsResourceResponse>,
}
impl crate::KafkaResponseBody for AlterConfigsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        33i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AlterConfigsResourceResponse {
    #[doc = "The resource error code."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The resource error message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreateAclsRequest {
    #[doc = "The ACLs that we want to create."]
    #[kafka(added = 0i16)]
    pub creations: Vec<CreatableAcl>,
}
impl crate::KafkaRequestBody for CreateAclsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        30i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableAcl {
    #[doc = "The type of the resource."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name for the ACL."]
    #[kafka(added = 0i16)]
    pub resource_name: String,
    #[doc = "The pattern type for the ACL."]
    #[kafka(added = 1i16, default = "3")]
    pub resource_pattern_type: i8,
    #[doc = "The principal for the ACL."]
    #[kafka(added = 0i16)]
    pub principal: String,
    #[doc = "The host for the ACL."]
    #[kafka(added = 0i16)]
    pub host: String,
    #[doc = "The operation type for the ACL (read, write, etc.)."]
    #[kafka(added = 0i16)]
    pub operation: i8,
    #[doc = "The permission type for the ACL (allow, deny, etc.)."]
    #[kafka(added = 0i16)]
    pub permission_type: i8,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreateAclsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each ACL creation."]
    #[kafka(added = 0i16)]
    pub results: Vec<CreatableAclResult>,
}
impl crate::KafkaResponseBody for CreateAclsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        30i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableAclResult {
    #[doc = "The result error, or zero if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The result message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct OffsetFetchRequest {
    #[doc = "The group to fetch offsets for."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "Each topic we would like to fetch offsets for, or null to fetch offsets for all topics."]
    #[kafka(added = 0i16)]
    pub topics: Option<Vec<OffsetFetchRequestTopic>>,
}
impl crate::KafkaRequestBody for OffsetFetchRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        9i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetFetchRequestTopic {
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The partition indexes we would like to fetch offsets for."]
    #[kafka(added = 0i16)]
    pub partition_indexes: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct OffsetFetchResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 3i16)]
    pub throttle_time_ms: i32,
    #[doc = "The responses per topic."]
    #[kafka(added = 0i16)]
    pub topics: Vec<OffsetFetchResponseTopic>,
    #[doc = "The top-level error code, or 0 if there was no error."]
    #[kafka(added = 2i16, default = "0")]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for OffsetFetchResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        9i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetFetchResponsePartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The committed message offset."]
    #[kafka(added = 0i16)]
    pub committed_offset: i64,
    #[doc = "The leader epoch."]
    #[kafka(added = 5i16)]
    pub committed_leader_epoch: i32,
    #[doc = "The partition metadata."]
    #[kafka(added = 0i16)]
    pub metadata: Option<String>,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct OffsetFetchResponseTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The responses per partition"]
    #[kafka(added = 0i16)]
    pub partitions: Vec<OffsetFetchResponsePartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct StopReplicaRequest {
    #[doc = "The controller id."]
    #[kafka(added = 0i16)]
    pub controller_id: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 0i16)]
    pub controller_epoch: i32,
    #[doc = "The broker epoch."]
    #[kafka(added = 1i16, default = "-1")]
    pub broker_epoch: i64,
    #[doc = "Whether these partitions should be deleted."]
    #[kafka(added = 0i16)]
    pub delete_partitions: bool,
    #[doc = "The partitions to stop."]
    #[kafka(added = 0i16)]
    pub partitions_v0: Vec<StopReplicaRequestPartitionV0>,
    #[doc = "The topics to stop."]
    #[kafka(added = 1i16)]
    pub topics: Vec<StopReplicaRequestTopic>,
}
impl crate::KafkaRequestBody for StopReplicaRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        5i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct StopReplicaRequestPartitionV0 {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct StopReplicaRequestTopic {
    #[doc = "The topic name."]
    #[kafka(added = 1i16)]
    pub name: String,
    #[doc = "The partition indexes."]
    #[kafka(added = 1i16)]
    pub partition_indexes: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct StopReplicaResponse {
    #[doc = "The top-level error code, or 0 if there was no top-level error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The responses for each partition."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<StopReplicaResponsePartition>,
}
impl crate::KafkaResponseBody for StopReplicaResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        5i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct StopReplicaResponsePartition {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The partition error code, or 0 if there was no partition error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreateDelegationTokenRequest {
    #[doc = "A list of those who are allowed to renew this token before it expires."]
    #[kafka(added = 0i16)]
    pub renewers: Vec<CreatableRenewers>,
    #[doc = "The maximum lifetime of the token in milliseconds, or -1 to use the server side default."]
    #[kafka(added = 0i16)]
    pub max_lifetime_ms: i64,
}
impl crate::KafkaRequestBody for CreateDelegationTokenRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        38i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableRenewers {
    #[doc = "The type of the Kafka principal."]
    #[kafka(added = 0i16)]
    pub principal_type: String,
    #[doc = "The name of the Kafka principal."]
    #[kafka(added = 0i16)]
    pub principal_name: String,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreateDelegationTokenResponse {
    #[doc = "The top-level error, or zero if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The principal type of the token owner."]
    #[kafka(added = 0i16)]
    pub principal_type: String,
    #[doc = "The name of the token owner."]
    #[kafka(added = 0i16)]
    pub principal_name: String,
    #[doc = "When this token was generated."]
    #[kafka(added = 0i16)]
    pub issue_timestamp_ms: i64,
    #[doc = "When this token expires."]
    #[kafka(added = 0i16)]
    pub expiry_timestamp_ms: i64,
    #[doc = "The maximum lifetime of this token."]
    #[kafka(added = 0i16)]
    pub max_timestamp_ms: i64,
    #[doc = "The token UUID."]
    #[kafka(added = 0i16)]
    pub token_id: String,
    #[doc = "HMAC of the delegation token."]
    #[kafka(added = 0i16)]
    pub hmac: Vec<u8>,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for CreateDelegationTokenResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        38i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct HeartbeatRequest {
    #[doc = "The group id."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The generation of the group."]
    #[kafka(added = 0i16)]
    pub generationid: i32,
    #[doc = "The member ID."]
    #[kafka(added = 0i16)]
    pub member_id: String,
}
impl crate::KafkaRequestBody for HeartbeatRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        12i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct HeartbeatResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for HeartbeatResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        12i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AddPartitionsToTxnRequest {
    #[doc = "The transactional id corresponding to the transaction."]
    #[kafka(added = 0i16)]
    pub transactional_id: String,
    #[doc = "Current producer id in use by the transactional id."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "Current epoch associated with the producer id."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
    #[doc = "The partitions to add to the transation."]
    #[kafka(added = 0i16)]
    pub topics: Vec<AddPartitionsToTxnTopic>,
}
impl crate::KafkaRequestBody for AddPartitionsToTxnRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        24i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AddPartitionsToTxnTopic {
    #[doc = "The name of the topic."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The partition indexes to add to the transaction"]
    #[kafka(added = 0i16)]
    pub partitions: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct AddPartitionsToTxnResponse {
    #[doc = "Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The results for each topic."]
    #[kafka(added = 0i16)]
    pub results: Vec<AddPartitionsToTxnTopicResult>,
}
impl crate::KafkaResponseBody for AddPartitionsToTxnResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        24i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AddPartitionsToTxnPartitionResult {
    #[doc = "The partition indexes."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The response error code."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AddPartitionsToTxnTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The results for each partition"]
    #[kafka(added = 0i16)]
    pub results: Vec<AddPartitionsToTxnPartitionResult>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeAclsRequest {
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub resource_type: i8,
    #[doc = "The resource name, or null to match any resource name."]
    #[kafka(added = 0i16)]
    pub resource_name_filter: Option<String>,
    #[doc = "The resource pattern to match."]
    #[kafka(added = 1i16, default = "3")]
    pub resource_pattern_type: i8,
    #[doc = "The principal to match, or null to match any principal."]
    #[kafka(added = 0i16)]
    pub principal_filter: Option<String>,
    #[doc = "The host to match, or null to match any host."]
    #[kafka(added = 0i16)]
    pub host_filter: Option<String>,
    #[doc = "The operation to match."]
    #[kafka(added = 0i16)]
    pub operation: i8,
    #[doc = "The permission type to match."]
    #[kafka(added = 0i16)]
    pub permission_type: i8,
}
impl crate::KafkaRequestBody for DescribeAclsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        29i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeAclsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
    #[doc = "Each Resource that is referenced in an ACL."]
    #[kafka(added = 0i16)]
    pub resources: Vec<DescribeAclsResource>,
}
impl crate::KafkaResponseBody for DescribeAclsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        29i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AclDescription {
    #[doc = "The ACL principal."]
    #[kafka(added = 0i16)]
    pub principal: String,
    #[doc = "The ACL host."]
    #[kafka(added = 0i16)]
    pub host: String,
    #[doc = "The ACL operation."]
    #[kafka(added = 0i16)]
    pub operation: i8,
    #[doc = "The ACL permission type."]
    #[kafka(added = 0i16)]
    pub permission_type: i8,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeAclsResource {
    #[doc = "The resource type."]
    #[kafka(added = 0i16)]
    pub _type: i8,
    #[doc = "The resource name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The resource pattern type."]
    #[kafka(added = 1i16, default = "3")]
    pub pattern_type: i8,
    #[doc = "The ACLs."]
    #[kafka(added = 0i16)]
    pub acls: Vec<AclDescription>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct RenewDelegationTokenRequest {
    #[doc = "The HMAC of the delegation token to be renewed."]
    #[kafka(added = 0i16)]
    pub hmac: Vec<u8>,
    #[doc = "The renewal time period in milliseconds."]
    #[kafka(added = 0i16)]
    pub renew_period_ms: i64,
}
impl crate::KafkaRequestBody for RenewDelegationTokenRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        39i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct RenewDelegationTokenResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The timestamp in milliseconds at which this token expires."]
    #[kafka(added = 0i16)]
    pub expiry_timestamp_ms: i64,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for RenewDelegationTokenResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        39i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreatePartitionsRequest {
    #[doc = "Each topic that we want to create new partitions inside."]
    #[kafka(added = 0i16)]
    pub topics: Vec<CreatePartitionsTopic>,
    #[doc = "The time in ms to wait for the partitions to be created."]
    #[kafka(added = 0i16)]
    pub timeout_ms: i32,
    #[doc = "If true, then validate the request, but don't actually increase the number of partitions."]
    #[kafka(added = 0i16)]
    pub validate_only: bool,
}
impl crate::KafkaRequestBody for CreatePartitionsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        37i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatePartitionsAssignment {
    #[doc = "The assigned broker IDs."]
    #[kafka(added = 0i16)]
    pub broker_ids: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatePartitionsTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The new partition count."]
    #[kafka(added = 0i16)]
    pub count: i32,
    #[doc = "The new partition assignments."]
    #[kafka(added = 0i16)]
    pub assignments: Option<Vec<CreatePartitionsAssignment>>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct CreatePartitionsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The partition creation results for each topic."]
    #[kafka(added = 0i16)]
    pub results: Vec<CreatePartitionsTopicResult>,
}
impl crate::KafkaResponseBody for CreatePartitionsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        37i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatePartitionsTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The result error, or zero if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The result message, or null if there was no error."]
    #[kafka(added = 0i16)]
    pub error_message: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct LeaderAndIsrRequest {
    #[doc = "The current controller ID."]
    #[kafka(added = 0i16)]
    pub controller_id: i32,
    #[doc = "The current controller epoch."]
    #[kafka(added = 0i16)]
    pub controller_epoch: i32,
    #[doc = "The current broker epoch."]
    #[kafka(added = 2i16, default = "-1")]
    pub broker_epoch: i64,
    #[doc = "Each topic."]
    #[kafka(added = 2i16)]
    pub topic_states: Vec<LeaderAndIsrRequestTopicState>,
    #[doc = "The state of each partition"]
    #[kafka(added = 0i16, removed = 1i16)]
    pub partition_states_v0: Vec<LeaderAndIsrRequestPartitionStateV0>,
    #[doc = "The current live leaders."]
    #[kafka(added = 0i16)]
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
}
impl crate::KafkaRequestBody for LeaderAndIsrRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        4i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct LeaderAndIsrRequestPartitionState {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 0i16)]
    pub controller_epoch: i32,
    #[doc = "The broker ID of the leader."]
    #[kafka(added = 0i16)]
    pub leader_key: i32,
    #[doc = "The leader epoch."]
    #[kafka(added = 0i16)]
    pub leader_epoch: i32,
    #[doc = "The in-sync replica IDs."]
    #[kafka(added = 0i16)]
    pub isr_replicas: Vec<i32>,
    #[doc = "The ZooKeeper version."]
    #[kafka(added = 0i16)]
    pub zk_version: i32,
    #[doc = "The replica IDs."]
    #[kafka(added = 0i16)]
    pub replicas: Vec<i32>,
    #[doc = "Whether the replica should have existed on the broker or not."]
    #[kafka(added = 1i16, default = "false")]
    pub is_new: bool,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct LeaderAndIsrRequestTopicState {
    #[doc = "The topic name."]
    #[kafka(added = 2i16)]
    pub name: String,
    #[doc = "The state of each partition"]
    #[kafka(added = 0i16)]
    pub partition_states: Vec<LeaderAndIsrRequestPartitionState>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct LeaderAndIsrRequestPartitionStateV0 {
    #[doc = "The topic name."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub topic_name: String,
    #[doc = "The partition index."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub partition_index: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub controller_epoch: i32,
    #[doc = "The broker ID of the leader."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub leader_key: i32,
    #[doc = "The leader epoch."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub leader_epoch: i32,
    #[doc = "The in-sync replica IDs."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub isr_replicas: Vec<i32>,
    #[doc = "The ZooKeeper version."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub zk_version: i32,
    #[doc = "The replica IDs."]
    #[kafka(added = 0i16, removed = 1i16)]
    pub replicas: Vec<i32>,
    #[doc = "Whether the replica should have existed on the broker or not."]
    #[kafka(added = 1i16, default = "false")]
    pub is_new: bool,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct LeaderAndIsrLiveLeader {
    #[doc = "The leader's broker ID."]
    #[kafka(added = 0i16)]
    pub broker_id: i32,
    #[doc = "The leader's hostname."]
    #[kafka(added = 0i16)]
    pub host_name: String,
    #[doc = "The leader's port."]
    #[kafka(added = 0i16)]
    pub port: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct LeaderAndIsrResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "Each partition."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<LeaderAndIsrResponsePartition>,
}
impl crate::KafkaResponseBody for LeaderAndIsrResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        4i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct LeaderAndIsrResponsePartition {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The partition error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct ExpireDelegationTokenRequest {
    #[doc = "The HMAC of the delegation token to be expired."]
    #[kafka(added = 0i16)]
    pub hmac: Vec<u8>,
    #[doc = "The expiry time period in milliseconds."]
    #[kafka(added = 0i16)]
    pub expiry_time_period_ms: i64,
}
impl crate::KafkaRequestBody for ExpireDelegationTokenRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        40i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct ExpireDelegationTokenResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The timestamp in milliseconds at which this token expires."]
    #[kafka(added = 0i16)]
    pub expiry_timestamp_ms: i64,
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
}
impl crate::KafkaResponseBody for ExpireDelegationTokenResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        40i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct SaslHandshakeRequest {
    #[doc = "The SASL mechanism chosen by the client."]
    #[kafka(added = 0i16)]
    pub mechanism: String,
}
impl crate::KafkaRequestBody for SaslHandshakeRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        17i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct SaslHandshakeResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The mechanisms enabled in the server."]
    #[kafka(added = 0i16)]
    pub mechanisms: Vec<String>,
}
impl crate::KafkaResponseBody for SaslHandshakeResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        17i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct TxnOffsetCommitRequest {
    #[doc = "The ID of the transaction."]
    #[kafka(added = 0i16)]
    pub transactional_id: String,
    #[doc = "The ID of the group."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The current producer ID in use by the transactional ID."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "The current epoch associated with the producer ID."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
    #[doc = "Each topic that we want to committ offsets for."]
    #[kafka(added = 0i16)]
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
}
impl crate::KafkaRequestBody for TxnOffsetCommitRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        28i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TxnOffsetCommitRequestPartition {
    #[doc = "The index of the partition within the topic."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The message offset to be committed."]
    #[kafka(added = 0i16)]
    pub committed_offset: i64,
    #[doc = "The leader epoch of the last consumed record."]
    #[kafka(added = 2i16, default = "-1")]
    pub committed_leader_epoch: i32,
    #[doc = "Any associated metadata the client wants to keep."]
    #[kafka(added = 0i16)]
    pub committed_metadata: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TxnOffsetCommitRequestTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The partitions inside the topic that we want to committ offsets for."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct TxnOffsetCommitResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The responses for each topic."]
    #[kafka(added = 0i16)]
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}
impl crate::KafkaResponseBody for TxnOffsetCommitResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        28i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TxnOffsetCommitResponsePartition {
    #[doc = "The partitition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct TxnOffsetCommitResponseTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The responses for each partition in the topic."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 4i16)]
pub struct JoinGroupRequest {
    #[doc = "The group identifier."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds."]
    #[kafka(added = 0i16)]
    pub session_timeout_ms: i32,
    #[doc = "The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group."]
    #[kafka(added = 1i16, default = "-1")]
    pub rebalance_timeout_ms: i32,
    #[doc = "The member id assigned by the group coordinator."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[doc = "The unique name the for class of protocols implemented by the group we want to join."]
    #[kafka(added = 0i16)]
    pub protocol_type: String,
    #[doc = "The list of protocols that the member supports."]
    #[kafka(added = 0i16)]
    pub protocols: Vec<JoinGroupRequestProtocol>,
}
impl crate::KafkaRequestBody for JoinGroupRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        11i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct JoinGroupRequestProtocol {
    #[doc = "The protocol name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The protocol metadata."]
    #[kafka(added = 0i16)]
    pub metadata: Vec<u8>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 4i16)]
pub struct JoinGroupResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 2i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The generation ID of the group."]
    #[kafka(added = 0i16)]
    pub generation_id: i32,
    #[doc = "The group protocol selected by the coordinator."]
    #[kafka(added = 0i16)]
    pub protocol_name: String,
    #[doc = "The leader of the group."]
    #[kafka(added = 0i16)]
    pub leader: String,
    #[doc = "The member ID assigned by the group coordinator."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[kafka(added = 0i16)]
    pub members: Vec<JoinGroupResponseMember>,
}
impl crate::KafkaResponseBody for JoinGroupResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        11i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct JoinGroupResponseMember {
    #[doc = "The group member ID."]
    #[kafka(added = 0i16)]
    pub member_id: String,
    #[doc = "The group member metadata."]
    #[kafka(added = 0i16)]
    pub metadata: Vec<u8>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 3i16)]
pub struct CreateTopicsRequest {
    #[doc = "The topics to create."]
    #[kafka(added = 0i16)]
    pub topics: Vec<CreatableTopic>,
    #[doc = "How long to wait in milliseconds before timing out the request."]
    #[kafka(added = 0i16)]
    pub timeout_ms: i32,
    #[doc = "If true, check that the topics can be created as specified, but don't create anything."]
    #[kafka(added = 1i16, default = "false")]
    pub validate_only: bool,
}
impl crate::KafkaRequestBody for CreateTopicsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        19i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableReplicaAssignment {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The brokers to place the partition on."]
    #[kafka(added = 0i16)]
    pub broker_ids: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreateableTopicConfig {
    #[doc = "The configuration name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The configuration value."]
    #[kafka(added = 0i16)]
    pub value: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The number of partitions to create in the topic, or -1 if we are specifying a manual partition assignment."]
    #[kafka(added = 0i16)]
    pub num_partitions: i32,
    #[doc = "The number of replicas to create for each partition in the topic, or -1 if we are specifying a manual partition assignment."]
    #[kafka(added = 0i16)]
    pub replication_factor: i16,
    #[doc = "The manual partition assignment, or the empty array if we are using automatic assignment."]
    #[kafka(added = 0i16)]
    pub assignments: Vec<CreatableReplicaAssignment>,
    #[doc = "The custom topic configurations to set."]
    #[kafka(added = 0i16)]
    pub configs: Vec<CreateableTopicConfig>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 3i16)]
pub struct CreateTopicsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 2i16)]
    pub throttle_time_ms: i32,
    #[doc = "Results for each topic we tried to create."]
    #[kafka(added = 0i16)]
    pub topics: Vec<CreatableTopicResult>,
}
impl crate::KafkaResponseBody for CreateTopicsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        19i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct CreatableTopicResult {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The error message, or null if there was no error."]
    #[kafka(added = 1i16)]
    pub error_message: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct UpdateMetadataRequest {
    #[doc = "The controller id."]
    #[kafka(added = 0i16)]
    pub controller_id: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 0i16)]
    pub controller_epoch: i32,
    #[doc = "The broker epoch."]
    #[kafka(added = 5i16, default = "-1")]
    pub broker_epoch: i64,
    #[doc = "Each topic that we would like to update."]
    #[kafka(added = 5i16)]
    pub topic_states: Vec<UpdateMetadataRequestTopicState>,
    #[doc = "Each partition that we would like to update."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub partition_states_v0: Vec<UpdateMetadataRequestPartitionStateV0>,
    #[kafka(added = 0i16)]
    pub brokers: Vec<UpdateMetadataRequestBroker>,
}
impl crate::KafkaRequestBody for UpdateMetadataRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        6i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct UpdateMetadataPartitionState {
    #[doc = "The partition index."]
    #[kafka(added = 5i16)]
    pub partition_index: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 5i16)]
    pub controller_epoch: i32,
    #[doc = "The ID of the broker which is the current partition leader."]
    #[kafka(added = 5i16)]
    pub leader: i32,
    #[doc = "The leader epoch of this partition."]
    #[kafka(added = 5i16)]
    pub leader_epoch: i32,
    #[doc = "The brokers which are in the ISR for this partition."]
    #[kafka(added = 5i16)]
    pub isr: Vec<i32>,
    #[doc = "The Zookeeper version."]
    #[kafka(added = 5i16)]
    pub zk_version: i32,
    #[doc = "All the replicas of this partition."]
    #[kafka(added = 5i16)]
    pub replicas: Vec<i32>,
    #[doc = "The replicas of this partition which are offline."]
    #[kafka(added = 5i16)]
    pub offline_replicas: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct UpdateMetadataRequestTopicState {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub topic_name: String,
    #[doc = "The partition that we would like to update."]
    #[kafka(added = 5i16)]
    pub partition_states: Vec<UpdateMetadataPartitionState>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct UpdateMetadataRequestPartitionStateV0 {
    #[doc = "The topic name."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub topic_name: String,
    #[doc = "The partition index."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub partition_index: i32,
    #[doc = "The controller epoch."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub controller_epoch: i32,
    #[doc = "The ID of the broker which is the current partition leader."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub leader: i32,
    #[doc = "The leader epoch of this partition."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub leader_epoch: i32,
    #[doc = "The brokers which are in the ISR for this partition."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub isr: Vec<i32>,
    #[doc = "The Zookeeper version."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub zk_version: i32,
    #[doc = "All the replicas of this partition."]
    #[kafka(added = 0i16, removed = 4i16)]
    pub replicas: Vec<i32>,
    #[doc = "The replicas of this partition which are offline."]
    #[kafka(added = 4i16)]
    pub offline_replicas: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct UpdateMetadataRequestEndpoint {
    #[doc = "The port of this endpoint"]
    #[kafka(added = 1i16)]
    pub port: i32,
    #[doc = "The hostname of this endpoint"]
    #[kafka(added = 1i16)]
    pub host: String,
    #[doc = "The listener name."]
    #[kafka(added = 3i16)]
    pub listener: String,
    #[doc = "The security protocol type."]
    #[kafka(added = 1i16)]
    pub security_protocol: i16,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct UpdateMetadataRequestBroker {
    #[kafka(added = 0i16)]
    pub id: i32,
    #[doc = "The broker hostname."]
    #[kafka(added = 0i16)]
    pub v0_host: String,
    #[doc = "The broker port."]
    #[kafka(added = 0i16)]
    pub v0_port: i32,
    #[doc = "The broker endpoints."]
    #[kafka(added = 1i16)]
    pub endpoints: Vec<UpdateMetadataRequestEndpoint>,
    #[doc = "The rack which this broker belongs to."]
    #[kafka(added = 2i16)]
    pub rack: Option<String>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 5i16)]
pub struct UpdateMetadataResponse {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for UpdateMetadataResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        6i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct LeaveGroupRequest {
    #[doc = "The ID of the group to leave."]
    #[kafka(added = 0i16)]
    pub group_id: String,
    #[doc = "The member ID to remove from the group."]
    #[kafka(added = 0i16)]
    pub member_id: String,
}
impl crate::KafkaRequestBody for LeaveGroupRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        13i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 2i16)]
pub struct LeaveGroupResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for LeaveGroupResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        13i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeLogDirsRequest {
    #[doc = "Each topic that we want to describe log directories for, or null for all topics."]
    #[kafka(added = 0i16)]
    pub topics: Option<Vec<DescribableLogDirTopic>>,
}
impl crate::KafkaRequestBody for DescribeLogDirsRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        35i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribableLogDirTopic {
    #[doc = "The topic name"]
    #[kafka(added = 0i16)]
    pub topic: String,
    #[doc = "The partition indxes."]
    #[kafka(added = 0i16)]
    pub partition_index: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct DescribeLogDirsResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The log directories."]
    #[kafka(added = 0i16)]
    pub results: Vec<DescribeLogDirsResult>,
}
impl crate::KafkaResponseBody for DescribeLogDirsResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        35i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeLogDirsPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The size of the log segments in this partition in bytes."]
    #[kafka(added = 0i16)]
    pub partition_size: i64,
    #[doc = "The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)"]
    #[kafka(added = 0i16)]
    pub offset_lag: i64,
    #[doc = "True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future."]
    #[kafka(added = 0i16)]
    pub is_future_key: bool,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeLogDirsTopic {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[kafka(added = 0i16)]
    pub partitions: Vec<DescribeLogDirsPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct DescribeLogDirsResult {
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The absolute log directory path."]
    #[kafka(added = 0i16)]
    pub log_dir: String,
    #[doc = "Each topic."]
    #[kafka(added = 0i16)]
    pub topics: Vec<DescribeLogDirsTopic>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct EndTxnRequest {
    #[doc = "The ID of the transaction to end."]
    #[kafka(added = 0i16)]
    pub transactional_id: String,
    #[doc = "The producer ID."]
    #[kafka(added = 0i16)]
    pub producer_id: i64,
    #[doc = "The current epoch associated with the producer."]
    #[kafka(added = 0i16)]
    pub producer_epoch: i16,
    #[doc = "True if the transaction was committed, false if it was aborted."]
    #[kafka(added = 0i16)]
    pub committed: bool,
}
impl crate::KafkaRequestBody for EndTxnRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        26i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 1i16)]
pub struct EndTxnResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 0i16)]
    pub throttle_time_ms: i32,
    #[doc = "The error code, or 0 if there was no error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
}
impl crate::KafkaResponseBody for EndTxnResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        26i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 10i16)]
pub struct FetchRequest {
    #[doc = "The broker ID of the follower, of -1 if this request is from a consumer."]
    #[kafka(added = 0i16)]
    pub replica_id: i32,
    #[doc = "The maximum time in milliseconds to wait for the response."]
    #[kafka(added = 0i16)]
    pub max_wait: i32,
    #[doc = "The minimum bytes to accumulate in the response."]
    #[kafka(added = 0i16)]
    pub min_bytes: i32,
    #[doc = "The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored."]
    #[kafka(added = 3i16, default = "0x7fffffff")]
    pub max_bytes: i32,
    #[doc = "This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records"]
    #[kafka(added = 4i16, default = "0")]
    pub isolation_level: i8,
    #[doc = "The fetch session ID."]
    #[kafka(added = 7i16, default = "0")]
    pub session_id: i32,
    #[doc = "The fetch session ID."]
    #[kafka(added = 7i16, default = "-1")]
    pub epoch: i32,
    #[doc = "The topics to fetch."]
    #[kafka(added = 0i16)]
    pub topics: Vec<FetchableTopic>,
    #[doc = "In an incremental fetch request, the partitions to remove."]
    #[kafka(added = 7i16)]
    pub forgotten: Vec<ForgottenTopic>,
}
impl crate::KafkaRequestBody for FetchRequest {
    fn api_key() -> crate::apikey::ApiKeys {
        1i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct FetchPartition {
    #[doc = "The partition index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The current leader epoch of the partition."]
    #[kafka(added = 9i16, default = "-1")]
    pub current_leader_epoch: i32,
    #[doc = "The message offset."]
    #[kafka(added = 0i16)]
    pub fetch_offset: i64,
    #[doc = "The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower."]
    #[kafka(added = 5i16, default = "-1")]
    pub log_start_offset: i64,
    #[doc = "The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored."]
    #[kafka(added = 0i16)]
    pub max_bytes: i32,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct FetchableTopic {
    #[doc = "The name of the topic to fetch."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The partitions to fetch."]
    #[kafka(added = 0i16)]
    pub fetch_partitions: Vec<FetchPartition>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct ForgottenTopic {
    #[doc = "The partition name."]
    #[kafka(added = 7i16)]
    pub name: String,
    #[doc = "The partitions indexes to forget."]
    #[kafka(added = 7i16)]
    pub forgotten_partition_indexes: Vec<i32>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
#[kafka(added = 0i16, removed = 10i16)]
pub struct FetchResponse {
    #[doc = "The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota."]
    #[kafka(added = 1i16)]
    pub throttle_time_ms: i32,
    #[doc = "The top level response error code."]
    #[kafka(added = 7i16)]
    pub error_code: i16,
    #[doc = "The fetch session ID, or 0 if this is not part of a fetch session."]
    #[kafka(added = 7i16, default = "0")]
    pub session_id: i32,
    #[doc = "The response topics."]
    #[kafka(added = 0i16)]
    pub topics: Vec<FetchableTopicResponse>,
}
impl crate::KafkaResponseBody for FetchResponse {
    fn api_key() -> crate::apikey::ApiKeys {
        1i16.into()
    }
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct AbortedTransaction {
    #[doc = "The producer id associated with the aborted transaction."]
    #[kafka(added = 4i16)]
    pub producer_id: i64,
    #[doc = "The first offset in the aborted transaction."]
    #[kafka(added = 4i16)]
    pub first_offset: i64,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct FetchablePartitionResponse {
    #[doc = "The partiiton index."]
    #[kafka(added = 0i16)]
    pub partition_index: i32,
    #[doc = "The error code, or 0 if there was no fetch error."]
    #[kafka(added = 0i16)]
    pub error_code: i16,
    #[doc = "The current high water mark."]
    #[kafka(added = 0i16)]
    pub high_watermark: i64,
    #[doc = "The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)"]
    #[kafka(added = 4i16, default = "-1")]
    pub last_stable_offset: i64,
    #[doc = "The current log start offset."]
    #[kafka(added = 5i16, default = "-1")]
    pub log_start_offset: i64,
    #[doc = "The aborted transactions."]
    #[kafka(added = 4i16)]
    pub aborted: Option<Vec<AbortedTransaction>>,
    #[doc = "The record data."]
    #[kafka(added = 0i16)]
    pub records: Option<Vec<u8>>,
}
#[derive(Debug, PartialEq, KafkaRpc, Clone)]
pub struct FetchableTopicResponse {
    #[doc = "The topic name."]
    #[kafka(added = 0i16)]
    pub name: String,
    #[doc = "The topic partitions."]
    #[kafka(added = 0i16)]
    pub partitions: Vec<FetchablePartitionResponse>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum RequestBody {
    AlterReplicaLogDirsRequest(AlterReplicaLogDirsRequest),
    ElectPreferredLeadersRequest(ElectPreferredLeadersRequest),
    InitProducerIdRequest(InitProducerIdRequest),
    DeleteTopicsRequest(DeleteTopicsRequest),
    DeleteAclsRequest(DeleteAclsRequest),
    DescribeConfigsRequest(DescribeConfigsRequest),
    DeleteRecordsRequest(DeleteRecordsRequest),
    DeleteGroupsRequest(DeleteGroupsRequest),
    FindCoordinatorRequest(FindCoordinatorRequest),
    AddOffsetsToTxnRequest(AddOffsetsToTxnRequest),
    SaslAuthenticateRequest(SaslAuthenticateRequest),
    ListOffsetRequest(ListOffsetRequest),
    DescribeGroupsRequest(DescribeGroupsRequest),
    OffsetCommitRequest(OffsetCommitRequest),
    ApiVersionsRequest(ApiVersionsRequest),
    WriteTxnMarkersRequest(WriteTxnMarkersRequest),
    DescribeDelegationTokenRequest(DescribeDelegationTokenRequest),
    ProduceRequest(ProduceRequest),
    ListGroupsRequest(ListGroupsRequest),
    OffsetForLeaderEpochRequest(OffsetForLeaderEpochRequest),
    MetadataRequest(MetadataRequest),
    ControlledShutdownRequest(ControlledShutdownRequest),
    SyncGroupRequest(SyncGroupRequest),
    AlterConfigsRequest(AlterConfigsRequest),
    CreateAclsRequest(CreateAclsRequest),
    OffsetFetchRequest(OffsetFetchRequest),
    StopReplicaRequest(StopReplicaRequest),
    CreateDelegationTokenRequest(CreateDelegationTokenRequest),
    HeartbeatRequest(HeartbeatRequest),
    AddPartitionsToTxnRequest(AddPartitionsToTxnRequest),
    DescribeAclsRequest(DescribeAclsRequest),
    RenewDelegationTokenRequest(RenewDelegationTokenRequest),
    CreatePartitionsRequest(CreatePartitionsRequest),
    LeaderAndIsrRequest(LeaderAndIsrRequest),
    ExpireDelegationTokenRequest(ExpireDelegationTokenRequest),
    SaslHandshakeRequest(SaslHandshakeRequest),
    TxnOffsetCommitRequest(TxnOffsetCommitRequest),
    JoinGroupRequest(JoinGroupRequest),
    CreateTopicsRequest(CreateTopicsRequest),
    UpdateMetadataRequest(UpdateMetadataRequest),
    LeaveGroupRequest(LeaveGroupRequest),
    DescribeLogDirsRequest(DescribeLogDirsRequest),
    EndTxnRequest(EndTxnRequest),
    FetchRequest(FetchRequest),
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum ResponseBody {
    AlterReplicaLogDirsResponse(AlterReplicaLogDirsResponse),
    ElectPreferredLeadersResponse(ElectPreferredLeadersResponse),
    InitProducerIdResponse(InitProducerIdResponse),
    DeleteTopicsResponse(DeleteTopicsResponse),
    DeleteAclsResponse(DeleteAclsResponse),
    DescribeConfigsResponse(DescribeConfigsResponse),
    DeleteRecordsResponse(DeleteRecordsResponse),
    DeleteGroupsResponse(DeleteGroupsResponse),
    FindCoordinatorResponse(FindCoordinatorResponse),
    AddOffsetsToTxnResponse(AddOffsetsToTxnResponse),
    SaslAuthenticateResponse(SaslAuthenticateResponse),
    ListOffsetResponse(ListOffsetResponse),
    DescribeGroupsResponse(DescribeGroupsResponse),
    OffsetCommitResponse(OffsetCommitResponse),
    ApiVersionsResponse(ApiVersionsResponse),
    WriteTxnMarkersResponse(WriteTxnMarkersResponse),
    DescribeDelegationTokenResponse(DescribeDelegationTokenResponse),
    ProduceResponse(ProduceResponse),
    ListGroupsResponse(ListGroupsResponse),
    OffsetForLeaderEpochResponse(OffsetForLeaderEpochResponse),
    MetadataResponse(MetadataResponse),
    ControlledShutdownResponse(ControlledShutdownResponse),
    SyncGroupResponse(SyncGroupResponse),
    AlterConfigsResponse(AlterConfigsResponse),
    CreateAclsResponse(CreateAclsResponse),
    OffsetFetchResponse(OffsetFetchResponse),
    StopReplicaResponse(StopReplicaResponse),
    CreateDelegationTokenResponse(CreateDelegationTokenResponse),
    HeartbeatResponse(HeartbeatResponse),
    AddPartitionsToTxnResponse(AddPartitionsToTxnResponse),
    DescribeAclsResponse(DescribeAclsResponse),
    RenewDelegationTokenResponse(RenewDelegationTokenResponse),
    CreatePartitionsResponse(CreatePartitionsResponse),
    LeaderAndIsrResponse(LeaderAndIsrResponse),
    ExpireDelegationTokenResponse(ExpireDelegationTokenResponse),
    SaslHandshakeResponse(SaslHandshakeResponse),
    TxnOffsetCommitResponse(TxnOffsetCommitResponse),
    JoinGroupResponse(JoinGroupResponse),
    CreateTopicsResponse(CreateTopicsResponse),
    UpdateMetadataResponse(UpdateMetadataResponse),
    LeaveGroupResponse(LeaveGroupResponse),
    DescribeLogDirsResponse(DescribeLogDirsResponse),
    EndTxnResponse(EndTxnResponse),
    FetchResponse(FetchResponse),
}
impl RequestBody {
    pub fn read<R: ::std::io::Read>(
        ctx: &mut kafka_protocol::DeserializeCtx<R>,
        api_key: i16,
    ) -> Result<Self, kafka_protocol::CodecError> {
        let api_key: crate::apikey::ApiKeys = crate::apikey::ApiKeys::from(api_key);
        match api_key {
            crate::apikey::ApiKeys::AlterReplicaLogDirs => {
                crate::util::version_check_read::<AlterReplicaLogDirsRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::ElectPreferredLeaders => {
                crate::util::version_check_read::<ElectPreferredLeadersRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::InitProducerId => {
                crate::util::version_check_read::<InitProducerIdRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteTopics => {
                crate::util::version_check_read::<DeleteTopicsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteAcls => {
                crate::util::version_check_read::<DeleteAclsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeConfigs => {
                crate::util::version_check_read::<DescribeConfigsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteRecords => {
                crate::util::version_check_read::<DeleteRecordsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteGroups => {
                crate::util::version_check_read::<DeleteGroupsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::FindCoordinator => {
                crate::util::version_check_read::<FindCoordinatorRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AddOffsetsToTxn => {
                crate::util::version_check_read::<AddOffsetsToTxnRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::SaslAuthenticate => {
                crate::util::version_check_read::<SaslAuthenticateRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ListOffset => {
                crate::util::version_check_read::<ListOffsetRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeGroups => {
                crate::util::version_check_read::<DescribeGroupsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetCommit => {
                crate::util::version_check_read::<OffsetCommitRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ApiVersions => {
                crate::util::version_check_read::<ApiVersionsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::WriteTxnMarkers => {
                crate::util::version_check_read::<WriteTxnMarkersRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeDelegationToken => {
                crate::util::version_check_read::<DescribeDelegationTokenRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Produce => {
                crate::util::version_check_read::<ProduceRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ListGroups => {
                crate::util::version_check_read::<ListGroupsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetForLeaderEpoch => {
                crate::util::version_check_read::<OffsetForLeaderEpochRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Metadata => {
                crate::util::version_check_read::<MetadataRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ControlledShutdown => {
                crate::util::version_check_read::<ControlledShutdownRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::SyncGroup => {
                crate::util::version_check_read::<SyncGroupRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AlterConfigs => {
                crate::util::version_check_read::<AlterConfigsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateAcls => {
                crate::util::version_check_read::<CreateAclsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetFetch => {
                crate::util::version_check_read::<OffsetFetchRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::StopReplica => {
                crate::util::version_check_read::<StopReplicaRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateDelegationToken => {
                crate::util::version_check_read::<CreateDelegationTokenRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Heartbeat => {
                crate::util::version_check_read::<HeartbeatRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AddPartitionsToTxn => {
                crate::util::version_check_read::<AddPartitionsToTxnRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeAcls => {
                crate::util::version_check_read::<DescribeAclsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::RenewDelegationToken => {
                crate::util::version_check_read::<RenewDelegationTokenRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::CreatePartitions => {
                crate::util::version_check_read::<CreatePartitionsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::LeaderAndIsr => {
                crate::util::version_check_read::<LeaderAndIsrRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ExpireDelegationToken => {
                crate::util::version_check_read::<ExpireDelegationTokenRequest, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::SaslHandshake => {
                crate::util::version_check_read::<SaslHandshakeRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::TxnOffsetCommit => {
                crate::util::version_check_read::<TxnOffsetCommitRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::JoinGroup => {
                crate::util::version_check_read::<JoinGroupRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateTopics => {
                crate::util::version_check_read::<CreateTopicsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::UpdateMetadata => {
                crate::util::version_check_read::<UpdateMetadataRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::LeaveGroup => {
                crate::util::version_check_read::<LeaveGroupRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeLogDirs => {
                crate::util::version_check_read::<DescribeLogDirsRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::EndTxn => {
                crate::util::version_check_read::<EndTxnRequest, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::Fetch => {
                crate::util::version_check_read::<FetchRequest, R>(ctx).map(Into::into)
            }
        }
    }
    pub fn size(&self, version: i16) -> usize {
        match self {
            RequestBody::AlterReplicaLogDirsRequest(inner) => inner.size(version),
            RequestBody::ElectPreferredLeadersRequest(inner) => inner.size(version),
            RequestBody::InitProducerIdRequest(inner) => inner.size(version),
            RequestBody::DeleteTopicsRequest(inner) => inner.size(version),
            RequestBody::DeleteAclsRequest(inner) => inner.size(version),
            RequestBody::DescribeConfigsRequest(inner) => inner.size(version),
            RequestBody::DeleteRecordsRequest(inner) => inner.size(version),
            RequestBody::DeleteGroupsRequest(inner) => inner.size(version),
            RequestBody::FindCoordinatorRequest(inner) => inner.size(version),
            RequestBody::AddOffsetsToTxnRequest(inner) => inner.size(version),
            RequestBody::SaslAuthenticateRequest(inner) => inner.size(version),
            RequestBody::ListOffsetRequest(inner) => inner.size(version),
            RequestBody::DescribeGroupsRequest(inner) => inner.size(version),
            RequestBody::OffsetCommitRequest(inner) => inner.size(version),
            RequestBody::ApiVersionsRequest(inner) => inner.size(version),
            RequestBody::WriteTxnMarkersRequest(inner) => inner.size(version),
            RequestBody::DescribeDelegationTokenRequest(inner) => inner.size(version),
            RequestBody::ProduceRequest(inner) => inner.size(version),
            RequestBody::ListGroupsRequest(inner) => inner.size(version),
            RequestBody::OffsetForLeaderEpochRequest(inner) => inner.size(version),
            RequestBody::MetadataRequest(inner) => inner.size(version),
            RequestBody::ControlledShutdownRequest(inner) => inner.size(version),
            RequestBody::SyncGroupRequest(inner) => inner.size(version),
            RequestBody::AlterConfigsRequest(inner) => inner.size(version),
            RequestBody::CreateAclsRequest(inner) => inner.size(version),
            RequestBody::OffsetFetchRequest(inner) => inner.size(version),
            RequestBody::StopReplicaRequest(inner) => inner.size(version),
            RequestBody::CreateDelegationTokenRequest(inner) => inner.size(version),
            RequestBody::HeartbeatRequest(inner) => inner.size(version),
            RequestBody::AddPartitionsToTxnRequest(inner) => inner.size(version),
            RequestBody::DescribeAclsRequest(inner) => inner.size(version),
            RequestBody::RenewDelegationTokenRequest(inner) => inner.size(version),
            RequestBody::CreatePartitionsRequest(inner) => inner.size(version),
            RequestBody::LeaderAndIsrRequest(inner) => inner.size(version),
            RequestBody::ExpireDelegationTokenRequest(inner) => inner.size(version),
            RequestBody::SaslHandshakeRequest(inner) => inner.size(version),
            RequestBody::TxnOffsetCommitRequest(inner) => inner.size(version),
            RequestBody::JoinGroupRequest(inner) => inner.size(version),
            RequestBody::CreateTopicsRequest(inner) => inner.size(version),
            RequestBody::UpdateMetadataRequest(inner) => inner.size(version),
            RequestBody::LeaveGroupRequest(inner) => inner.size(version),
            RequestBody::DescribeLogDirsRequest(inner) => inner.size(version),
            RequestBody::EndTxnRequest(inner) => inner.size(version),
            RequestBody::FetchRequest(inner) => inner.size(version),
        }
    }
    pub fn write<W: ::std::io::Write>(
        &self,
        ctx: &mut kafka_protocol::SerializeCtx<W>,
    ) -> Result<(), kafka_protocol::CodecError> {
        match self {
            RequestBody::AlterReplicaLogDirsRequest(inner) => inner.write(ctx),
            RequestBody::ElectPreferredLeadersRequest(inner) => inner.write(ctx),
            RequestBody::InitProducerIdRequest(inner) => inner.write(ctx),
            RequestBody::DeleteTopicsRequest(inner) => inner.write(ctx),
            RequestBody::DeleteAclsRequest(inner) => inner.write(ctx),
            RequestBody::DescribeConfigsRequest(inner) => inner.write(ctx),
            RequestBody::DeleteRecordsRequest(inner) => inner.write(ctx),
            RequestBody::DeleteGroupsRequest(inner) => inner.write(ctx),
            RequestBody::FindCoordinatorRequest(inner) => inner.write(ctx),
            RequestBody::AddOffsetsToTxnRequest(inner) => inner.write(ctx),
            RequestBody::SaslAuthenticateRequest(inner) => inner.write(ctx),
            RequestBody::ListOffsetRequest(inner) => inner.write(ctx),
            RequestBody::DescribeGroupsRequest(inner) => inner.write(ctx),
            RequestBody::OffsetCommitRequest(inner) => inner.write(ctx),
            RequestBody::ApiVersionsRequest(inner) => inner.write(ctx),
            RequestBody::WriteTxnMarkersRequest(inner) => inner.write(ctx),
            RequestBody::DescribeDelegationTokenRequest(inner) => inner.write(ctx),
            RequestBody::ProduceRequest(inner) => inner.write(ctx),
            RequestBody::ListGroupsRequest(inner) => inner.write(ctx),
            RequestBody::OffsetForLeaderEpochRequest(inner) => inner.write(ctx),
            RequestBody::MetadataRequest(inner) => inner.write(ctx),
            RequestBody::ControlledShutdownRequest(inner) => inner.write(ctx),
            RequestBody::SyncGroupRequest(inner) => inner.write(ctx),
            RequestBody::AlterConfigsRequest(inner) => inner.write(ctx),
            RequestBody::CreateAclsRequest(inner) => inner.write(ctx),
            RequestBody::OffsetFetchRequest(inner) => inner.write(ctx),
            RequestBody::StopReplicaRequest(inner) => inner.write(ctx),
            RequestBody::CreateDelegationTokenRequest(inner) => inner.write(ctx),
            RequestBody::HeartbeatRequest(inner) => inner.write(ctx),
            RequestBody::AddPartitionsToTxnRequest(inner) => inner.write(ctx),
            RequestBody::DescribeAclsRequest(inner) => inner.write(ctx),
            RequestBody::RenewDelegationTokenRequest(inner) => inner.write(ctx),
            RequestBody::CreatePartitionsRequest(inner) => inner.write(ctx),
            RequestBody::LeaderAndIsrRequest(inner) => inner.write(ctx),
            RequestBody::ExpireDelegationTokenRequest(inner) => inner.write(ctx),
            RequestBody::SaslHandshakeRequest(inner) => inner.write(ctx),
            RequestBody::TxnOffsetCommitRequest(inner) => inner.write(ctx),
            RequestBody::JoinGroupRequest(inner) => inner.write(ctx),
            RequestBody::CreateTopicsRequest(inner) => inner.write(ctx),
            RequestBody::UpdateMetadataRequest(inner) => inner.write(ctx),
            RequestBody::LeaveGroupRequest(inner) => inner.write(ctx),
            RequestBody::DescribeLogDirsRequest(inner) => inner.write(ctx),
            RequestBody::EndTxnRequest(inner) => inner.write(ctx),
            RequestBody::FetchRequest(inner) => inner.write(ctx),
        }
    }
}
impl ResponseBody {
    pub fn read<R: ::std::io::Read>(
        ctx: &mut kafka_protocol::DeserializeCtx<R>,
        api_key: i16,
    ) -> Result<Self, kafka_protocol::CodecError> {
        let api_key: crate::apikey::ApiKeys = crate::apikey::ApiKeys::from(api_key);
        match api_key {
            crate::apikey::ApiKeys::AlterReplicaLogDirs => {
                crate::util::version_check_read::<AlterReplicaLogDirsResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::ElectPreferredLeaders => {
                crate::util::version_check_read::<ElectPreferredLeadersResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::InitProducerId => {
                crate::util::version_check_read::<InitProducerIdResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteTopics => {
                crate::util::version_check_read::<DeleteTopicsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteAcls => {
                crate::util::version_check_read::<DeleteAclsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeConfigs => {
                crate::util::version_check_read::<DescribeConfigsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteRecords => {
                crate::util::version_check_read::<DeleteRecordsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DeleteGroups => {
                crate::util::version_check_read::<DeleteGroupsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::FindCoordinator => {
                crate::util::version_check_read::<FindCoordinatorResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AddOffsetsToTxn => {
                crate::util::version_check_read::<AddOffsetsToTxnResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::SaslAuthenticate => {
                crate::util::version_check_read::<SaslAuthenticateResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ListOffset => {
                crate::util::version_check_read::<ListOffsetResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeGroups => {
                crate::util::version_check_read::<DescribeGroupsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetCommit => {
                crate::util::version_check_read::<OffsetCommitResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ApiVersions => {
                crate::util::version_check_read::<ApiVersionsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::WriteTxnMarkers => {
                crate::util::version_check_read::<WriteTxnMarkersResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeDelegationToken => {
                crate::util::version_check_read::<DescribeDelegationTokenResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Produce => {
                crate::util::version_check_read::<ProduceResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ListGroups => {
                crate::util::version_check_read::<ListGroupsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetForLeaderEpoch => {
                crate::util::version_check_read::<OffsetForLeaderEpochResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Metadata => {
                crate::util::version_check_read::<MetadataResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ControlledShutdown => {
                crate::util::version_check_read::<ControlledShutdownResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::SyncGroup => {
                crate::util::version_check_read::<SyncGroupResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AlterConfigs => {
                crate::util::version_check_read::<AlterConfigsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateAcls => {
                crate::util::version_check_read::<CreateAclsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::OffsetFetch => {
                crate::util::version_check_read::<OffsetFetchResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::StopReplica => {
                crate::util::version_check_read::<StopReplicaResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateDelegationToken => {
                crate::util::version_check_read::<CreateDelegationTokenResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::Heartbeat => {
                crate::util::version_check_read::<HeartbeatResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::AddPartitionsToTxn => {
                crate::util::version_check_read::<AddPartitionsToTxnResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeAcls => {
                crate::util::version_check_read::<DescribeAclsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::RenewDelegationToken => {
                crate::util::version_check_read::<RenewDelegationTokenResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::CreatePartitions => {
                crate::util::version_check_read::<CreatePartitionsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::LeaderAndIsr => {
                crate::util::version_check_read::<LeaderAndIsrResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::ExpireDelegationToken => {
                crate::util::version_check_read::<ExpireDelegationTokenResponse, R>(ctx)
                    .map(Into::into)
            }
            crate::apikey::ApiKeys::SaslHandshake => {
                crate::util::version_check_read::<SaslHandshakeResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::TxnOffsetCommit => {
                crate::util::version_check_read::<TxnOffsetCommitResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::JoinGroup => {
                crate::util::version_check_read::<JoinGroupResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::CreateTopics => {
                crate::util::version_check_read::<CreateTopicsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::UpdateMetadata => {
                crate::util::version_check_read::<UpdateMetadataResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::LeaveGroup => {
                crate::util::version_check_read::<LeaveGroupResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::DescribeLogDirs => {
                crate::util::version_check_read::<DescribeLogDirsResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::EndTxn => {
                crate::util::version_check_read::<EndTxnResponse, R>(ctx).map(Into::into)
            }
            crate::apikey::ApiKeys::Fetch => {
                crate::util::version_check_read::<FetchResponse, R>(ctx).map(Into::into)
            }
        }
    }
    pub fn size(&self, version: i16) -> usize {
        match self {
            ResponseBody::AlterReplicaLogDirsResponse(inner) => inner.size(version),
            ResponseBody::ElectPreferredLeadersResponse(inner) => inner.size(version),
            ResponseBody::InitProducerIdResponse(inner) => inner.size(version),
            ResponseBody::DeleteTopicsResponse(inner) => inner.size(version),
            ResponseBody::DeleteAclsResponse(inner) => inner.size(version),
            ResponseBody::DescribeConfigsResponse(inner) => inner.size(version),
            ResponseBody::DeleteRecordsResponse(inner) => inner.size(version),
            ResponseBody::DeleteGroupsResponse(inner) => inner.size(version),
            ResponseBody::FindCoordinatorResponse(inner) => inner.size(version),
            ResponseBody::AddOffsetsToTxnResponse(inner) => inner.size(version),
            ResponseBody::SaslAuthenticateResponse(inner) => inner.size(version),
            ResponseBody::ListOffsetResponse(inner) => inner.size(version),
            ResponseBody::DescribeGroupsResponse(inner) => inner.size(version),
            ResponseBody::OffsetCommitResponse(inner) => inner.size(version),
            ResponseBody::ApiVersionsResponse(inner) => inner.size(version),
            ResponseBody::WriteTxnMarkersResponse(inner) => inner.size(version),
            ResponseBody::DescribeDelegationTokenResponse(inner) => inner.size(version),
            ResponseBody::ProduceResponse(inner) => inner.size(version),
            ResponseBody::ListGroupsResponse(inner) => inner.size(version),
            ResponseBody::OffsetForLeaderEpochResponse(inner) => inner.size(version),
            ResponseBody::MetadataResponse(inner) => inner.size(version),
            ResponseBody::ControlledShutdownResponse(inner) => inner.size(version),
            ResponseBody::SyncGroupResponse(inner) => inner.size(version),
            ResponseBody::AlterConfigsResponse(inner) => inner.size(version),
            ResponseBody::CreateAclsResponse(inner) => inner.size(version),
            ResponseBody::OffsetFetchResponse(inner) => inner.size(version),
            ResponseBody::StopReplicaResponse(inner) => inner.size(version),
            ResponseBody::CreateDelegationTokenResponse(inner) => inner.size(version),
            ResponseBody::HeartbeatResponse(inner) => inner.size(version),
            ResponseBody::AddPartitionsToTxnResponse(inner) => inner.size(version),
            ResponseBody::DescribeAclsResponse(inner) => inner.size(version),
            ResponseBody::RenewDelegationTokenResponse(inner) => inner.size(version),
            ResponseBody::CreatePartitionsResponse(inner) => inner.size(version),
            ResponseBody::LeaderAndIsrResponse(inner) => inner.size(version),
            ResponseBody::ExpireDelegationTokenResponse(inner) => inner.size(version),
            ResponseBody::SaslHandshakeResponse(inner) => inner.size(version),
            ResponseBody::TxnOffsetCommitResponse(inner) => inner.size(version),
            ResponseBody::JoinGroupResponse(inner) => inner.size(version),
            ResponseBody::CreateTopicsResponse(inner) => inner.size(version),
            ResponseBody::UpdateMetadataResponse(inner) => inner.size(version),
            ResponseBody::LeaveGroupResponse(inner) => inner.size(version),
            ResponseBody::DescribeLogDirsResponse(inner) => inner.size(version),
            ResponseBody::EndTxnResponse(inner) => inner.size(version),
            ResponseBody::FetchResponse(inner) => inner.size(version),
        }
    }
    pub fn write<W: ::std::io::Write>(
        &self,
        ctx: &mut kafka_protocol::SerializeCtx<W>,
    ) -> Result<(), kafka_protocol::CodecError> {
        match self {
            ResponseBody::AlterReplicaLogDirsResponse(inner) => inner.write(ctx),
            ResponseBody::ElectPreferredLeadersResponse(inner) => inner.write(ctx),
            ResponseBody::InitProducerIdResponse(inner) => inner.write(ctx),
            ResponseBody::DeleteTopicsResponse(inner) => inner.write(ctx),
            ResponseBody::DeleteAclsResponse(inner) => inner.write(ctx),
            ResponseBody::DescribeConfigsResponse(inner) => inner.write(ctx),
            ResponseBody::DeleteRecordsResponse(inner) => inner.write(ctx),
            ResponseBody::DeleteGroupsResponse(inner) => inner.write(ctx),
            ResponseBody::FindCoordinatorResponse(inner) => inner.write(ctx),
            ResponseBody::AddOffsetsToTxnResponse(inner) => inner.write(ctx),
            ResponseBody::SaslAuthenticateResponse(inner) => inner.write(ctx),
            ResponseBody::ListOffsetResponse(inner) => inner.write(ctx),
            ResponseBody::DescribeGroupsResponse(inner) => inner.write(ctx),
            ResponseBody::OffsetCommitResponse(inner) => inner.write(ctx),
            ResponseBody::ApiVersionsResponse(inner) => inner.write(ctx),
            ResponseBody::WriteTxnMarkersResponse(inner) => inner.write(ctx),
            ResponseBody::DescribeDelegationTokenResponse(inner) => inner.write(ctx),
            ResponseBody::ProduceResponse(inner) => inner.write(ctx),
            ResponseBody::ListGroupsResponse(inner) => inner.write(ctx),
            ResponseBody::OffsetForLeaderEpochResponse(inner) => inner.write(ctx),
            ResponseBody::MetadataResponse(inner) => inner.write(ctx),
            ResponseBody::ControlledShutdownResponse(inner) => inner.write(ctx),
            ResponseBody::SyncGroupResponse(inner) => inner.write(ctx),
            ResponseBody::AlterConfigsResponse(inner) => inner.write(ctx),
            ResponseBody::CreateAclsResponse(inner) => inner.write(ctx),
            ResponseBody::OffsetFetchResponse(inner) => inner.write(ctx),
            ResponseBody::StopReplicaResponse(inner) => inner.write(ctx),
            ResponseBody::CreateDelegationTokenResponse(inner) => inner.write(ctx),
            ResponseBody::HeartbeatResponse(inner) => inner.write(ctx),
            ResponseBody::AddPartitionsToTxnResponse(inner) => inner.write(ctx),
            ResponseBody::DescribeAclsResponse(inner) => inner.write(ctx),
            ResponseBody::RenewDelegationTokenResponse(inner) => inner.write(ctx),
            ResponseBody::CreatePartitionsResponse(inner) => inner.write(ctx),
            ResponseBody::LeaderAndIsrResponse(inner) => inner.write(ctx),
            ResponseBody::ExpireDelegationTokenResponse(inner) => inner.write(ctx),
            ResponseBody::SaslHandshakeResponse(inner) => inner.write(ctx),
            ResponseBody::TxnOffsetCommitResponse(inner) => inner.write(ctx),
            ResponseBody::JoinGroupResponse(inner) => inner.write(ctx),
            ResponseBody::CreateTopicsResponse(inner) => inner.write(ctx),
            ResponseBody::UpdateMetadataResponse(inner) => inner.write(ctx),
            ResponseBody::LeaveGroupResponse(inner) => inner.write(ctx),
            ResponseBody::DescribeLogDirsResponse(inner) => inner.write(ctx),
            ResponseBody::EndTxnResponse(inner) => inner.write(ctx),
            ResponseBody::FetchResponse(inner) => inner.write(ctx),
        }
    }
}
