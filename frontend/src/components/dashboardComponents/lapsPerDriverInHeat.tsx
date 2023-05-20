import {ChartDataInput, IDashboardComp} from "../charts/chart";
import {ApiHeatResult} from "../../rust";
import BarChart from "../charts/barChart";
import {GlobalData, loadData} from "./index";


export const renderStatic = async ({dataType, id, title, type, dataModificationScript}: IDashboardComp, globalData: GlobalData) => {
    let finalData: ChartDataInput[] = [];
    let data = (await loadData(globalData, dataType, id)) as ApiHeatResult;
    data.results.forEach((result) => {
        finalData.push({
            value: result.laps.length,
            type: 'column',
            driver: result.driver.driver_name,
        }) ;
    });

    const base64 = btoa(`${dataType}/${id}/${title}/${type}`);


    // render chart
    return (
        <BarChart
            key={base64}
            dataIn={finalData}
            labelKey={"driver"}/>
    )
}

