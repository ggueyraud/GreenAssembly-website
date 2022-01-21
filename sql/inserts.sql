INSERT INTO users (email, firstname, lastname, job, password, is_employed, description, "order") VALUES
('g.gueyraud@greenassembly.fr', 'Guillaume', 'Gueyraud', 'Dirigeant • Développeur', 'lorem', true, 'Parce que la fiabilité d''un site web fait toute la différence, je mets un point d''honneur à créer des sites web sécurisés et performants, répondant en tout point à vos attentes.', 1),
('v.brehaut@greenassembly.fr', 'Vincent', 'Bréhaut', 'Dirigeant • Développeur', 'lorem', true, 'C''est un fait, les internautes accordent peu de temps à leurs recherches sur le web. D''où l''importance d''avoir un site web réactif, clair et intuitif, tout en réduisant au maximum l''impact écologique.', 2),
('hello@ludivinefarat.fr', 'Ludivine', 'Farat', 'Designer Graphique', 'lorem', true, 'J''ai à cœur de trouver ce petit plus qui fait que vous êtes vous et pas un autre, et je le retranscris dans l''ensemble de votre communication visuelle.', 3);

INSERT INTO files (path) VALUES
('gg.png'),
('vb.png'),
('lf.png');

INSERT INTO users (email, firstname, lastname, job, description, password, is_employed, "order") VALUES
('ggueyraud@greenassembly.fr', 'Guillaume', 'Gueyraud', 'Dirigeant • Développeur', 'Parce que la fiabilité d''un site web fait toute la différence, je mets un point d''honneur à créer des sites web sécurisés et performants, répondant en tout point à vos attentes.', '', true, 1),
('vbrehaut@greenassembly.fr', 'Vincent', 'Bréhaut', 'Dirigeant • Développeur', 'C''est un fait, les internautes accordent peu de temps à leurs recherches sur le web. D''où l''importance d''avoir un site web réactif, clair et intuitif, tout en réduisant au maximum l''impact écologique.', '', true, 2),
('hello@ludivinefarat.fr', 'Ludivine', 'Farat', 'Designer Graphique - Freelance', 'J''ai à cœur de trouver ce petit plus qui fait que vous êtes vous et pas un autre, et je le retranscris dans l''ensemble de votre communication visuelle.', '', true, 3);

INSERT INTO pages (title, path, description, is_seo) VALUES
('Accueil', '/', 'GreenAssembly l''Agence Digitale Verte spécialisée dans la création de site web éco-conçu sur-mesure, d''identité visuelle et de motion design à votre image.', true),
('Agence', '/agence-digitale-verte', 'Démarquez-vous sur le web avec un site éco-conçu sur mesure – Une identité visuelle unique et singulière – Des vidéos Motion Design créatives et originales.', true),
('Contact', '/contact', 'Notre agence web à Annecy est spécialisée dans la conception de sites internet sur-mesure, la création d''identité visuelle et le montage de vidéos Motion Design.', true),
('Création de site E-commerce', '/creation-site-web/e-commerce', 'Gagnez des parts de marché et vendez vos produits avec un site e-commerce éco-conçu - développé sur-mesure - unique - 100% sécurisé - optimisé SEO', true),
('Mentions légales', '/mentions-legales', 'Mentions légales de l''agence digitale verte GreenAssembly', true),
('Création de site OnePage', '/creation-site-web/onepage', 'Démarquez-vous avec un site OnePage à l''image de votre entreprise. Conseils et développement sur-mesure, éco-conception web, optez pour la performance.', true),
('Portfolio', '/portfolio', 'Site e-commerce, site vitrine, identité visuelle, motion design. Découvrez les projets originaux GreenAssembly – Agence digitale spécialisée éco-conception web.', true),
('Création de site Vitrine', '/creation-site-web/vitrine', 'Gagnez en visibilité et attirez des contacts qualifiés avec un site vitrine éco-conçu, design et performant 100% personnalisé. Développement sur-mesure et SEO.', true),
('Création de site web', '/creation-site-web', 'Démarquez-vous avec un site sur-mesure à votre image. Refonte ou création de site vitrine et e-commerce, faites le choix de la performance et de la singularité.', true),
('FAQ', '/faq', 'Toutes les réponses aux questions les plus fréquemment posées aux équipes de notre agence digitale verte synthétisées dans notre FAQ.', true),
('Blog', '/blog', 'Description à remplir', true);

