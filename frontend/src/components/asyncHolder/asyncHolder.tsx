import React, {useState, useEffect} from "react";
import LoadingSpinner from "../loadingSpinner/loadingSpinner";

export type AsyncHolderProps = {
    title: string;
    renderDelay?: number;
    promise: Promise<any>;
}

export const AsyncHolder = ({promise, title, renderDelay = 0}: AsyncHolderProps) => {
    const [data, setData] = useState<JSX.Element>(<> </>);
    const [error, setError] = useState<any>(null);
    const [loading, setLoading] = useState<boolean>(true);

    useEffect(() => {
        promise.then((data) => {
            setTimeout(() => {
                setData(data);
                setLoading(false);
            }, renderDelay);
        }).catch((err) => {
            setError(err);
            setLoading(false);
        })
    }, [promise]);

    if (loading) {
        return <LoadingSpinner loadingText={`Loading ${title}`} typingText={'...'}/>
    }

    return (<>
    {data}
   </>
    )
};
