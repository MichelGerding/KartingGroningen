import LapTimeBarChart from "../charts/LaptimeBarChart";
import {ChartDataInput, IDashboardComp} from "../charts/chart";
import {ApiHeatResult} from "../../rust";
import {GlobalData, loadData} from "./index";

export const renderStatic = async ({dataType, id, showText, title, type, showVsc = true, demo = false}: IDashboardComp, globalData: GlobalData) => {

    let data = (await loadData(globalData, dataType, id)) as ApiHeatResult;

    // parse data
    let finalData: ChartDataInput[] = [];
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


    const base64 = btoa(`${dataType}/${id}/${title}/${type}`);

    // render chart
    return (
        <LapTimeBarChart
            key={base64}
            showVSC={true}
            dataIn={finalData}
            labelKey={"driver"}
            showText={showText}
            demo={demo}/>
    )
}

