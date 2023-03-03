import React from "react";
import styles from "./styledInput.module.css";

interface StyledInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    borderColour?: string;
}

export default function StyledInput ({borderColour, ...passedProps}: StyledInputProps) {
    borderColour = borderColour || "white";

    return (
        <div className={styles.inputWrapper}>
            <span className={styles.inputAfter} style={{background: borderColour}}></span>
            <input {...passedProps} className={styles.input} style={{
                color: borderColour,
                borderLeft: `2px solid ${borderColour}`,
                borderBottom: `2px solid ${borderColour}`,
            }}/>
        </div>

    )
}