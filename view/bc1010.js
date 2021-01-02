ju_list_list = '';
ds_count = 0;
ds = [];

function start_page()
{
    $$("state_2").setValue("일봉 차트 비교 [bc1010]");

    $$('day_start').setValue(get_pm_mon(get_today(), -12));
    $$('day_end').setValue(get_today());        
    
    $$('chart_min').setValue('700');
    $$('chart_max').setValue('1300');

    get_jong_list();

}

function get_jong_list()
{
    var send_json = 
    {
        "q_para":
        {
            "시장":$$('combobox_si').getValue(),
            "etf":$$('combobox_etf').getValue(),
        },        
        "e_para":
        {
            "업종":$$('combobox_uj').getValue(),
            "테마":$$('combobox_tm').getValue(),
        }
    };	

	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_jong_list", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);		

		$$('list_jong_all').clearAll(); 
		$$('list_jong_all').parse(jsd);
	});		
}


function insert_select_jong()
{
    for (i=0 ; i<$$("list_jong_select").count() ; i++)
    {
        if ($$("list_jong_select").getItem($$("list_jong_select").getIdByIndex(i)).단축코드 == $$('list_jong_all').getItem($$('list_jong_all').getSelectedId()).단축코드)
        {
            webix.message("이미 선택했어요");
            return 0;
        }
    }        

    $$('list_jong_select').add({
        단축코드:$$('list_jong_all').getItem($$('list_jong_all').getSelectedId()).단축코드,
        종목명:$$('list_jong_all').getItem($$('list_jong_all').getSelectedId()).종목명
        }, 0
    );       
}

function get_chart(p_jong_name)
{
    var send_json = 
    {
        "j_para":{},
        "q_para":
        {
            "단축코드":p_jong_name,
            "시작일자":$$('day_start').getValue(),
            "종료일자":$$('day_end').getValue(),    
        }
    };	    

	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_chart_basic", send_json, function(text)
	{  
        var jsd = JSON.parse(text)["data"];

        $$("chart_1").단축코드 = jsd[0].단축코드;
        $$("chart_1").종목명 = jsd[0].종목명;
        
        $$("state_1").setValue(
            jsd[0].단축코드 + ":"+
            jsd[0].종목명 +
            "  시가총액:" + comma1000(jsd[0].시가총액)+
            "  상장주식:" + comma1000(jsd[0].상장주식)+
            "  per:" + comma1000(jsd[0].per)+
            "  pbr:" + comma1000(jsd[0].pbr)+
            "  외국인보유:" + comma1000(jsd[0].외국인보유)+
            "  per:" + comma1000(jsd[0].per)+
            "  차이:" + Math.round(((jsd[0].최고주가-jsd[0].최저주가)/jsd[0].최고주가)*100) +"%"
        );

        var max_p = parseInt(jsd[0].최고주가);
        var min_p = parseInt(jsd[0].최저주가);
        var step_10 = parseInt((max_p - min_p) / 10);
        var max_g = parseInt(jsd[0].최고거래);

        $$("chart_1").removeAllSeries();
        $$("chart_1").addSeries({  type:"line", value:"#주가#", color:"#36abee", item:{ radius:0 }, 
                                    line:{ color:"#1293f8", width:3 },
                                    yAxis:{ lineColor:"#cccccc", start:min_p, end:max_p, step:step_10 },
                                    xAxis:{ template: "#월단위#", lines: false }
                                });

        $$("chart_1").addSeries({  type:"line", value:"#주가30일#", color:"#eeeeee", item:{ radius:0 }, 
                                line:{ color:"#de619c", width:1 },
                                yAxis:{ start:min_p, end:max_p, step:step_10 },
                                xAxis:{ template: "#월단위#", lines: false }
                            });
                            
        $$("chart_1").addSeries({  type:"line", value:"#주가60일#", color:"#eeeeee", item:{ radius:0 }, 
                                line:{ color:"#ff8c00", width:1 },
                                yAxis:{ start:min_p, end:max_p, step:step_10 },
                                xAxis:{ template: "#월단위#", lines: false }
                            });

        $$("chart_2").removeAllSeries();
        $$("chart_2").addSeries({  type:"bar", value:"#거래량#", color:"#aaaaaa", item:{ radius:0 }, 
                                    yAxis:{ start:0, end:max_g, step:Math.round(max_g/3) },
                                    xAxis:{ template: "#월단위#", lines: false }
                                });

        webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_chart", send_json, function(text2)
        {  
            var jsd2 = JSON.parse(text2)["data"];

            $$('chart_1').clearAll(); 
            $$('chart_1').parse(jsd2);            

            $$('chart_2').clearAll(); 
            $$('chart_2').parse(jsd2);
        });
    });		        
}

