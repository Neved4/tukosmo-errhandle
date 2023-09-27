#[derive(Clone, Debug, PartialEq)]
pub enum BlogSubmoduleName {
    Shared,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoreSubmoduleName {
    Language,
    Shared,
    Tag,
    User,
}

#[derive(Clone, Debug)]
pub struct DomainError {
    pub context_code: &'static str,
    pub error_code: &'static str,
    pub layer: LayerName,
    pub message: &'static str,
    pub module: ModuleName,
    // TODO: Add optional context to include extra useful values.
    // Currently we use constants, so adding an optional context with
    // dynamic arguments (id of some entity, the user, etc.) is not trivial.
}

#[derive(Clone, Debug, PartialEq)]
pub enum InfrastructureName {
    DieselOrm,
    LeptosActix,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayerName {
    Application,
    Domain,
    Infrastructure(InfrastructureName),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleName {
    Blog(BlogSubmoduleName),
    Core(CoreSubmoduleName),
}

pub const fn build_domain_error(
    context_code: &'static str,
    error_code: &'static str,
    message: &'static str
) -> DomainError {
    DomainError {
        context_code,
        error_code,
        layer: LayerName::Infrastructure(InfrastructureName::DieselOrm),
        message,
        module: ModuleName::Core(CoreSubmoduleName::Language),
    }
}

const fn get_domain_error(
    error_code: &'static str,
    message: &'static str
) -> DomainError {
    build_domain_error(CONTEXT_CODE, error_code, message)
}

const CONTEXT_CODE: &'static str = "I18N_TEXT_OPERATION";

const _ERR_UNEXPECTEDLY_FAILED_TO_GET_CURRENT_TIME_OPENSSL: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GET_CURRENT_TIME_OPENSSL",
        "Unexpectedly failed to get current time (OpenSSL)."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_OPEN_CHAIN_CERT_FILE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_OPEN_CHAIN_CERT_FILE",
        "Unexpectedly failed to open chain certificate file (OpenSSL)."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_OPEN_PKEY_FILE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_OPEN_PKEY_FILE",
        "Unexpectedly failed to open pkey certificate file (OpenSSL)."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_PARSE_X509_FILE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_PARSE_X509_FILE",
        "Unexpectedly failed to parse the X509 certificate (OpenSSL)."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_PARSE_CHAIN_FILE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_PARSE_CHAIN_FILE",
        "Failed to parse certificate chain."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_EXTRACT_PKCS8_KEYS: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_EXTRACT_PKCS8_KEYS",
        "Unexpectedly failed to extract PKCS8-encoded private key."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_FIND_TIME_DIFF: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_FIND_TIME_DIFF",
        "Unexpectedly failed to find time difference between two times."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_GENERATE_SELF_SIGNED_CERT: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GENERATE_SELF_SIGNED_CERT",
        "Unexpectedly failed to generate self-signed certificate."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_SERIALIZE_CERT_ASCII_PEM: DomainError =
    get_domain_error(
        "ERR_UNEXPECTEDLY_FAILED_TO_SERIALIZE_CERT_ASCII_PEM",
        "Unexpectedly failed to serialize certificate into ASCII PEM."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_CERT_PKEY_ROUTE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_CERT_PKEY_ROUTE",
        "Unexpectedly failed to write certificate private key."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_CERT_ROUTE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_CERT_ROUTE",
        "Unexpectedly failed to write certificate."
    );
const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_CERT_CHAIN_ROUTE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_CERT_CHAIN_ROUTE",
        "Unexpectedly failed to write certificate chain."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_CERTS_DIR: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CREATE_CERTS_DIR",
        "Unexpectedly failed to create the certificates directory."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_ACME_CHALLENGE_DIR: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CREATE_ACME_CHALLENGE_DIR",
        "Unexpectedly failed to create the ACME challenge directory."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_A_DIR_FROM_URL: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CREATE_A_DIR_FROM_URL",
        "Unexpectedly failed to create a directory from the URL."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_REGISTER_ACCOUNT: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_REGISTER_ACCOUNT",
        "Unexpectedly failed to register the account."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_GET_ACME_PRIVATE_KEY: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GET_ACME_PRIVATE_KEY",
        "Unexpectedly failed to get the ACME private key."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_LOAD_ACCOUNT: DomainError = get_domain_error(
    "UNEXPECTEDLY_FAILED_TO_LOAD_ACCOUNT",
    "Unexpectedly failed to load the account."
);

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_NEW_CERT_ORDER: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CREATE_NEW_CERT_ORDER",
        "Unexpectedly failed to create a new certificate order."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_PROVIDE_AUTHORIZATIONS: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_PROVIDE_AUTHORIZATIONS",
        "Unexpectedly failed to provide authorizations."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_GET_HTTP_CHALLENGE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GET_HTTP_CHALLENGE",
        "Unexpectedly failed to get the HTTP challenge."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_GET_HTTP_PROOF: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GET_HTTP_PROOF",
        "Unexpectedly failed to get the HTTP proof."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_PROOF_SLICE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_PROOF_SLICE",
        "Unexpectedly failed to write the proof slice."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_VALIDATE_PROOF: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_VALIDATE_PROOF",
        "Unexpectedly failed to validate the proof."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_REFRESH_ORDER_STATE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_REFRESH_ORDER_STATE",
        "Unexpectedly failed to refresh the order state."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_P384_KEYPAIR: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CREATE_P384_KEYPAIR",
        "Unexpectedly failed to create a P384 keypair."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CREATE_CSR: DomainError = get_domain_error(
    "UNEXPECTEDLY_FAILED_TO_CREATE_CSR",
    "Unexpectedly failed to create a CSR (Certificate Signing Request)."
);

const _ERR_UNEXPECTEDLY_FAILED_TO_REQUEST_CERT_DOWNLOAD: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_REQUEST_CERT_DOWNLOAD",
        "Unexpectedly failed to request certificate download."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_REMOVE_ACME_CHALLENGE_DIR: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_REMOVE_ACME_CHALLENGE_DIR",
        "Unexpectedly failed to remove the ACME challenge directory."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_MAKE_BLOCKING_GET_REQUEST: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_MAKE_BLOCKING_GET_REQUEST",
        "Unexpectedly failed to make a blocking GET request."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_GET_RESPONSE_BODY_AS_BYTES: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_GET_RESPONSE_BODY_AS_BYTES",
        "Unexpectedly failed to get the response body as bytes."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_INTERMEDIATE_CERT_ROUTE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_INTERMEDIATE_CERT_ROUTE",
        "Unexpectedly failed to write intermediate certificate route."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_READ_CERT_ROUTE_TO_STRING: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_READ_CERT_ROUTE_TO_STRING",
        "Unexpectedly failed to read certificate route to string."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_READ_INTERMEDIATE_CERT_ROUTE_TO_STRING: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_READ_INTERMEDIATE_CERT_ROUTE_TO_STRING",
        "Unexpectedly failed to read intermediate certificate route to string."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_WRITE_CERT_CHAIN_ROUTE_PLUS_INTERMEDIATE_CERT: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_WRITE_CERT_CHAIN_ROUTE_PLUS_INTERMEDIATE_CERT",
        "Unexpectedly failed to write certificate chain route plus intermediate certificate."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_READ_CERT_ROUTE: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_READ_CERT_ROUTE",
        "Unexpectedly failed to read certificate route."
    );

const _ERR_UNEXPECTEDLY_FAILED_TO_CONVERT_SELF_ERROR: DomainError =
    get_domain_error(
        "UNEXPECTEDLY_FAILED_TO_CONVERT_SELF_ERROR",
        "Unexpectedly failed to convert self error."
    );

fn main() {
    let mut error_messages = Vec::new();

    for _ in 0..8192 {
        let _err = _ERR_UNEXPECTEDLY_FAILED_TO_GET_CURRENT_TIME_OPENSSL;
        error_messages.push(format!("Error: {:?}", _err));
    }

    let total_errors = error_messages.len();
    println!("Errors collected: {}", total_errors);
}
