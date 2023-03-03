import styles from "./styledButton.module.css";

interface StyledButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    text: string;
    inverted?: boolean;
}

export default function StyledButton({text, inverted, onClick, ...passedProps} : StyledButtonProps) {
    inverted = inverted || false;

    let className = inverted ? styles.buttonInverted : styles.button;

    return (
        <button className={className} onClick={onClick} {...passedProps}>{text}</button>
    );
}