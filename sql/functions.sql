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
            AND  metrics.page_id = pages.id
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
            AND  metrics.page_id = pages.id
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
        AND metrics.end_date IS NOT NULL;

        SELECT INTO total_bounce_counter
            COUNT(id)
        FROM metrics
        WHERE page_id = row.id
        AND DATE(date) BETWEEN $1 AND $2
        AND metrics.end_date IS NOT NULL
        AND EXTRACT(EPOCH FROM (metrics.end_date - date)) <= 2;
        
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
        AND DATE(date) BETWEEN $1 AND $2;

        title := row.title;
        seconds := avg_page_time;

        RETURN NEXT;
    END LOOP;
END;
$$ LANGUAGE PLPGSQL;