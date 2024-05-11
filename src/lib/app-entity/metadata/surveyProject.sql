
create table  stats_survey_project
(
    id    INTEGER      not null
        primary key autoincrement,

    text    TEXT ,
    code    TEXT ,
    project_type    TEXT ,
    desc    TEXT ,
    icon    TEXT ,
    create_time    real ,
    update_time    real ,
    creator    TEXT ,
    modified_by    TEXT
);