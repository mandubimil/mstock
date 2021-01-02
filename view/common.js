var wid = 0;
var temp_child_str = '';
var wid = 0;

//result 는 공통 grid 창을 위한 변수
var result_text = '';
var result_infor = '';

webix.ready(function()
{   
    webix.ui({view:"window", id:"win_loading", head:"로딩중 ", width:600, height:100,
        body: { view:"template",  template:"<p><center>자료를 불러오는 중입니다.</center>",width:600},                
    });
});

var menu_data = 
[
    {   id:"bc",value:"차트", submenu:
        [
            { id:"bc1010", value:"일봉차트 비교"},
            { id:"bc1020", value:"다중 차트"},
        ]
    },
    { id:"rt",value:"레포트", submenu:
        [
            { id:"rt1010", value:"기본 차트"},
            { id:"rt1020", value:"순위"},
            { id:"rt1040", value:"거래/등락 비교"},
        ]
    },        
    { id:"ju",value:"매매", submenu:
        [
            { id:"ju1010", value:"주식서버/DB 비교"},
        ]
    },        
    { id:"bt",value:"백테스팅", submenu:
        [
            { id:"bt1010", value:"패턴_매도 결과"},
        ]
    },        
    { id:"zz",value:"기타", submenu:
        [
            { id:"zz9990", value:"객체 보기"},
            { id:"zz0000", value:"-------"},
            { id:"pu1020", value:"종목관리&수기매매 팝업 열기"},
            { id:"pu1040", value:"python cron 설정"},
            { id:"pu1070", value:"query & grid"},
        ]
    }
];

var bar_top_main =
{
    view:"toolbar", elements:
    [
        { view:"menu", data: menu_data, width:280, css:"blue",
            on:
            {
                onMenuItemClick:function(id)
                {
                    gogo_windows(id);                        
                }
            }
        }
    ]                    
};

var bar_top =
{
    view:"toolbar", elements:
    [
        {view:"label", id: "state_1", label:"state", align:"left" },
        {view:"label", id: "state_2", label:"main00", width:200, align:"right" },
    ]                    
};

function gogo_windows(id)
{
    if (id==='bc1030'||id==='bc1050')
    {                            
        webix.message('팝업만 하는 윈도우~');
    }
    else if (id==='pu1020')
    {
        temp_child_str = "bank"
        var url ="/view/pu1020.html"
        var title = "pu1020";
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=1200, top=0,left=20"    
        pu1020 = window.open(url, title, status);    
    }
    else if (id==='pu1040')
    {
        temp_child_str = "bank"
        var url ="/view/pu1040.html"
        var title = "pu1040";
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=500, top=0,left=20"    
        pu1040 = window.open(url, title, status);    
    }
    else if (id==='bc1020')
    {
        temp_child_str = "bank"
        var url ="/view/bc1020.html"
        var title = "bc1020";
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1400, height=500, top=0,left=20"    
        bc1020 = window.open(url, title, status);    
    }
    else if (id==='pu1070')
    {
        var url ="/view/pu1070.html"
        var title = "pu1070"+wid;
        wid = wid + 1;
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=1200, top=0,left=20"
    
        pu1070 = window.open(url, title, status);                    
    }                   
    else if (id!='bc'&&id!='rt'&&id!='ju'&&id!='zz'&&id!='bt')
    {
        temp_child_str = "bank"
        var url ="/view/"+id+".html"
        var title = id;
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=2000, height=1200, top=0,left=20"    
        new_win = window.open(url, title, status);    
        //location.href = '/view/'+id+'.html';
    }    
}

var bar_buttom_1 =
{
    height:50,
    view:"toolbar", elements:
    [
        {
            width:140,
            rows:
            [
                {view:"label", label:"종목", height:19, width:180},
                { view:"text", id:"text_jong_name" },
            ]
        },
        {
            width:130,
            rows:
            [
                {view:"label", id: "label_file", label:"업종", height:19},
                { 
                    view:"richselect", id:'combobox_uj', value:'ZZ',
                    suggest: 
                    { 
                        body:
                        {
                            yCount:20,
                            url:"/common_url_get/combobox_uj"
                        }
                    }
                },                    
            ]
        },
        {
            width:180,
            rows:
            [
                {view:"label", label:"테마", height:19},
                { 
                    view:"richselect", id:'combobox_tm', value:'ZZ',
                    suggest: 
                    { 
                        body:
                        {
                            yCount:20,
                            url:"/common_url_get/combobox_tm"
                        }
                    }
                },                    
            ]
        },
        {
            width:80,
            rows:
            [
                {view:"label", label:"시장", height:19},
                { 
                    view:"richselect", id:'combobox_si', value:'ZZ',
                    options:
                    [ 
                        { id:'ZZ', value:"전체" }, 
                        { id:'1', value:"코스피" }, 
                        { id:'2', value:"코스닥" }
                    ]
                },                    
            ]
        },
        {
            width:70,
            rows:
            [
                {view:"label", label:"ETF", height:19},
                { 
                    view:"richselect", id:'combobox_etf', value:'ZZ',
                    options:
                    [ 
                        { id:'ZZ', value:"전체" }, 
                        { id:'0', value:"일반" }, 
                        { id:'1', value:"ETF" }, 
                        { id:'2', value:"ETN" }
                    ]
                },                    
            ]
        },
        {
            width:170,
            rows:
            [
                {view:"label", label:"기간", height:19},
                {
                    cols:
                    [
                        { view:"text", id:"day_start" },
                        {view:"label", label:"~", width:10},
                        { view:"text", id:"day_end" },                                
                    ]                                    
                }
            ]
        },                        
        {},
    ]
};

