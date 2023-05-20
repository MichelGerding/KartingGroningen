import React, {useState, useEffect, useRef} from "react";

import Editor from '@monaco-editor/react';
import LoadingSpinner from "../loadingSpinner/loadingSpinner";
import StyledButton from "../styledButton/styledButton";
import {ChartDataInput} from "../charts/chart";

type StrIndex = {
    [key: string]: any;
}

export const defaultCode: StrIndex = {
    "javascript": `// define a function to transform data
// the provided type is the type of the data that is returned from the api
// the return type is the type of the data that is passed to the chart


// typedef of the in and output
// type ChartDataInput = {
//     driver?: string;
//     lap_in_heat?: number;
//     laptime: number;
//     kart?: number;
//     date?: string;
//     type?: string;
// }[]

// function that transforms the data
return (data) => {
    // subtract 2s off of each laptime
    return data.map((d) => {
        return {
            ...d,
            laptime: d.laptime - 2,
        }
    })
}`
}

export type CodeEditorProps = {
    code?: string;
    saveResult: (data: (data: ChartDataInput[]) => ChartDataInput[]) => void
};


export const CodeEditor = ({saveResult, code = ''}: CodeEditorProps) => {
    let currentLanguage = 'javascript'
    let currentCodeValue = defaultCode[currentLanguage];

    if (code !== '') {
        currentCodeValue = code;
    }


    return (
        <div>
            <Editor
                defaultLanguage={currentLanguage}
                defaultValue={currentCodeValue}
                height="90vh"
                theme="vs-dark"
                loading={<LoadingSpinner loadingText={"Loading Editor"}/>}
                onChange={(value) => {
                    saveResult(eval(value ?? ""))
                }}
            />
        </div>
    )

};

