import {ChartDataInput} from "./chart";
import Chart from "react-apexcharts";
import {useResizeDetector} from "react-resize-detector";
import StyledInput from "../styledInput/StyledInput";
import {useState} from "react";
import DropdownSelect from "../dropdown/dropdown";
import moment from "moment";
import {getUserLocale} from "get-user-locale";

type LapTimeChartProps = {
    dataIn: ChartDataInput[];
    labelKey: string;
    showVSC?: boolean;
    showText?: boolean;
    demo?: boolean
}

interface FilterByLabel {
    [key: string]: ChartDataInput[];
}

type Laptime = {
    laptime: number;
    lap_in_heat: number;
}


export default function LapTimeBarChart({dataIn, labelKey, showVSC, showText = true, demo = false}: LapTimeChartProps) {
    const {width, ref} = useResizeDetector();
    const [slowPercent, setSlowPercent] = useState(35);
    const [showLaptimes, setShowLaptimes] = useState(!demo);
    const [VSCShown, setVSCShown] = useState(showVSC ?? true);
    const xAnnos: XAxisAnnotations[] = [];

    // parse the data
    let data: FilterByLabel = {};
    dataIn.forEach((d) => {
        let k = d[labelKey];
        // check if k is a valid date
        if (labelKey === "date") {
            const date = moment(k, "YYYY-MM-DDTHH:mm");
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


        if (data[k as string] === undefined) {
            data[k as string] = [];
        }
        data[k as string].push(d);
    });


    // setup the chart options
    const options = {
        chart: {
            id: "laptimes",
            stacked: true,
        },
        zoom: {
            enabled: true,
            type: 'x',
            autoScaleYaxis: true
        },
        plotOptions: {
            bar: {
                horizontal: true,
            }
        },
        dataLabels: {
            enabled: showLaptimes,
        },
        legend: {
            show: false,
        },
        xaxis: {
            categories: Object.keys(data),
        },
        annotations: {
            xaxis: xAnnos,
        },
    }


    const series: ApexAxisChartSeries = [];
    const maxLaps = Math.max(...dataIn.map((d) => d.lap_in_heat ? d.lap_in_heat : 0))

    for (let i = 1; i < maxLaps + 1; i++) {
        let laps = Object.keys(data).map((key) => {
            const lap = data[key].find((d) => d.lap_in_heat === i);
            return {
                x: key,
                y: lap ? lap.value : 0,
            }
        })

        series.push({
            name: `Lap ${i}`,
            data: laps
        })
    }

    if (VSCShown) {
        calculateVSCMoments(data, slowPercent).map((e) => xAnnos.push(e))
    }


    const chartWidth = width ?? 500;
    const aspectRatio = 9 / 22; // height/width instead of width/height
    const chartHeight = chartWidth * aspectRatio;

    const showSlowLapsText = () => {
        if (showVSC) {
            return `and a button to show or hide the slow moments`
        }
    }

    const showSlowLapsButton = () => {
        if (showVSC) {
            return <DropdownSelect
                options={[
                    {value: "true", label: "shown"},
                    {value: "false", label: "hidden"},
                ]}
                onChange={(e) => {
                    setVSCShown(e.target.value === "true");
                }}
                label={"Slow laps"}/>
        }
    }

    const renderInput = () => {
        if (!showVSC) {
            return (<> </>)
        }

        return (
            <StyledInput
                defaultValue={slowPercent}
                borderColour={"black"}
                type={"number"}
                onChange={(e) => {
                    setSlowPercent(parseInt(e.target.value.length > 0 ? e.target.value : "0"));
                }}/>

        )
    }

    const renderText = () => {
        if (showText && !demo) {
            return (
                <>
                    <h2>Timeline</h2>
                    <p>
                        This chart shows a timeline of the laps a driver has made.
                        This is useful to see when all drivers where slow and the lap is not representative. as each color
                        corresponds to a lap in heat
                        The red areas make this easier by showing a estimation when all drivers where slow.
                        This is shown when at least {slowPercent}% have a outlier lap.
                        This is not 100% accurate but gives a good indication.
                        <br/>
                        <br/>
                        this percentage can be changed by changing the input value below the chart. if set to 0% all moments a
                        driver had a outlier lap will be shown
                        below the chart is also a dropdown to show or hide the laptime values. {showSlowLapsText()}
                    </p>
                </>
            )
        }
    }

    if (demo) {
        return (
            <div ref={ref}>
                <Chart
                    options={options}
                    series={series}
                    type="bar"
                    width={chartWidth}
                    height={chartHeight}
                />
            </div>
        )
    }
    return (
        <div ref={ref}>
            {renderText()}
            <Chart
                options={options}
                series={series}
                type="bar"
                width={chartWidth}
                height={chartHeight}
            />
            {renderInput()}
            <DropdownSelect
                options={[
                    {value: "true", label: "shown"},
                    {value: "false", label: "hidden"},
                ]}
                onChange={(e) => {
                    setShowLaptimes(e.target.value === "true");
                }}
                label={"Laptimes"}/>
            {showSlowLapsButton()}
        </div>
    )
}


function calculateVSCMoments(data: FilterByLabel, slowPercent: number) {
    const heatTime = Math.max(...Object.keys(data).map((driver) => {
        return data[driver].reduce((acc, d) => acc + d.value, 0)
    }));

    const outlierLaps = getOutlierLapsPerDriver(data);
    const outlierLapsStartTimes = getSlowLapsPerDriver(outlierLaps, data);
    const slowSeconds = slowSecondsInHeat(outlierLaps, outlierLapsStartTimes, heatTime, slowPercent);
    const outlierTimes = slowSecondsToZones(slowSeconds);

    // return them
    return outlierTimes.map((d) => {
        return {
            x: d.start,
            x2: d.end,
            borderColor: 'red',
            fillColor: 'red',
            opacity: 0.4,
        }
    })
}

function getOutlierLapsPerDriver(data: FilterByLabel) {
    const outlierLaps: { [key: string]: Laptime[] } = {};

    Object.keys(data).forEach((driver) => {
        const mean = data[driver].reduce((acc, d) => acc + d.value, 0) / data[driver].length;
        const std = Math.sqrt(data[driver].reduce((acc, d) => acc + Math.pow(d.value - mean, 2), 0) / data[driver].length);
        // assign the outlier laps to the driver
        outlierLaps[driver] = data[driver]
            .filter((d) => (d.value - mean > std * 2) || d.value < 45)
            .map(e => {
                return {laptime: e.value, lap_in_heat: e.lap_in_heat} as Laptime
            });
    });

    return outlierLaps;
}

function getSlowLapsPerDriver(outlierLaps: { [p: string]: Laptime[] }, data: FilterByLabel) {
    const outlierLapsStartTimes: {
        [key: string]: {
            start: number
            end: number
        }[]
    } = {};
    Object.keys(outlierLaps).forEach((driver) => {
        // get the time it has taken to get to this lap
        if (outlierLaps[driver].length === 0) {
            return [];
        }

        const startTimes = [];
        let laps = data[driver];

        let totaltime = 0;
        let outlierLap = 0;
        for (let i = -0; i < laps.length; i++) {
            let lap = laps[i];

            if (lap.lap_in_heat == outlierLaps[driver][outlierLap].lap_in_heat) {
                startTimes.push({start: totaltime, end: totaltime + lap.value});
                outlierLap++;

                if (outlierLap >= outlierLaps[driver].length) {
                    break;
                }
            }
            totaltime += lap.value;
        }

        outlierLapsStartTimes[driver] = startTimes;
    })

    return outlierLapsStartTimes;
}

function slowSecondsInHeat(
    outlierLaps: { [key: string]: Laptime[] },
    outlierLapsStartTimes: { [key: string]: { start: number, end: number }[] },
    heatTime: number,
    minSlowPercent: number) {
    // get the seconds that are slow in the heat
    let slowSeconds = [];
    for (let i = 0; i < heatTime; i++) {
        // get the amount of drivers that are slow at this second
        let slowDrivers = 0;
        Object.keys(outlierLaps).forEach((driver) => {
            if (!(driver in outlierLapsStartTimes)) {
                return
            }
            if (outlierLapsStartTimes[driver].some((d) => d.start <= i && d.end >= i)) {
                slowDrivers++;
            }
        })
        if (minSlowPercent === 0) {
            if(slowDrivers > 0) {

                slowSeconds.push(i);
            }
        }
        // if more than 35% of the drivers are slow at this second we mark it as a slow second
        if (slowDrivers / Object.keys(outlierLaps).length >= minSlowPercent / 100) {
            slowSeconds.push(i);
        }
    }

    return slowSeconds;
}


function slowSecondsToZones(slowSeconds: number[], minLength: number = 5, mergeLength: number = 1) {
    let outlierTimes: {
        start: number;
        end: number;
    }[] = [];

    // get the start and end time of the consecutive slow seconds
    let start = slowSeconds[0];
    let end = slowSeconds[0];
    for (let i = 1; i < slowSeconds.length; i++) {
        // check if there is more than 1-second difference between the current and the last second
        if (slowSeconds[i] - end > mergeLength) {
            // this is only used if there are multiple slow zones
            outlierTimes.push({start: start, end: end});
            start = slowSeconds[i]; // reset the start
        }

        end = slowSeconds[i];
    }
    outlierTimes.push({start: start, end: end}); // add the last slow zone

    // if any are less then 5 seconds long remove them
    return outlierTimes.filter((d) => d.end - d.start > minLength);
}

