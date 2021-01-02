function start_page()
{
    get_setting();
}



function get_setting()
{
    var send_json = {};
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1040/get_setting", JSON.stringify(send_json), function(text)
        {  
            set_grid(text, $$("grid_setting"));            
            win_loading_hide();
        });		
    });
}

function set_setting()
{

    if (($$("cron_server").getValue().trim() == "") ||
       ($$("cron_file").getValue().trim() == "") ||
       ($$("cron_gu").getValue().trim() == ""))
    {
        webix.message("필수 항목을 확인하세요.");
        return 1;
    }

    var send_json = 
    {
        "q_para":
        {
            "서버":$$("cron_server").getValue(),
            "파일":$$("cron_file").getValue(),
            "구분":$$("cron_gu").getValue(),
        },
        "e_para":
        {
            "서버":$$("cron_server").getValue(),
            "파일":$$("cron_file").getValue(),
            "구분":$$("cron_gu").getValue(),
            "설명":$$("cron_content").getValue(),
            "실행":$$("cron_exe").getValue(),
            "비고":$$("cron_bigo").getValue(),
        }
    };	

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1040/set_setting", JSON.stringify(send_json), function(text)
        {  
            webix.message(text);
            win_loading_hide();
            get_setting();
        });		
    });
}

function del_setting()
{
    var send_json = 
    {
        "q_para":
        {
            "서버":$$("cron_server").getValue(),
            "파일":$$("cron_file").getValue(),
            "구분":$$("cron_gu").getValue(),
        }
    };	

	webix.confirm({title:$$("cron_server").getValue()+"/"+$$("cron_file").getValue()+"/"+$$("cron_gu").getValue(), text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
            win_loading_show(function()
            {
                webix.ajax().headers({"Content-type":"application/json"}).post("/pu1040/del_setting", JSON.stringify(send_json), function(text)
                {  
                    win_loading_hide();
                    get_setting();
                });		
            });
		}
	}});               
}

function clear_setting()
{
    $$('cron_server').setValue("");
    $$('cron_file').setValue("");
    $$('cron_gu').setValue("");
    $$('cron_content').setValue("");
    $$('cron_exe').setValue("");
    $$('cron_bigo').setValue("");
}