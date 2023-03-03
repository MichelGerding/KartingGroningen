
export interface ChartDataInput {
    [key: string]: string | number | Date;

    driver?: string;
    lap_in_heat?: number;
    laptime: number;
    kart?: number;
    date?: string;
    type?: string;
}

type DataType = string | number | Date;

export interface ChartDataSet {
    labels: string[];
    datasets: ChartData[];
}

export interface ChartData {
    id: number;
    label: string;
    data: DataType[];
}