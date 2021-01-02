function start_page()
{
    $$('day_start').setValue(get_pm_mon(get_today(), -12));
    $$('day_end').setValue(get_today());        

    if (opener.temp_child_str != "bank")
    {
        let s_data = JSON.parse(opener.temp_child_str);
        add_imsi_jong(s_data["단축코드"], s_data["종목명"]);    
    }

    get_group();
    get_list_bo();
    get_reservation();
}


function add_imsi_jong(p_jong_code, p_jong_name)
{
    for (i=0 ; i<$$("list_imsi_jong").count() ; i++)
    {
        if ($$("list_imsi_jong").getItem($$("list_imsi_jong").getIdByIndex(i)).단축코드 == p_jong_code)
        {
            webix.message("이미 선택했어요");
            return 0;
        }
    }        

    $$('list_imsi_jong').add({
        단축코드:p_jong_code,
        종목명:p_jong_name
        }, 0
    );       
}


function insert_group()
{
    function gogo_1(callback)
    {

        for (i=0 ; i<$$("list_imsi_jong").count() ; i++)
        {
            let send_json = 
            {
                "q_para":
                {
                    "그룹명":$$("text_group").getValue(),
                    "단축코드":$$("list_imsi_jong").getItem($$("list_imsi_jong").getIdByIndex(i)).단축코드,
                }
            };	

            webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/insert_group", JSON.stringify(send_json), function(text)
            {
                // webix.message(i);                
            });
        }        

        callback();
    }

    gogo_1(function(){
        get_group();
    });
}


function get_reservation()
{
    var send_json =
    {
        "q_para":
        {
            "시작일자":$$("day_start").getValue(),
            "종료일자":$$("day_end").getValue(),
        },        
    };	
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/get_reservation", JSON.stringify(send_json), function(text)
        {  
            set_grid(text, $$("grid_reservation"));            
            win_loading_hide();
        });		
    });
}

function get_group()
{
    var send_json = {};

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/get_group", JSON.stringify(send_json), function(text)
        {  
            set_grid(text, $$("grid_group"));            
            win_loading_hide();
        });		
    });    

    $$("grid_group_jong").clearAll();
}

function get_group_jong(p_group_name)
{
    var send_json =
    {
        "q_para":
        {
            "그룹명":p_group_name,
        },        
    };	

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/get_group_jong", JSON.stringify(send_json), function(text)
        {  
            var jsd = JSON.parse(text);
    
            $$("grid_group_jong").clearAll();          
            $$("grid_group_jong").parse(jsd["data"]);       
            $$("text_group_name").setValue(p_group_name);

            win_loading_hide();
        });		
    });    
}

function del_group()
{
	if ($$("grid_group").getSelectedId() == undefined)
	{
		webix.message("그룹을 선택을 하세요");
		return 1;
	}

	webix.confirm({title:$$("grid_group").getItem($$("grid_group").getSelectedId()).그룹명+"", text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
			var send_json = 
			{
				"q_para":
				{
					"그룹명":$$("grid_group").getItem($$("grid_group").getSelectedId()).그룹명+"",
				}
			};	
			webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/del_group", JSON.stringify(send_json), function(text)
			{  
                $$('pop_1').hide();
                get_group();
			});	
		}
	}});  
}


function del_group_jong()
{
	if ($$("grid_group_jong").getSelectedId() == undefined)
	{
		webix.message("종목을 선택을 하세요");
		return 1;
	}

	webix.confirm({title:$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).그룹명+"", text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
			var send_json = 
			{
				"q_para":
				{
					"그룹명":$$("grid_group").getItem($$("grid_group").getSelectedId()).그룹명+"",
					"단축코드":$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).단축코드+"",
				}
			};	
			webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/del_group_jong", JSON.stringify(send_json), function(text)
			{  
                webix.message(text);
                $$('pop_2').hide();
                get_group();
                //get_group_jong($$('grid_group').getItem($$('grid_group').getSelectedId()).그룹명);
			});	
		}
	}});  
}

function get_list_bo()
{
    var send_json = {};	

	webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/get_list_bo", JSON.stringify(send_json), function(text)
	{  
		set_grid(text, $$("grid_bo"));
	});	
}

function clear_jong()
{
    $$("list_imsi_jong").clearAll();
}

function update_group_jong()
{
	if ($$("grid_group_jong").getSelectedId() == undefined)
	{
		webix.message("종목을 선택을 하세요");
		return 1;
	}

    var send_json = 
    {
        "q_para":
        {
            "그룹명":$$("grid_group").getItem($$("grid_group").getSelectedId()).그룹명+"",
            "수정_그룹명":$$("text_group_name").getValue()+"",
            "단축코드":$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).단축코드+"",
            "비고1":$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).비고1+"",
            "비고2":$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).비고2+"",
            "비고3":$$("grid_group_jong").getItem($$("grid_group_jong").getSelectedId()).비고3+"",
        }
    };	
    webix.ajax().headers({"Content-type":"application/json"}).post("/pu1020/update_group_jong", JSON.stringify(send_json), function(text)
    {  
        webix.message(text);
    });	
}