function all_insert_chart()
{
    init_chart();

    if ($$("list_jong_select").count() < 50)
    {
        for (i=0 ; i<$$("list_jong_select").count() ; i++)
        {
            insert_chart($$("list_jong_select").getItem($$("list_jong_select").getIdByIndex(i))["단축코드"], 
                         $$("list_jong_select").getItem($$("list_jong_select").getIdByIndex(i))["종목명"]);
        }
    }
    else
        webix.message('50 이상은 좀..'); 
}    

function button_insert_chart()
{
    insert_chart($$("chart_1").단축코드, $$("chart_1").종목명);
}

function insert_chart(p_jong_code, p_jong_name)
{
    if (p_jong_code == "")
    {
        webix.message("차트를 출력하세요");
        return 1;
    }

    var send_json = 
    {
        "j_para":{},
        "q_para":
        {
            "단축코드":p_jong_code,
            "시작일자":$$('day_start').getValue(),
            "종료일자":$$('day_end').getValue(),    
        }
    };	    

	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_chart_select", send_json, function(text)
	{  
        var jsd = JSON.parse(text)["data"];

        ds_count = ds_count + 1;

        if (ds_count == 1)
        {
            for (i=0 ; i<jsd.length ; i++)
            {
                ds.push({"일자":jsd[i].일자, "월단위":jsd[i].월단위, "p1":parseInt(jsd[i].주가)});
            }
    
            $$('chart_tot').parse(ds);           
            $$("chart_tot").removeAllSeries();    
        }
        else
        {
            for (i=0 ; i<jsd.length ; i++)
                for (j=0 ; j<ds.length ; j++)
                    if (ds[j]["일자"] == jsd[i]["일자"])
                        ds[j]["p"+ds_count] = jsd[i]["주가"];                        
        }



        temp_color = get_r_color();
        $$("chart_tot").addSeries({  type:"line", value:"#p"+ds_count+"#", color:"#36abee", item:{ radius:0 }, 
                                    line:{ color:temp_color, width:2 },
                                    yAxis:{ lineColor:"#cccccc", start:$$("chart_min").getValue(), end:$$("chart_max").getValue(), step:100},
                                    xAxis:{ template: "#월단위#", lines: false }
                                });
                                
        $$('chart_tot').refresh();

        
        ju_list_list = ju_list_list + '<font color='+temp_color+'>'+p_jong_name+'_ </font>';
        $$('ju_list_label').setHTML(ju_list_list);
    });    
}

function init_chart()
{
    $$('chart_tot').clearAll(); 
    $$("chart_tot").removeAllSeries();
   	$$('ju_list_label').setHTML('');
   
    ju_list_list = '';
    ds_count = 0;
    ds = [];       
}

function insert_all_jong()
{
    if ($$("list_jong_all").count() < 50)
    {
        for (i=0 ; i<$$("list_jong_all").count() ; i++)
        {
            $$('list_jong_select').add({
                단축코드:$$('list_jong_all').getItem($$('list_jong_all').getIdByIndex(i)).단축코드,
                종목명:$$('list_jong_all').getItem($$('list_jong_all').getIdByIndex(i)).종목명
                }, 0
            );
        }
    }
    else
        webix.message('50 이상은 좀..'); 
}

