DROP FUNCTION IF EXISTS unique_views_per_page(DATE, DATE);
CREATE FUNCTION unique_views_per_page(start_date DATE, end_date DATE)
RETURNS TABLE (title VARCHAR, views BIGINT) AS $$
BEGIN
    RETURN QUERY SELECT 
        pages.title,
        (
            SELECT COUNT(DISTINCT metrics.ip)
            FROM metrics
            WHERE DATE(date) BETWEEN $1 AND $2
            AND metrics.page_id = pages.id
            AND browser NOT IN (
                'LinkedInBot',
                'ZoominfoBot',
                'Baiduspider',
                'bingbot',
                'Applebot',
                'Python Requests',
                'MJ12bot',
                'Go-http-client',
                'Googlebot',
                'PetalBot',
                'FacebookBot',
                'SemrushBot',
                'AdsBot-Google',
                'AhrefsBot',
                'curl',
                'crawler',
                'BLEXBot',
                'Apache-HttpClient'
            )
        )
        FROM pages;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS views_per_page(DATE, DATE);
CREATE FUNCTION views_per_page(start_date DATE, end_date DATE)
RETURNS TABLE (title VARCHAR, views BIGINT) AS $$
BEGIN
    RETURN QUERY SELECT 
        pages.title,
        (
            SELECT COUNT(id)
            FROM metrics
            WHERE DATE(date) BETWEEN $1 AND $2
            AND metrics.page_id = pages.id
            AND browser NOT IN (
                'LinkedInBot',
                'ZoominfoBot',
                'Baiduspider',
                'bingbot',
                'Applebot',
                'Python Requests',
                'MJ12bot',
                'Go-http-client',
                'Googlebot',
                'PetalBot',
                'FacebookBot',
                'SemrushBot',
                'AdsBot-Google',
                'AhrefsBot',
                'curl',
                'crawler',
                'BLEXBot',
                'Apache-HttpClient'
            )
        )
        FROM pages;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS bounce_rate_per_page(DATE, DATE);
CREATE FUNCTION bounce_rate_per_page(start_date DATE, end_date DATE)
RETURNS TABLE (title VARCHAR, percent REAL) AS $$
DECLARE
    row RECORD;
    total_views_counter BIGINT;
    total_bounce_counter BIGINT;
BEGIN
    FOR row IN SELECT id, pages.title FROM pages
    LOOP
        SELECT INTO total_views_counter
            COUNT(id)
        FROM metrics
        WHERE page_id = row.id
        AND DATE(date) BETWEEN $1 AND $2
        AND metrics.end_date IS NOT NULL
        AND browser NOT IN (
            'LinkedInBot',
            'ZoominfoBot',
            'Baiduspider',
            'bingbot',
            'Applebot',
            'Python Requests',
            'MJ12bot',
            'Go-http-client',
            'Googlebot',
            'PetalBot',
            'FacebookBot',
            'SemrushBot',
            'AdsBot-Google',
            'AhrefsBot',
            'curl',
            'crawler',
            'BLEXBot',
            'Apache-HttpClient'
        );

        SELECT INTO total_bounce_counter
            COUNT(id)
        FROM metrics
        WHERE page_id = row.id
        AND DATE(date) BETWEEN $1 AND $2
        AND metrics.end_date IS NOT NULL
        AND EXTRACT(EPOCH FROM (metrics.end_date - date)) <= 2
        AND browser NOT IN (
            'LinkedInBot',
            'ZoominfoBot',
            'Baiduspider',
            'bingbot',
            'Applebot',
            'Python Requests',
            'MJ12bot',
            'Go-http-client',
            'Googlebot',
            'PetalBot',
            'FacebookBot',
            'SemrushBot',
            'AdsBot-Google',
            'AhrefsBot',
            'curl',
            'crawler',
            'BLEXBot',
            'Apache-HttpClient'
        );
        
        IF total_views_counter > 0 THEN
            title := row.title;
            percent := (total_bounce_counter::REAL / total_views_counter::REAL) * 100;

            RETURN NEXT;
        END IF;
    END LOOP;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS avg_time_page(DATE, DATE);
CREATE FUNCTION avg_time_page(start_date DATE, end_date DATE)
RETURNS TABLE (title VARCHAR, seconds REAL) AS $$
DECLARE
    row RECORD;
    avg_page_time REAL;
BEGIN
    FOR row IN SELECT id, pages.title FROM pages
    LOOP
        SELECT INTO avg_page_time
            EXTRACT(EPOCH FROM AVG(metrics.end_date - date))
        FROM metrics
        WHERE page_id = row.id
        AND metrics.end_date IS NOT NULL
        AND DATE(date) BETWEEN $1 AND $2
        AND browser NOT IN (
            'LinkedInBot',
            'ZoominfoBot',
            'Baiduspider',
            'bingbot',
            'Applebot',
            'Python Requests',
            'MJ12bot',
            'Go-http-client',
            'Googlebot',
            'PetalBot',
            'FacebookBot',
            'SemrushBot',
            'AdsBot-Google',
            'AhrefsBot',
            'curl',
            'crawler',
            'BLEXBot',
            'Apache-HttpClient'
        );

        title := row.title;
        seconds := avg_page_time;

        RETURN NEXT;
    END LOOP;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS unique_views(DATE, DATE);
CREATE FUNCTION unique_views(start_date DATE, end_date DATE)
RETURNS INT AS $$
DECLARE
    counter INT;
BEGIN
    SELECT INTO counter
        COUNT(DISTINCT ip)
        FROM metrics
        WHERE DATE(date) BETWEEN $1 AND $2
        AND browser NOT IN (
            'LinkedInBot',
            'ZoominfoBot',
            'Baiduspider',
            'bingbot',
            'Applebot',
            'Python Requests',
            'MJ12bot',
            'Go-http-client',
            'Googlebot',
            'PetalBot',
            'FacebookBot',
            'SemrushBot',
            'AdsBot-Google',
            'AhrefsBot',
            'curl',
            'crawler',
            'BLEXBot',
            'Apache-HttpClient'
        );

    RETURN counter;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS sites_origin(DATE, DATE);
CREATE FUNCTION sites_origin(start_date DATE, end_date DATE)
RETURNS TABLE (unique_visitors BIGINT, name VARCHAR) AS $$
BEGIN
    RETURN QUERY SELECT
        COUNT(DISTINCT ip),
        referer
        FROM metrics
        WHERE referer NOT ILIKE '%greenassembly.fr%'
        AND referer NOT ILIKE '%92.222.180.35%'
        AND DATE(date) BETWEEN $1 AND $2
        AND browser NOT IN (
            'LinkedInBot',
            'ZoominfoBot',
            'Baiduspider',
            'bingbot',
            'Applebot',
            'Python Requests',
            'MJ12bot',
            'Go-http-client',
            'Googlebot',
            'PetalBot',
            'FacebookBot',
            'SemrushBot',
            'AdsBot-Google',
            'AhrefsBot',
            'curl',
            'crawler',
            'BLEXBot',
            'Apache-HttpClient'
        )
        GROUP BY referer;
END;
$$ LANGUAGE PLPGSQL;