import React, {useEffect, useState} from "react";
import DropdownSelect from "../dropdown/dropdown";
import StyledInput from "../styledInput/StyledInput";
import StyledTable, {TableDataType} from "../styledTable/styledTable";

import styles from "./searchableTable.module.css";
import StyledButton from "../styledButton/styledButton";

interface SearchableTableProps {
    endpoint: string;
    hasSearch: boolean;
    keys: string[];
    type: string;
}


export default function SearchableTable({endpoint, keys, hasSearch, type}: SearchableTableProps) {

    const [data, setData] = useState<object[]>([]);
    const [page, setPage] = useState<number>(0);
    const [pageSize, setPageSize] = useState<number>(50);
    const [search, setSearch] = useState<string>("");
    const [searching, setSearching] = useState<boolean>(false);
    const [sortColumn, setSortColumn] = useState<string>(keys[0]);
    const [sortDirection, setSortDirection] = useState<string>("asc");


    // load data when page is loaded
    useEffect(() => {
        async function fetchData() {
            let url = `${endpoint}`

            if (endpoint.endsWith("/search") || endpoint.endsWith("/api/karts/all")) {
                url = `${endpoint}?q=${search}`
                if (pageSize > 0) {
                    url += `&page=${page}&page_size=${pageSize}&sort_col=${sortColumn}&sort_dir=${sortDirection}`
                }
            }


            console.log(url)

            const response = await fetch(url);
            return await response.json();
        }
        setSearching(true);
        fetchData().then((data) => {
            // ensure that we have a max of pageSize items
            // needed because kart doesn't have a search endpoint because that is useless for karts
            if (data.length > pageSize) {
                let start = pageSize * page;
                let end = start + pageSize;
                setData(data.slice(start, end));
            } else {
                setData(data);
            }
            setSearching(false);
        });
    }, [page, pageSize, search, endpoint, sortColumn, sortDirection]);

    // reload data when state changes

    const renderPrevButton = () => {
        if (page < 1) {
            return (<div></div>)
        }
        return (
            <StyledButton
                inverted={true}
                onClick={() => {
                    setPage(page - 1);
                }}
                text={"previous"}/>
        )
    }

    const renderNextButton = () => {
        if (data.length < pageSize) {
            return (<div></div>)
        }
        return (
            <StyledButton
                inverted={true}
                onClick={() => {
                    setPage(page + 1);
                }}
                text={"next"}
            />
        )
    }

    const renderSearchBar = () => {
        if (!hasSearch) {
            return (<div></div>)
        }
        return (
            <StyledInput
                placeholder="Search"
                borderColour={"black"}
                onChange={(event) => {
                    setSearch(event.target.value.trim())
                }}/>
        )
    }

    return (
        <div className={styles.main}>

            {/************* TABLE CONTROLS ****************/}
            <div className={styles.tableControls}>
                <DropdownSelect
                    options={[
                        {value: '10', label: '10'},
                        {value: '25', label: '25'},
                        {value: '50', label: '50'},
                        {value: '100', label: '100'},
                        {value: '500', label: '500'},
                        {value: '-1', label: 'all'}
                    ]}
                    defaultValue={pageSize.toString()}
                    onChange={(event) => {
                        console.log(parseInt(event.target.value))
                        setPageSize(parseInt(event.target.value))
                    }}
                    label={"Per Page"}/>
                {renderSearchBar()}
            </div>

            {/************* TABLE ****************/}
            <StyledTable
                data={data as TableDataType[]}
                keys={keys}
                searchState={searching}
                endpoint={type}
                indexStart={1 + (pageSize * page)}
                useSorting={true}
                sortData={(col, direction) => {
                    setSortColumn(col);
                    setSortDirection(direction);
                    return  [];
                }}
            />


            <div className={styles.tableButtons}>
                {renderPrevButton()}
                {renderNextButton()}
            </div>
        </div>


    );
}
