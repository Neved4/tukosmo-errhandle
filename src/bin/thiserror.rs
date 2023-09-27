use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Unexpectedly failed to get current time (OpenSSL).")]
    UnexpectedlyFailedToGetCurrentTimeOpenSSL,
    #[error("Unexpectedly failed to open chain certificate file (OpenSSL).")]
    UnexpectedlyFailedToOpenChainCertFile,
    #[error("Unexpectedly failed to open pkey certificate file (OpenSSL).")]
    UnexpectedlyFailedToOpenPkeyFile,
    #[error("Unexpectedly failed to parse the X509 certificate (OpenSSL).")]
    UnexpectedlyFailedToParseX509File,
    #[error("Failed to parse certificate chain.")]
    UnexpectedlyFailedToParseChainFile,
    #[error("Unexpectedly failed to extract PKCS8-encoded private key.")]
    UnexpectedlyFailedToExtractPKCS8Keys,
    #[error("Unexpectedly failed to find time difference between two times.")]
    UnexpectedlyFailedToFindTimeDiff,
    #[error("Unexpectedly failed to generate self-signed certificate.")]
    UnexpectedlyFailedToGenerateSelfSignedCert,
    #[error("Unexpectedly failed to serialize certificate into ASCII PEM.")]
    UnexpectedlyFailedToSerializeCertASCIIPem,
    #[error("Unexpectedly failed to write certificate private key.")]
    UnexpectedlyFailedToWriteCertPkeyRoute,
    #[error("Unexpectedly failed to write certificate.")]
    UnexpectedlyFailedToWriteCertRoute,
    #[error("Unexpectedly failed to write certificate chain.")]
    UnexpectedlyFailedToWriteCertChainRoute,
    #[error("Unexpectedly failed to create the certificates directory.")]
    UnexpectedlyFailedToCreateCertsDir,
    #[error("Unexpectedly failed to create the ACME challenge directory.")]
    UnexpectedlyFailedToCreateACMEChallengeDir,
    #[error("Unexpectedly failed to create a directory from the URL.")]
    UnexpectedlyFailedToCreateADirFromURL,
    #[error("Unexpectedly failed to register the account.")]
    UnexpectedlyFailedToRegisterAccount,
    #[error("Unexpectedly failed to get the ACME private key.")]
    UnexpectedlyFailedToGetACMEPrivateKey,
    #[error("Unexpectedly failed to load the account.")]
    UnexpectedlyFailedToLoadAccount,
    #[error("Unexpectedly failed to create a new certificate order.")]
    UnexpectedlyFailedToCreateNewCertOrder,
    #[error("Unexpectedly failed to provide authorizations.")]
    UnexpectedlyFailedToProvideAuthorizations,
    #[error("Unexpectedly failed to get the HTTP challenge.")]
    UnexpectedlyFailedToGetHTTPChallenge,
    #[error("Unexpectedly failed to get the HTTP proof.")]
    UnexpectedlyFailedToGetHTTPProof,
    #[error("Unexpectedly failed to write the proof slice.")]
    UnexpectedlyFailedToWriteProofSlice,
    #[error("Unexpectedly failed to validate the proof.")]
    UnexpectedlyFailedToValidateProof,
    #[error("Unexpectedly failed to refresh the order state.")]
    UnexpectedlyFailedToRefreshOrderState,
    #[error("Unexpectedly failed to create a P384 keypair.")]
    UnexpectedlyFailedToCreateP384Keypair,
    #[error(
        "Unexpectedly failed to create a CSR (Certificate Signing Request)."
    )]
    UnexpectedlyFailedToCreateCSR,
    #[error("Unexpectedly failed to request certificate download.")]
    UnexpectedlyFailedToRequestCertDownload,
    #[error("Unexpectedly failed to remove the ACME challenge directory.")]
    UnexpectedlyFailedToRemoveACMEChallengeDir,
    #[error("Unexpectedly failed to make a blocking GET request.")]
    UnexpectedlyFailedToMakeBlockingGetRequest,
    #[error("Unexpectedly failed to get the response body as bytes.")]
    UnexpectedlyFailedToGetResponseBodyAsBytes,
    #[error("Unexpectedly failed to write intermediate certificate route.")]
    UnexpectedlyFailedToWriteIntermediateCertRoute,
    #[error("Unexpectedly failed to read certificate route to string.")]
    UnexpectedlyFailedToReadCertRouteToString,
    #[error(
        "Unexpectedly failed to read intermediate certificate route to string."
    )]
    UnexpectedlyFailedToReadIntermediateCertRouteToString,
    #[error(
        "Unexpectedly failed to write certificate chain route plus intermediate certificate."
    )]
    UnexpectedlyFailedToWriteCertChainRoutePlusIntermediateCert,
    #[error("Unexpectedly failed to read certificate route.")]
    UnexpectedlyFailedToReadCertRoute,
    #[error("Unexpectedly failed to convert self error.")]
    UnexpectedlyFailedToConvertSelfError,
}

#[derive(Error, Debug)]
pub enum MyDomainError {
    #[error("Domain-specific error.")]
    DomainSpecificError,
}

impl From<MyError> for MyDomainError {
    fn from(_error: MyError) -> Self {
        MyDomainError::DomainSpecificError
    }
}

fn main() {
    let mut error_messages = Vec::new();

    for _ in 0..8192 {
        let _err = MyError::UnexpectedlyFailedToGetCurrentTimeOpenSSL;
        error_messages.push(format!("Error: {:?}", _err));
    }

    let total_errors = error_messages.len();
    println!("Errors collected: {}", total_errors);
}
