use custom_error::custom_error;

custom_error! {
    pub TukosmoDomainError
    UnexpectedlyFailedToGetCurrentTimeOpenSSL =
        "Unexpectedly failed to get current time (OpenSSL).",
    UnexpectedlyFailedToOpenChainCertFile =
        "Unexpectedly failed to open chain certificate file (OpenSSL).",
    UnexpectedlyFailedToOpenPkeyFile =
        "Unexpectedly failed to open pkey certificate file (OpenSSL).",
    UnexpectedlyFailedToParseX509File =
        "Unexpectedly failed to parse the X509 certificate (OpenSSL).",
    UnexpectedlyFailedToParseChainFile = "Failed to parse certificate chain.",
    UnexpectedlyFailedToExtractPKCS8Keys =
        "Unexpectedly failed to extract PKCS8-encoded private key.",
    UnexpectedlyFailedToFindTimeDiff =
        "Unexpectedly failed to find time difference between two times.",
    UnexpectedlyFailedToGenerateSelfSignedCert =
        "Unexpectedly failed to generate self-signed certificate.",
    UnexpectedlyFailedToSerializeCertASCIIPem =
        "Unexpectedly failed to serialize certificate into ASCII PEM.",
    UnexpectedlyFailedToWriteCertPkeyRoute =
        "Unexpectedly failed to write certificate private key.",
    UnexpectedlyFailedToWriteCertRoute =
        "Unexpectedly failed to write certificate.",
    UnexpectedlyFailedToWriteCertChainRoute =
        "Unexpectedly failed to write certificate chain.",
    UnexpectedlyFailedToCreateCertsDir =
        "Unexpectedly failed to create the certificates directory.",
    UnexpectedlyFailedToCreateACMEChallengeDir =
        "Unexpectedly failed to create the ACME challenge directory.",
    UnexpectedlyFailedToCreateADirFromURL =
        "Unexpectedly failed to create a directory from the URL.",
    UnexpectedlyFailedToRegisterAccount =
        "Unexpectedly failed to register the account.",
    UnexpectedlyFailedToGetACMEPrivateKey =
        "Unexpectedly failed to get the ACME private key.",
    UnexpectedlyFailedToLoadAccount =
        "Unexpectedly failed to load the account.",
    UnexpectedlyFailedToCreateNewCertOrder =
        "Unexpectedly failed to create a new certificate order.",
    UnexpectedlyFailedToProvideAuthorizations =
        "Unexpectedly failed to provide authorizations.",
    UnexpectedlyFailedToGetHTTPChallenge =
        "Unexpectedly failed to get the HTTP challenge.",
    UnexpectedlyFailedToGetHTTPProof =
        "Unexpectedly failed to get the HTTP proof.",
    UnexpectedlyFailedToWriteProofSlice =
        "Unexpectedly failed to write the proof slice.",
    UnexpectedlyFailedToValidateProof =
        "Unexpectedly failed to validate the proof.",
    UnexpectedlyFailedToRefreshOrderState =
        "Unexpectedly failed to refresh the order state.",
    UnexpectedlyFailedToCreateP384Keypair =
        "Unexpectedly failed to create a P384 keypair.",
    UnexpectedlyFailedToCreateCSR =
        "Unexpectedly failed to create a CSR (Certificate Signing Request).",
    UnexpectedlyFailedToRequestCertDownload =
        "Unexpectedly failed to request certificate download.",
    UnexpectedlyFailedToRemoveACMEChallengeDir =
        "Unexpectedly failed to remove the ACME challenge directory.",
    UnexpectedlyFailedToMakeBlockingGetRequest =
        "Unexpectedly failed to make a blocking GET request.",
    UnexpectedlyFailedToGetResponseBodyAsBytes =
        "Unexpectedly failed to get the response body as bytes.",
    UnexpectedlyFailedToWriteIntermediateCertRoute =
        "Unexpectedly failed to write intermediate certificate route.",
    UnexpectedlyFailedToReadCertRouteToString =
        "Unexpectedly failed to read certificate route to string.",
    UnexpectedlyFailedToReadIntermediateCertRouteToString =
        "Unexpectedly failed to read intermediate certificate route to string.",
    UnexpectedlyFailedToWriteCertChainRoutePlusIntermediateCert =
        "Unexpectedly failed to write certificate chain route plus intermediate certificate.",
    UnexpectedlyFailedToReadCertRoute =
        "Unexpectedly failed to read certificate route.",
    UnexpectedlyFailedToConvertSelfError =
        "Unexpectedly failed to convert self error.",
}

#[derive(Debug)]
pub enum MyDomainError {
    DomainSpecificError,
}

impl From<TukosmoDomainError> for MyDomainError {
    fn from(_error: TukosmoDomainError) -> Self {
        MyDomainError::DomainSpecificError
    }
}

fn main() {
    let _err = TukosmoDomainError::UnexpectedlyFailedToGetCurrentTimeOpenSSL;
}
