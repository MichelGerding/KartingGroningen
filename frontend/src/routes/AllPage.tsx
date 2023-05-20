import {useParams} from "react-router-dom";
import SearchableTable from "../components/searchableTable/searchableTable";
import React from "react";

const allKeys: {
    [key: string]: string[];
} = {
    "drivers": ["name", "fastest_lap_time", "avg_lap_time", "median_lap_time", "total_heats", "total_laps", "rating", "analyze-name"],
    "heats": ["start_time", "heat_type", "amount_of_laps", "amount_of_drivers", "fastest_lap_time", "average_lap_time", "analyze-heat_id"],
    "karts": ["number", "is_child_kart", "lap_count", "driver_count", "analyze-number"],
}

type params = {
    type: string;
}

export default function AllPage() {
    let { type } = useParams<params>();
    type = type === undefined ? "drivers" : type;

    const keys = allKeys[type];


    let endpoint = `${import.meta.env.VITE_API_BASEURL}/api/${type}/` + (type === "karts" ? "all" : "search");

    return (
        <div style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            maxWidth: '1000px',
            margin: '4em auto',
        }}>
            <h1 style={{
                color: "black",
                alignSelf: "flex-start",
                margin: "1rem",
                paddingLeft: "2rem",
                fontSize: "3rem",
            }}> All {type} </h1>
            <div style={{
                overflowX: 'auto',
                width: '100%',
            }}>
                <SearchableTable key={type} keys={keys} endpoint={endpoint} type={type} hasSearch={type === "drivers"}/>
            </div>

        </div>

    )

}