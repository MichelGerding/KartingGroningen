export const loadData = (globalData: GlobalData, dataType: string, id: string) => {
    const gdk = `${dataType}|${id}`;
    if (globalData[gdk] === undefined) {
        globalData[gdk] = new Promise(async (resolve) => {
            const response = await fetch(`${import.meta.env.VITE_API_BASEURL}/api/${dataType}/${id}/full`);
            let data = await response.json() as any;
            resolve(data);
        });
    }

    return globalData[gdk];
}

export type GlobalData = {
    [key: string]: Promise<any>
}