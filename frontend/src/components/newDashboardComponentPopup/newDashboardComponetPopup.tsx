import React, {useEffect, useRef, useState} from "react";
import {IDashboardComp, IDashboardCompSettings, IFormOptionType} from "../charts/chart";
import {ChartType} from "../../routes/Dashboard";
import {GlobalData} from "../dashboardComponents";
import StyledButton from "../styledButton/styledButton";
import StyledInput from "../styledInput/StyledInput";
import {CodeEditor} from "../codeEditor/codeEditor";

import styles from "./newDashboardComponetPopup.module.css"

export type NewDashboardComponentPopupProps = {
    current?: IDashboardCompSettings
    saveFunction: (dashboardComponent: IDashboardComp) => void
}

async function renderDashboardItem(comp: IDashboardComp) {
    const {renderStatic} = await import(`../dashboardComponents/${comp.type}.tsx`);
    try {
        const dc = await renderStatic(comp, dashboardComponentsDataMap);
        return (
            <> {dc}</>
        )
    } catch (e) {
        return <h4> Failed to load component</h4>
    }
}

const dashboardComponentsDataMap: GlobalData = {}


type StrIndex = {
    [key: string]: any
}
const getSettings = () => {
    return [
        {
            type: ChartType.lapChart,
            title: "All laps in a heat",
            dataType: "heats",
            id: "C9855916612B41FB98C786F86059C866",
            showText: false,
            dashTitle: {
                type: "text",
                label: "Title",
            },
            dashId: {
                type: "text",
                label: "Heat ID",
            },
        }, {
            type: ChartType.allLapsDriverChart,
            title: "all laps of single driver",
            dataType: "drivers",
            id: "aalt van de pol",
            showText: false,
            dashTitle: {
                type: "text",
                label: "Title",
            },
            dashId: {
                type: "text",
                label: "Driver ID",
            },
        }, {
            type: ChartType.lapsPerDriver,
            title: "Laps per driver in heat",
            dataType: "heats",
            id: "C9855916612B41FB98C786F86059C866",
            dashTitle: {
                type: "text",
                label: "Title",
            },
            dashId: {
                type: "text",
                label: "Heat ID",
            },
        }, {
            type: ChartType.lapBarChart,
            title: "All laps in heat as bar chart",
            dataType: "heats",
            id: "C9855916612B41FB98C786F86059C866",
            showText: false,
            showVsc: false,
            demo: true,
            dashTitle: {
                type: "text",
                label: "Title",
            },
            dashId: {
                type: "text",
                label: "Heat ID",
            }
            ,
            dashShowVSC: {
                type: "checkbox",
                label: "Show VSC",
            },
        },
    ] as IDashboardCompSettings[];
}


export default ({saveFunction}: NewDashboardComponentPopupProps) => {
    const [examples, setExamples] = useState<StrIndex>({});
    const [selectedExample, setSelectedExample] = useState<string>("");


    let exampleRefs = useRef(Array(getSettings().length).fill(null));
    const enableCustomCodeRef = useRef<any>(null);


    useEffect(() => {
        let promisses = getSettings().map((opt) => {
            return renderDashboardItem(opt);
        });

        Promise.all(promisses).then((ex) => {


            let t: StrIndex = {};
            ex.forEach((e, i) => {


                t[getSettings()[i].title] = (
                    <div ref={(element) => {
                        exampleRefs.current[i] = element;
                    }}
                         style={{
                             border: "2px solid transparent",
                         }}>
                        {e}

                        <StyledButton
                            text={"Select"}
                            onClick={() => {
                                // remove border form all examples
                                exampleRefs.current.forEach((ref) => {
                                    ref.style.border = "2px solid transparent";
                                });
                                //
                                let eRef = exampleRefs.current[i];
                                if (eRef !== null) {
                                    eRef.style.border = "2px solid black";
                                }
                                setSelectedExample(getSettings()[i].type);
                            }} inverted={true}/>
                    </div>
                );
            });
            setExamples(t);
        })

    }, []);

    const optionValues: StrIndex = {}
    let options: JSX.Element[] = [];


    let selectedComponent: IDashboardCompSettings | null = null;
    // find the options for the selected example
    if (selectedExample != "") {

        selectedComponent = findSelectedExample(selectedExample);
        if (selectedComponent !== null) {
            options = generateSelectedOptions(selectedComponent as IDashboardCompSettings, optionValues)
        }
    }

    const renderOptions = () => {
        if (options.length <= 0) {
            return <></>
        }

        return (
            <div>
                <h2>Options for "{selectedComponent?.title}"</h2>
                {options}

                <StyledButton text={"Add component"} onClick={() => {
                    for (let i = 0; i < getSettings().length; i++) {
                        if (getSettings()[i].type == selectedExample) {
                            const selectedType = getSettings()[i]
                            console.log(optionValues, selectedType)

                            // build a new componentOption
                            let newComp = selectedType as IDashboardComp;
                            newComp.id = optionValues['dashId']
                            newComp.title = optionValues['dashTitle']
                            if (enableCustomCodeRef.current.value) {
                                newComp.dataModificationScript = optionValues["dashCode"]
                            }

                            saveFunction(newComp);
                        }
                    }
                    // saveFunction()
                }} inverted={true}/>

                <h3> Advanced options</h3>
                <h4> Custom data processing. enable:
                    <input
                        type="checkbox"
                        ref={(e) => {
                            enableCustomCodeRef.current = e
                        }}/>
                </h4>
                <CodeEditor
                    code={""}
                    saveResult={(func: any) => {
                        optionValues["dashCode"] = func;

                }}/>
            </div>
        )
    };

    return (
        <div className={styles.overlay}>
            <div className={styles.popup}>
                <h2 className={styles.textCenter}> Add new component</h2>
                <div className={styles.examples}>

                    {Object.keys(examples).map((key) => {
                        const example = examples[key];
                        return (
                            <div>
                                <h3>{key}</h3>
                                {example}
                            </div>
                        )
                    })}

                </div>
                {renderOptions()}
            </div>
        </div>
    )
}

function findSelectedExample(selectedExample: string): IDashboardCompSettings | null {
    for (let i = 0; i < getSettings().length; i++) {
        if (getSettings()[i].type == selectedExample) {
            return getSettings()[i];
        }
    }

    return null;
}

function generateSelectedOptions(selectedExample: IDashboardCompSettings, optionValues: any): JSX.Element[] {
    if (selectedExample == null) {
        return [];
    }


    const keys: string[] = Object.keys(selectedExample)
    const options: JSX.Element[] = Array(keys.length);

    keys.forEach((key) => {
        if (!key.startsWith("dash")) {
            return;
        }

        const option: IFormOptionType = selectedExample[key];
        options.push(
            <div>
                <StyledInput
                    borderColour={"black"}
                    type={option.type}
                    id={key}
                    name={key}
                    placeholder={option.label}
                    onChange={() => {
                        optionValues[key] = (document.getElementById(key) as HTMLInputElement).value;
                    }}/>
            </div>);
    });

    return options;
}