function start_page()
{
    $$("state_2").setValue("주식서버/DB 비교 [ju1010]");
	
    $$('meme_start').setValue(get_pm_mon(get_today(), -12));
    $$('meme_end').setValue(get_today()); 	 

	var send_json = {
		"para":{
			"exe_id":"get_account_amt",
			"q_para":{}
		}
	};
	webix.ajax().headers({"Content-type":"application/json"}).post("/python_exe", JSON.stringify(send_json), function(text)
	{  
		jsd = JSON.parse(text);

        $$("state_1").setValue(
            "   예탁자산:" + comma1000(jsd["예탁자산총액"])+
            "   평가금액:" + comma1000(jsd["잔고평가금액"])+
            "   주문가능:" + comma1000(jsd["현금주문가능금액"])+
            "   출금가능:" + comma1000(jsd["출금가능금액"])+
            "   손익:" + comma1000(jsd["손익율"])
		);		
	});		    

	get_list_bo();    
	get_list_bo_db();    
	get_list_meme();    

    $$("text_mesu_jo_name").disable();
    $$("textarea_mesu_jo_ne").disable();
}

function get_list_bo()
{
	var send_json = {
		"para":{
			"exe_id":"get_list_bo",
			"q_para":{}
		}
	};
	webix.ajax().headers({"Content-type":"application/json"}).post("/python_exe", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);  
	
	    $$("grid_js").clearAll();   
		$$("grid_js").parse(jsd["data"]);

		set_su();
	});		    
}


function get_list_bo_db()
{
    var send_json = {};	

	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/get_list_bo", JSON.stringify(send_json), function(text)
	{  
		set_grid(text, $$("grid_db"));

		set_su();
	});	
}

function get_list_meme()
{
	var send_json = 
	{
		"q_para":
		{
			"시작일자":$$("meme_start").getValue(),
			"종료일자":$$("meme_end").getValue(),
		}
	};	

	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/get_list_meme", JSON.stringify(send_json), function(text)
	{  
		set_grid(text, $$("grid_meme"));
	});	
}

function set_su()
{
	let grid1 = $$("grid_js");
	let grid2 = $$("grid_db");

	for (i=0 ; i<grid1.count() ; i++)
	{    
		grid1.getItem(grid1.getIdByIndex(i)).db = 0;

		for (j=0 ; j<grid2.count() ; j++)
		{
			if (grid1.getItem(grid1.getIdByIndex(i)).단축코드 === grid2.getItem(grid2.getIdByIndex(j)).단축코드)
			{
				grid1.getItem(grid1.getIdByIndex(i)).db = grid2.getItem(grid2.getIdByIndex(j)).수량;
				// if (grid1.getItem(grid1.getIdByIndex(i)).잔고 != grid1.getItem(grid1.getIdByIndex(i)).db)
				// {
				// 	grid1.getItem(grid1.getIdByIndex(i)).차이 = grid1.getItem(grid1.getIdByIndex(i)).잔고 - grid1.getItem(grid1.getIdByIndex(i)).db;
				// }

				break;;
			}	
		}		
	}	
	for  (i=0 ; i<grid1.count() ; i++)
		if (grid1.getItem(grid1.getIdByIndex(i)).잔고 != grid1.getItem(grid1.getIdByIndex(i)).db)
			grid1.getItem(grid1.getIdByIndex(i)).차이 = grid1.getItem(grid1.getIdByIndex(i)).잔고 - grid1.getItem(grid1.getIdByIndex(i)).db;
		else
		grid1.getItem(grid1.getIdByIndex(i)).차이 = 0;

	grid1.refresh();

	

	for (i=0 ; i<grid2.count() ; i++)
	{    
		grid2.getItem(grid2.getIdByIndex(i)).주식서버 = 0;

		for (j=0 ; j<grid1.count() ; j++)
		{
			if (grid2.getItem(grid2.getIdByIndex(i)).단축코드 === grid1.getItem(grid1.getIdByIndex(j)).단축코드)
			{
				grid2.getItem(grid2.getIdByIndex(i)).주식서버 = grid1.getItem(grid1.getIdByIndex(j)).잔고;
				// if (grid2.getItem(grid2.getIdByIndex(i)).수량 != grid2.getItem(grid2.getIdByIndex(i)).주식서버)
				// {
				// 	grid2.getItem(grid2.getIdByIndex(i)).차이 = grid2.getItem(grid2.getIdByIndex(i)).수량 - grid2.getItem(grid2.getIdByIndex(i)).주식서버;
				// }
		
				break;;
			}	
		}		
	}	
	for  (i=0 ; i<grid2.count() ; i++)
		if (grid2.getItem(grid2.getIdByIndex(i)).수량 != grid2.getItem(grid2.getIdByIndex(i)).주식서버)
			grid2.getItem(grid2.getIdByIndex(i)).차이 = grid2.getItem(grid2.getIdByIndex(i)).수량 - grid2.getItem(grid2.getIdByIndex(i)).주식서버;
		else
			grid2.getItem(grid2.getIdByIndex(i)).차이 = 0;

	grid2.refresh();
}


