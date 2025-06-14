CREATE TABLE views_queue
(
    post_id UInt64,
    creator_id UInt32,
    time DateTime
) ENGINE = Kafka('stats_broker:9092', 'views', 'clickhouse', 'JSONEachRow');
CREATE TABLE views
(
    post_id UInt64,
    creator_id UInt32,
    time DateTime
) ENGINE = MergeTree ORDER BY (post_id, creator_id);
CREATE MATERIALIZED VIEW views_mv TO views AS
SELECT *
FROM views_queue;
CREATE TABLE likes_queue
(
    post_id UInt64,
    creator_id UInt32,
    time DateTime
) ENGINE = Kafka('stats_broker:9092', 'likes', 'clickhouse', 'JSONEachRow');
CREATE TABLE likes
(
    post_id UInt64,
    creator_id UInt32,
    time DateTime
) ENGINE = MergeTree ORDER BY (post_id, creator_id);
CREATE MATERIALIZED VIEW likes_mv TO likes AS
SELECT *
FROM likes_queue;
CREATE TABLE comments_queue
(
    post_id UInt64,
    creator_id UInt32,
    comment String,
    time DateTime
) ENGINE = Kafka('stats_broker:9092', 'comments', 'clickhouse', 'JSONEachRow');
CREATE TABLE comments
(
    post_id UInt64,
    creator_id UInt32,
    comment String,
    time DateTime
) ENGINE = MergeTree ORDER BY (post_id, creator_id);
CREATE MATERIALIZED VIEW comments_mv TO comments AS
SELECT *
FROM comments_queue;
