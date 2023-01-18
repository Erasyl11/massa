//! Copyright (c) 2022 MASSA LABS <info@massa.net>
//! Json RPC API for a massa-node
use std::{net::SocketAddr, pin::Pin};

use crate::config::GrpcConfig;
use massa_consensus_exports::ConsensusChannels;
use massa_pool_exports::PoolChannels;

pub mod massa {
    tonic::include_proto!("grpc.massa.protobuf");
}

use massa::massa_server::{Massa, MassaServer};
use tonic::codegen::futures_core;

/// Grpc API content
pub struct MassaService {
    /// link(channels) to the consensus component
    pub consensus_channels: ConsensusChannels,
    /// link(channels) to the pool component
    pub pool_channels: PoolChannels,
    /// gRPC settings
    pub grpc_settings: GrpcConfig,
    /// node version
    pub version: massa_models::version::Version,
}

impl MassaService {
    /// generate a new massa API
    pub fn new(
        consensus_channels: ConsensusChannels,
        pool_channels: PoolChannels,
        grpc_settings: GrpcConfig,
        version: massa_models::version::Version,
    ) -> Self {
        MassaService {
            consensus_channels,
            pool_channels,
            grpc_settings,
            version,
        }
    }

    async fn serve(
        service: MassaService,
        grpc_config: &GrpcConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let svc = MassaServer::new(service);
        tonic::transport::Server::builder()
            .add_service(svc)
            .serve(grpc_config.bind_grpc_api)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl Massa for MassaService {
    async fn get_version(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<massa::Version>, tonic::Status> {
        Ok(tonic::Response::new(massa::Version {
            version: self.version.to_string(),
        }))
    }

    type SendBlocksStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<massa::BlockId, tonic::Status>> + Send + 'static,
        >,
    >;

    async fn send_blocks(
        &self,
        request: tonic::Request<tonic::Streaming<massa::SendBlocksRequest>>,
    ) -> Result<tonic::Response<Self::SendBlocksStream>, tonic::Status> {
        unimplemented!()
    }

    type SendEndorsementsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<massa::EndorsementId, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn send_endorsements(
        &self,
        request: tonic::Request<tonic::Streaming<massa::SendEndorsementsRequest>>,
    ) -> Result<tonic::Response<Self::SendEndorsementsStream>, tonic::Status> {
        unimplemented!()
    }

    type SendOperationsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<massa::OperationId, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn send_operations(
        &self,
        request: tonic::Request<tonic::Streaming<massa::SendOperationsRequest>>,
    ) -> Result<tonic::Response<Self::SendOperationsStream>, tonic::Status> {
        unimplemented!()
    }
}
