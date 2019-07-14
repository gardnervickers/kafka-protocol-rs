pub type ApiKey = i16;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum ApiKeys {
    InitProducerId = 22i16,
    CreateTopics = 19i16,
    UpdateMetadata = 6i16,
    DeleteAcls = 31i16,
    ElectPreferredLeaders = 43i16,
    WriteTxnMarkers = 27i16,
    LeaveGroup = 13i16,
    AlterConfigs = 33i16,
    ControlledShutdown = 7i16,
    TxnOffsetCommit = 28i16,
    OffsetCommit = 8i16,
    Metadata = 3i16,
    EndTxn = 26i16,
    DescribeLogDirs = 35i16,
    DescribeGroups = 15i16,
    SyncGroup = 14i16,
    LeaderAndIsr = 4i16,
    DescribeConfigs = 32i16,
    AddOffsetsToTxn = 25i16,
    DeleteRecords = 21i16,
    JoinGroup = 11i16,
    CreateAcls = 30i16,
    OffsetForLeaderEpoch = 23i16,
    DeleteGroups = 42i16,
    CreateDelegationToken = 38i16,
    DescribeAcls = 29i16,
    ListOffset = 2i16,
    FindCoordinator = 10i16,
    ExpireDelegationToken = 40i16,
    ListGroups = 16i16,
    Fetch = 1i16,
    ApiVersions = 18i16,
    Heartbeat = 12i16,
    SaslHandshake = 17i16,
    DeleteTopics = 20i16,
    DescribeDelegationToken = 41i16,
    SaslAuthenticate = 36i16,
    StopReplica = 5i16,
    Produce = 0i16,
    OffsetFetch = 9i16,
    RenewDelegationToken = 39i16,
    AlterReplicaLogDirs = 34i16,
    AddPartitionsToTxn = 24i16,
    CreatePartitions = 37i16,
}
impl ApiKeys {
    pub fn key(self) -> ApiKey {
        unsafe { std::mem::transmute(self) }
    }
}
impl From<ApiKey> for ApiKeys {
    fn from(v: ApiKey) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}
impl Into<i16> for ApiKeys {
    fn into(self) -> i16 {
        self as i16
    }
}
