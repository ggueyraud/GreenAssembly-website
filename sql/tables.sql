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
    quote VARCHAR(255),
    /* drawing_order nous permettra de définir l'ordre d'affichage,
       il suffira de trier avec ce champ par ordre croissant */
    drawing_order SMALLINT DEFAULT 0,

    UNIQUE(firstname, lastname)
);