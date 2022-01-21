DROP TABLE IF EXISTS files CASCADE;
CREATE TABLE files (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(120),
    path VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email VARCHAR(255) NOT NULL UNIQUE,
    firstname VARCHAR(32) NOT NULL,
    lastname VARCHAR(32) NOT NULL,
    nickname VARCHAR(32),
    job VARCHAR(64) NOT NULL,
    password VARCHAR(255) NOT NULL,
    picture_id INT
        REFERENCES files (id)
        ON DELETE SET NULL,
    is_employed BOOLEAN,
    description VARCHAR(500),
    "order" SMALLINT -- Order of display on the agency page
);

DROP TABLE IF EXISTS pages CASCADE;
CREATE TABLE pages (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(255) NOT NULL,
    path VARCHAR(255) UNIQUE NOT NULL,
    description VARCHAR(320),
    is_seo BOOLEAN NOT NULL DEFAULT FALSE,
    is_visible BOOLEAN NOT NULL DEFAULT FALSE
);

DROP TABLE IF EXISTS metric_sessions CASCADE;
CREATE TABLE metric_sessions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    ip VARCHAR(120),
    expiration_date TIMESTAMP WITH TIME ZONE DEFAULT NOW() + interval '30 minutes' NOT NULL
);

DROP TABLE IF EXISTS metrics CASCADE;
CREATE TABLE metrics (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id uuid REFERENCES metric_sessions (id) ON DELETE SET NULL,
    page_id SMALLINT
        REFERENCES pages (id)
        ON DELETE SET NULL,
    project_id SMALLINT
        REFERENCES portfolio_projects (id)
        ON DELETE SET NULL,
    ip VARCHAR(120),
    browser VARCHAR(20),
    os VARCHAR(20),
    device_type VARCHAR(20),
    referer VARCHAR(255),
    "date" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    end_date TIMESTAMP WITH TIME ZONE
);

-- TODO : create hook to check if all fields are null, if true remove insert

DROP TABLE IF EXISTS ips_banned CASCADE;
CREATE TABLE ips_banned (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    ip VARCHAR(60) NOT NULL,
    "date" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

DROP TABLE IF EXISTS faq_categories CASCADE;
CREATE TABLE faq_categories (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(60) NOT NULL,
    "order" SMALLINT NOT NULL,
    UNIQUE (id, "order")
);

DROP TABLE IF EXISTS faq_answers CASCADE;
CREATE TABLE faq_answers (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category_id SMALLINT NOT NULL
        REFERENCES faq_categories (id)
        ON DELETE CASCADE,
    question VARCHAR(120) NOT NULL,
    answer TEXT NOT NULL,
    "order" SMALLINT NOT NULL
    -- UNIQUE (category_id, "order")
);

DROP TABLE IF EXISTS portfolio_categories CASCADE;
CREATE TABLE portfolio_categories (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(120) NOT NULL,
    "order" SMALLINT NOT NULL
);

DROP TABLE IF EXISTS portfolio_projects CASCADE;
CREATE TABLE portfolio_projects (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category_id SMALLINT NOT NULL
        REFERENCES portfolio_categories (id),
    name VARCHAR(120) NOT NULL,
    description VARCHAR(300),
    content TEXT NOT NULL,
    is_visible BOOLEAN DEFAULT FALSE,
    date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_update_date TIMESTAMP WITH TIME ZONE
);

DROP TABLE IF EXISTS portfolio_project_pictures CASCADE;
CREATE TABLE portfolio_project_pictures (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    project_id SMALLINT NOT NULL
        REFERENCES portfolio_projects (id)
        ON DELETE CASCADE,
    file_id INT NOT NULL
        REFERENCES files (id),
    "order" SMALLINT NOT NULL
);

DROP TABLE IF EXISTS blog_categories CASCADE;
CREATE TABLE blog_categories (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(60) NOT NULL,
    uri VARCHAR(70) NOT NULL,
    description VARCHAR(255),
    is_visible BOOLEAN,
    is_seo BOOLEAN,
    "order" SMALLINT NOT NULL
);

DROP TABLE IF EXISTS blog_posts CASCADE;
CREATE TABLE blog_posts (
    id SMALLINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    category_id SMALLINT
        REFERENCES blog_categories (id)
        ON DELETE SET NULL,
    cover_id INT NOT NULL
        REFERENCES files (id)
        ON DELETE SET NULL,
    user_id SMALLINT NOT NULL
        REFERENCES users (id),
    name VARCHAR(255) NOT NULL,
    description VARCHAR(320),
    content TEXT NOT NULL,
    uri VARCHAR(260) NOT NULL,
    "date" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    modified_date TIMESTAMP WITH TIME ZONE,
    is_published BOOLEAN DEFAULT FALSE,
    is_seo BOOLEAN DEFAULT FALSE
);

DROP TABLE IF EXISTS blog_post_images CASCADE;
CREATE TABLE blog_post_images (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    post_id SMALLINT NOT NULL
        REFERENCES blog_posts (id)
        ON DELETE CASCADE,
    file_id INT NOT NULL
        REFERENCES files (id)
);