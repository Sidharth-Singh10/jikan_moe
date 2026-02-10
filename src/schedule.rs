// schedule.rs
use crate::{JikanClient, JikanError};
use crate::{enums::schedule::ScheduleFilter, response::Response, structs::anime::Anime};

impl JikanClient {
    pub async fn get_schedules(
        &self,
        filter: Option<ScheduleFilter>,
        kids: Option<bool>,
        sfw: Option<bool>,
        unapproved: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Response<Vec<Anime>>, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            params.push(format!("filter={}", f.as_str()));
        }

        if let Some(k) = kids {
            params.push(format!("kids={}", k));
        }

        if let Some(s) = sfw {
            params.push(format!("sfw={}", s));
        }

        if let Some(u) = unapproved
            && u {
                params.push("unapproved".to_string());
            }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/schedules{}", query)).await
    }
}
