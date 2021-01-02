function start_page()
{
    let s_data = JSON.parse(opener.temp_child_str);
    
    $$('tms2').setHTML(s_data["jong_list_name"]+"<br><br>"+s_data["day_start"]+" ~ "+s_data["day_end"]);

    let send_json = 
    {
        "q_para":
        {
            "시작일자":s_data["day_start"],
            "종료일자":s_data["day_end"],
        },
        "e_para":
        {
            "단축코드":s_data["jong_list"],
        }
    };	    

	webix.ajax().headers({"Content-type":"application/json"}).post("/pu1080/get_chart_basic", JSON.stringify(send_json), function(text)
	{  
        let jsd = JSON.parse(text)["data"];

        let max_p = parseInt(jsd[0].최고주가);
        let min_p = parseInt(jsd[0].최저주가);
        let step_10 = parseInt((max_p - min_p) / 5);

        $$("chart_1").removeAllSeries();
        $$("chart_1").addSeries({  type:"line", value:"#주가#", item:{ radius:0 }, 
                                    line:{ color:"#1293f8", width:3 },
                                    yAxis:{ lineColor:"#cccccc", start:min_p, end:max_p, step:step_10 },
                                    xAxis:{ template: "#월단위#", lines: false }
                                });

        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1080/get_chart", send_json, function(text2)
        {  
            var jsd2 = JSON.parse(text2)["data"];

            $$('chart_1').clearAll(); 
            $$('chart_1').parse(jsd2);
        });               
    });    



    webix.ajax().headers({"Content-type":"application/json"}).post("/pu1080/get_ger_chart_basic", send_json, function(text)
    {  
        var jsd = JSON.parse(text)["data"];

        let value_max = Math.max(parseInt(jsd[0]["개인max"]), parseInt(jsd[0]["외국인max"]), parseInt(jsd[0]["외국계max"]), parseInt(jsd[0]["기관max"]), parseInt(jsd[0]["프로그램max"]));
        let value_min = Math.min(parseInt(jsd[0]["개인min"]), parseInt(jsd[0]["외국인min"]), parseInt(jsd[0]["외국계min"]), parseInt(jsd[0]["기관min"]), parseInt(jsd[0]["프로그램min"]));
        let val_term = Math.max(Math.abs(value_min), Math.abs(value_max));
        let val_len = (val_term).toString().length;

        webix.message(jsd[0]["개인max"]);

        let max_ja = 1;
        for (i=0 ; i<val_len-1 ; i++)
            max_ja = max_ja * 10;
        
        let max_num = Math.round(val_term/max_ja) * max_ja;

        $$("chart_2").removeAllSeries();
        $$("chart_2").addSeries({  type:"line", value:"#누적거래량#", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc"},
        xAxis:{ template: "#월단위#", lines: false }
        });

        $$("chart_3").removeAllSeries();
        $$("chart_3").addSeries({  type:"line", value:"#개인누적#", color:"#eeeeee", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc", start:max_num*-1, end:max_num, step:max_num },                                    
        xAxis:{ template: "#월단위#", lines: false }
        });
                    
        $$("chart_4").removeAllSeries();
        $$("chart_4").addSeries({  type:"line", value:"#기관누적#", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc", start:max_num*-1, end:max_num, step:max_num },                                    
        xAxis:{ template: "#월단위#", lines: false }
        });

        $$("chart_5").removeAllSeries();
        $$("chart_5").addSeries({  type:"line", value:"#외국인누적#", color:"#eeeeee", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc", start:max_num*-1, end:max_num, step:max_num },                                    
        xAxis:{ template: "#월단위#", lines: false }
        });

        $$("chart_6").removeAllSeries();
        $$("chart_6").addSeries({  type:"line", value:"#외국계누적#", color:"#eeeeee", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc", start:max_num*-1, end:max_num, step:max_num },                                    
        xAxis:{ template: "#월단위#", lines: false }
        });

        $$("chart_7").removeAllSeries();
        $$("chart_7").addSeries({  type:"line", value:"#프로그램누적#", color:"#eeeeee", item:{ radius:0 }, 
        line:{ color:"#1293f8", width:3 },
        yAxis:{ lineColor:"#cccccc", start:max_num*-1, end:max_num, step:max_num },                                    
        xAxis:{ template: "#월단위#", lines: false }
        });

        webix.ajax().headers({"Content-type":"application/json"}).post("/pu1080/get_ger_chart", send_json, function(text3)
        {  
            var jsd3 = JSON.parse(text3)["data"];

            $$('chart_2').clearAll(); 
            $$('chart_2').parse(jsd3);
            
            $$('chart_3').clearAll(); 
            $$('chart_3').parse(jsd3);    

            $$('chart_4').clearAll(); 
            $$('chart_4').parse(jsd3);    

            $$('chart_5').clearAll(); 
            $$('chart_5').parse(jsd3);    

            $$('chart_6').clearAll(); 
            $$('chart_6').parse(jsd3);    

            $$('chart_7').clearAll(); 
            $$('chart_7').parse(jsd3);  
        });         
    });       
}