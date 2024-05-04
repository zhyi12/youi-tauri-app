
create table  youi_dsl_script
(
    id    INTEGER      not null
        primary key autoincrement,

    text    TEXT ,
    pid    real ,
    num    real ,
    group    TEXT ,
    content  TEXT,
    col_widths TEXT,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);