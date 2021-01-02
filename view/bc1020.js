function start_page()
{
    $$('day_start').setValue(get_pm_mon(get_today(), -6));
    $$('day_end').setValue(get_today());            

    get_list();
}

function get_list()
{
    var send_json = {};	
	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1020/get_si", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);
		set_grid(text, $$("grid_si"));
    });		
	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1020/get_etf", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);
		set_grid(text, $$("grid_etf"));
    });		
	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1020/get_uj", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);
		set_grid(text, $$("grid_uj"));
    });		
	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1020/get_tm", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);
		set_grid(text, $$("grid_tm"));
    });		
}

function cancel_select_jo()
{
    $$('grid_si').unselectAll();
    $$('grid_etf').unselectAll();
    $$('grid_uj').unselectAll();
    $$('grid_tm').unselectAll();
}

function gogo_search()
{
    let jo_si = "ZZ";
    if ( $$('grid_si').getSelectedId() != undefined )
    {
        jo_si = $$('grid_si').getItem($$('grid_si').getSelectedId()).코드;
    }
    let jo_etf = "ZZ";
    if ( $$('grid_etf').getSelectedId() != undefined )
    {
        jo_etf = $$('grid_etf').getItem($$('grid_etf').getSelectedId()).코드;
    }
    let jo_uj = "ZZ";
    if ( $$('grid_uj').getSelectedId() != undefined )
    {
        jo_uj = $$('grid_uj').getItem($$('grid_uj').getSelectedId()).코드;
    }
    let jo_tm = "ZZ";
    if ( $$('grid_tm').getSelectedId() != undefined )
    {
        jo_tm = $$('grid_tm').getItem($$('grid_tm').getSelectedId()).코드;
    }

    let send_json = 
    {
        "q_para":
        {
            "시장":jo_si,
            "etf구분":jo_etf,
        },
        "e_para":
        {
            "업종":jo_uj,
            "테마":jo_tm,
        }
    };	
    //send_json = {};
	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1020/get_jong_list", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);
		set_grid(text, $$("grid_jong"));
    });
}

function add_jong()
{
    var grid = $$("grid_jong_select")
    for (i=0 ; i<grid.count() ; i++)
    {
        if (grid.getItem(grid.getIdByIndex(i)).코드 == $$('grid_jong').getItem($$('grid_jong').getSelectedId()).코드)
        {
            webix.message(grid.getItem(grid.getIdByIndex(i)).코드+grid.getItem(grid.getIdByIndex(i)).코드명+" 같은 종목이 있습니다.");
            return 1;
        }
    }

    $$("grid_jong_select").add({
        "코드":$$('grid_jong').getItem($$('grid_jong').getSelectedId()).코드,
        "코드명":$$('grid_jong').getItem($$('grid_jong').getSelectedId()).코드명,
    }, 0);       
    
    $$("grid_jong_select").adjustColumn("코드", "all");
    $$("grid_jong_select").adjustColumn("코드명", "all");
    $$("grid_jong_select").refresh();    
}

function add_jong_all()
{
    var grid_1 = $$("grid_jong")
    var grid_2 = $$("grid_jong_select")
    for (i1=0 ; i1<grid_1.count() ; i1++)
    {
        let check_jong = false;
        for (i2=0 ; i2<grid_2.count() ; i2++)
        {
            if (grid_1.getItem(grid_1.getIdByIndex(i1)).코드 == grid_2.getItem(grid_2.getIdByIndex(i2)).코드)
            {
                webix.message(grid_2.getItem(grid_2.getIdByIndex(i2)).코드+grid_2.getItem(grid_2.getIdByIndex(i2)).코드명+" 같은 종목이 있습니다.");
                check_jong = true;
                break;
            }
        }

        if (check_jong == false)
        {
            $$("grid_jong_select").add({
                "코드":grid_1.getItem(grid_1.getIdByIndex(i1)).코드,
                "코드명":grid_1.getItem(grid_1.getIdByIndex(i1)).코드명,
            }, 0);                   
        }
    }
    
    $$("grid_jong_select").adjustColumn("코드", "all");
    $$("grid_jong_select").adjustColumn("코드명", "all");
    $$("grid_jong_select").refresh();    
}

function del_jong()
{
    $$("grid_jong_select").remove($$("grid_jong_select").getSelectedId());
}

function del_jong_all()
{
    $$('grid_jong_select').clearAll();
}

