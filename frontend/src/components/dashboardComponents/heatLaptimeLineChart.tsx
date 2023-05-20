import {ChartDataInput, IDashboardComp} from "../charts/chart";
import {ApiHeatResult} from "../../rust";
import LapTimeChart from "../charts/laptimeChart";

import {GlobalData, loadData} from "./index";

export const renderStatic = async ({dataType, id, showText, title, type}: IDashboardComp, globalData: GlobalData) => {
    let finalData: ChartDataInput[] = [];
    let data = (await loadData(globalData, dataType, id)) as ApiHeatResult

    data.results.forEach((result) => {
        return result.laps.forEach((lap) => {
            finalData.push({
                lap_in_heat: lap.lap_number,
                date: data.start_time,
                driver: result.driver.driver_name,
                value: lap.lap_time,
                kart: result.kart,
            } as any as ChartDataInput);
        })
    });

    // base 64 encode datatype, title, and id
    const base64 = btoa(`${dataType}/${id}/${title}/${type}`);


    // render chart
    return (
        <LapTimeChart
            key={base64}
            dataIn={finalData}
            labelKey={"driver"}
            showText={showText}/>
    )
}