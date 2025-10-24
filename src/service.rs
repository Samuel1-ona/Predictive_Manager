#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot, linera_base_types::{Amount, WithServiceAbi}, views::View, Service,
    ServiceRuntime,
};

use predictive_manager::Operation;
use predictive_manager::state::*;

pub struct PredictiveManagerService {
    state: PredictionMarketState,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(PredictiveManagerService);

impl WithServiceAbi for PredictiveManagerService {
    type Abi = predictive_manager::PredictiveManagerAbi;
}

impl Service for PredictiveManagerService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = PredictionMarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        PredictiveManagerService {
            state,
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            QueryRoot {
                total_supply: *self.state.total_supply.get(),
            },
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}

struct QueryRoot {
    total_supply: Amount,
}

#[Object]
impl QueryRoot {
    async fn total_supply(&self) -> &Amount {
        &self.total_supply
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_graphql::{Request, Response, Value};
    use futures::FutureExt as _;
    use linera_sdk::{linera_base_types::Amount, util::BlockingWait, views::View, Service, ServiceRuntime};
    use serde_json::json;

    use super::{PredictiveManagerService, PredictionMarketState};

    #[test]
    fn query() {
        let total_supply = Amount::from_tokens(1000);
        let runtime = Arc::new(ServiceRuntime::<PredictiveManagerService>::new());
        let mut state = PredictionMarketState::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store");
        state.total_supply.set(total_supply);

        let service = PredictiveManagerService { state, runtime };
        let request = Request::new("{ totalSupply }");

        let response = service
            .handle_query(request)
            .now_or_never()
            .expect("Query should not await anything");

        let expected = Response::new(Value::from_json(json!({"totalSupply": "1000000000000"})).unwrap());

        assert_eq!(response, expected)
    }
}
