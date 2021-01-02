function start_page()
{
    set_grid(opener.window.result_text, $$('grid_1'));
    // tput1(opener.window.result_infor);

    $$('day_start').setValue(get_pm_mon(get_today(), -10));
    $$('day_end').setValue(get_today());       
    
    $$('combobox_uj').disable();
    $$('combobox_tm').disable();            
    $$('combobox_si').disable();
    $$('combobox_etf').disable();            
}


function get_set_jong_name()
{

}