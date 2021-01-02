function start_page()
{
    $$("state_2").setValue("기본 레포트 [rt1010]");

    //get_medo_list();
}

function clear_jo_edit()
{
    $$('bi_num').setValue('');
    $$('ju_num').setValue('');
    $$('hak').setValue('');
    $$('gunsu').setValue('');
    $$('sang_gijun').setValue('');
    $$('sang_pan').setValue('');
    $$('ha_pan').setValue('');
    $$('boday').setValue('');
}

function get_medo_list()
{
    let send_json = 
    {
        "q_para":
        {
            '비교일자1':$$('bi_num').getValue(), '비교일자2':$$('bi_num').getValue(), 
            '주기준1':$$('ju_num').getValue(), '주기준2':$$('ju_num').getValue(), 
            '확률1':$$('hak').getValue(), '확률2':$$('hak').getValue(), 
            '건수1':$$('gunsu').getValue(), '건수2':$$('gunsu').getValue(), 
            '상승기준1':$$('sang_gijun').getValue(), '상승기준2':$$('sang_gijun').getValue(), 
            '상승판매1':$$('sang_pan').getValue(), '상승판매2':$$('sang_pan').getValue(), 
            '하락판매1':$$('ha_pan').getValue(), '하락판매2':$$('ha_pan').getValue(), 
            '보유일1':$$('boday').getValue(), '보유일2':$$('boday').getValue(),            },        
        "e_para":
        {
            '테이블이름':$$('table_name').getValue(),
        }
    };	

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/bt1010/get_medo_list_detail", send_json, function(text)
        {
            set_grid(text, $$("grid_1"));
            win_loading_hide();
        });
    });
}

function get_medo_list_detail()
{
    let send_json = 
    {
        "q_para":
        {
            '비교일자1':$$('bi_num').getValue(), '비교일자2':$$('bi_num').getValue(), 
            '주기준1':$$('ju_num').getValue(), '주기준2':$$('ju_num').getValue(), 
            '확률1':$$('hak').getValue(), '확률2':$$('hak').getValue(), 
            '건수1':$$('gunsu').getValue(), '건수2':$$('gunsu').getValue(), 
            '상승기준1':$$('sang_gijun').getValue(), '상승기준2':$$('sang_gijun').getValue(), 
            '상승판매1':$$('sang_pan').getValue(), '상승판매2':$$('sang_pan').getValue(), 
            '하락판매1':$$('ha_pan').getValue(), '하락판매2':$$('ha_pan').getValue(), 
            '보유일1':$$('boday').getValue(), '보유일2':$$('boday').getValue(),            },        
        "e_para":
        {
            '테이블이름':$$('table_name').getValue(),
        }
    };	

    win_loading_show(function()
    {
        webix.ajax().headers({"Content-type":"application/json"}).post("/bt1010/get_medo_list_detail_hadan", send_json, function(text)
        {
            set_grid(text, $$("grid_2"));
            win_loading_hide();
        });
    });    
}

function get_medo_detail(gubun, medo_id, medo_jo)
{
    result_infor = medo_jo;
    let send_json = 
    {
        "q_para":
        {
            '비교일자1':$$('bi_num').getValue(), '비교일자2':$$('bi_num').getValue(), 
            '주기준1':$$('ju_num').getValue(), '주기준2':$$('ju_num').getValue(), 
            '확률1':$$('hak').getValue(), '확률2':$$('hak').getValue(), 
            '건수1':$$('gunsu').getValue(), '건수2':$$('gunsu').getValue(), 
            '상승기준1':$$('sang_gijun').getValue(), '상승기준2':$$('sang_gijun').getValue(), 
            '상승판매1':$$('sang_pan').getValue(), '상승판매2':$$('sang_pan').getValue(), 
            '하락판매1':$$('ha_pan').getValue(), '하락판매2':$$('ha_pan').getValue(), 
            '보유일1':$$('boday').getValue(), '보유일2':$$('boday').getValue(),            },        
        "e_para":
        {
            '테이블이름':$$('table_name').getValue(),
        }
    };	  
    webix.ajax().headers({"Content-type":"application/json"}).post("/bt1010/get_medo_detail_"+gubun, send_json, function(text)
    {  
        result_text = text;

        var url ="/view/rt1030.html"
        var title = "rt1030"+wid;
        wid = wid + 1;
        var status = "toolbar=no,directories=no,scrollbars=no,resizable=no,status=no,menubar=no,width=1500, height=800, top=0,left=20"
        
        var rt1050 = window.open(url, title, status);                    
    });
}