function get_for_modify()
{

	//let jong_code = $$('grid_db').getItem($$('grid_db').getSelectedId()).단축코드;
	let jong_code = $$('grid_modify').단축코드;

	var send_json = 
	{
		"q_para":
		{
			"단축코드":jong_code
		}
	};	
	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/get_bo_for_modify", JSON.stringify(send_json), function(text)
	{  
		var jsd = JSON.parse(text);  
		
	    $$("grid_modify").clearAll();   
		$$("grid_modify").parse(jsd["data"]);
		$$('text_medo_jo_name').setValue("");
		$$('textarea_medo_jo_ne').setValue("");
		$$('text_mesu_jo_name').setValue("");
		$$('textarea_mesu_jo_ne').setValue("");
	});	
}

function update_meme()
{
	if ($$("grid_modify").getSelectedId() == undefined)
	{
		webix.message("매매순번을 선택을 하세요");
		return 1;
	}

	var send_json = 
	{
		"q_para":
		{
			"매매순번":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매매순번+"",
			"매수수량":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매수수량+"",
			"실매수수량":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).실매수수량+"",
			"매수단가":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매수단가+"",
			"실매수단가":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).실매수단가+"",
			"매도수량":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매도수량+"",
			"실매도수량":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).실매도수량+"",
			"매도단가":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매도단가+"",
			"실매도단가":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).실매도단가+"",
			"매도조건명":$$("text_medo_jo_name").getValue()+"",
			"매도조건내역":$$("textarea_medo_jo_ne").getValue()+"",
		}
	};	
	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/update_meme", JSON.stringify(send_json), function(text)
	{  
		//$$('win_modify').hide();
		get_for_modify();
		get_list_bo_db();
		get_list_meme();    
	});	
}


function new_meme()
{
	if ($$("grid_modify").getSelectedId() == undefined)
	{
		webix.message("매매순번을 선택을 하세요");
		return 1;
	}

	var send_json = 
	{
		"q_para":
		{
			"매매순번":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매매순번+"",
		}
	};	
	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/new_meme", JSON.stringify(send_json), function(text)
	{  
		//$$('win_modify').hide();
		get_for_modify();
		get_list_bo_db();
		get_list_meme();    
	});	
}

function del_meme()
{
	if ($$("grid_modify").getSelectedId() == undefined)
	{
		webix.message("매매순번을 선택을 하세요");
		return 1;
	}

	webix.confirm({title:$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매매순번+"", text:"정말 삭제 하시겠습니까?", callback:function(result)
	{
		if (result === true)
		{
			var send_json = 
			{
				"q_para":
				{
					"매매순번":$$("grid_modify").getItem($$("grid_modify").getSelectedId()).매매순번+"",
				}
			};	
			webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/del_meme", JSON.stringify(send_json), function(text)
			{  
				//$$('win_modify').hide();
				get_for_modify();
				get_list_bo_db();
				get_list_meme();    
			});	
		}
	}});    
}

function new_mesu()
{

	var send_json = 
	{
		"q_para":
		{
			"단축코드":$$("grid_js").getItem($$("grid_js").getSelectedId()).단축코드+"",
			"매수수량":$$("grid_js").getItem($$("grid_js").getSelectedId()).잔고+"",
			"매수단가":$$("grid_js").getItem($$("grid_js").getSelectedId()).현재가+"",
		}
	};	
	webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/new_mesu", JSON.stringify(send_json), function(text)
	{  
		//$$('win_modify').hide();
		//get_for_modify();
		get_list_bo_db();
		get_list_meme();    
	});	
}


function all_new_mesu()
{

	let grid1 = $$("grid_js");
	
	for (i=0 ; i<grid1.count() ; i++)
	{    
		if (grid1.getItem(grid1.getIdByIndex(i)).db == 0)
		{
			var send_json = 
			{
				"q_para":
				{
					"단축코드":grid1.getItem(grid1.getIdByIndex(i)).단축코드+"",
					"매수수량":grid1.getItem(grid1.getIdByIndex(i)).잔고+"",
					"매수단가":grid1.getItem(grid1.getIdByIndex(i)).현재가+"",
				}
			};	
			webix.ajax().headers({"Content-type":"application/json"}).post("/ju1010/new_mesu", JSON.stringify(send_json), function(text)
			{  
				//$$('win_modify').hide();
				//get_for_modify();								
			});	
		}
	}		

	$$('pop_1').hide();
}

