import React from "react";
import {Link} from "react-router-dom";

import styles from "./navbar.module.css";

interface LinkProp {
    name: string,
    path: string
}

export interface NavbarProps {
    links: LinkProp[]
}



export default function Navbar({links}: NavbarProps) {
    const [left, setLeft] = React.useState("-100%");

    return (
        <nav className={styles.nav}>
            <div className={styles.div}>
                <Link className={styles.brandLink}
                      to={"/"}>KartAnalyzer</Link>
                <div id="navbar-sticky">
                    <ul className={styles.links} style={{left: left}}>
                        {links.map((link) => {
                            return (
                                <li key={link.path}>
                                    <Link className={styles.navLink} to={link.path}>{link.name}</Link>
                                </li>
                            )
                        })
                        }
                    </ul>
                    <div className={styles.hamburger} onClick={() => {
                        const links = document.querySelector("." + styles.links);
                        console.log(left);
                        if (links) {
                            // set the left of the element to 0 if it is not already
                            if (left === "0px") {
                                setLeft("-100%");
                            } else {
                                setLeft("0px");
                            }

                            // set the content of the hamburger to an X
                            const hamburger = document.getElementsByClassName(styles.hamburger)[0];
                            if (hamburger) {
                                if (hamburger.innerHTML === "=") {
                                    hamburger.innerHTML = "X";
                                } else {
                                    hamburger.innerHTML = "=";
                                }
                            }
                        }
                    }}>=</div>

                </div>
            </div>
        </nav>
    )
}