function test()
{
    alert("gogo");
}

function tput1(str_temp)
{
	$$('tms1').setHTML(str_temp);
}

function get_pm_mon(p_day, p_pm)
{
	var dat1 = new Date(p_day.substring(0,4), p_day.substring(4,6), p_day.substring(6,8));
	dat1.setMonth(dat1.getMonth() + p_pm);

    var year = dat1.getFullYear();
    var month = dat1.getMonth();
    var day = dat1.getDate();
 
    if ((day+"").length < 2) { day = "0" + day; }
    if ((month+"").length < 2) { month = "0" + month; }
 
    return year +''+ month +''+ day;
}


function get_today()
{
    var date = new Date();
 
    var year = date.getFullYear();
    var month = date.getMonth()+1;
    var day = date.getDate();
 
    if ((day+"").length < 2) {
        day = "0" + day;
    }

    if ((month+"").length < 2) {
        month = "0" + month;
    }
 
    return year +''+ month +''+ day;
}

function get_obj_list(data)
{
	var str_temp = '';
	
	if ((typeof data) === 'string')
		str_temp = str_temp + '[string] ' + data;
	if ((typeof data) === 'number')
		str_temp = str_temp + '[number] ' + data;
	else
		for (var key in data) { 
			//str_temp = str_temp + 'type: [' + typeof data[key] + '], key: [' + key + '], value: [' + data[key] + ']'+'\n';
			str_temp = str_temp + '[' + typeof data[key] + '] ' + key + ' : ' + data[key] + '\n';
		}

	return str_temp;
}

function comma1000(num)
{
	num = String(num);
    return num.replace(/(\d)(?=(?:\d{3})+(?!\d))/g,"$1,");
}

function get_r_color () { 

    var color = "#"; 
        var max = Math.pow( 256, 3 ); 
    var random = Math.floor( Math.random() * max ).toString( 16 ); 
    var gap = 6 - random.length; 

    if ( gap > 0 ) {    for ( var x = 0; x < gap; x++ ) color += "0";    } 

    return color + random; 
} 


function set_grid(text, grid)
{
    var jsd = JSON.parse(text);
    
    grid.clearAll();   
    grid.config.columns = [];
    grid.refreshColumns();

    grid.parse(jsd["data"]);

    var i = 0;
    for (var key in jsd["data"][0])
    {
        if ((key != '일자') && (key != '단축코드') && (key != '종목코드') && (key != 'id') &&  (key != '종목번호') && 
        (key[key.length-1] != '_') && (key != '테마') && (key != '업종') && (key != '기준') &&  (key != '코드') && 
        (!(isNaN(parseInt(jsd["data"][0][key]))))) 
        {
            grid.config.columns[i].sort = 'int';
            grid.config.columns[i].format = webix.i18n.intFormat;
            //grid.config.columns[i].cssFormat  = {'text-align':'right'};
            //grid.config.columns[i].cssFormat = {"background-color":"#FFAAAA"};
        }

        grid.adjustColumn(key, 'all');
        i = i + 1;
    }
 
    grid.refresh();
}

function set_grid_pm(text, grid, gijun)
{
    function mark_ger(value)
    {
        if (parseInt(value.replace(",","")) > gijun)
            return {"background-color":"#FFAAAA", 'text-align':'right'};
        else 
            return {"background-color":"#AAAAFF", 'text-align':'right'};
    }    

    var jsd = JSON.parse(text);
    
    grid.clearAll();   
    grid.config.columns = [];
    grid.refreshColumns();

    grid.parse(jsd["data"]);

    var i = 0;
    for (var key in jsd["data"][0])
    {
        if ((key != '일자') && (key != '단축코드') && (key != '종목코드') && (key != 'id') &&  (key != '종목번호') && 
            (key[key.length-1] != '_') && (key != '테마') && (key != '업종') && (key != '기준') &&  (key != '코드') && 
           (!(isNaN(parseInt(jsd["data"][0][key]))))) 
        {
            grid.config.columns[i].sort = 'int';
            grid.config.columns[i].format = webix.i18n.intFormat;
            grid.config.columns[i].cssFormat = mark_ger;
        }

        grid.adjustColumn(key, 'all');
        i = i + 1;
    }
 
    grid.refresh();
}

