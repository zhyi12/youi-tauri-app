
create table  youi_geo_json
(
    id    INTEGER      not null
        primary key autoincrement,

    area_id    TEXT ,
    geo_json    TEXT ,
    version    TEXT ,
    rect    TEXT ,
    center    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT ,
);