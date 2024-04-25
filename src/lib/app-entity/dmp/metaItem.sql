
create table  youi_dmp_meta_item
(
    id    INTEGER      not null
        primary key autoincrement,

    name    TEXT ,
    column_name    TEXT ,
    caption    TEXT ,
    full_caption    TEXT ,
    data_type    TEXT ,
    len    INTEGER ,
    code    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);