INSERT INTO faq_categories (name, "order") VALUES
('Général', 1),
('Environnement', 2),
('Paiement • Garanties', 3),
('SEO', 4),
('Support • Maintenance', 5),
('Hébergement • Domaine', 6),
('Design • Développement', 7);

INSERT INTO faq_answers (category_id, question, answer, "order") VALUES
(1, 'Pourquoi devrai-je engager une agence Web ?', 'Bien que des solutions gratuites et clefs en main existent (tels que les services SaaS ou encore les CMS) pour créer vous-même votre site internet, si vous souhaitez un site web qui reflète l''âme de votre projet, qui soit sécurisé 🛡️ et qui ait un impact minime sur l''environnement 🌍 dans ce cas faites appel à une <a href="agence-digitale-verte" o-follow o-preload-once>agence web qualifiée</a>.<br /><br />Cela tombe bien nous répondons justement à ces critères ! 😎😁', 1),
(1, 'Est-il possible d''avoir une formation ?', 'Tout site internet conçu par <a href="agence-digitale-verte" o-follow o-preload-once>notre agence</a> est livré avec une formation d’environ une heure sur la prise en main des différentes fonctionnalités dont il est composé. Et parce que nous sommes précautionneux, une documentation écrite vous est fournie.', 2),
(1, 'Est-ce que je serai le propriétaire du site internet une fois la prestation terminée ?', 'Notre contrat de prestation de service nous engage à vous livrer le code source de votre site web, vous permettant ainsi en cas de nécessité de faire appel à une société tierce.', 3),
(1, 'Pouvez-vous modifier mon site internet développé par une autre agence ?', 'Peu probable 😮‍💨, il y a de grandes chances que ce dernier soit propulsé par un CMS ou repose sur une technologie datente. Ici nous préparons le futur 🚀, rejoignez-nous !', 4),
(1, 'Peut-on travailler à distance ?', 'Le numérique est un outil qui va au- delà des frontières géographiques 🌍, quelle que soit votre localisation en France, nous sommes en mesure de communiquer.<br />Le premier entretien nous permettra de déterminer ensemble votre besoin et les points suivants vous permettent de suivre l’évolution de votre projet ainsi que de l’adéquation avec vos attentes !', 5),
(1, 'Qui dois-je contacter, dans quel cas ?', 'En cas de question sur un projet en cours de réalisation avec l’équipe GreenAssembly remettez-vous en au chef de projet en charge de votre prestation, ce dernier étant le plus apte à gérer les échanges entre vous et les collaborateurs mobilisés sur votre projet.<br />Si votre question est de toute autre nature appelez 📱 notre standard au <a href="tel:+33650308591">06•50•30•85•91</a> ou bien envoyez-nous un message 📝 depuis la <a href="contact" o-follow o-preload-once>page de contact</a>.', 6),
(1, 'Avez-vous la possibilité d’accueillir des stagiaires ou des alternants ?', 'Nous avons à cœur de transmettre nos connaissances malheureusement la société n’a à ce jour pas de locaux. De fait, nous acceptons uniquement les étudiants dont l’établissement de formation accepte le fait d’effectuer le stage en télétravail et auxquels nous sommes certains d’apporter une réelle connaissance supplémentaire.', 7),
(1, 'Votre agence web fournit-elle des services en rédaction web ?', 'La rédaction web vous permet d’avoir un contenu de qualité compréhensible par l’internaute mais aussi chérie par les robots parcourant le web, le SEO n’est pas une simple histoire de sémantique !<br /><br />Envie d’avoir un contenu rédactionnel de qualité ? Nous serions heureux de vous communiquer le contact de la rédactrice web indépendante avec laquelle nous collaborons !', 8),
(1, 'Dois-je préparer mon contenu avant de contacter votre agence web ?', 'Notre rôle est de vous accompagner dans la conception de votre projet. Venez avec vos idées nous en discuterons ensemble ! 🗣️💡', 9),
(2, 'En quoi les sites-web développés par l’agence web GreenAssembly sont respectueux de l’environnement ?', 'GreenAssembly est engagée dans la maîtrise de l’impact environnemental des sites-web développés. 🛡️🌍<br />Cela se caractérise par le développement sur-mesure des fonctionnalités dont a besoin le client, l’utilisation de technologies peu gourmandes en ressources matériels, la limitation du poids des ressources (images, fichiers de définition de style, ..) et d’autres pratiques techniques sûrement peu passionnantes pour vous ! (Non ?)<br />En complément de cela nous participons à la reforestation en plantant un à plusieurs arbres  🌱 contribuant à la captation carbone des gaz à effets de serres émis par le site internet.', 1),
(3, 'Quels sont les coûts liés à l’entretien d’un site web ?', 'La conception d’un site internet est un coût fixe, cependant attention aux coûts récurrents à prendre en considération ! ⚠️<br />Coûts récurrents :<ul class="list_disc"><li>achat du nom de domaine (renouvelable une fois par an au minimum)</li><li>location mensuelle ou annuelle d’un serveur</li></ul>Simplifiez vous la vie avec notre formule de maintenance de site web tout en profitant de ses avantages, nous nous occupons de tout cela pour vous !', 2),
(3, 'Quels sont les moyens de paiement que vous acceptez et les modalités de paiement ?', 'Seuls les virements bancaires sont acceptés, le paiement est ventilée de la manière suivante :<ul class="list_disc"><li>50% à la signature du contrat débutant ainsi la procédure de création</li><li>30% pour débuter la phase de recettage</li><li>20% pour la livraison (dans le cadre d’une prestation de conception de site web vos accès vous seront communiqués à ce moment et vous bénéficierez d’une formation au site web conçu)</li></ul>', 2),
(4, 'Suite à la mise en production de mon site web ce dernier n''apparaît pas dans les résultats de Google, que faire ?', 'Les robots de Google (🤖 crawlers) écument le web en continu, ainsi il est très probable que ces derniers soient déjà passés sur votre site et aient connaissance de ce dernier. L’affichage des résultats sur Google est défini par un algorithme dit d’indexation : en des termes plus clairs de classement des résultats; cet algorithme peut mettre de quelques jours à plusieurs semaines à rendre effectif son résultat.<br />Si au terme d’une à plusieurs semaines vous ne voyez pas d’amélioration de positionnement, n''hésitez pas à <a href="contact" o-follow o-preload-once>nous contacter</a> ! 📱', 1),
(5, 'Je rencontre un problème technique, que faire ?', 'Appelez-nous 📱 au <a href="tel:+33650308591">06•50•30•85•91</a> ou envoyez-nous un email depuis le <a href="contact" o-follow o-preload-once>formulaire 📝 de contact</a> nous interviendrons dans les meilleurs délais afin de régler le problème technique.', 1),
(5, 'Est-ce que j’aurai facilement accès aux sauvegardes de mon site web ?', 'Le contrat de maintenance nous engage à effectuer des sauvegardes récurrentes des données contenues par votre site. Ainsi si vous avez la nécessité d’accéder aux sauvegardes  à l’instant T, des 7 dernières semaines ou bien du 1er des trois derniers mois.', 2),
(6, 'Proposez-vous des services d’hébergement de sites-web ?', 'En complément des sites-web nous conseillons à nos clients notre formule de maintenance, la sérénité d’avoir un site déployé sur un serveur adapté à accueillir la charge prévue et que les données soient sauvegardées dans un environnement sûr 🛡️.<br />En cas de dysfonctionnement du site le contrat de maintenance vous assure d’être dépanné sous 24h ouvrés. 🧑‍💻', 1),
(7, 'Êtes-vous en capacité de développer des fonctionnalités bien précises pour mon site web ?', 'Assurément ! 👌<br />Ayant développé sur-mesure l’ensemble du site web nous avons fait nôtres les lignes dont il est composé, ainsi à votre demande nous pouvons ajouter, modifier ou supprimer quelque contenu ou fonctionnalité que ce soit !', 1),
(7, 'Est-ce que le webdesign de mon site web sera personnalisé ?', 'La création d’expérience est au cœur de notre initiative de <a href="creation-site-web" o-follow o-preload-once>création de site internet</a>, de fait tout projet réalisé par <a href="agence-digitale-verte" o-follow o-preload-once>notre agence</a> passera entre les mains des nos talentueux collaborateurs designer graphique ✨ !', 2);

INSERT INTO blog_categories (name, uri, is_visible, "order") VALUES
('Lorem', 'lorem-1', true, 1),
('Ipsum', 'lorem-2', true, 2),
('Dolor', 'lorem-3', true, 3);

INSERT INTO blog_posts (cover_id, user_id, name, content, uri, is_published) VALUES
(1, 1, 'Lorem ipsum dolor sit amet', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.', 'lorem-1', true);