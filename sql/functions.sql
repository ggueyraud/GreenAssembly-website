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
                'Apache-HttpClient',
                'Discordbot'
            )
        )
        FROM pages
        UNION
        SELECT
            name,
            (
                SELECT COUNT(DISTINCT metrics.ip)
                FROM metrics
                WHERE DATE(date) BETWEEN $1 AND $2
                AND metrics.project_id = portfolio_projects.id
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
                    'Apache-HttpClient',
                    'Discordbot'
                )
            )
            FROM portfolio_projects;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS views_per_page(DATE, DATE);
CREATE FUNCTION views_per_page(start_date DATE DEFAULT CURRENT_DATE, end_date DATE DEFAULT CURRENT_DATE)
RETURNS TABLE ("date" DATE, title VARCHAR, views BIGINT) AS $$
BEGIN
    RETURN QUERY SELECT
        day::DATE,
        p.title,
        (
            SELECT COUNT(id)
            FROM metrics
            WHERE DATE(metrics.date) = day::DATE
            AND metrics.page_id = p.id
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
                'Apache-HttpClient',
                'Discordbot'
            )
        )
    FROM generate_series($1::timestamp, $2, '1 day') day,
    pages p;

    -- RETURN QUERY SELECT 
    --     pages.title,
    --     (
    --         SELECT COUNT(id)
    --         FROM metrics
    --         WHERE DATE(date) BETWEEN $1 AND $2
    --         AND metrics.page_id = pages.id
    --         AND browser NOT IN (
    --             'LinkedInBot',
    --             'ZoominfoBot',
    --             'Baiduspider',
    --             'bingbot',
    --             'Applebot',
    --             'Python Requests',
    --             'MJ12bot',
    --             'Go-http-client',
    --             'Googlebot',
    --             'PetalBot',
    --             'FacebookBot',
    --             'SemrushBot',
    --             'AdsBot-Google',
    --             'AhrefsBot',
    --             'curl',
    --             'crawler',
    --             'BLEXBot',
    --             'Apache-HttpClient',
    --             'Discordbot'
    --         )
    --     )
    --     FROM pages;
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
            'Apache-HttpClient',
            'Discordbot'
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
            'Apache-HttpClient',
            'Discordbot'
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
            'Apache-HttpClient',
            'Discordbot'
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
            'Apache-HttpClient',
            'Discordbot'
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
            'Apache-HttpClient',
            'Discordbot'
        )
        GROUP BY referer;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS avg_pages_session(DATE, DATE);
CREATE FUNCTION avg_pages_session(start_date DATE, end_date DATE)
RETURNS INT AS $$
DECLARE
    counter INT;
BEGIN
    SELECT INTO counter AVG(count)
        FROM (
            SELECT
                COUNT(id) as count
            FROM metrics
            WHERE DATE(date) BETWEEN $1 AND $2
            AND session_id IS NOT NULL
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
                'Apache-HttpClient',
                'Discordbot'
            )
            GROUP BY session_id
        ) as counts;

    RETURN counter;
END;
$$ LANGUAGE PLPGSQL;

DROP FUNCTION IF EXISTS sessions(DATE, DATE);
CREATE FUNCTION sessions(start_date DATE DEFAULT CURRENT_DATE, end_date DATE DEFAULT CURRENT_DATE)
RETURNS INT AS $$
DECLARE
    counter INT;
BEGIN
    SELECT INTO COUNTER
        COUNT(id)
    FROM metric_sessions
    WHERE expiration_date BETWEEN $1 AND $2;

    RETURN counter;
END;
$$ LANGUAGE PLPGSQL;