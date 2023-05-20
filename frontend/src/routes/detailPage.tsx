import {useParams} from "react-router-dom";
import React, {useEffect, useState} from "react";
import {ApiDriverResult, ApiHeatResult} from "../rust";
import {ChartDataInput} from "../components/charts/chart";
import LapTimeChart from "../components/charts/laptimeChart";
import LapTimeBarChart from "../components/charts/LaptimeBarChart";
import StyledTable, {TableDataType} from "../components/styledTable/styledTable";

type params = {
    type: string;
    id: string;
}


const dataTypeOptions: {
    [key: string]: {
        keys:string[];
        chartLabel: string;
        tableEndpoint: string;
    };
} = {
    heats: {
        keys: ["name", "fastest_lap_time", "avg_lap_time", "median_lap_time", "total_laps", "analyze-name"],
        chartLabel: "driver",
        tableEndpoint: "drivers",

    },
    drivers: {
        keys: ["start_time", "amount_of_laps", "fastest_lap_time", "average_lap_time", "analyze-heat_id"],
        chartLabel: "date",
        tableEndpoint: "heats",
    },
};

export default function DetailPage() {
    let {type, id} = useParams<params>();
    let [chartData, setChartData] = useState<ChartDataInput[]>([]);
    let [tableData, setTableData] = useState<TableDataType[]>([]);


    // get the data from the backend
    const fetchData = async () => {
        const response = await fetch(`${import.meta.env.VITE_API_BASEURL}/api/${type}/${id}/full`);
        let data = await response.json() as any;

        let newChartData: ChartDataInput[] = [];
        let newTableData: TableDataType[] = [];


        switch (type) {
            case "drivers":
                let driverData = data as ApiDriverResult;
                driverData.heats.forEach((heat) => {
                    return heat.laps.forEach((lap) => {
                        newChartData.push({
                            date: heat.start_date.toString(),
                            value: lap.lap_time,
                            kart: heat.kart,
                            lap_in_heat: lap.lap_number,
                        } as ChartDataInput);
                    })
                });

                driverData.heats.forEach((heat) => {
                    let fastestLap = 999;
                    let totalTime = 0;
                    let laps = heat.laps.length;
                    heat.laps.forEach((lap) => {
                        if (lap.lap_time < fastestLap) {
                            fastestLap = lap.lap_time;
                        }
                        totalTime += lap.lap_time;
                    });
                    let avg = totalTime / laps;

                    newTableData.push({
                        heat_id: heat.heat_id,
                        start_time: heat.start_date.toLocaleString("nl-NL"),
                        amount_of_laps: heat.laps.length,
                        fastest_lap_time: fastestLap,
                        average_lap_time: avg,
                    });
                })
                break;
            case "heats":
                let heatData = data as ApiHeatResult;
                heatData.results.forEach((result) => {
                    return result.laps.forEach((lap) => {
                        newChartData.push({
                            lap_in_heat: lap.lap_number,
                            date: data.start_time,
                            driver: result.driver.driver_name,
                            value: lap.lap_time,
                            kart: result.kart,
                        } as ChartDataInput);
                    })
                });

                heatData.results.forEach((result) => {
                    let fastestLap = 999;
                    let totalTime = 0;
                    let laps = result.laps.length;
                    result.laps.forEach((lap) => {
                        if (lap.lap_time < fastestLap) {
                            fastestLap = lap.lap_time;
                        }
                        totalTime += lap.lap_time;
                    });
                    let avgLap = totalTime / laps;

                    let medianLap = result.laps.sort((a, b) => a.lap_time - b.lap_time)[Math.floor(laps / 2)].lap_time;

                    newTableData.push({
                        name: result.driver.driver_name,
                        fastest_lap_time: fastestLap,
                        avg_lap_time: avgLap,
                        median_lap_time: medianLap,
                        total_laps: result.laps.length,
                    })
                });

                break;

        }


        setChartData(newChartData);
        setTableData(newTableData);
    }
    useEffect(() => {
        fetchData().then(() => {});
    }, [type, id])

    const pageOptions = dataTypeOptions[type ?? ""];

    const renderChart = () => {
        //TODO:: make the text adapt to the type of page
        if (chartData.length > 0) {
            return (
                <>
                    <LapTimeChart
                        dataIn={chartData}
                        labelKey={pageOptions.chartLabel}/>
                    <LapTimeBarChart
                        showVSC={type == "heats"}
                        dataIn={chartData}
                        labelKey={pageOptions.chartLabel}/>
                </>
            )
        }
    }

    const renderTableText = () => {
        switch (type) {
            case "drivers":
                return (
                    <div>
                        <h2>Overal performances</h2>
                        <p>
                            This table shows the overal performance of the driver over all heats.
                            in this table you can find the amount of laps driven, fastest and average lap time for the specific heat.
                            followed by a button to analyze the heat
                        </p>
                    </div>
                );
            case "heats":
                return (
                    <div>
                        <h2>Overal performances</h2>
                        <p>
                            This table shows the overal performance of the drivers in the current heat.
                            in this table you can find the name of the driver, fastest, average, median laptimes and the amount of laps driven.
                            followed by a button to analyze the driver
                        </p>
                    </div>
                );

        }
    }

    let dataType = (type ?? "drivers").slice(0, -1);
    dataType = dataType.charAt(0).toUpperCase() + dataType.slice(1);

    const renderSubtitle = () => {
        switch (type) {
            case "drivers":
                return (
                    <p>this page gives a detailed view of the drivers performance</p>
                );
            case "heats":
                return (
                    <p>this page shows the performance of the drivers in this heat</p>
                );
        }
    }

    return (
        <div style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            maxWidth: '1000px',
            margin: '4em auto',
        }}>
            <div style={{
                display: "flex",
                width: "100%",
                flexDirection: "column",
                alignItems: "start",
            }}>
                <h1 style={{
                    fontSize: "2em",
                    fontWeight: "bold",
                    margin: "0",
                }}>{dataType}: {id}</h1>
                {renderSubtitle()}

            </div>

            {renderChart()}

            {renderTableText()}
            <StyledTable
                data={tableData}
                keys={pageOptions.keys}
                searchState={false}
                endpoint={pageOptions.tableEndpoint}
                useSorting={true}
            />
        </div>
    );
}