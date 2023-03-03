import {ChartDataInput} from "./chart";
import Chart from "react-apexcharts";
import {useResizeDetector} from "react-resize-detector";
import DropdownSelect from "../dropdown/dropdown";
import {useEffect, useState} from "react";
import {getUserLocale} from "get-user-locale";
import moment from "moment/moment";

interface LapTmeChartProps {
    dataIn: ChartDataInput[];
    xKey?: string;
    labelKey: string;
    showOutlierFilter?: boolean;
}

interface FilterByLabel {
    [key: string]: ChartDataInput[];
}

export default function LapTimeChart({dataIn, labelKey, xKey = "lap_in_heat", showOutlierFilter= true}: LapTmeChartProps) {
    //TODO:: place dots where the prev and next laptimes are null

    // parse the data
    let data: FilterByLabel = {};
    dataIn.forEach((d) => {
        let k = d[labelKey];
        // check if k is a valid date
        if (labelKey === "date") {

            const date = moment(k, "YYYY-MM-DDTHH:mm");
            if (date.isValid()) {
                k = date.toDate().toLocaleString(
                    getUserLocale({fallbackLocale: "en-US", useFallbackLocale: true}) as string,
                    {
                        year: "numeric",
                        month: "long",
                        day: "numeric",
                        weekday: "long",
                        hour: "numeric",
                        minute: "numeric",
                        hour12: false
                    }
                );
            }


        }
        if (data[k as string] === undefined) {
            data[k as string] = [];
        }
        data[k as string].push(d);
    });

    const {width, ref} = useResizeDetector();
    const [showVSC, setShowVSC] = useState(true);
    const [series, setSeries] = useState<ApexAxisChartSeries>([]);
    const [shownLabels, setShownLabels] = useState<string[]>(Object.keys(data));
    const [annotations, setAnnotations] = useState<ApexAnnotations>({});


    const stdDev = Math.sqrt(dataIn.reduce((acc, d) => acc + Math.pow(d.laptime - dataIn.reduce((acc, d) => acc + d.laptime, 0) / dataIn.length, 2), 0) / dataIn.length);
    // get the minimum in for the charts of type line
    const fastestLapTime = Math.min(...dataIn.filter((d) => d.type === "line" || d.type === undefined).map((d) => d.laptime));
    const slowestLapTime = Math.max(...dataIn.filter((d) => d.type === "line" || d.type === undefined).map((d) => d.laptime));

    const options = {
        chart: {
            id: "laptimes",
            events: {
                legendClick: function (chartContext: unknown, seriesIndex: unknown) {
                    // @ts-ignore
                    const series = chartContext.series.w.globals.seriesNames;
                    const label = series[seriesIndex as number];

                    // toggle the label
                    if (shownLabels.includes(label)) {
                        setShownLabels(shownLabels.filter((l) => l !== label));
                    } else {
                        setShownLabels([...shownLabels, label]);
                    }

                }
            },
        },
        zoom: {
            enabled: true,
            type: 'x',
            autoScaleYaxis: true
        },
        xaxis: {
            categories: Array.from(Array(
                Math.max(
                    ...dataIn.map((d) => d.lap_in_heat ? d.lap_in_heat : 0))
            ).keys()).map((i) => i + 1),
        },
        yaxis: {
            min: fastestLapTime - Math.min(5, stdDev),
            max: slowestLapTime + 5,
            labels: {
                formatter: (val: number) => {
                    if (val !== null) {
                        return `${val.toFixed(3)}s`;
                    }
                    return "N/A";
                }
            }
        },
        annotations: annotations,
        stroke: {
            width: 2
        },
        markers: {
            size: 5,
        }
    }



    // reset the shown labels when the data changes
    // the onld ones are for different data then we currently have loaded
    useEffect(() => {
        setShownLabels(Object.keys(data));
    }, [dataIn]);

    useEffect(() => {
        let series = Object.keys(data).map((label) => {
            return {
                name: label,
                type: data[label][0].type ?? "line",
                data: data[label].map((d) => {
                    if (showVSC) {
                        return {
                            x: d[xKey],
                            y: d.laptime,
                        };
                    } else {
                        const avgLaptime = data[label].reduce((acc, d) => acc + d.laptime, 0) / data[label].length;
                        const std = Math.sqrt(data[label].reduce((acc, d) => acc + Math.pow(d.laptime - avgLaptime, 2), 0) / data[label].length);
                        return {
                            x: d.lap_in_heat,
                            y: (d.laptime - avgLaptime) < std ? d.laptime : null,
                        }
                    }
                })
            }
        });


        let labelsAreDates = moment(series[0].data[0].x, "YYYY-MM-DDTHH:mm").isValid();
        // sort all the data by lap in heat
        for (let i = 0; i < series.length; i++) {
            series[i].data = series[i].data.sort((a: any, b: any) => {
                if (labelsAreDates) {
                    return moment(a.x, "YYYY-MM-DDTHH:mm").diff(moment(b.x, "YYYY-MM-DDTHH:mm"));
                }
                return a.x - b.x
            });
        }


        // get the average laptime  of active labels. only take the ones that are a line chart
        const filteredSeries = series.filter((s) => shownLabels.includes(s.name) && (s.type ?? "line") == "line");
        let usedLaps = 0;
        const totalLaptime = filteredSeries.reduce((acc, s) => acc + s.data.filter((d) => d.x !== null).reduce((acc, d) => {
            if (d.y == null) {
                return acc;
            }

            usedLaps++;
            return acc + d.y;
        }, 0), 0);
        const avgLaptime = totalLaptime / usedLaps;

        // const totalLaptime = series.filter((s) => shownLabels.includes(s.name)).reduce((acc, s) => acc + s.data.filter((d) => d.x !== null).reduce((acc, d) => acc + (d.y == null ? 0 : d.y), 0), 0);
        // const usedLaps = series.filter((s) => shownLabels.includes(s.name)).reduce((acc, s) => acc + s.data.length, 0);
        // const avgLaptime = totalLaptime / usedLaps;
        setAnnotations({
                yaxis: [{
                    y: avgLaptime,
                    borderColor: '#00E396',
                    borderWidth: 4,
                    label: {
                        borderColor: '#00E396',
                        style: {
                            color: '#fff',
                            background: '#00E396'
                        },
                        text: `average laptime: ${avgLaptime.toFixed(2)}`
                    }
                }]
            }
        )
        setSeries(series);
    }, [showVSC, shownLabels, dataIn]);

    const renderDropdown = () => {
        if (showOutlierFilter) {
            return <DropdownSelect
                options={[
                    {value: "false", label: "Hidden"},
                    {value: "true", label: "Shown"}
                ]}
                onChange={(e) => {
                    setShowVSC(e.target.value === "true")
                }}
                defaultValue={showVSC ? "true" : "false"}
                label={"Outlier Laps"}
            />
        }
    }

    const chartWidth = width ?? 500;
    const aspectRatio = 9 / 22;
    const chartHeight = chartWidth * aspectRatio;
    return (
        <div ref={ref}>
            <h2> All laps</h2>
            <p>
                The chart below shows all laps of the selected drivers. The average laptime is marked with a green line.
                To show the average laptime of a subset of drivers, click on the legend to toggle the drivers.
                You can also zoom in and out of the chart by clicking and dragging on the chart.<br/>
                <br/>
                finally you can toggle the outlier laps by clicking on the dropdown below the chart.
            </p>


            <Chart
                options={options}
                series={series}
                type="line"
                width={chartWidth}
                height={chartHeight}
            />
            {renderDropdown()}
        </div>
    )


}