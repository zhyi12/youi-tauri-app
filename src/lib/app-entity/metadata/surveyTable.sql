
create table  stats_survey_table
(
    id    INTEGER      not null
        primary key autoincrement,

    code    TEXT ,
    year    TEXT ,
    text    TEXT ,
    period_type    TEXT ,
    respondent_type    TEXT ,
    table_scope    TEXT ,
    submit_time_desc    TEXT ,
    check_time_desc    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);