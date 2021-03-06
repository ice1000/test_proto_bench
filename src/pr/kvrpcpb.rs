#[derive(Clone, PartialEq, Message)]
pub struct LockInfo {
    #[prost(bytes, tag="1")]
    pub primary_lock: Vec<u8>,
    #[prost(uint64, tag="2")]
    pub lock_version: u64,
    #[prost(bytes, tag="3")]
    pub key: Vec<u8>,
    #[prost(uint64, tag="4")]
    pub lock_ttl: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct KeyError {
    /// Client should backoff or cleanup the lock then retry.
    #[prost(message, optional, tag="1")]
    pub locked: ::std::option::Option<LockInfo>,
    /// Client may restart the txn. e.g write conflict.
    #[prost(string, tag="2")]
    pub retryable: String,
    /// Client should abort the txn.
    #[prost(string, tag="3")]
    pub abort: String,
    /// Write conflict is moved from retryable to here.
    #[prost(message, optional, tag="4")]
    pub conflict: ::std::option::Option<WriteConflict>,
}
#[derive(Clone, PartialEq, Message)]
pub struct WriteConflict {
    #[prost(uint64, tag="1")]
    pub start_ts: u64,
    #[prost(uint64, tag="2")]
    pub conflict_ts: u64,
    #[prost(bytes, tag="3")]
    pub key: Vec<u8>,
    #[prost(bytes, tag="4")]
    pub primary: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Context {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
    #[prost(message, optional, tag="2")]
    pub region_epoch: ::std::option::Option<super::metapb::RegionEpoch>,
    #[prost(message, optional, tag="3")]
    pub peer: ::std::option::Option<super::metapb::Peer>,
    #[prost(uint64, tag="5")]
    pub term: u64,
    #[prost(enumeration="CommandPri", tag="6")]
    pub priority: i32,
    #[prost(enumeration="IsolationLevel", tag="7")]
    pub isolation_level: i32,
    #[prost(bool, tag="8")]
    pub not_fill_cache: bool,
    #[prost(bool, tag="9")]
    pub sync_log: bool,
    /// true means return handle time detail
    #[prost(bool, tag="10")]
    pub handle_time: bool,
    /// true means return scan cf's detail
    #[prost(bool, tag="11")]
    pub scan_detail: bool,
}
#[derive(Clone, PartialEq, Message)]
pub struct HandleTime {
    /// time in queue
    #[prost(int64, tag="1")]
    pub wait_ms: i64,
    /// process time without wait time.
    #[prost(int64, tag="2")]
    pub process_ms: i64,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanInfo {
    /// total count
    #[prost(int64, tag="1")]
    pub total: i64,
    /// processed count
    #[prost(int64, tag="2")]
    pub processed: i64,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanDetail {
    #[prost(message, optional, tag="1")]
    pub write: ::std::option::Option<ScanInfo>,
    #[prost(message, optional, tag="2")]
    pub lock: ::std::option::Option<ScanInfo>,
    #[prost(message, optional, tag="3")]
    pub data: ::std::option::Option<ScanInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ExecDetails {
    /// set when ctx.handle_time = true or meet slow query
    #[prost(message, optional, tag="1")]
    pub handle_time: ::std::option::Option<HandleTime>,
    /// set when ctx.scan_detail = true or meet slow query
    #[prost(message, optional, tag="2")]
    pub scan_detail: ::std::option::Option<ScanDetail>,
}
#[derive(Clone, PartialEq, Message)]
pub struct GetRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(uint64, tag="3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct GetResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(bytes, tag="3")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub start_key: Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(uint64, tag="4")]
    pub version: u64,
    #[prost(bool, tag="5")]
    pub key_only: bool,
    #[prost(bool, tag="6")]
    pub reverse: bool,
    /// For compatibility, when scanning forward, the range to scan is [start_key, end_key), where start_key < end_key;
    /// and when scanning backward, it scans [end_key, start_key) in descending order, where end_key < start_key.
    #[prost(bytes, tag="7")]
    pub end_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct KvPair {
    #[prost(message, optional, tag="1")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Mutation {
    #[prost(enumeration="Op", tag="1")]
    pub op: i32,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct PrewriteRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(message, repeated, tag="2")]
    pub mutations: ::std::vec::Vec<Mutation>,
    /// primary_lock_key
    #[prost(bytes, tag="3")]
    pub primary_lock: Vec<u8>,
    #[prost(uint64, tag="4")]
    pub start_version: u64,
    #[prost(uint64, tag="5")]
    pub lock_ttl: u64,
    #[prost(bool, tag="6")]
    pub skip_constraint_check: bool,
}
#[derive(Clone, PartialEq, Message)]
pub struct PrewriteResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub errors: ::std::vec::Vec<KeyError>,
}
#[derive(Clone, PartialEq, Message)]
pub struct CommitRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub start_version: u64,
    #[prost(bytes, repeated, tag="3")]
    pub keys: ::std::vec::Vec<Vec<u8>>,
    #[prost(uint64, tag="4")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct CommitResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ImportRequest {
    #[prost(message, repeated, tag="1")]
    pub mutations: ::std::vec::Vec<Mutation>,
    #[prost(uint64, tag="2")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct ImportResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct BatchRollbackRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub start_version: u64,
    #[prost(bytes, repeated, tag="3")]
    pub keys: ::std::vec::Vec<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct BatchRollbackResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, Message)]
pub struct CleanupRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(uint64, tag="3")]
    pub start_version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct CleanupResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
    /// set this if the key is already committed
    #[prost(uint64, tag="3")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct BatchGetRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag="2")]
    pub keys: ::std::vec::Vec<Vec<u8>>,
    #[prost(uint64, tag="3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct BatchGetResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanLockRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub max_version: u64,
    #[prost(bytes, tag="3")]
    pub start_key: Vec<u8>,
    #[prost(uint32, tag="4")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, Message)]
pub struct ScanLockResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(message, repeated, tag="3")]
    pub locks: ::std::vec::Vec<LockInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TxnInfo {
    #[prost(uint64, tag="1")]
    pub txn: u64,
    #[prost(uint64, tag="2")]
    pub status: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct ResolveLockRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub start_version: u64,
    /// If the txn is rolled back, do not set it.
    #[prost(uint64, tag="3")]
    pub commit_version: u64,
    #[prost(message, repeated, tag="4")]
    pub txn_infos: ::std::vec::Vec<TxnInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ResolveLockResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, Message)]
