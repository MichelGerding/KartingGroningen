import {Hero} from "../components/hero/hero";
import React from "react";
import {Link} from "react-router-dom";

const linkStyle = {
    color: "#e02200"
} as React.CSSProperties;

export default function HomePage() {
    return (
        <div>
            <Hero
                image={"https://www.groningerondernemerscourant.nl/uploads/nieuwsitempagina/6516c7e8-7e20-5813-b999-bf487c26880b/3273408542/Kartracing%20Groningen%201.jpg"}>
                <h1 style={{
                    fontSize: "4rem",
                }}> Find Your Rhythm</h1>
                <p>
                    analyze all your own and your opponents laps.<br/>
                    KartAnalyzer has all the laps driven at Kartbaan Groningen and lets you analyze them to be you
                    fastest self.
                </p>
            </Hero>


            <div className="info" style={{
                padding: "2rem",
                maxWidth: "1000px",
                margin: "auto",
            }}>


                <h2 style={{
                    fontSize: "2rem",
                }}> More info </h2>
                <p>
                    KartAnalyzer is a application made to analyze all heats and laps driven at kartbaan Groningen.
                    This application allows you to analyze your own laps and compare them to your opponents laps.
                    We also have a leaderboard with a custom ranking system to see who is the best.
                    We also allow yoy to view the current performance of the kart which allows you to pick the best kart
                    of that day.
                </p>

                <h3 style={{
                    fontSize: "1.5rem",
                }}> Where to find your heats</h3>
                <p>
                    if the heat you have driven is at least 3 hours old you should be able to find your heat in the list
                    of heats at the <Link to="/heats" style={linkStyle}>heats page</Link>.
                    or if you are interested in your own laps you cna look for your name on the <Link to="/drivers"
                                                                                                      style={linkStyle}>leaderboard</Link>.
                    if you can't find your heat or laps you can add them yourself by entering the heat id in the input
                    field above.
                    You can find your heat id in the email that you got send after the heat. the heatid is present in
                    the url of the results page.
                </p>

                <h3 style={{
                    fontSize: "1.5rem",
                }}> Why does this exist</h3>
                <p>
                    this is a opensource hobby project made by <Link to="https://github.com/michelgerding"
                                                                     target="_blank" style={linkStyle}> me Michel </Link>
                    and the code can be found at <Link to="https://github.com/michelgerding/kartbaan_groningen"
                                                       target="_blank"
                                                       style={linkStyle}> the Github repo</Link>.

                    this project has been created because i wanted to be more competative when karting. so i made this
                    application for my local karting track.
                    using this application i was able to estemate the best kart for the day and improve my consistency
                    at the track

                    If you have any feature request feel free to create a issue on the repo.
                </p>
            </div>

        </div>
    );
}