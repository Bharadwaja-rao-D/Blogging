import {useState, useEffect} from 'react'

const useFetch = (url) => {

    const [articles, setArticles] = useState(null)
    const [isPending, setIsPending] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {setTimeout(() => {
        fetch(url)
        .then(res =>{
            if (!res.ok){
                throw Error("Fetching failed");
            }
        return res.json();
        })
        .then(data => {
            setIsPending(false);
            setArticles(data);
            setError(null);
            setIsPending(false);
        })
        .catch( err => {
            setError(err.message);
            setIsPending(false);
        })
    },0);

}, [url])

    return {articles, isPending, error};
}

export default useFetch
