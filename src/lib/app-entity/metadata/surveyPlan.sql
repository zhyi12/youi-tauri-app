
create table  stats_survey_plan
(
    id    INTEGER      not null
        primary key autoincrement,

    project_id    real ,
    code    TEXT ,
    year    TEXT ,
    text    TEXT ,
    title    TEXT ,
    sub_title    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);