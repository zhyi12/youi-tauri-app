
create table  stats_survey_task
(
    id    INTEGER      not null
        primary key autoincrement,

    code    TEXT ,
    text    TEXT ,
    period_type    TEXT ,
    stats_scope    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);