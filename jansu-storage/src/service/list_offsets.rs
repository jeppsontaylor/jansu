// Copyright ⓒ 2024-2025 Peter Morgan <peter.james.morgan@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref as _,
};

use jansu_sans_io::{
    ApiKey, ErrorCode, IsolationLevel, ListOffset, ListOffsetsRequest, ListOffsetsResponse,
    list_offsets_response::{ListOffsetsPartitionResponse, ListOffsetsTopicResponse},
};
use rama::{Context, Service};
use tracing::{debug, error, instrument};

use super::leader_epoch::{leader_epoch_history, leader_epoch_or_unknown};
use crate::{Error, Result, Storage, Topition};

/// A [`Service`] using [`Storage`] as [`Context`] taking [`ListOffsetsRequest`] returning [`ListOffsetsResponse`].
/// ```
/// use rama::{Context, Layer as _, Service, layer::MapStateLayer};
/// use jansu_sans_io::{
///     ErrorCode, IsolationLevel, ListOffset, ListOffsetsRequest,
///     list_offsets_request::{ListOffsetsPartition, ListOffsetsTopic},
/// };
/// use jansu_storage::{Error, ListOffsetsService, StorageContainer};
/// use url::Url;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Error> {
/// const HOST: &str = "localhost";
/// const PORT: i32 = 9092;
/// const NODE_ID: i32 = 111;
///
/// let storage = StorageContainer::builder()
///     .cluster_id("jansu")
///     .node_id(NODE_ID)
///     .advertised_listener(Url::parse(&format!("tcp://{HOST}:{PORT}"))?)
///     .storage(Url::parse("memory://jansu/")?)
///     .build()
///     .await?;
///
/// let service = MapStateLayer::new(|_| storage).into_layer(ListOffsetsService);
///
/// let topic = "abcba";
///
/// let response = service
///     .serve(
///         Context::default(),
///         ListOffsetsRequest::default()
///             .isolation_level(Some(IsolationLevel::ReadUncommitted.into()))
///             .replica_id(NODE_ID)
///             .topics(Some(
///                 [ListOffsetsTopic::default()
///                     .name(topic.into())
///                     .partitions(Some(
///                         [ListOffsetsPartition::default()
///                             .current_leader_epoch(Some(-1))
///                             .max_num_offsets(Some(3))
///                             .partition_index(0)
///                             .timestamp(ListOffset::Earliest.try_into()?)]
///                         .into(),
///                     ))]
///                 .into(),
///             )),
///     )
///     .await?;
///
/// let topics = response.topics.as_deref().unwrap_or_default();
/// assert_eq!(1, topics.len());
/// assert_eq!(topic, topics[0].name);
///
/// let partitions = topics[0].partitions.as_deref().unwrap_or_default();
/// assert_eq!(1, partitions.len());
/// assert_eq!(0, partitions[0].partition_index);
/// assert!(partitions[0].old_style_offsets.is_none());
/// assert_eq!(
///     ErrorCode::None,
///     ErrorCode::try_from(partitions[0].error_code)?
/// );
/// assert_eq!(Some(-1), partitions[0].timestamp);
/// assert_eq!(Some(0), partitions[0].offset);
/// assert_eq!(Some(0), partitions[0].leader_epoch);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListOffsetsService;

impl ApiKey for ListOffsetsService {
    const KEY: i16 = ListOffsetsRequest::KEY;
}

impl<G> Service<G, ListOffsetsRequest> for ListOffsetsService
where
    G: Storage,
{
    type Response = ListOffsetsResponse;
    type Error = Error;

    #[instrument(skip(ctx, req))]
    async fn serve(
        &self,
        ctx: Context<G>,
        req: ListOffsetsRequest,
    ) -> Result<Self::Response, Self::Error> {
        let throttle_time_ms = Some(0);

        let isolation_level = req
            .isolation_level
            .map_or(Ok(IsolationLevel::ReadUncommitted), |isolation_level| {
                IsolationLevel::try_from(isolation_level)
            })?;

        let topics = if let Some(topics) = req.topics {
            let mut offsets = vec![];

            for topic in topics {
                if let Some(ref partitions) = topic.partitions {
                    for partition in partitions {
                        let tp = Topition::new(topic.name.clone(), partition.partition_index);
                        let offset = ListOffset::try_from(partition.timestamp)?;

                        offsets.push((tp, offset));
                    }
                }
            }

            let offsets = ctx
                .state()
                .list_offsets(isolation_level, offsets.deref())
                .await
                .inspect(|r| debug!(?r, ?offsets))
                .inspect_err(|err| error!(?err, ?offsets))?;

            let mut leader_epochs = BTreeMap::new();
            for (topition, offset) in &offsets {
                if offset.error_code() == ErrorCode::None && !leader_epochs.contains_key(topition) {
                    let history = leader_epoch_history(&ctx, topition)
                        .await
                        .inspect_err(|err| debug!(?err, ?topition))
                        .unwrap_or_default();
                    _ = leader_epochs.insert(topition.clone(), history);
                }
            }

            let topic_names: BTreeSet<_> = offsets
                .iter()
                .map(|(topition, _)| topition.topic())
                .collect();

            let topics: Vec<_> = topic_names
                .iter()
                .map(|topic_name| {
                    ListOffsetsTopicResponse::default()
                        .name((*topic_name).into())
                        .partitions(Some(
                            offsets
                                .iter()
                                .filter_map(|(topition, offset)| {
                                    if topition.topic() == *topic_name {
                                        let epoch = if offset.error_code() == ErrorCode::None {
                                            leader_epochs
                                                .get(topition)
                                                .map(|history| leader_epoch_or_unknown(history))
                                                .unwrap_or(-1)
                                        } else {
                                            -1
                                        };
                                        Some(
                                            ListOffsetsPartitionResponse::default()
                                                .partition_index(topition.partition())
                                                .error_code(offset.error_code().into())
                                                .old_style_offsets(None)
                                                .timestamp(
                                                    offset
                                                        .timestamp()
                                                        .unwrap_or(Some(-1))
                                                        .or(Some(-1)),
                                                )
                                                .offset(offset.offset().or(Some(-1)))
                                                .leader_epoch(Some(epoch)),
                                        )
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        ))
                })
                .collect();

            Some(topics)
        } else {
            None
        };

        Ok(ListOffsetsResponse::default()
            .throttle_time_ms(throttle_time_ms)
            .topics(topics))
        .inspect(|r| debug!(?r))
    }
}
