import StyledButton from "../styledButton/styledButton";
import {useNavigate} from "react-router-dom";

interface LinkProps {
    to: string;
}


export default function RedirectButton ({to}: LinkProps) {
    const navigate = useNavigate();


    return (
        <StyledButton text={"Analyze"} inverted={true} onClick={() => {
            navigate(to);
        }}/>
    )
}
