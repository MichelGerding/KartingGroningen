
export interface ChartDataInput {
    [key: string]: string | number | Date | undefined;

    driver?: string;
    lap_in_heat?: number;
    value: number;
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

export type IDashboardComp = {
    type: string,
    title: string,
    dataType: string,
    id: string,
    showText?: boolean,
    showVsc?: boolean,
    dataModificationScript?: (data: ChartDataInput[]) => ChartDataInput[],
    demo?: boolean,
}


export type IFormOptionType = {
    label: string;
    type: string;
}

export interface IDashboardCompSettings extends IDashboardComp {
    [key: string]: any;

    dashTitle?: IFormOptionType;
    dashId?: IFormOptionType;
    dashShowVSC?: IFormOptionType;
}