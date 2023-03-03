import React, {ChangeEvent} from "react";

interface SelectOption {
    value: string;
    label: string;
}

interface SelectProps {
    options: SelectOption[];
    onChange: (value: ChangeEvent<HTMLSelectElement>) => void;
    defaultValue?: string;
    label: string;
}

export default function DropdownSelect({label, options, onChange, defaultValue}: SelectProps) {
    return (
        <div >
            <label htmlFor="select"> {label} </label>
            <select
                id="select"
                defaultValue={defaultValue}
                onChange={(event) =>{onChange(event)}}
                style={{
                    background: "none",
                    border: "none",
                    fontSize: "1em",
                    color: "inherit",
                    font: "inherit",
                    fontWeight: "inherit",
                    cursor: "pointer",
                }}
            >
                {options.map((option) => (
                    <option value={option.value} key={option.label}> {option.label} </option>
                ))}
            </select>
        </div>
    )
}