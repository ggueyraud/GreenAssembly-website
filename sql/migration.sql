DROP TABLE IF EXISTS files CASCADE;
CREATE TABLE files (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(120),
    path VARCHAR(255) NOT NULL
);

ALTER TABLE metrics ALTER COLUMN page_id DROP NOT NULL;
ALTER TABLE metrics DROP CONSTRAINT metrics_page_id_fkey;
ALTER TABLE metrics
ADD CONSTRAINT metrics_page_id_fkey FOREIGN KEY (page_id) REFERENCES pages (id) ON DELETE SET NULL;
ALTER TABLE metrics
ADD COLUMN project_id SMALLINT REFERENCES portfolio_projects (id) ON DELETE SET NULL;

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

INSERT INTO portfolio_categories (name, "order") VALUES ('Site-web', 1);

INSERT INTO portfolio_projects (category_id, name, content, is_visible) VALUES
(1, 'Woden', 'Woden est un concept d’application en SaaS de gestion (facturation, stocks, prestations de service, ..) <strong>développé sur-mesure par l’agence GreenAssembly</strong>.<br /><br />

Le projet est découpé en trois sous-ensemble de projet :
<ul>
    <li>une application web</li>
    <li>un <a o-follow o-preload-once href="/creation-site-web/vitrine">site vitrine</a></li>
    <li>une API restful</li>
</ul>
<br /><br />
Développé avec le couple VueJS et NodeJS le projet réponds à un certains nombres de contraintes techniques :
<ul>
    <li>Haute disponibilité du service</li>
    <li>Réplication en temps réel des changements entre les clients connectés (websocket)</li>
    <li>Système de paiement de consommable et gestion d’abonnement au service (Stripe)</li>
    <li>Multilingue</li>
    <li>Interconnexion avec des API tierces (Stripe, OVH, géolocalisation d’IP)</li>
</ul>
<br /><br />
L’application web Woden est développée avec VueJS et le site vitrine est développé en NuxtJS/NodeJS, toutes deux communiquent avec l’API.
La partie graphique a été imaginée par GreenAssembly et le logo a été réalisé par <a href="https://laelian67.wixsite.com/portfolio" target="_blank">Laélia Nadolski</a> que nous remercions !', false),
(1, 'Ludivine Farat', 'Designeuse graphique toujours partante pour relever de nouveaux défis, Ludivine Farat vous accompagne dans la <strong>création d''identité visuelle</strong> de votre marque !<br /><br />
Ludivine souhaitait un site sur lequel présenter ses créations, ses prestations de services afin d’attirer ses prospects et qu’ils puissent la contacter.<br />
La conception graphique entièrement réalisée par ses soins, Ludivine a chargé notre <strong>GreenAssembly, agence spécialisée en éco-conception</strong>, du <a o-follow o-preload-once href="/creation-site-web/vitrine">développement sur-mesure de son site vitrine</a>.
<br /><br />
Un besoin en création graphique ? 🧑🏻‍🎨<br />
Jettez un coup d''oeil à son nouveau <strong>site internet à faible impact écologique</strong> ! 👉 <a href="https://ludivinefarat.fr/" target="_blank">ludivinefarat.fr</a>', true);

INSERT INTO files (path) VALUES
('woden0.png'),
('woden1.jpg'),
('woden2.png'),
('woden3.png'),
('woden4.png'),
('farat0.jpg'),
('farat1.jpg'),
('farat2.png'),
('farat3.png');

INSERT INTO portfolio_project_pictures (project_id, file_id, "order") VALUES
(1, 1, 0),
(1, 2, 1),
(1, 3, 2),
(1, 4, 3),
(1, 5, 4),
(2, 6, 0),
(2, 7, 1),
(2, 8, 2),
(2, 9, 3);