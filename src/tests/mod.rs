#[cfg(test)]
mod tests {
    use crate::{controllers, create_pool};
    use actix_web::{test, web, App};
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn integration_test_index() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(controllers::index)).await;
        let resp = test::TestRequest::get()
            .uri("/")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_agence() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(controllers::agency)).await;
        let resp = test::TestRequest::get()
            .uri("/agence")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_portfolio() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(controllers::portfolio),
        )
        .await;
        let resp = test::TestRequest::get()
            .uri("/portfolio")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_legals() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(controllers::legals)).await;
        let resp = test::TestRequest::get()
            .uri("/mentions-legales")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_sitemap() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(controllers::sitemap)).await;
        let resp = test::TestRequest::get()
            .uri("/sitemap.xml")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_robots() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(controllers::robots)).await;
        let resp = test::TestRequest::get()
            .uri("/robots.txt")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_contact() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("/contact").service(controllers::contact::page)),
        )
        .await;
        let resp = test::TestRequest::get()
            .uri("/contact")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn intesgration_test_send_message() {
        dotenv().ok();
        use serde_json::{json, Value};

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("/contact").service(controllers::contact::send)),
        )
        .await;
        let resp = test::TestRequest::post()
            .uri("/contact")
            .set_json(&json!({
                "new_project": false,
                "firstname": "Guillaume",
                "lastname": "Gueyraud",
                "compagny": Value::Null,
                "email": "g.gueyraud@greenassembly.fr",
                "phone": Value::Null,
                "services": Value::Null,
                "message": "Lorem ipsum dolor sit amet....",
                "budget": Value::Null,
                "found_by": Value::Null
            }))
            .send_request(&mut app)
            .await;

        // Should panic with status 500 in none production environement
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_website_creation() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(App::new().data(pool.clone()).service(
            web::scope("/creation-site-web").service(controllers::website::website_creation),
        ))
        .await;
        let resp = test::TestRequest::get()
            .uri("/creation-site-web")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_onepage() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("/creation-site-web").service(controllers::website::onepage)),
        )
        .await;
        let resp = test::TestRequest::get()
            .uri("/creation-site-web/onepage")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_showcase() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("/creation-site-web").service(controllers::website::showcase)),
        )
        .await;
        let resp = test::TestRequest::get()
            .uri("/creation-site-web/vitrine")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn integration_test_e_commerce() {
        dotenv().ok();

        let pool = create_pool().await.unwrap();
        let mut app =
            test::init_service(App::new().data(pool.clone()).service(
                web::scope("/creation-site-web").service(controllers::website::e_commerce),
            ))
            .await;
        let resp = test::TestRequest::get()
            .uri("/creation-site-web/e-commerce")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }
}
