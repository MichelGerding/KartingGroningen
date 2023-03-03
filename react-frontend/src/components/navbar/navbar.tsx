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
    return (
        <nav className={styles.nav}>
            <div className={styles.div}>
                <Link className={styles.brandLink}
                      to={"/"}>KartAnalyzer</Link>
                <div id="navbar-sticky">
                    <ul className={styles.links}>
                        {links.map((link) => {
                            return (
                                <li key={link.path}>
                                    <Link className={styles.navLink} to={link.path}>{link.name}</Link>
                                </li>
                            )
                        })
                        }
                    </ul>
                </div>
            </div>
        </nav>
    )
}