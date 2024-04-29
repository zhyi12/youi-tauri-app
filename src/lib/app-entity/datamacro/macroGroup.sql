
create table  stats_macro_group
(
    id    INTEGER      not null
        primary key autoincrement,

    text    TEXT ,
    group_type    TEXT ,
    desc    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);