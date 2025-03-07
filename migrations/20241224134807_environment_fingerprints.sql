-- Add migration script here
create table environment_fingerprints
(
    id                    integer
        primary key autoincrement,
    user_uuid             text,
    browser               text    not NULL,
    ua                    text    not null,
    os                    text    not null,
    country               TEXT,
    region                TEXT,
    city                  TEXT,
    language_type         integer not null,
    languages             text    not null,
    gmt                   text    not null,
    geography             text    not null,
    geo_tips              integer not null,
    geo_rule              integer not null,
    longitude             integer,
    latitude              integer,
    radius                integer,
    height                integer,
    width                 integer,
    fonts_type            integer not null,
    fonts                 text,
    font_fingerprint      integer not null,
    web_rtc               integer not null,
    web_rtc_local_ip      text,
    canvas                integer not null,
    webgl                 integer not null,
    hardware_acceleration integer not null,
    webgl_info            integer not null,
    audio_context         integer not null,
    speech_voices         integer not null,
    media                 integer not null,
    cpu                   integer not null,
    memory                integer not null,
    do_not_track          integer not null,
    battery               integer not null,
    port_scan             integer not null,
    white_list            text,
    created_at            datetime default current_timestamp,
    updated_at            datetime default current_timestamp,
    deleted_at            datetime,
    check (audio_context in (0, 1)),
    check (hardware_acceleration in (0, 1)),
    check (language_type in (0, 1, 2)),
    check (media in (0, 1)),
    check (memory in (0, 2, 4, 6, 8)),
    check (port_scan in (0, 1)),
    check (speech_voices in (0, 1)),
    check (web_rtc between 0 and 4),
    check (webgl in (0, 1)),
    check (webgl_info in (0, 1)),
    check (battery in (0, 1, 2)),
    check (canvas in (0, 1)),
    check (cpu in (0, 2, 4, 6, 8, 10, 12, 16)),
    check (do_not_track in (0, 1, 2)),
    check (font_fingerprint in (0, 1)),
    check (fonts_type in (0, 1)),
    check (geo_rule in (0, 1)),
    check (geo_tips in (0, 2)),
    foreign key (user_uuid) references users (uuid)
);