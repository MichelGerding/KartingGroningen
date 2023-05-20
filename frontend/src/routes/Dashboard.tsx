import React, {useEffect, useRef, useState} from "react";

// import {IDashboardComp} from "../components/charts/chart";
// import {AsyncHolder} from "../components/asyncHolder/asyncHolder";
// import {GlobalData} from "../components/dashboardComponents";
// import NewDashboardComponentPopup from "../components/newDashboardComponentPopup/newDashboardComponetPopup";
//
//
// export enum ChartType {
//     lapChart = "heatLaptimeLineChart",
//     lapBarChart = "heatLapTimesBarChart",
//     lapsPerDriver = "lapsPerDriverInHeat",
//     allLapsDriverChart = "allLapsDriverChart",
// }
//
// async function renderDashboardItem(comp: IDashboardComp) {
//     const {renderStatic} = await import(`../components/dashboardComponents/${comp.type}.tsx`);
//     try {
//         const dc = await renderStatic(comp, dashboardComponentsDataMap);
//         return (
//             <div>
//                 <center>
//                     <h2>{comp.title}</h2>
//                 </center>
//                 {dc}
//             </div>
//         )
//     } catch (e) {
//         return <h4> Failed to load component</h4>
//     }
// }
//
// const charts: IDashboardComp[] = [
//     {
//         type: ChartType.lapChart,
//         title: "All laps",
//         dataType: "heats",
//         id: "C9855916612B41FB98C786F86059C866",
//         showText: false
//     },
//     {
//         type: ChartType.lapBarChart,
//         title: "All laps Bar",
//         dataType: "heats",
//         id: "C9855916612B41FB98C786F86059C866",
//         showText: false,
//         showVsc: true
//     },
//     {
//         type: ChartType.lapsPerDriver,
//         title: "LapsPerDriver",
//         dataType: "heats",
//         id: "C9855916612B41FB98C786F86059C866",
//         dataModificationScript: (dataIn) => {
//             return dataIn.map((d) => {
//                 return {
//                     ...d,
//                     value: d.value - 1
//                 }
//             })
//         }
//     },
//     {
//         type: ChartType.allLapsDriverChart,
//         title: "all laps of driver",
//         dataType: "drivers",
//         id: "aalt van de pol",
//         showText: false
//     },
// ]
//
// const dashboardComponentsDataMap: GlobalData = {}
//
// async function getDashComponents(): Promise<IDashboardComp[]> {
//     return charts
// }
//
// function loadComponents(setDashboardComponents: any) {
//     getDashComponents().then((comps) => {
//         let t: any[] = [];
//         comps.forEach((comp) => {
//             t.push(<AsyncHolder promise={renderDashboardItem(comp)} title={comp.title}/>);
//         });
//
//         setDashboardComponents(t);
//     });
// }


export default function Dashboard() {
    // const [dashboardComponents, setDashboardComponents] = useState<Map<string, JSX.Element>>(new Map<string, JSX.Element>());
    // const [dashboardComponents, setDashboardComponents] = useState<JSX.Element[]>([]);
    //
    //
    //
    // useEffect(() => {
    //     loadComponents(setDashboardComponents);
    // }, []);
    //
    // const saveData = (comp: IDashboardComp) => {
    //     const component = <AsyncHolder promise={renderDashboardItem(comp)} title={comp.title}/>;
    //     dashboardComponents.unshift(component);
    //     setDashboardComponents(dashboardComponents);
    // }
    //
    // const dashboardRef = useRef<any>(null);

    return (
        <> </>
    );
    // return (
        // <div>
        //     <h1>Dashboard</h1>
        //     <div style={{
        //         display: "grid",
        //         gridTemplateColumns: "repeat(auto-fit, minmax(800px, 1fr))",
        //         gridGap: "1rem",
        //
        //     }}>
        //         <NewDashboardComponentPopup ref={(e) => dashboardRef.current = e} saveFunction={saveData}/>
        //         {/*<CodeEditor />*/}
        //         {dashboardComponents}
        //     </div>
        // </div>
    // );
}