// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::convert::TryInto;

use anyhow::{bail, Result};
use common_arrow::arrow_flight::flight_service_client::FlightServiceClient;
use common_arrow::arrow_flight::Action;
use common_flights::store_do_action::{CreateDatabaseAction, StoreDoAction};
use common_planners::CreateDatabasePlan;
use tonic::Request;

pub struct StoreClient {
    client: FlightServiceClient<tonic::transport::channel::Channel>,
}

impl StoreClient {
    pub async fn try_create(addr: String) -> Result<Self> {
        let client = FlightServiceClient::connect(format!("http://{}", addr)).await?;
        Ok(Self { client })
    }

    pub async fn create_database(&mut self, plan: CreateDatabasePlan) -> Result<()> {
        let action = StoreDoAction::CreateDatabase(CreateDatabaseAction { plan });
        let _body = self.do_action(&action).await?;
        Ok(())
    }

    // Execute do_action.
    async fn do_action(&mut self, action: &StoreDoAction) -> Result<Vec<u8>> {
        let request: Request<Action> = action.try_into()?;
        let mut stream = self.client.do_action(request).await?.into_inner();
        match stream.message().await? {
            None => {
                bail!(
                    "Can not receive data from store flight server, action: {:?}",
                    action
                )
            }
            Some(resp) => Ok(resp.body),
        }
    }
}