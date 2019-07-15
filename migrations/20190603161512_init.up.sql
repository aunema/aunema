CREATE TABLE IF NOT EXISTS media
(
    id UUID PRIMARY KEY,
    unique_identifier STRING NOT NULL UNIQUE,
    data_url STRING NULL,

    duration INT8 NULL,
    used_in UUID NULL,

    use_status INT2 NOT NULL,
    social_network INT2 NOT NULL,
    media_type INT2 NOT NULL,

    created_at INT8 NOT NULL
);

CREATE TABLE IF NOT EXISTS links
(
    id UUID PRIMARY KEY,

    provider STRING NOT NULL UNIQUE,
    media_limit INT2 NOT NULL,
    social_network INT2 NOT NULL,

    created_at INT8 NOT NULL
);

CREATE TABLE IF NOT EXISTS publishers
(
    id UUID PRIMARY KEY,
    chat_id INT8 NOT NULL,

    social_network INT2 NOT NULL,
    supported_media INT2[] NOT NULL,

    caption STRING NULL,
    repeats STRING[] NOT NULL,

    created_at INT8 NOT NULL
);
