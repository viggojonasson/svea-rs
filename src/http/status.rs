#[derive(Debug, PartialEq)]
pub enum Status {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Continue => "100 Continue",
            Status::SwitchingProtocols => "101 Switching Protocols",
            Status::Processing => "102 Processing",
            Status::EarlyHints => "103 Early Hints",
            Status::Ok => "200 OK",
            Status::Created => "201 Created",
            Status::Accepted => "202 Accepted",
            Status::NonAuthoritativeInformation => "203 Non-Authoritative Information",
            Status::NoContent => "204 No Content",
            Status::ResetContent => "205 Reset Content",
            Status::PartialContent => "206 Partial Content",
            Status::MultiStatus => "207 Multi-Status",
            Status::AlreadyReported => "208 Already Reported",
            Status::IMUsed => "226 IM Used",
            Status::MultipleChoices => "300 Multiple Choices",
            Status::MovedPermanently => "301 Moved Permanently",
            Status::Found => "302 Found",
            Status::SeeOther => "303 See Other",
            Status::NotModified => "304 Not Modified",
            Status::UseProxy => "305 Use Proxy",
            Status::TemporaryRedirect => "307 Temporary Redirect",
            Status::PermanentRedirect => "308 Permanent Redirect",
            Status::BadRequest => "400 Bad Request",
            Status::Unauthorized => "401 Unauthorized",
            Status::PaymentRequired => "402 Payment Required",
            Status::Forbidden => "403 Forbidden",
            Status::NotFound => "404 Not Found",
            Status::MethodNotAllowed => "405 Method Not Allowed",
            Status::NotAcceptable => "406 Not Acceptable",
            Status::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            Status::RequestTimeout => "408 Request Timeout",
            Status::Conflict => "409 Conflict",
            Status::Gone => "410 Gone",
            Status::LengthRequired => "411 Length Required",
            Status::PreconditionFailed => "412 Precondition Failed",
            Status::PayloadTooLarge => "413 Payload Too Large",
            Status::URITooLong => "414 URI Too Long",
            Status::UnsupportedMediaType => "415 Unsupported Media Type",
            Status::RangeNotSatisfiable => "416 Range Not Satisfiable",
            Status::ExpectationFailed => "417 Expectation Failed",
            Status::ImATeapot => "418 I'm a teapot",
            Status::MisdirectedRequest => "421 Misdirected Request",
            Status::UnprocessableEntity => "422 Unprocessable Entity",
            Status::Locked => "423 Locked",
            Status::FailedDependency => "424 Failed Dependency",
            Status::TooEarly => "425 Too Early",
            Status::UpgradeRequired => "426 Upgrade Required",
            Status::PreconditionRequired => "428 Precondition Required",
            Status::TooManyRequests => "429 Too Many Requests",
            Status::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            Status::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
            Status::InternalServerError => "500 Internal Server Error",
            Status::NotImplemented => "501 Not Implemented",
            Status::BadGateway => "502 Bad Gateway",
            Status::ServiceUnavailable => "503 Service Unavailable",
            Status::GatewayTimeout => "504 Gateway Timeout",
            Status::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
            Status::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            Status::InsufficientStorage => "507 Insufficient Storage",
            Status::LoopDetected => "508 Loop Detected",
            Status::NotExtended => "510 Not Extended",
            Status::NetworkAuthenticationRequired => "511 Network Authentication Required",
        }
        .to_string()
    }
}

impl TryFrom<u16> for Status {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Status::Continue),
            101 => Ok(Status::SwitchingProtocols),
            102 => Ok(Status::Processing),
            103 => Ok(Status::EarlyHints),
            200 => Ok(Status::Ok),
            201 => Ok(Status::Created),
            202 => Ok(Status::Accepted),
            203 => Ok(Status::NonAuthoritativeInformation),
            204 => Ok(Status::NoContent),
            205 => Ok(Status::ResetContent),
            206 => Ok(Status::PartialContent),
            207 => Ok(Status::MultiStatus),
            208 => Ok(Status::AlreadyReported),
            226 => Ok(Status::IMUsed),
            300 => Ok(Status::MultipleChoices),
            301 => Ok(Status::MovedPermanently),
            302 => Ok(Status::Found),
            303 => Ok(Status::SeeOther),
            304 => Ok(Status::NotModified),
            305 => Ok(Status::UseProxy),
            307 => Ok(Status::TemporaryRedirect),
            308 => Ok(Status::PermanentRedirect),
            400 => Ok(Status::BadRequest),
            401 => Ok(Status::Unauthorized),
            402 => Ok(Status::PaymentRequired),
            403 => Ok(Status::Forbidden),
            404 => Ok(Status::NotFound),
            405 => Ok(Status::MethodNotAllowed),
            406 => Ok(Status::NotAcceptable),
            407 => Ok(Status::ProxyAuthenticationRequired),
            408 => Ok(Status::RequestTimeout),
            409 => Ok(Status::Conflict),
            410 => Ok(Status::Gone),
            411 => Ok(Status::LengthRequired),
            412 => Ok(Status::PreconditionFailed),
            413 => Ok(Status::PayloadTooLarge),
            414 => Ok(Status::URITooLong),
            415 => Ok(Status::UnsupportedMediaType),
            416 => Ok(Status::RangeNotSatisfiable),
            417 => Ok(Status::ExpectationFailed),
            418 => Ok(Status::ImATeapot),
            421 => Ok(Status::MisdirectedRequest),
            422 => Ok(Status::UnprocessableEntity),
            423 => Ok(Status::Locked),
            424 => Ok(Status::FailedDependency),
            425 => Ok(Status::TooEarly),
            426 => Ok(Status::UpgradeRequired),
            428 => Ok(Status::PreconditionRequired),
            429 => Ok(Status::TooManyRequests),
            431 => Ok(Status::RequestHeaderFieldsTooLarge),
            451 => Ok(Status::UnavailableForLegalReasons),
            500 => Ok(Status::InternalServerError),
            501 => Ok(Status::NotImplemented),
            502 => Ok(Status::BadGateway),
            503 => Ok(Status::ServiceUnavailable),
            504 => Ok(Status::GatewayTimeout),
            505 => Ok(Status::HTTPVersionNotSupported),
            506 => Ok(Status::VariantAlsoNegotiates),
            507 => Ok(Status::InsufficientStorage),
            508 => Ok(Status::LoopDetected),
            510 => Ok(Status::NotExtended),
            511 => Ok(Status::NetworkAuthenticationRequired),
            _ => Err(Self::Error::from(format!("Invalid status code: {}", value))),
        }
    }
}