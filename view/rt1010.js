function start_page()
{
    $$("state_2").setValue("기본 레포트 [rt1010]");

    $$('day_start').disable();
    $$('day_end').disable();        
}

function gogo_search()
{
   let job_id = "";
    var send_json = {};
    if ($$("radiobox_1").getValue() == 1)
    {
        job_id = "get_jong";

        send_json = 
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
    }        
    else if ($$("radiobox_1").getValue() == 2)
        job_id = "get_uj";
    else if ($$("radiobox_1").getValue() == 3)
        job_id = "get_tm";
    else if ($$("radiobox_1").getValue() == 4)
        job_id = "get_uj_tm";
    else if ($$("radiobox_1").getValue() == 5)
        job_id = "get_tm_uj";
    else 
        job_id = "?";
    
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/rt1010/"+job_id, JSON.stringify(send_json), function(text)
        {  
            if ($$("radiobox_2").getValue() == 1)
                set_grid(text, $$("grid_1"));
            else 
                set_grid(text, $$("grid_2"));
            
            win_loading_hide();
        });		
    });    
}

