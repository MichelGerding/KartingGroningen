import React, {useState, useEffect} from "react";
import StyledInput from "../styledInput/StyledInput";

interface AnimatedInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    interval: number;
}


export const AnimatedInput = ({interval, placeholder: passedPlaceholder = "", ...passedProps}: AnimatedInputProps) => {
    const [placeholder, setPlaceholder] = useState(passedPlaceholder.slice(0, 0));
    const [placeholderIndex, setPlaceholderIndex] = useState(0);

    useEffect(() => {
        const intr = setInterval(() => {
            setPlaceholder(passedPlaceholder.slice(0, placeholderIndex));
            if (placeholderIndex + 1 > passedPlaceholder.length) {
                setTimeout(() => {
                    setPlaceholderIndex(0)
                }, interval * 4);
            } else {
                    setPlaceholderIndex(placeholderIndex + 1)
            }
        }, interval);
        return () => {
            clearInterval(intr)
        }
    },);


    return <StyledInput {...passedProps} placeholder={placeholder}/>
};