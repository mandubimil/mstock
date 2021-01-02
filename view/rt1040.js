function start_page()
{
    $$("state_2").setValue("거래/등락 비교 [rt1040]");

    $$('day_start').setValue(get_pm_mon(get_today(), -6));
    $$('day_start').disable();
    $$('day_end').setValue(get_today());       
}

function gogo_search()
{
    let job_id = "";
    let gijun = 0;
    let send_json = {};
    
    if ($$("radiobox_1").getValue() == 10)
        job_id = "get_ger_jong";
    else if ($$("radiobox_1").getValue() == 11)
    {
        job_id = "get_ger_si";
        gijun = 0;
    }        
    else if ($$("radiobox_1").getValue() == 12)
    {
        job_id = "get_ger_uj";
        gijun = 0;
    }        
    else if ($$("radiobox_1").getValue() == 13)
    {
        job_id = "get_ger_tm";
        gijun = 0;
    }        
    else if ($$("radiobox_1").getValue() == 14)
    {
        job_id = "get_ger_uj_si";
        gijun = 0;
    }        
    else if ($$("radiobox_1").getValue() == 15)
    {
        job_id = "get_ger_tm_si";
        gijun = 0;
    }        
    else if ($$("radiobox_1").getValue() == 20)
    {
        job_id = "get_dr_jong";
        gijun = 100;
    }        
    else if ($$("radiobox_1").getValue() == 21)
    {
        job_id = "get_dr_si";
        gijun = 100;
    }        
    else if ($$("radiobox_1").getValue() == 22)
    {
        job_id = "get_dr_uj";
        gijun = 100;
    }        
    else if ($$("radiobox_1").getValue() == 23)
    {
        job_id = "get_dr_tm";
        gijun = 100;
    }        
    else if ($$("radiobox_1").getValue() == 24)
    {
        job_id = "get_dr_uj_si";
        gijun = 100;
    }        
    else if ($$("radiobox_1").getValue() == 25)
    {
        job_id = "get_dr_tm_si";
        gijun = 100;
    }        
    else 
        job_id = "?";

    send_json = 
    {
        "j_para":
        {
            "수량금액":$$('radiobox_21').getValue()+"",
        },
        "q_para":
        {
            "시장":$$('combobox_si').getValue(),
            "etf":$$('combobox_etf').getValue(),
            "일자":$$('day_end').getValue(),
        },
        "e_para":
        {
            "업종":$$('combobox_uj').getValue(),
            "테마":$$('combobox_tm').getValue(),
        }
    };	
    
    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/rt1040/"+job_id, JSON.stringify(send_json), function(text)
        {  
            if ($$("radiobox_22").getValue() == 1)
                grid = $$("grid_1");
            else 
                grid = $$("grid_2");

            set_grid_pm(text, grid, gijun);           
            
            win_loading_hide();
        });		
    });    
}

function gogo_ext()
{
    if ($$("radiobox_22").getValue() == 1)
    {
        $$("grid_2").define("height", 1);
        $$("grid_2").resize();
        $$("grid_1").define("height", 0);
        $$("grid_1").resize();
    }
    else 
    {
        $$("grid_1").define("height", 1);
        $$("grid_1").resize();
        $$("grid_2").define("height", 0);
        $$("grid_2").resize();
    }
}