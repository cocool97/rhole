
export const timestampToDate = (timestamp) => {
    return (new Date(timestamp * 1000)).toLocaleString()
}