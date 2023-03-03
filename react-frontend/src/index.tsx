import React from "react";
import ReactDOM from "react-dom/client";
import {
    BrowserRouter,
    Route,
    Routes,
} from "react-router-dom";

import Root from "./root";
import HomePage from "./routes/HomePage";
import AllPage from "./routes/AllPage";
import DetailPage from "./routes/detailPage";
import KartDetailPage from "./routes/kartPage";

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);

root.render(
    // <React.StrictMode>
        <BrowserRouter>
            <Routes>
                <Route path={"/"} element={<Root/>}>
                    <Route path={"/"} element={<HomePage />} />
                    <Route path={"/all/:type"} element={<AllPage />} />

                    <Route path={"/karts/:id"} element={<KartDetailPage />} />

                    <Route path={"/:type/:id"} element={<DetailPage />} />

                    <Route path={"*"} element={<h1>404</h1>}/>
                </Route>
            </Routes>
        </BrowserRouter>
    // </React.StrictMode>
);