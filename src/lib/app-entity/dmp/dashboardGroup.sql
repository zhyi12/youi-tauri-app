
create table  youi_dmp_dashboard_group
(
    id    INTEGER      not null
        primary key autoincrement,

    pid    TEXT ,
    num    TEXT ,
    text    TEXT ,
    full_text    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);