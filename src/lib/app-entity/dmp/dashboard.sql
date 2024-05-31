
create table  youi_dmp_dashboard
(
    id    INTEGER      not null
        primary key autoincrement,

    group_id    INTEGER ,
    num    TEXT ,
    text    TEXT ,
    full_text    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);