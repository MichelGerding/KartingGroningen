import {ChartDataInput, IDashboardComp} from "../charts/chart";
import {ApiDriverResult} from "../../rust";
import LapTimeChart from "../charts/laptimeChart";
import {GlobalData, loadData} from "./index";

export const renderStatic = async ({dataType, id, showText, title, type}: IDashboardComp, globalData: GlobalData) => {

    let data = (await loadData(globalData, dataType, id)) as ApiDriverResult;
    // parse data
    let finalData: ChartDataInput[] = [];

    data.heats.forEach((heat) => {
        return heat.laps.forEach((lap) => {
            finalData.push({
                date: heat.start_date.toString(),
                value: lap.lap_time,
                kart: heat.kart,
                lap_in_heat: lap.lap_number,
            } as ChartDataInput);
        })
    });


    // base 64 encode datatype, title, and id
    const base64 = btoa(`${dataType}/${id}/${title}/${type}`);


    // render chart
    return (
        <LapTimeChart
            key={base64}
            dataIn={finalData}
            labelKey={"date"}
            showText={showText}/>
    )
}