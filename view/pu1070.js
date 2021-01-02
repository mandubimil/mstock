function start_page()
{
    $$('day_start').setValue(get_pm_mon(get_today(), -10));
    $$('day_end').setValue(get_today());       

    get_query();

    $$('contents_1').setValue("select * from 종목마스터 where 단축코드 like '0000%'");     
}


function get_query()
{
    var send_json = {};    
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1070/get_query", JSON.stringify(send_json), function(text)
        {  
            set_grid(text, $$("grid_query"));
            
            win_loading_hide();
        });		
    });    
}

function get_query_content()
{
    $$("text_name").setValue($$('grid_query').getItem($$('grid_query').getSelectedId()).분류1);
    var send_json =
    {
        "q_para":
        { 
            "분류1":$$('grid_query').getItem($$('grid_query').getSelectedId()).분류1,
        }
    };
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1070/get_query_content", JSON.stringify(send_json), function(text)
        {  
            var jsd = JSON.parse(text);
            $$('contents_1').setValue(jsd["data"][0]['내용1']);
            
            win_loading_hide();
        });		
    });    	    
}

function del_query()
{
	if ($$("grid_query").getSelectedId() == undefined)
	{
		webix.message("쿼리를 선택을 하세요");
		return 1;
	}

	webix.confirm({title:$$("grid_query").getItem($$("grid_query").getSelectedId()).분류1+"", text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
			var send_json = 
			{
				"q_para":
				{
					"분류1":$$('grid_query').getItem($$('grid_query').getSelectedId()).분류1+"",
				}
			};	
			webix.ajax().headers({"Content-type":"application/json"}).post("/pu1070/del_query", JSON.stringify(send_json), function(text)
			{  
                $$('text_name').setValue("");
                $$('contents_1').setValue("");     
				get_query();
			});	
		}
	}});    
}

function run_query()
{    
    var send_json = {
		"para":{
			"exe_id":"query_select",
            "q_para":
            { 
                "내용":$$('contents_1').getValue() 
            }
		}
	};
    
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/python_exe", JSON.stringify(send_json), function(text)
        {  
            set_grid_python_select(text, $$('grid_result'));            

            win_loading_hide();
        });		
    });    	        
}


function save_query()
{    
    var send_json = 
    {
        "j_para":
        { 
            "분류1":$$('text_name').getValue(),
        },
        "q_para":
        { 
            "분류1":$$('text_name').getValue(),
            "내용1":$$('contents_1').getValue() 
        }
    };	    

	webix.ajax().headers({"Content-type":"application/json"}).post("/pu1070/save_query", send_json, function(text)
	{  
        webix.message(text);
        get_query();
    });    
}
