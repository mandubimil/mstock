function start_page()
{
    $$("state_2").setValue("순위 레포트 [rt1020]");

    $$('day_start').setValue(get_pm_mon(get_today(), -10));
    $$('day_end').setValue(get_today());       
    
    $$('combobox_uj').disable();
    $$('combobox_tm').disable();        
}

function gogo_search()
{

    let job_id = "";
    var send_json = 
    {
        "q_para":
        {
            "시장":$$('combobox_si').getValue(),
            "etf":$$('combobox_etf').getValue(),
            "종료일자":$$('day_end').getValue(),
        },        
    };	

    if ($$("radiobox_1").getValue() == 1)
        job_id = "get_ju_sang";
    else if ($$("radiobox_1").getValue() == 2)
        job_id = "get_tm_ju_sang";
    else if ($$("radiobox_1").getValue() == 3)
        job_id = "get_uj_ju_sang";
    else if ($$("radiobox_1").getValue() == 4)
        job_id = "get_ger_sang";
    else if ($$("radiobox_1").getValue() == 5)
        job_id = "get_tm_ger_sang";
    else if ($$("radiobox_1").getValue() == 6)
        job_id = "get_uj_ger_sang";
    else 
        job_id = "?";

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/rt1020/"+job_id, JSON.stringify(send_json), function(text)
        {  
            if ($$("radiobox_2").getValue() == 1)
                set_grid(text, $$("grid_1"));
            else 
                set_grid(text, $$("grid_2"));
            
            win_loading_hide();
        });		
    });    
}


function test()
{
    webix.message("good~~");
}