function set_grid_python_select(text, grid)
{
    var jsd = JSON.parse(text);

    grid.clearAll();          

    if (jsd['check_result'] == 'good')
    {
        var col_title = [];
        for (var key in jsd['cols'])
            col_title.push({'id':key, 'col_name':jsd['cols'][key], 'header':jsd['cols'][key], adjust:true});

        grid.config.columns = col_title;
        grid.refreshColumns();
        
        grid.parse(jsd['rows']);        

        for (var key in jsd['cols'])
            grid.adjustColumn(key, 'all');
    }
    else
        alert(jsd['error_message']);
}


function open_chart_window(p_jong_code, p_day_start, p_day_end)
{
    let s_para = {
        "jong_code": p_jong_code,
        "day_start": p_day_start,
        "day_end": p_day_end,
    }

    temp_child_str = JSON.stringify(s_para);
    
    var url ="/view/pu1010.html"
    var title = "pu1010"+wid;
    wid = wid + 1;
    var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=300, top=0,left=20"

    var pu1010 = window.open(url, title, status);
}

function open_ger_chart_window(p_jong_code, p_day_start, p_day_end)
{
    let s_para = {
        "jong_code": p_jong_code,
        "day_start": p_day_start,
        "day_end": p_day_end,
    }

    temp_child_str = JSON.stringify(s_para);
    
    var url ="/view/pu1050.html"
    var title = "pu1050"+wid;
    wid = wid + 1;
    var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=1200, top=0,left=20"

    var pu1050 = window.open(url, title, status);
}

function open_all_chart_window(p_jong_json)
{
    temp_child_str = JSON.stringify(p_jong_json);
    
    var url ="/view/pu1060.html"
    var title = "pu1060"+wid;
    wid = wid + 1;
    var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=500, top=0,left=20"

    var pu1060 = window.open(url, title, status);
}

var pu1020;
function open_interest_window(p_jong_code, p_jong_name)
{
    if (pu1020 != null && !pu1020.closed) 
    {
        webix.message("종목관리 창이 열려있습니다.");
        pu1020.add_imsi_jong(p_jong_code, p_jong_name);
    } 
    else 
    {
        let s_para = {
            "단축코드": p_jong_code,
            "종목명": p_jong_name,
        }
    
        temp_child_str = JSON.stringify(s_para);

        var url ="/view/pu1020.html"
        var title = "pu1020";
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=1200, top=0,left=20"
    
        pu1020 = window.open(url, title, status);    
    }
}

var pu1030;
function open_meme_window(p_jong_code, p_jong_name, p_meme_su, p_meme_gu, p_meme_sun)
{
    if (pu1030 != null && !pu1030.closed) 
    {
        webix.message("주문 창이 열려있습니다.");
        pu1030.init_meme(p_jong_code, p_jong_name, p_meme_su, p_meme_gu, p_meme_sun);
    } 
    else 
    {
        let s_para = {
            "단축코드": p_jong_code,
            "종목명": p_jong_name,
            "매매구분": p_meme_gu,
            "매매수량": p_meme_su,
            "매매순번": p_meme_sun,
        }
    
        temp_child_str = JSON.stringify(s_para);

        var url ="/view/pu1030.html"
        var title = "pu1030";
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=500, top=0,left=20"
    
        pu1030 = window.open(url, title, status);    
    }
}

function format_date(str_date)
{
    if ((str_date===null) || (str_date===''))
        return ''
    else
        return str_date.substring(0,4)+'-'+str_date.substring(4,6)+'-'+str_date.substring(6,8)
}

function win_loading_show(callback)
{
    $$('win_loading').show({x:700, y:300});
    callback();
}

function win_loading_hide()
{
    $$('win_loading').hide();
}

function open_sum_chart_window(jong_list, jong_list_name, p_day_start, p_day_end)
{
    let s_para = {
        "jong_list": jong_list,
        "jong_list_name": jong_list_name,
        "day_start": p_day_start,
        "day_end": p_day_end,
    }

    temp_child_str = JSON.stringify(s_para);

    var url ="/view/pu1080.html"
    var title = "pu1080"+wid;
    wid = wid + 1;
    var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1080, height=1200, top=0,left=20"

    var pu1080 = window.open(url, title, status);    
}

function is_jong_code(p_grid)
{
    for (let i=0 ; i<p_grid.config.columns.length ; i++)
    {
        if (p_grid.config.columns[i].id == "단축코드")
            return true;

    }
    
    return false;
}