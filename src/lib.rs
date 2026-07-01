use prost::Name;
tonic::include_proto!("account");
impl Name for AccountReply {
    const NAME: &'static str = "AccountReply";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for ClientInfoReplay {
    const NAME: &'static str = "ClientInfoReplay";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for CountReply {
    const NAME: &'static str = "CountReply";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for SimpleString {
    const NAME: &'static str = "SimpleString";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for AuthTokenReply {
    const NAME: &'static str = "AuthTokenReply";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for ClientInfoPageReply {
    const NAME: &'static str = "ClientInfoPageReply";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for ClientInfoAddRequest {
    const NAME: &'static str = "ClientInfoAddRequest";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for ClientInfoUpdateRequest {
    const NAME: &'static str = "ClientInfoUpdateRequest";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for ClientInfoPageRequest {
    const NAME: &'static str = "ClientInfoPageRequest";
    const PACKAGE: &'static str = "account_proto";
}