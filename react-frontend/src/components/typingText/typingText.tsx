import React, {useEffect, useState} from "react";

interface TypingTextProps extends React.InputHTMLAttributes<HTMLSpanElement>{
    text: string;
    interval: number;
}

export default function TypingText({text, interval, ...passedProps}: TypingTextProps) {
    const [placeholder, setPlaceholder] = useState(text.slice(0, 0));
    const [placeholderIndex, setPlaceholderIndex] = useState(0);

    useEffect(() => {
        const intr = setInterval(() => {
            setPlaceholder(text.slice(0, placeholderIndex));
            if (placeholderIndex + 1 > text.length) {
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


    return <span {...passedProps}>{placeholder}</span>

}