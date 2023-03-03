export interface ApiHeatResult{
    heat_id: string;
    heat_type: string;
    start_time: Date;
    results: ApiHeatDriverResult[];
}

export interface ApiHeatDriverResult {
    kart: number;
    driver: ApiDriver
    laps: ApiLap[];
}

export interface ApiDriver{
    driver_name: string;
}

export interface ApiLap{
    lap_number: number;
    lap_time: number;
}


export interface ApiDriverResult {
    name: string;
    rating: number;
    heats: ApiDriverHeat[];
}

export interface ApiDriverHeat {
    heat_id: string;
    start_date: Date;
    laps: ApiLap[];
    kart: number;
}

export interface ApiKartResult {
    number: number;
    is_child_kart: boolean;
    heats: ApiKartHeat[];
}

export interface ApiKartHeat {
    heat_id: string;
    start_date: Date;
    driver: ApiKartDriver;
    laps: ApiLap[];
}

export interface ApiKartDriver {
    name: string;
}