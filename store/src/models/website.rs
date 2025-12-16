use crate::store::Store;
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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


impl Store {
    pub async fn create_website(
        &mut self,
        u_i: String,
        new_url: String,
        input_about: String,
    ) -> Result<Website, Error> {
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
            .get_result(&mut self.conn).await;

        match created_website {
            Ok(w) => Ok(w),
            Err(e) => Err(e),
        }
    }

    pub async fn get_website_details_hourly(
        &mut self,
        input_website_url: String,
        input_user_id: String,
        mut hours: String,
    ) -> Result<Vec<HourlyView>, Error> {
        use crate::schema::websites::dsl::*;

        if hours.trim().is_empty() {
            hours = "2 hours".to_string();
        }

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn).await?;

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
            .load::<HourlyView>(&mut self.conn).await?;

        Ok(results)
    }

    pub async fn get_website_details_daily(
        &mut self,
        input_website_url: String,
        input_user_id: String,
        mut days: String,
    ) -> Result<Vec<DailyView>, Error> {
        use crate::schema::websites::dsl::*;

        if days.trim().is_empty() {
            days = "2 day".to_string();
        }

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn).await?;

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
            .load::<DailyView>(&mut self.conn).await?;

        Ok(results)
    }

    pub async fn get_website_details_last_hour(
        &mut self,
        input_website_url: String,
        input_user_id: String,
    ) -> Result<Vec<MinuteView>, Error> {
        use crate::schema::websites::dsl::*;

        let _website_result = websites
            .filter(url.eq(&input_website_url))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn).await?;

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
            .load::<MinuteView>(&mut self.conn).await?;

        Ok(results)
    }

    pub async fn search_website(&mut self, input_url: &str) -> Result<Website, Error> {
        use crate::schema::websites::dsl::*;

        let found_website = websites
            .filter(url.eq(input_url))
            .select(Website::as_select())
            .first(&mut self.conn).await?;

        Ok(found_website)
    }

    pub async fn get_all_websites(
        &mut self,
    ) -> Result<Vec<(String, String, String, bool)>, diesel::result::Error> {
        use crate::schema::websites::dsl::*;

        let websites_result = websites
            .select((url, id, user_id, is_snippet_added))
            .load::<(String, String, String, bool)>(&mut self.conn).await?;

        Ok(websites_result)
    }

    pub async fn get_users_all_websites(&mut self, input_user_id: String) -> Result<Vec<Website>, Error> {
        use crate::schema::websites::dsl::*;

        let websites_result = websites
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .load(&mut self.conn).await?;

        Ok(websites_result)
    }

    pub async fn update_website_snippet(&mut self, input_website_url: &str) -> () {
        let query = r#"
        UPDATE websites SET is_snippet_added=TRUE where url = $1;
        "#;

        let _ = diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(input_website_url)
            .execute(&mut self.conn).await;
    }

    pub async fn get_per_page_views(&mut self, input_website: String) -> Result<Vec<TotalViewsPerPage>, Error> {
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

        let res = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(input_website).load::<TotalViewsPerPage>(&mut self.conn).await?;

        Ok(res)
    }

    pub async fn get_total_unique_users(&mut self, input_website: String) -> Result<TotalUniqueUsers, Error> {
        let query = r#"
        SELECT
        COUNT(DISTINCT visitor_id) AS unique_users
        FROM 
        page_visits 
        WHERE website = $1;
        "#;

        let res = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(input_website).get_result::<TotalUniqueUsers>(&mut self.conn).await?;

        Ok(res)
    }

    pub async fn get_total_views(&mut self, input_website: String) -> Result<TotalViews, Error> {
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
            .get_result::<TotalViews>(&mut self.conn).await?;
    
        Ok(res)
    }
    
}
