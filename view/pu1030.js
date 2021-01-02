function start_page()
{
    let s_data = JSON.parse(opener.temp_child_str);

    $$("ju_gu").disable();

    $$("ju_jong_code").disable();
    $$("ju_meme_sun").disable();

    init_meme(s_data["단축코드"], s_data["종목명"], s_data["매매수량"], s_data["매매구분"], s_data["매매순번"]);
}

function init_meme(p_jong_code, p_jong_name, p_meme_su, p_meme_gu, p_meme_sun)
{
    $$('ju_label').setHTML("<center><h1>"+p_meme_gu+"</h1></center>");

    if (p_meme_gu == "매수")
    {
        $$('ju_medo_jo').enable();    
        $$('ju_medo_jo_ne').enable();    
    }
    else
    {
        $$('ju_medo_jo').disable();    
        $$('ju_medo_jo_ne').disable();    
    }

    $$("ju_gu").setValue(p_meme_gu);
    $$("ju_jong_code").setValue(p_jong_code);
    $$("ju_jong_name").setValue(p_jong_name);

    $$("ju_su").setValue(p_meme_su);
    $$("ju_ga").setValue("");

    $$('ju_day').setValue(get_today());        
    $$('ju_time').setValue("0910");        

    $$('ju_meme_sun').setValue(p_meme_sun);

    $$('ju_medo_jo').setValue("가격비교");

    let json_medo_jo_ne = {
        "매도조건명": "가격비교",
        "상승판매": "125",
        "하락판매": "90",
        "보유일": "35"
    };

    $$('ju_medo_jo_ne').setValue(JSON.stringify(json_medo_jo_ne, null, 2));

    get_ju();
    get_hoga(p_jong_code);
}

function get_hoga(p_jong_code)
{
    var send_json = {
		"para":{
			"exe_id":"get_hoga",
			"jong_code":p_jong_code
		}
	};
	webix.ajax().headers({"Content-type":"application/json"}).post("/python_exe", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);  
	
	    $$("grid_hoga").clearAll();   
		$$("grid_hoga").parse(jsd["data"]);
	});		            
}

function go_meme()
{
    if (($$("ju_ga").getValue() == "") || ($$("ju_su").getValue() == "") ||
        ($$("ju_ga").getValue() == "0") || ($$("ju_su").getValue() == "0") ||
        ($$("ju_ga").getValue() == null) || ($$("ju_su").getValue() == null)
        )
    {
        webix.message("수량, 단가를 확인하세요.");
        return 1;
    }


	webix.confirm({title:$$("ju_jong_name").getValue()+"", text:"정말 주문 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
            var send_json = {
                "para":{
                    "exe_id":"insert_db_and_meme",
                    "단축코드":$$("ju_jong_code").getValue(),
                    "매매구분":$$("ju_gu").getValue(),
                    "수량":$$("ju_su").getValue(),
                    "단가":$$("ju_ga").getValue(),
                    "금액구분":$$("ju_ga_gu").getValue(),
                    "예약일자":$$("ju_day").getValue(),
                    "예약시간":$$("ju_time").getValue(),
                    "매매순번":$$("ju_meme_sun").getValue(),
                    "매매조건명":$$("ju_medo_jo").getValue(),
                    "매매조건내역":$$("ju_medo_jo_ne").getValue(),
                }
            };
            webix.ajax().headers({"Content-type":"application/json"}).post("/python_exe", JSON.stringify(send_json), function(text)
            {  
                webix.message(text);
            });		    
		}
	}});      
}


function get_ju()
{
    var send_json = {};	

	webix.ajax().headers({"Content-type":"application/json"}).post("/pu1030/get_ju", JSON.stringify(send_json), function(text)
	{  
		set_grid(text, $$("grid_ju"));
	});	
}