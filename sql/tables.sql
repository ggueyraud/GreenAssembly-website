DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    login VARCHAR(64) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL
);

DROP TABLE IF EXISTS employees CASCADE;
CREATE TABLE employees (
    /* On utilisera l'id pour le nom de l'avatar,
       donc pas besoin de créer un champ pour */
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    firstname VARCHAR(32) NOT NULL,
    lastname VARCHAR(32) NOT NULL,
    job VARCHAR(64) NOT NULL,
    description VARCHAR(500),
    picture VARCHAR(32) NOT NULL,
    /* drawing_order nous permettra de définir l'ordre d'affichage,
       il suffira de trier avec ce champ par ordre croissant */
    "order" SMALLINT DEFAULT 0
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
    ip VARCHAR(60),
    valid_until_date TIMESTAMP WITH TIME ZONE DEFAULT NOW() + interval '30 minutes'
);

DROP TABLE IF EXISTS metrics CASCADE;
CREATE TABLE metrics (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    page_id SMALLINT NOT NULL
        REFERENCES pages (id)
        ON DELETE CASCADE,
    ip VARCHAR(60),
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