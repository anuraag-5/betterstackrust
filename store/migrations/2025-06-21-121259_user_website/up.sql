-- 1. Create table: plan
CREATE TABLE "plan" (
    "id" TEXT NOT NULL,
    "name" TEXT UNIQUE NOT NULL,
    "price" TEXT NOT NULL,
    CONSTRAINT "Plan_pkey" PRIMARY KEY ("id")
);

-- 2. Create table: users
CREATE TABLE "users" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "email" TEXT UNIQUE NOT NULL,
    "password" TEXT NOT NULL,
    "plan_name" TEXT NOT NULL,
    CONSTRAINT "Users_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "users_plan_name_fkey"
        FOREIGN KEY ("plan_name") REFERENCES "plan"("name")
        ON DELETE RESTRICT ON UPDATE CASCADE
);

-- 3. Create table: region
CREATE TABLE "region" (
    "id" TEXT NOT NULL,
    "name" TEXT UNIQUE NOT NULL,
    CONSTRAINT "Region_pkey" PRIMARY KEY ("id")
);

-- 4. Create table: websites
CREATE TABLE "websites" (
    "id" TEXT NOT NULL,
    "url" TEXT UNIQUE NOT NULL,
    "time_added" TIMESTAMP(3) NOT NULL,
    "user_id" TEXT NOT NULL,
    "is_snippet_added" BOOLEAN NOT NULL DEFAULT FALSE,
    "about" TEXT NOT NULL,
    "plan_name" TEXT NOT NULL,
    CONSTRAINT "Websites_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "websites_user_id_fkey"
        FOREIGN KEY ("user_id") REFERENCES "users"("id")
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "websites_plan_name_fkey"
        FOREIGN KEY ("plan_name") REFERENCES "plan"("name")
        ON DELETE RESTRICT ON UPDATE CASCADE
);

-- 5. Create table: website_tick
CREATE TABLE "website_tick" (
    "id" TEXT NOT NULL,
    "response_time_ms" INTEGER NOT NULL,
    "status" TEXT NOT NULL,
    "region_id" TEXT NOT NULL,
    "website_url" TEXT NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "WebsiteTick_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_tick_region_id_fkey"
        FOREIGN KEY ("region_id") REFERENCES "region"("id")
        ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "website_tick_website_url_fkey"
        FOREIGN KEY ("website_url") REFERENCES "websites"("url")
        ON DELETE RESTRICT ON UPDATE CASCADE
);

-- 6. Create table: page_visits
CREATE TABLE "page_visits" (
    "id" BIGSERIAL PRIMARY KEY,
    "website" TEXT NOT NULL REFERENCES "websites"("url") ON DELETE CASCADE,
    "visitor_id" TEXT NOT NULL,
    "page_path" TEXT NOT NULL,
    "referrer" TEXT NOT NULL,
    "user_agent" TEXT NOT NULL,
    "visited_at" TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP
);