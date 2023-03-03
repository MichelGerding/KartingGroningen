import styles from "./hero.module.css";
import {AnimatedInput} from "../animatedInput/animatedInput";
import React, {useState} from "react";
import {useNavigate} from "react-router-dom";
import StyledButton from "../styledButton/styledButton";


export default function Cta() {
    const navigate = useNavigate();
    let [heatId, setHeatId] = useState<string>("");
    let [error, setError] = useState<string>("");

    function redirect() {
        if (heatId === "") {
            setError("Please enter a heat id");
            return;
        }

        // load heat into db
        let formData = new FormData();
        formData.append("heat_id", heatId);
        const requestOptions = {
            method: 'POST',
            body: formData
        }

        fetch(`http://localhost:8080/api/heats/new`, requestOptions)
            .then(response => {
                if (response.status === 200) {
                    return response.json();
                } else if (response.status === 404) {
                    throw new Error("Heat not found");
                } else if (response.status === 400) {
                    throw new Error("Invalid heat id");
                }
            })
            .then(() => {
                navigate(`/heat/${heatId}`);
            })
            .catch(error => {
                setError(error.message);
            });
    }

    function inputChange(e: React.ChangeEvent<HTMLInputElement>) {
        let newHeatId = e.target.value;

        setHeatId(newHeatId)
        if (newHeatId !== "") {
            setError("");
        }
    }

    return (
        <div>
            <h2 className={styles.ctaHeader}> View your heat </h2>
            <div style={{
                display: "flex",
                height: "min-content"
            }}>

                <AnimatedInput interval={150} placeholder="Enter heat id" onChange={inputChange}/>
                <svg className={styles.ctaIcon}
                     onClick={() => {
                         //TODO:: show modal on click instead of alert
                         alert("The heat id can be found in the url of the website provided to view the results of the heat.")
                     }}
                     viewBox="0 0 320 512" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M204.3 32.01H96c-52.94 0-96 43.06-96 96c0 17.67 14.31 31.1 32 31.1s32-14.32 32-31.1c0-17.64 14.34-32 32-32h108.3C232.8 96.01 256 119.2 256 147.8c0 19.72-10.97 37.47-30.5 47.33L127.8 252.4C117.1 258.2 112 268.7 112 280v40c0 17.67 14.31 31.99 32 31.99s32-14.32 32-31.99V298.3L256 251.3c39.47-19.75 64-59.42 64-103.5C320 83.95 268.1 32.01 204.3 32.01zM144 400c-22.09 0-40 17.91-40 40s17.91 39.1 40 39.1s40-17.9 40-39.1S166.1 400 144 400z"/>
                </svg>
            </div>
            <p className={styles.ctaError}> {error}</p>
            <StyledButton
                text={"View Heat"}
                onClick={redirect}
                style={{
                    fontSize: "1.5rem",}}/>
        </div>

    );
}