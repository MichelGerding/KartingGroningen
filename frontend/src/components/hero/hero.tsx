import React from "react";
import Cta from "./cta";

import styles from "./hero.module.css";

export interface HeroProps {
    image: string;
    children: React.ReactNode[];
}

export function Hero({image, children}: HeroProps) {
    // allow exactly 1 h1 and 1 p tag as child
    if (children.length !== 2) {
        throw new Error("Hero must have exactly 2 children");
    }

    return (

        <div className="hero">
            <div className={styles.image} style={{
                backgroundImage: `url(${image})`
            }}>
                <div className={styles.content}>
                    <div className={styles.text}>
                        {children}
                    </div>

                    <Cta />
                </div>
            </div>
        </div>
    );
}