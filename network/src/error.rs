// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::peer_manager::PeerManagerError;
use futures::channel::{mpsc, oneshot};
use libra_types::validator_verifier::VerifyError;
use std::io;
use thiserror::Error;

/// Errors propagated from the network module.
#[derive(Debug, Error)]
#[error("{inner}")]
pub struct NetworkError {
    inner: anyhow::Error,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Error)]
pub enum NetworkErrorKind {
    #[error("IO error")]
    IoError,

    #[error("Error parsing protobuf message")]
    ProtobufParseError,

    #[error("Invalid signature error")]
    SignatureError,

    #[error("Failed to parse multiaddrs")]
    MultiaddrError,

    #[error("Error sending on mpsc channel")]
    MpscSendError,

    #[error("Oneshot channel unexpectedly dropped")]
    OneshotCanceled,

    #[error("Error setting timeout")]
    TimerError,

    #[error("Operation timed out")]
    TimedOut,

    #[error("Unknown tokio::time Error variant")]
    UnknownTimerError,

    #[error("PeerManager error")]
    PeerManagerError,

    #[error("Parsing error")]
    ParsingError,

    #[error("Peer not connected")]
    NotConnected,
}

impl From<NetworkErrorKind> for NetworkError {
    fn from(kind: NetworkErrorKind) -> NetworkError {
        NetworkError {
            inner: anyhow::Error::new(kind),
        }
    }
}

impl From<anyhow::Error> for NetworkError {
    fn from(inner: anyhow::Error) -> NetworkError {
        NetworkError { inner }
    }
}

impl From<io::Error> for NetworkError {
    fn from(err: io::Error) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::IoError)
            .into()
    }
}

impl From<VerifyError> for NetworkError {
    fn from(err: VerifyError) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::SignatureError)
            .into()
    }
}

impl From<prost::EncodeError> for NetworkError {
    fn from(err: prost::EncodeError) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::ProtobufParseError)
            .into()
    }
}

impl From<prost::DecodeError> for NetworkError {
    fn from(err: prost::DecodeError) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::ProtobufParseError)
            .into()
    }
}

impl From<parity_multiaddr::Error> for NetworkError {
    fn from(err: parity_multiaddr::Error) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::MultiaddrError)
            .into()
    }
}

impl From<mpsc::SendError> for NetworkError {
    fn from(err: mpsc::SendError) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::MpscSendError)
            .into()
    }
}

impl From<oneshot::Canceled> for NetworkError {
    fn from(err: oneshot::Canceled) -> NetworkError {
        anyhow::Error::new(err)
            .context(NetworkErrorKind::OneshotCanceled)
            .into()
    }
}

impl From<PeerManagerError> for NetworkError {
    fn from(err: PeerManagerError) -> NetworkError {
        match err {
            PeerManagerError::IoError(_) => anyhow::Error::new(err)
                .context(NetworkErrorKind::IoError)
                .into(),
            PeerManagerError::NotConnected(_) => anyhow::Error::new(err)
                .context(NetworkErrorKind::NotConnected)
                .into(),
            err => anyhow::Error::new(err)
                .context(NetworkErrorKind::PeerManagerError)
                .into(),
        }
    }
}

impl From<tokio::time::Elapsed> for NetworkError {
    fn from(_err: tokio::time::Elapsed) -> NetworkError {
        NetworkErrorKind::TimedOut.into()
    }
}
