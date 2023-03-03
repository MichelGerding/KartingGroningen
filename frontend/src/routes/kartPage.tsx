import {useParams} from "react-router-dom";
import React, {useEffect, useState} from "react";
import {ApiKartHeat, ApiKartResult, ApiLap} from "../rust";
import {ChartDataInput} from "../components/charts/chart";
import LapTimeChart from "../components/charts/laptimeChart";
import StyledTable, {TableDataType} from "../components/styledTable/styledTable";
import moment from "moment";
import BarChart from "../components/charts/barChart";

type params = {
    type: string;
    id: string;
}

export default function KartDetailPage() {
    const {id} = useParams<params>();
    const type = "karts";

    const [lapSeries, setLapSeries] = useState<ChartDataInput[]>([]);
    const [driverAmountSeries, setDriverAmountSeries] = useState<ChartDataInput[]>([]);
    const [tableData, setTableData] = useState<TableDataType[]>([]);
    const keys = ["heat_id", "fastest_lap_time", "avg_lap_time", "median_lap_time", "total_laps", "analyze-heat_id"]

    // get the data from the backend
    const fetchData = async () => {
        const response = await fetch(`/api/${type}/${id}/full`);
        let data = await response.json() as ApiKartResult;

        let newLapSeries: ChartDataInput[] = [];
        let newDriverAmountSeries: ChartDataInput[] = [];
        let newTableData: TableDataType[] = [];

        // group laps per day
        let lapsPerDay: { [keys: string]: any; } = {}
        data.heats.forEach((heat: ApiKartHeat) => {
            let date = moment(heat.start_date).format("YYYY-MM-DD");
            if (lapsPerDay[date] == undefined) {
                lapsPerDay[date] = [];
            }

            let sortedLaps = heat.laps.sort((a: ApiLap, b: ApiLap) => a.lap_time - b.lap_time);
            let median = sortedLaps[Math.floor(sortedLaps.length / 2)].lap_time;
            let fastest = sortedLaps[0].lap_time;
            let avg = heat.laps.reduce((acc: number, lap: ApiLap) => acc + lap.lap_time, 0) / heat.laps.length;


            // build the tabel
            newTableData.push({
                heat_id: heat.heat_id,
                fastest_lap_time: fastest,
                avg_lap_time: avg,
                median_lap_time: median,
                total_laps: sortedLaps.length,
            });


            lapsPerDay[date].push(...heat.laps.map((lap: ApiLap) => {
                return {
                    lap_time: lap.lap_time,
                    average: avg,
                    median: median,
                    fastest: fastest,
                    date: date,
                }
            }));
        });


        // calculate the average per day
        Object.keys(lapsPerDay).forEach((date: string) => {
            // sort laptimes from small to big
            let laps = lapsPerDay[date].sort((a: any, b: any) => a.lap_time - b.lap_time);

            let median = laps[Math.floor(laps.length / 2)].lap_time;
            let average = laps.reduce((a: any, b: any) => a + b.lap_time, 0) / laps.length;
            let amountOfLaps = laps.length;

            let fastestLap = laps[0].lap_time;
            newLapSeries.push({
                laptime: fastestLap,
                date: date,
                driver: "fastest",
            })
            newLapSeries.push({
                laptime: average,
                date: date,
                driver: "average",
            });
            newLapSeries.push({
                laptime: median,
                date: date,
                driver: "median",
            });
            // set the amount of new drivers
            newDriverAmountSeries.push({
                laptime: amountOfLaps,
                type: "column",
                date: date,
                driver: "Amount of laps driven",
            });

        });


        setTableData(newTableData);
        setDriverAmountSeries(newDriverAmountSeries);
        setLapSeries(newLapSeries);
    }

    useEffect(() => {
        fetchData().then (() => {});
    }, [type, id])

    const renderChart = () => {
        if (lapSeries.length > 0) {
            return (
                <>
                    <LapTimeChart
                        dataIn={lapSeries}
                        labelKey={"driver"}
                        xKey={"date"}
                        showOutlierFilter={false}/>
                </>
            )
        }
    }

    const renderDriverAmountChart = () => {
        if (driverAmountSeries.length > 0) {
            return (
                <div style={{width: "100%"}}>
                    <BarChart
                        dataIn={driverAmountSeries}
                        labelKey={"date"}/>
                </div>
            )
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
                }}>Kart: {id}</h1>
                <p> this page shows the performance of a kart over all heats.</p>

            </div>
            {renderChart()}
            <div style={{width: "100%"}}>
                    <h2 style={{textAlign: "left", width: "100%"}}> Laps per day</h2>
                <p> this chart shows the amount of laps a kart has driven per day.</p>
            </div>
            {renderDriverAmountChart()}

            <div style={{width: "100%"}}>
                <h2 style={{textAlign: "left", width: "100%"}}> General Overview</h2>
                <p>
                    This table shows the overal performance of the kart over all heats.
                    in this table you can find the heat id, fastest, average, median laptimes and the amount of laps driven of each heat.
                    followed by a button to analyze the heat.
                </p>
            </div>
            <StyledTable
                data={tableData}
                keys={keys}
                searchState={false}
                endpoint={"heats"}
                useSorting={true}
            />
        </div>
    );
}