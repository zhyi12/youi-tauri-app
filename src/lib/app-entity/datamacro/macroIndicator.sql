
create table  stats_macro_indicator
(
    id    INTEGER      not null
        primary key autoincrement,

    text    TEXT ,
    pid    INTEGER ,
    num    real ,
    desc    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);