function start_page()
{
    let s_data = JSON.parse(opener.temp_child_str);

    if ((s_data["day_start"]== "") || (s_data["day_start"]==undefined) || (s_data["day_start"]==null))
        s_data["day_start"] = get_pm_mon(get_today(), -12);

    if ((s_data["day_end"]== "") || (s_data["day_end"]==undefined) || (s_data["day_end"]==null))
        $$('day_end').setValue(get_today());        

    let send_json = 
    {
        "q_para":
        {
            "단축코드":s_data["jong_code"],
            "시작일자":s_data["day_start"],
            "종료일자":s_data["day_end"],
        }
    };	    

	webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_chart_basic", JSON.stringify(send_json), function(text)
	{  
        let jsd = JSON.parse(text)["data"];

        let temp_str =
            '<b>'+jsd[0].단축코드+'</b> <br><b>'+jsd[0].종목명+
            '</b> <p>시작일자:<b>'+format_date(s_data["day_start"])+'</b> <br>종료일자:<b>'+format_date(s_data["day_end"])+
            '</b> <br>평균거래량:<b>'+comma1000(jsd[0].평균거래)+
            '</b> <p>시가총액:<b>'+comma1000(jsd[0].시가총액)+'</b>'+
            '<br>상장주식:<b>'+comma1000(jsd[0].상장주식)+'</b>'+
            '<p>최고 주가:<b>'+comma1000(jsd[0].최고주가)+'</b>'+
            '<br>최저 주가:<b>'+comma1000(jsd[0].최저주가)+'</b>'+
            '<p>52주 최고:<b>'+comma1000(jsd[0].최고52주)+'</b>'+
            '<br>52주 최저:<b>'+comma1000(jsd[0].최저52주)+'</b>'+
            '<p>외국인 보유:<b>'+comma1000(jsd[0].외국인보유)+'</b>'+
            '<br>per:<b>'+comma1000(jsd[0].per)+'</b>'+
            '<br>pbr:<b>'+comma1000(jsd[0].pbr)+'</b>'+
            '<br>최고/최저:<b>'+  Math.round(((jsd[0].최고주가-jsd[0].최저주가)/jsd[0].최고주가)*100)+'%</b>';

        $$('tms2').setHTML(temp_str);

        let max_p = parseInt(jsd[0].최고주가);
        let min_p = parseInt(jsd[0].최저주가);
        let step_10 = parseInt((max_p - min_p) / 10);
        var max_g = parseInt(jsd[0].최고거래);

        $$("chart_1").removeAllSeries();
        $$("chart_1").addSeries({  type:"line", value:"#주가#", color:"#36abee", item:{ radius:0 }, 
                                    line:{ color:"#1293f8", width:3 },
                                    yAxis:{ lineColor:"#cccccc", start:min_p, end:max_p, step:step_10 },
                                    xAxis:{ template: "#월단위#", lines: false }
                                });

        $$("chart_1").addSeries({  type:"line", value:"#주가30일#", color:"#eeeeee", item:{ radius:0 }, 
                                line:{ color:"#de619c", width:1 },
                                yAxis:{ start:min_p, end:max_p, step:step_10 },
                                xAxis:{ template: "#월단위#", lines: false }
                            });
                            
        $$("chart_1").addSeries({  type:"line", value:"#주가60일#", color:"#eeeeee", item:{ radius:0 }, 
                                line:{ color:"#ff8c00", width:1 },
                                yAxis:{ start:min_p, end:max_p, step:step_10 },
                                xAxis:{ template: "#월단위#", lines: false }
                            });

        $$("chart_2").removeAllSeries();
        $$("chart_2").addSeries({  type:"line", value:"#거래량#", color:"#eeeeee", item:{ radius:0 }, 
                            yAxis:{ start:0, end:max_g, step:Math.round(max_g/3) },
                            xAxis:{ template: "#월단위#", lines: false }
        });

        webix.ajax().headers({"Content-type":"application/json"}).post("/bc1010/get_chart", send_json, function(text2)
        {  
            var jsd2 = JSON.parse(text2)["data"];

            $$('chart_1').clearAll(); 
            $$('chart_1').parse(jsd2);            

            $$('chart_2').clearAll(); 
            $$('chart_2').parse(jsd2);    
        });     
    });

}



