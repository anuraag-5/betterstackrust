use crate::store::Store;
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error, sql_types::Double};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum WebsiteStatus {
    UP,
    DOWN,
    UNKNOWN,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime,
    pub is_snippet_added: bool,
    pub about: String,
    pub plan_name: String,
}

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website_tick)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WebsiteTick {
    pub id: String,
    pub response_time_ms: i32,
    pub status: String,
    pub region: String,
    pub website_url: String,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct HourlyView {
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub hour: chrono::NaiveDateTime,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub views: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct Status {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub status: String
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct DailyView {
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub day: chrono::NaiveDateTime,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub views: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct MinuteView {
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub minute: chrono::NaiveDateTime,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub views: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct TotalViewsPerPage {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub page_path: String,

    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_views: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct TotalUniqueUsers {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub unique_users: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct TotalViews {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub total_views: i64,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct AvgRespTime {
    #[diesel(sql_type = diesel::sql_types::Nullable<Double>)]
    pub avg: Option<f64>,
}

#[derive(QueryableByName, Debug, Serialize, Deserialize)]
pub struct UptimePercentage {
    #[diesel(sql_type = diesel::sql_types::Nullable<Double>)]
    pub uptime_percent: Option<f64>,
}

impl Store {
    pub async fn create_website(
        &self,
        u_i: String,
        new_url: String,
        input_about: String,
    ) -> Result<Website, Error> {
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let new_website = Website {
            id: Uuid::new_v4().to_string(),
            url: new_url,
            time_added: Utc::now().naive_local(),
            user_id: u_i,
            is_snippet_added: false,
            about: input_about,
            plan_name: "Basic".to_owned(),
        };

        let created_website = diesel::insert_into(crate::schema::websites::table)
            .values(new_website)
            .returning(Website::as_returning())
            .get_result(&mut conn)
            .await;

        match created_website {
            Ok(w) => Ok(w),
            Err(e) => Err(e),
        }
    }

    pub async fn get_website_recent_status(
        &self,
        input_website_url: String,
        input_user_id: String,
    ) -> Result<Status, Error> {
        use crate::schema::websites::dsl::*;
    
        let mut conn = self.pool.get().await.map_err(|e| {
            println!("{}", e);
            Error::NotFound
        })?;
    
        // 🔐 Verify website ownership
        let _website = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(&input_user_id))
            .select(Website::as_select())
            .first(&mut conn)
            .await?;
    
        // 🔎 Fetch most recent tick
        let query = r#"
            SELECT status
            FROM website_tick
            WHERE website_url = $1
            ORDER BY "createdAt" DESC
            LIMIT 1;
        "#;
    
        let result = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(&input_website_url)
            .get_result::<Status>(&mut conn)
            .await;
    
        match result {
            Ok(s) => {
                Ok(s)
            }
            Err(_) => Ok(Status { status: "Unknown".into() })
        }
    }
    
    pub async fn get_website_details_hourly(
        &self,
        input_website_url: String,
        input_user_id: String,
        mut hours: String,
    ) -> Result<Vec<HourlyView>, Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        if hours.trim().is_empty() {
            hours = "2 hours".to_string();
        }

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut conn)
            .await?;

        let query = r#"
        SELECT 
            DATE_TRUNC('hour', visited_at) AS hour,
            COUNT(*) AS views
        FROM page_visits
        WHERE 
            website = $1
            AND visited_at >= (NOW() AT TIME ZONE 'UTC') - ($2::text)::interval
        GROUP BY hour
        ORDER BY hour;
        "#;

        let results = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website_url)
            .bind::<diesel::sql_types::Text, _>(hours)
            .load::<HourlyView>(&mut conn)
            .await?;

        Ok(results)
    }

    pub async fn get_website_details_daily(
        &self,
        input_website_url: String,
        input_user_id: String,
        mut days: String,
    ) -> Result<Vec<DailyView>, Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        if days.trim().is_empty() {
            days = "2 day".to_string();
        }

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut conn)
            .await?;

        let query = r#"
            SELECT 
                d.day,
                COALESCE(COUNT(pv.id), 0) AS views
            FROM 
                generate_series(
                    NOW() - ($2::text)::interval,
                    NOW(), 
                    INTERVAL '1 day'
                ) AS d(day)
            LEFT JOIN page_visits pv 
                ON DATE_TRUNC('day', pv.visited_at) = DATE_TRUNC('day', d.day)
                AND pv.website = $1
            GROUP BY d.day
            ORDER BY d.day;
        "#;

        let results = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website_url)
            .bind::<diesel::sql_types::Text, _>(days)
            .load::<DailyView>(&mut conn)
            .await?;

        Ok(results)
    }

    pub async fn get_website_details_last_hour(
        &self,
        input_website_url: String,
        input_user_id: String,
    ) -> Result<Vec<MinuteView>, Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut conn)
            .await?;

        // Query: generate a 1-minute series for the past 60 minutes
        let query = r#"
            SELECT 
                d.minute AS minute,
                COALESCE(COUNT(pv.id), 0) AS views
            FROM 
                generate_series(
                    NOW() - INTERVAL '1 hour',
                    NOW(),
                    INTERVAL '1 minute'
                ) AS d(minute)
            LEFT JOIN page_visits pv 
                ON DATE_TRUNC('minute', pv.visited_at) = DATE_TRUNC('minute', d.minute)
                AND pv.website = $1
            GROUP BY d.minute
            ORDER BY d.minute;
        "#;

        let results = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website_url)
            .load::<MinuteView>(&mut conn)
            .await?;

        Ok(results)
    }

    pub async fn search_website(&self, input_url: &str) -> Result<Website, Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let found_website = websites
            .filter(url.eq(input_url))
            .select(Website::as_select())
            .first(&mut conn)
            .await?;

        Ok(found_website)
    }

    pub async fn get_all_websites(
        &self,
    ) -> Result<Vec<(String, String, String, bool)>, diesel::result::Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let websites_result = websites
            .select((url, id, user_id, is_snippet_added))
            .load::<(String, String, String, bool)>(&mut conn)
            .await?;

        Ok(websites_result)
    }

    pub async fn get_users_all_websites(
        &self,
        input_user_id: String,
    ) -> Result<Vec<Website>, Error> {
        use crate::schema::websites::dsl::*;

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let websites_result = websites
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .load(&mut conn)
            .await?;

        Ok(websites_result)
    }

    pub async fn update_website_snippet(&self, input_website_url: &str) -> Result<(), Error> {

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"
        UPDATE websites SET is_snippet_added=TRUE where url = $1;
        "#;

        let _ = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website_url)
            .execute(&mut conn)
            .await;

        Ok(())
    }

    pub async fn get_per_page_views(
        &self,
        input_website: String,
    ) -> Result<Vec<TotalViewsPerPage>, Error> {

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"SELECT 
            page_path,
            COUNT(*) AS total_views
            FROM 
            page_visits
            WHERE
            website = $1
            GROUP BY 
            page_path
            ORDER BY 
            total_views DESC;
        "#;

        let res = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website)
            .load::<TotalViewsPerPage>(&mut conn)
            .await?;

        Ok(res)
    }

    pub async fn get_total_unique_users(
        &self,
        input_website: String,
    ) -> Result<TotalUniqueUsers, Error> {

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"
        SELECT
        COUNT(DISTINCT visitor_id) AS unique_users
        FROM 
        page_visits 
        WHERE website = $1;
        "#;

        let res = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website)
            .get_result::<TotalUniqueUsers>(&mut conn)
            .await?;

        Ok(res)
    }

    pub async fn get_total_views(&self, input_website: String) -> Result<TotalViews, Error> {

        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"
            SELECT 
                COUNT(*) AS total_views
            FROM 
                page_visits
            WHERE 
                website = $1;
        "#;

        let res = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website)
            .get_result::<TotalViews>(&mut conn)
            .await?;

        Ok(res)
    }

    pub async fn get_average_resp_time(&self, input_website: String) -> Result<AvgRespTime, Error>{
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"
        SELECT AVG(response_time_ms)::DOUBLE PRECISION AS avg
        FROM website_tick
        WHERE website_url = $1;
        "#;

        let result: AvgRespTime = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(input_website).get_result::<AvgRespTime>(&mut conn).await?;

        return Ok(result);
    }

    pub async fn get_average_resp_time_by_region(&self, input_website: String, input_region: String) -> Result<AvgRespTime, Error>{
        
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;

        let query = r#"
        SELECT AVG(response_time_ms)::DOUBLE PRECISION AS avg
        FROM website_tick
        WHERE website_url = $1 
        AND region = $2;
        "#;

        let result: AvgRespTime = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(input_website).bind::<diesel::sql_types::Text, _>(input_region).get_result::<AvgRespTime>(&mut conn).await?;

        return Ok(result);
    }

    pub async fn get_average_uptime_percentage(
        &self,
        input_website: String,
    ) -> Result<UptimePercentage, Error> {
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;
        let query = r#"
            SELECT 
                (COUNT(*) FILTER (WHERE status = 'Up') * 100.0 / NULLIF(COUNT(*), 0))::DOUBLE PRECISION
                AS uptime_percent
            FROM website_tick
            WHERE website_url = $1;
        "#;
    
        let result: UptimePercentage = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website)
            .get_result::<UptimePercentage>(&mut conn)
            .await?;
    
        Ok(result)
    }
    
    pub async fn get_average_uptime_percentage_by_region(
        &self,
        input_website: String,
        input_region: String
    ) -> Result<UptimePercentage, Error> {
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;
        let query = r#"
            SELECT 
                (COUNT(*) FILTER (WHERE status = 'Up') * 100.0 / NULLIF(COUNT(*), 0))::DOUBLE PRECISION
                AS uptime_percent
            FROM website_tick
            WHERE website_url = $1 AND region = $2;
        "#;
    
        let result: UptimePercentage = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website)
            .bind::<diesel::sql_types::Text, _>(input_region)
            .get_result::<UptimePercentage>(&mut conn)
            .await?;
    
        Ok(result)
    }
    
}
