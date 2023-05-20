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

function formatData(inputData: ChartDataInput[], labelKey: string) {
    let data: FilterByLabel = {};
    inputData.forEach((d) => {
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

    return data
}

function createChartOption(data: FilterByLabel, labelKey: string) {
    const cats = Object.keys(data).map((k) => {
        if (labelKey !== "date") {
            return k;
        }
        return moment.unix(parseInt(k)).format("YYYY-MM-DD")
    });

    return {

        zoom: {
            enabled: true,
            type: 'x',
            autoScaleYaxis: true
        },
        xaxis: {
            categories: cats,
        },
    }
}

function parseDataToLapsPerDay(data: FilterByLabel) {
    const dataArray: {date: string; amount_of_laps: number}[] = [];
    Object.keys(data).forEach((k) => {
        const laps = data[k];
        const laptime = laps.reduce((acc, lap) => acc + lap.value, 0) / laps.length;
        dataArray.push({
            date: k,
            amount_of_laps: laptime,
        })
    });

    return dataArray.sort((a, b) => {
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
}

export default function BarChart({dataIn, labelKey}: LapTmeChartProps) {
    const {width, ref} = useResizeDetector();

    const data = formatData(dataIn, labelKey);
    const options = createChartOption(data, labelKey);

    const sorted = parseDataToLapsPerDay(data);

    const chartWidth = width ?? 500;
    const aspectRatio = 9 / 22; // height/width instead of width/height
    const chartHeight = chartWidth * aspectRatio;

    const series = [{
        name: "Laps",
        data: sorted.map((k) => k.amount_of_laps),
    }];

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
