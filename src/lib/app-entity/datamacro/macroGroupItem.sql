
create table  stats_macro_group_item
(
    id    INTEGER      not null
        primary key autoincrement,

    text    TEXT ,
    pid    INTEGER ,
    group_id    INTEGER ,
    num    real ,
    desc    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);