pub struct GcRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub safe_point: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct GcResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawGetRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(string, tag="3")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawGetResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
    #[prost(bytes, tag="3")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawPutRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub value: Vec<u8>,
    #[prost(string, tag="4")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawPutResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchPutRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::std::vec::Vec<KvPair>,
    #[prost(string, tag="3")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchPutResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchGetRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag="2")]
    pub keys: ::std::vec::Vec<Vec<u8>>,
    #[prost(string, tag="3")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchGetResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawDeleteRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
    #[prost(string, tag="3")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawDeleteResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchDeleteRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag="2")]
    pub keys: ::std::vec::Vec<Vec<u8>>,
    #[prost(string, tag="3")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchDeleteResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct DeleteRangeRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub start_key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub end_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct DeleteRangeResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawDeleteRangeRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub start_key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub end_key: Vec<u8>,
    #[prost(string, tag="4")]
    pub cf: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawDeleteRangeResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawScanRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub start_key: Vec<u8>,
    #[prost(uint32, tag="3")]
    pub limit: u32,
    #[prost(bool, tag="4")]
    pub key_only: bool,
    #[prost(string, tag="5")]
    pub cf: String,
    #[prost(bool, tag="6")]
    pub reverse: bool,
    /// For compatibility, when scanning forward, the range to scan is [start_key, end_key), where start_key < end_key;
    /// and when scanning backward, it scans [end_key, start_key) in descending order, where end_key < start_key.
    #[prost(bytes, tag="7")]
    pub end_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawScanResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub kvs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, Message)]
pub struct KeyRange {
    #[prost(bytes, tag="1")]
    pub start_key: Vec<u8>,
    #[prost(bytes, tag="2")]
    pub end_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchScanRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    /// scanning range
    #[prost(message, repeated, tag="2")]
    pub ranges: ::std::vec::Vec<KeyRange>,
    /// max number of returning kv pairs for each scanning range
    #[prost(uint32, tag="3")]
    pub each_limit: u32,
    #[prost(bool, tag="4")]
    pub key_only: bool,
    #[prost(string, tag="5")]
    pub cf: String,
    #[prost(bool, tag="6")]
    pub reverse: bool,
}
#[derive(Clone, PartialEq, Message)]
pub struct RawBatchScanResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag="2")]
    pub kvs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccWrite {
    #[prost(enumeration="Op", tag="1")]
    pub type_: i32,
    #[prost(uint64, tag="2")]
    pub start_ts: u64,
    #[prost(uint64, tag="3")]
    pub commit_ts: u64,
    #[prost(bytes, tag="4")]
    pub short_value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccValue {
    #[prost(uint64, tag="1")]
    pub start_ts: u64,
    #[prost(bytes, tag="2")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccLock {
    #[prost(enumeration="Op", tag="1")]
    pub type_: i32,
    #[prost(uint64, tag="2")]
    pub start_ts: u64,
    #[prost(bytes, tag="3")]
    pub primary: Vec<u8>,
    #[prost(bytes, tag="4")]
    pub short_value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccInfo {
    #[prost(message, optional, tag="1")]
    pub lock: ::std::option::Option<MvccLock>,
    #[prost(message, repeated, tag="2")]
    pub writes: ::std::vec::Vec<MvccWrite>,
    #[prost(message, repeated, tag="3")]
    pub values: ::std::vec::Vec<MvccValue>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccGetByKeyRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccGetByKeyResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
    #[prost(message, optional, tag="3")]
    pub info: ::std::option::Option<MvccInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccGetByStartTsRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag="2")]
    pub start_ts: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct MvccGetByStartTsResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
    #[prost(bytes, tag="3")]
    pub key: Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub info: ::std::option::Option<MvccInfo>,
}
#[derive(Clone, PartialEq, Message)]
pub struct SplitRegionRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub split_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct SplitRegionResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag="2")]
    pub left: ::std::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="3")]
    pub right: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, Message)]
pub struct UnsafeDestroyRangeRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag="2")]
    pub start_key: Vec<u8>,
    #[prost(bytes, tag="3")]
    pub end_key: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct UnsafeDestroyRangeResponse {
    #[prost(message, optional, tag="1")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(string, tag="2")]
    pub error: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
pub enum CommandPri {
    /// Normal must the default value
    Normal = 0,
    Low = 1,
    High = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
pub enum IsolationLevel {
    /// SI = snapshot isolation
    Si = 0,
    /// RC = read committed
    Rc = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
    Rollback = 3,
}
