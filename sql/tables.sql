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
    identifier VARCHAR(255) UNIQUE NOT NULL,
    description VARCHAR(320),
    is_seo BOOLEAN NOT NULL DEFAULT FALSE,
    is_visible BOOLEAN NOT NULL DEFAULT FALSE
);

DROP TABLE IF EXISTS metrics CASCADE;
CREATE TABLE metrics (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    page_id SMALLINT NOT NULL
        REFERENCES pages (id)
        ON DELETE CASCADE,
    ip VARCHAR(60),
    browser VARCHAR(20),
    os VARCHAR(20),
    device_type VARCHAR(20),
    referer VARCHAR(255),
    "date" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
-- TODO : create hook to check if all fields are null, if true remove insert

DROP TABLE IF EXITS ips_banned CASCADE;
CREATE TABLE ips_banned (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    ip VARCHAR(60) NOT NULL,
    "date" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);