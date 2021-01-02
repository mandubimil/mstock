ju_list_list = '';
ds_count = 0;
ds = [];

function start_page()
{
    let s_data = JSON.parse(opener.temp_child_str);

    for (let key in s_data["종목들"])
    {
        insert_chart(s_data["종목들"][key].단축코드, s_data["종목들"][key].종목명, s_data.시작일자, s_data.종료일자);
    }
}


function insert_chart(p_jong_code, p_jong_name, p_day_start, p_day_end)
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
            "시작일자":p_day_start,
            "종료일자":p_day_end,
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