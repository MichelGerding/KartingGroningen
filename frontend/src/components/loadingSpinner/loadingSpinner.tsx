import React from "react";

import {Dna} from "react-loader-spinner";
import styles from "./loadingSpinner.module.css";
import TypingText from "../typingText/typingText";

type loadingSpinnerProps = {
    loadingText?: string;
    typing?: boolean;
    typingText?: string;
    typingSpeed?: number;
}

export default function LoadingSpinner({loadingText = '...', typing = true, typingSpeed = 250, typingText = loadingText}: loadingSpinnerProps) {
    // if (loadingText === '...') {
    //     loadingText = 'Loading';
    // }

    const renderText = () => {
        if (typing) {
            return <p> {loadingText} <TypingText text={typingText} interval={typingSpeed} /> </p>
        } else {

            return <p>{loadingText}</p>
        }

    }

    return (
        <div className={styles.loadWrapper}>
            <Dna
                visible={true}
                wrapperClass={styles.spinnerWrapper}
            />
            {renderText()}
        </div>


    );
}
