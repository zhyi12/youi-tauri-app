/********************************************************************
 * 菜单服务
 *******************************************************************/

import type {MenuInfo} from "../../app-entity/base/menu";

/**
 * 获取app菜单
 */
export const findAppMenus = async ():Promise<MenuInfo[]> => {

    const menus:MenuInfo[] = [
        {id:'020000',name:'geo',text:'地理信息',href:"/geo",
            children:[
                {id:'020100',icon:'area',name:'area',text:'行政区划',href:"/geo/area"},
                {id:'020300',name:'address',icon:'address',text:'地址库',href:"/geo/address",children: [
                        {id:'020301',name:'address',icon:'upload',text:'地址导入',href:"/geo/address/imp"},
                        {id:'020302',name:'search',icon:'query',text:'地址检索',href:"/geo/address/search"},
                        {id:'020303',name:'merge',icon:'merge',text:'地址合并',href:"/geo/address/merge"},
                        {id:'020304',name:'check',icon:'link',text:'地址核对',href:"/geo/address/check"},
                        {id:'020305',name:'match',icon:'match',text:'区划匹配',href:"/geo/address/match"},
                        {id:'020306',name:'stats',icon:'chart',text:'统计分析',href:"/geo/address/stats"},
                    ]},
                {id:'020400',name:'building',icon:'building',text:'建筑物',href:"/geo/building"},
                {id:'020500',name:'respondent',icon:'respondent',text:'名录库',href:"/geo/respondent",children: [
                        {id:'020501',name:'building',icon:'query',text:'互联网数据',href:"/geo/cods"},
                        {id:'020502',name:'respondentImp',icon:'upload',text:'名录导入',href:"/geo/respondent/imp"},
                        {id:'020503',name:'match',icon:'link',text:'区划匹配',href:"/geo/respondent/match"},
                        {id:'020504',name:'respondentCheck',icon:'match',text:'名录核对',href:"/geo/respondent/check"},
                        {id:'020505',name:'respondentCheck',icon:'moveIn',text:'名录迁入',href:"/geo/respondent/move-in"},
                        {id:'020506',name:'respondentCheck',icon:'moveOut',text:'名录迁出',href:"/geo/respondent/move-out"},
                    ]},
                {id:'020600',name:'roadNet',icon:'road',text:'路网管理',href:"/geo/road"},
            ]
        },
        {id:'030000',name:'res',text:'资源管理',href:"/res",
            children:[
                {id:'030100',name:'local',text:'本地',href:'/res/local'},
                {id:'030200',name:'cloud',text:'云盘',href:'/res/cloud'},
            ]
        },
        {id:'040000',name:'metadata',icon:'metadata',text:'元数据',href:"/metadata/meta/item",
            children:[
                {id: "040100",name: "metaItem",icon:'metadata',text: "数据项",href: "/metadata/meta/item"},
                {id: "040200",name: "metaDb",icon:"database",text: "数据资源",href: "/metadata/meta/db"},
                {id: "040300",name: "metaTable",text: "数据库表",icon:'report',href: "/metadata/meta/table"},
                {id: "040400",name: "surveyPlan",icon:"surveyPlan",text: "统计制度",href: "/metadata/stats",children: [
                        {id: "040401",name: "surveyTable",icon:'surveyTable',text: "调查表",href: "/metadata/stats-object/table"},
                        {id: "040402",name: "surveyIndicator",icon:'surveyIndicator',text: "指标",href: "/metadata/stats-object/indicator"},
                        {id: "040403",name: "surveyGroup",icon:'surveyGroup',text: "分组",href: "/metadata/stats-object/group"},
                        {id: "040404",name: "surveyCatalog",icon:'surveyCategory',text: "目录",href: "/metadata/stats-object/catalog"},
                        {id: "040405",name: "surveyAttr",icon:'timeFrame',text: "时间框架",href: "/metadata/stats-object/attr"},
                        {id: "040406",name: "quest",icon:'flat',text: "题目",href: "/metadata/stats-object/quest"},
                        {id: "040407",name: "governance",icon:'metadata',text: "数据治理",href: "/metadata/stats-object/governance"},
                    ]},
            ]
        },
        {id:'050000',name:'dataproc',icon:'dataproc',text:'数据生产',href:"/dataproc/stats/imp",children:[
            {id:"050100",name:"planDataImp",icon:"upload",text:"数据入库",href:'/dataproc/stats/imp',
                children: [
                    {id:"050101",name:"impJp2023",icon:"upload",text:"五经普导入",href:'/dataproc/stats/imp/JP_2023'},
                    {id:"050102",name:"impSt",icon:"upload",text:"一套表导入",href:'/dataproc/stats/imp/ST'},
                ]
            },
            {id:"050200",name:"planData",text:"数据加工",href:'',icon:"table",children: [
                {id:"050201",name:"dataQuality",icon:"quality",text:"质量检查",href:'/dataproc/stats/quality'},
                {id:"050202",name:"label",icon:"tag",text:"数据标签",href:'/dataproc/stats/label'},
                {id:"050203",name:"calculatorColumn",icon:"calculator",text:"计算指标",href:'/dataproc/stats/calculator'},
                {id:"050204",name:"planDataQuery",icon:"query",text:"自助查询",href:'/dataproc/stats/query'},
                {id:"050205",name:"planDataCube",icon:"cube",text:"自助汇总",href:'/dataproc/stats/report'},
                {id:"050206",name:"planDataReport",icon:"report",text:"固定汇总",href:'/dataproc/stats/fixed-report'},
                {id:"050208",name:"publication",icon:"report",text:"产品编制",href:'/dataproc/stats/publication'},
                {id:"050209",name:"publicationDoc",icon:"report",text:"出版物",href:'/dataproc/stats/publication-doc'}]
            }, {id:"050300",name:"projectData",text:"数据融合",href:'',icon:"table",children: [
                {id:"050301",name:"mappingMetaItem",icon:"mapping",text:"专题融合库",href:'/dataproc/stats-analysis/mapping'},
                {id:"050302",name:"dataFusion",icon:"fusion",text:"数据融合",href:'/dataproc/stats-analysis/fusion'}
            ]},
                /**
            {id: "050200",group:"dataproc-mydata",name:'mydata', route: "(submenu)/dataproc/mydata/d-all/[...folder]",text: "我的数据",href: "/dataproc/mydata/d-all/top",icon:'folder',children:[
                        {id: "050201",group:"dataproc-mydata",name:'selfQuery', route: "(submenu)/dataproc/mydata/d-query/[...folder]",text: "自助查询",href: "/dataproc/mydata/d-query/top",icon:"query"},
                        {id: "050202",group:"dataproc-mydata",name:'pivotable', route: "(submenu)/dataproc/mydata/d-pivotable/[...folder]",text: "透视表",href: "/dataproc/mydata/d-pivotable/top",icon:"crossTable"},
                        {id: "050203",group:"dataproc-mydata",name:'chart', route: "(submenu)/dataproc/mydata/d-chart/[...folder]",text: "图表",href: "/dataproc/mydata/d-chart/top",icon:"chart"},
                        {id: "050204",group:"dataproc-mydata",name:'etl', route: "(submenu)/dataproc/mydata/d-etl/[...folder]",text: "数据清洗",href: "/dataproc/mydata/d-etl/top",icon:"etl"},
                        {id: "050205",group:"dataproc-mydata",name:'report', route: "(submenu)/dataproc/mydata/d-report/[...folder]",text: "报表",href: "/dataproc/mydata/d-report/top",icon:"report"}
                    ]},*/
            ]},
        {id:'060000',name:'datamacro',text:'宏观数据',icon:'cube',href: '/datamacro/metadata',
            children:[
                {
                    id:'060100',name:'macroMetadata',text:'元数据',icon:'metadata',href: '/datamacro/metadata',
                    children: [
                        {id:'060102',name:'macroIndicator',text:'宏观指标',icon:'surveyIndicator',href: '/datamacro/metadata/indicator'},
                        {id:'060103',name:'macroIndicator',text:'宏观分组',icon:'surveyGroup',href: '/datamacro/metadata/group'},
                        {id:'060104',name:'macroIndicator',text:'计量单位',icon:'surveyAttr',href: '/datamacro/metadata/unit'},
                    ]
                },{
                    id:'060200',name:'macroData',text:'数据管理',icon:'cube',
                    children: [
                        {id:'060201',name:'macroSurveyPlan',text:'制度提取',icon:'topic',href: '/datamacro/data/source-survey-table'},
                        {id:'060202',name:'macroDataMng',text:'数据管理',icon:'surveyIndicator',href: '/datamacro/data'},
                        {id:'060203',name:'macroDataConfirm',text:'数据确权',icon:'surveyGroup',href: '/datamacro/data/confirm'},
                    ]
                },{
                    id:'060300',name:'macroData',text:'查询管理',icon:'cube',
                    children: [
                        {id:'060301',name:'macroTopic',text:'查询主题',icon:'topic',href: '/datamacro/metadata/topic'},
                        {id:'060302',name:'macroDataMng',text:'查询场景',icon:'surveyIndicator',href: '/datamacro/metadata/scene'},
                    ]
                }
            ]
        },
        {id:'070000',name:'datanalysis',icon:'dataAnalysis',text:'数据应用',href:'/datanalysis',
            children:[
                {id:'070100',name:'analysisdata',text:'数据准备',icon:'database',href:'/datanalysis/data'},
                {id:'070200',name:'dashboard',text:'数据看板',icon:'dashboard',href:'/datanalysis/dashboard'},
                {id:'070300',name:'chart',text:'数据图表',icon:'chart',href:'/datanalysis/chart'},
                {id:'070400',name:'analysisdoc',text:'分析报告',icon:'doc',href:'/datanalysis/doc'},
                {id:'070500',name:'geodata',text:'数据地图',icon:'geo',href:'/datanalysis/geo'},
                {id:'070600',name:'statsml',text:'高阶统计分析',icon:'ml',href:'/datanalysis/ml'},
            ]
        },
        // {id:'910000',name:'notebook',text:'笔记',href: '/notebook/local',icon:'notebook'},
        // {id:'920000',name:'taskList',text:'日程',href: '/task-list/month',icon:'taskList'},
        {id:'990000',name:'settings',text:'配置',href: '/settings',
            children:[
                {id: "990100",name: "icons",text: "图标",href: "/settings/icons"},
                {id: "990200",name: "apiProxy",text: "三方接口",href: "/settings/http-proxy"},
                {id: "990300",icon:"link",name: "websiteProxy",text: "网站代理",href: "/settings/website-proxy"},
                {id: "990400",name: "codemap",text: "数据代码集",href: "/settings/codemap"},
                {id: "990500",name: "shortcut",text: "快捷访问",href: "/settings/shortcut"},
                {id: "990600",name: "list",text: "DSL函数",href: "/settings/dsl"},
            ]},
        {id:'970000',name:'chat',text:'聊天',href: '/chat',icon:"chat"},
        {id:'980000',name:'terminal',text:'终端',href: '/terminal',icon:"console"}
    ];

    return menus;
}
