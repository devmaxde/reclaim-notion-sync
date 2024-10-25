use crate::config::{DatabaseConfig, SyncConfig, SyncPriority};
use notion_client::endpoints::databases::query::request::{
    QueryDatabaseRequest, Sort, SortDirection, Timestamp,
};
use notion_client::endpoints::Client;
use notion_client::objects::page::Page;
use notion_client::NotionClientError;
use std::cmp::PartialEq;
use tracing::{error, info};
use crate::page::PageExtended;

static NOTION_KEY_REQUIREMENTS: [(&str); 10] = [
    "priority",
    "time_needed",
    "min_duration",
    "max_duration",
    "schedule_after",
    "due_date",
    "notes",
    "visibility",
    "reclaim_id",
    "snc_information",
];

pub struct DatabaseSync {
    config: SyncConfig,
    notion_client: Client,
}

impl DatabaseSync {
    pub fn new(config: SyncConfig) -> DatabaseSync {
        let notion_api_key = config.notion_api_key.clone();
        Self {
            config,
            notion_client: Client::new(notion_api_key, None)
                .expect("Can't connect to the Notion API"),
        }
    }

    pub async fn sync(&self) {
        info!("Syncing tasks");
        for db in &self.config.databases {
            self.sync_database(db).await;
        }
    }

    async fn sync_database(&self, database: &DatabaseConfig) {
        if database.sync_priority == SyncPriority::RECLAIM {
            todo!("Currently unsupported")
        }

        let entries = self
            .get_notion_db_entries(&database.notion_database_id)
            .await;

        let entries = entries.unwrap();
        for i in entries.iter() {
            if !i.validate() {
                println!("Invalid notion page {}", i.id);
            };

            if i.get_notion_id().is_none(){
                self.create_reclaim_task(i).await;
            } else {
                self.sync_reclaim_task(i).await;
            }
        }

        // TODO implement Deletion of old Tasks
    }

    async fn create_reclaim_task(&self, _page: &Page) {
        todo!()
    }

    async fn sync_reclaim_task(&self, _page: &Page) {
        todo!()
    }

    async fn delete_reclaim_task(&self) {
        todo!("Not implemented")
    }

    async fn get_notion_db_entries(
        &self,
        notion_db_id: &str,
    ) -> Result<Vec<Page>, NotionClientError> {
        let request = QueryDatabaseRequest {
            sorts: Some(vec![Sort::Timestamp {
                timestamp: Timestamp::CreatedTime,
                direction: SortDirection::Ascending,
            }]),
            ..Default::default()
        };

        // Send request
        let res = self
            .notion_client
            .databases
            .query_a_database(notion_db_id, request)
            .await;

        let res = res?;

        Ok(res.results)
    }
}
