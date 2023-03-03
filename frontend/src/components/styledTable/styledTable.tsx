import React, {useEffect, useState} from "react";
import styles from "./table.module.css";

import { getUserLocale } from 'get-user-locale';
import {Dna} from "react-loader-spinner";
import TypingText from "../typingText/typingText";
import RedirectButton from "../redirectButton/redirectButton";

export type Direction = "asc" | "desc";

export type TableDataType = {
    [key: string]: string | number;
}

interface TableProps {
    data: TableDataType[];
    keys: string[];
    indexStart?: number;
    searchState: boolean;
    endpoint?: string;
    useSorting?: boolean;
    sortData?(col: string, direction: Direction): TableDataType[]
}



export default function StyledTable({data, keys, searchState, endpoint, indexStart = 0, sortData, useSorting = false}: TableProps) {

    const [sort, setSort] = useState<string>("")

    if (sortData == undefined) {
        sortData = (col: string, direction: Direction) => {
            let sortFunction;
            if (direction === "asc") {
                sortFunction = (a: TableDataType, b: TableDataType) => a[col] < b[col] ? 1 : -1;
            } else {
                sortFunction = (a: TableDataType, b: TableDataType) => a[col] > b[col] ? 1 : -1;
            }

            return data.sort(sortFunction)
        }
    }


    const renderData = () => {
        if (searchState) {
                return <tr><td className={styles.emptyTable} colSpan={keys.length + 1}>
                    <div className={styles.loadWrapper}>
                        <Dna
                            visible={true}
                            wrapperClass={styles.spinnerWrapper}
                        />
                        <p>Loading<TypingText text={"..."} interval={250} /></p>
                    </div>
                </td></tr>
        }

        if (data.length === 0) {
            return <tr><td className={styles.emptyTable} colSpan={keys.length + 1}> No data to display </td></tr>
        }

        function renderVal(key: string, row: TableDataType) {
            if (key.startsWith("analyze") && endpoint !== undefined) {
                let indexKey = key.split("-")[1];
                return (<RedirectButton to={`/${endpoint}/${row[indexKey]}`} />)
            } else {
                return formatVal(row[key]);
            }
        }

        return data.map((row, index) => {
            const firstVal = Object.values(row)[0];
            return (
                <tr key={firstVal} className={styles.tableRow}>
                    <td className={styles.tableCell}>{indexStart + index}</td>
                    {keys.map(key => {
                        let val = row[key];
                        return (
                            <td key={Object.values(row)[0] + '-' + val + '-' + key}
                                className={styles.tableCell}>{renderVal(key, row)}</td>
                        )
                    })}
                </tr>
            )})

    }

    const toggleSort = (column: string) => {
        const parts = sort.split('-');
        const sortedCol = parts[0];
        const direction = parts[1];

        let newCol = column;
        let newDirection: Direction = "asc";

        if (sortedCol == column) {
            //flip sorting
            newDirection = direction === "asc" ? "desc" : "asc"
        }

        console.log(newCol + "-" + newDirection)
        setSort(newCol + "-" + newDirection);
        if (sortData) {
            sortData(newCol, direction as Direction);
        }
    }

    const renderSortedIcon = (key: string) => {
        let parts = sort.split("-");
        let col = parts[0];
        if (col == key) {
            let direction = parts[1] as Direction;

            if (direction == "asc") {
                return '↑';
            } else {
                return '↓';
            }
        }
    }

    const renderHeaderCell = (key: string) => {
        if (key.startsWith("analyze") ) {
            return ""
        } else {
            return formatVal(key)
        }
    }

    const renderHeader = () => {
        return (
            <tr>
            <th className={styles.tableHeader}></th>
            {keys.map(column => (
                    <th className={styles.tableHeader}
                        key={column}
                        onClick={() => {
                        if (useSorting) {
                            toggleSort(column)
                        }}}>
                        {renderHeaderCell(column)} {renderSortedIcon(column)}
                    </th>
                ))
            }
            </tr>
        )
    }


    useEffect(() => {
        if (useSorting) {
            toggleSort(keys[0])
        }
    }, [])


    return (
        <table className={styles.table}>
            <thead>
                {renderHeader()}
            </thead>
            <tbody>
                {renderData()}
            </tbody>
        </table>
    )
}


function formatVal(val: any) {
    if (typeof val === "number" ) {
        if (!Number.isInteger(val)) {
            return val
                .toFixed(2)
                .replace(".00", "")
                .toString();
        }

        return (
            <>{val}<span style={{opacity: "0"}}>.00</span></>
        );
    }



    if (typeof val === "boolean") {
        return val ? "Yes" : "No";
    }

    if (typeof val === "string") {
        const date = new Date(val);
        if (!isNaN(date.getTime())) {
            val = date.toLocaleString(
                getUserLocale({ fallbackLocale: "en-US", useFallbackLocale: true}) as string,
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

        return val
            .toLowerCase()
            .replaceAll('_', ' ')
            .split(' ')
            .map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
            .join(' ');
    }


    return val
}


// a: time-series databases zijn databases die speciaal ontworpen zijn om time-series data op te slaan.
// Dit is data die een tijdselement heeft, zoals een temperatuur met een timestamp.
// Dit is anders dan andere databases omdat deze data vaak op een andere manier wordt opgeslagen en geïndexeerd.
// Een voorbeeld van een time-series database is InfluxDB. Een toepassing hiervan is het bijhouden van de temperatuur van een serverruimte.
// Hierbij is het belangrijk dat de data snel opgehaald kan worden en dat de data op een logische manier wordt opgeslagen.
// Dit is bijvoorbeeld niet het geval bij een normale database, omdat de data dan opgeslagen wordt op basis van de timestamp.
// Dit is niet handig omdat je dan niet snel de temperatuur van een bepaalde dag kan opvragen.
// InfluxDB slaat de data op in een tabel met een timestamp als index. Hierdoor kan je snel de temperatuur van een bepaalde dag opvragen.
// Dit is een voorbeeld van een toepassing van een time-series database.

// q: at zijn time-series databases? Waarin verschillen ze van andere databases? Kan je hier een toepassing van geven?
