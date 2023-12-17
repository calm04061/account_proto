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

impl Name for SimpleString {
    const NAME: &'static str = "SimpleString";
    const PACKAGE: &'static str = "account_proto";
}

impl Name for AuthTokenReply {
    const NAME: &'static str = "AuthTokenReply";
    const PACKAGE: &'static str = "account_proto";
}