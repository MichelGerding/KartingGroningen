import React from "react";

const globalStyles = {
    a: {
        textDecoration: "none",
        color: "inherit",
    } as React.CSSProperties,

    list: {
        listStyle: "none",
    } as React.CSSProperties,

    flexRowSpaced: {
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
    } as React.CSSProperties,


    flexColumn: {
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
    } as React.CSSProperties,

}

export default globalStyles;