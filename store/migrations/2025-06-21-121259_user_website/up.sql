-- Your SQL goes here
-- CreateEnum

-- CreateTable
CREATE TABLE "website" (
    "id" TEXT NOT NULL,
    "url" TEXT UNIQUE NOT NULL,
    "time_added" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Website_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "region" (
    "id" TEXT NOT NULL,
    "name" TEXT UNIQUE NOT NULL,

    CONSTRAINT "Region_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "website_tick" (
    "id" TEXT NOT NULL,
    "response_time_ms" INTEGER NOT NULL,
    "status" TEXT NOT NULL,
    "region_id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,

    CONSTRAINT "WebsiteTick_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "website_tick" ADD CONSTRAINT "website_tick_region_id_fkey" FOREIGN KEY ("region_id") REFERENCES "region"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "website_tick" ADD CONSTRAINT "website_tick_website_id_fkey" FOREIGN KEY ("website_id") REFERENCES "website"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "website" ADD COLUMN     "user_id" TEXT NOT NULL;

-- CreateTable
CREATE TABLE "user" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
-- 1. Add FK before renaming "user"
ALTER TABLE "website" 
ADD CONSTRAINT "website_user_id_fkey" 
FOREIGN KEY ("user_id") REFERENCES "user"("id") 
ON DELETE RESTRICT ON UPDATE CASCADE;

-- 2. Add column with default timestamp
ALTER TABLE "website_tick" 
ADD COLUMN "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- 3. Rename table from "user" to "users"
ALTER TABLE "user" RENAME TO "users";

-- 4. Add unique constraint on email
ALTER TABLE "users" 
ADD CONSTRAINT "U_email" UNIQUE ("email");

CREATE TABLE page_visits (
    id BIGSERIAL PRIMARY KEY,
    website_id TEXT REFERENCES website(id) ON DELETE CASCADE NOT NULL,
    visitor_id TEXT NOT NULL,
    page_url TEXT NOT NULL,
    referrer TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    visited_at TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE "website" 
ADD COLUMN  "is_snippet_added" BOOLEAN DEFAULT FALSE;

ALTER TABLE "website"
ALTER COLUMN "is_snippet_added" SET NOT NULL;

ALTER TABLE "website" RENAME TO "websites";
ALTER TABLE "websites" 
ADD COLUMN  "about" TEXT NOT NULL;