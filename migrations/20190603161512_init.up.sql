CREATE TABLE IF NOT EXISTS media
(
    id UUID PRIMARY KEY,
    unique_identifier TEXT NOT NULL UNIQUE,
    data_url TEXT NULL,

    duration BIGINT NULL,
    used_in UUID NULL,

    use_status SMALLINT NOT NULL,
    social_network SMALLINT NOT NULL,
    media_type SMALLINT NOT NULL,

    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS links
(
    id UUID PRIMARY KEY,

    provider TEXT NOT NULL UNIQUE,
    media_limit SMALLINT NOT NULL,
    social_network SMALLINT NOT NULL,

    created_at BIGINT NOT NULL
);
