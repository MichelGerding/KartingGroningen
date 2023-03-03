import Navbar from "./components/navbar/navbar";
import {Outlet} from "react-router-dom";

export default function Root() {
    return (
        <>
            <Navbar links={[
                {name: "Home", path: "/"},
                {name: "Heats", path: "all/heats"},
                {name: "Karts", path: "all/karts"},
                {name: "Drivers", path: "all/drivers"}]}/>

            <Outlet/>

        </>
    );
}