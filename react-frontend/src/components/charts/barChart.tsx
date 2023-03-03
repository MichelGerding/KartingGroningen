import {ChartDataInput} from "./chart";
import Chart from "react-apexcharts";
import {useResizeDetector} from "react-resize-detector";
import moment from "moment";

interface LapTmeChartProps {
    dataIn: ChartDataInput[];
    labelKey: string;
}

interface FilterByLabel {
    [key: string]: ChartDataInput[];
}

export default function BarChart({dataIn, labelKey}: LapTmeChartProps) {
    const {width, ref} = useResizeDetector();
    // parse the data

    let data: FilterByLabel = {};
    dataIn.forEach((d) => {
        let k = d[labelKey];
        // check if k is a valid date
        const date = moment(k, "YYYY-MM-DDTHH:mm");
        if (date.isValid()) {
            k = date.unix()
        }


        if (data[k as string] === undefined) {
            data[k as string] = [];
        }
        data[k as string].push(d);
    });

    // setup the chart options
    const options = {
        chart: {
            id: "ubhkjnuohigyuvbuhg7yib",
        },
        zoom: {
            enabled: true,
            type: 'x',
            autoScaleYaxis: true
        },
        xaxis: {
            categories: Object.keys(data).map((k) => moment.unix(parseInt(k)).format("YYYY-MM-DD"))
        },
    }


    const series: ApexAxisChartSeries = [];
    const dataArray: {date: string; amount_of_laps: number}[] = [];
    Object.keys(data).forEach((k) => {
        const laps = data[k];
        const laptime = laps.reduce((acc, lap) => acc + lap.laptime, 0) / laps.length;
        dataArray.push({
            date: k,
            amount_of_laps: laptime,
        })
    });

    // sort by date using moment
    const sorted = dataArray.sort((a, b) => {
        const aDate = moment(a.date, "YYYY-MM-DDTHH:mm");
        const bDate = moment(b.date, "YYYY-MM-DDTHH:mm");
        if (aDate.isBefore(bDate)) {
            return -1;
        } else if (aDate.isAfter(bDate)) {
            return 1;
        } else {
            return 0;
        }
    });

    series.push({
        name: "Laps",
        data: sorted.map((k) => k.amount_of_laps),
    });



    const chartWidth = width ?? 500;
    const aspectRatio = 9 / 22; // height/width instead of width/height
    const chartHeight = chartWidth * aspectRatio;

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
