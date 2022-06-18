import useFetch from './useFetch'
import {Loading, ErrorComponent} from './Common'
const {REACT_APP_BASE_URL} = process.env;


const handleClick = (creator_name, title) => {
	window.location.href=`http://localhost:3000/${creator_name}/${title}`
}


const ContentJist = ({blogs}) => {
    return (
        <div className="all-blogs">
        {
            blogs &&
            blogs.map(blog => (
                <div className="content-jist" key = {blog.content_id} onClick={() => {handleClick(blog.creator_name, blog.title)}}>
                    <h2>{blog.title.charAt(0).toUpperCase() + blog.title.slice(1)}</h2>
                    <p>Description: {blog.description}</p>
                    <p>Author: {blog.creator_name}</p>
                </div>
            ))
        }
        </div>
    );
}

const Allblogs = () => {


	console.log(REACT_APP_BASE_URL);
    const {articles, isPending, error} = useFetch('http://localhost:8080/blog');

    return (
        <div className="blog-container">
            { isPending && <Loading/ >}
            { error && <ErrorComponent/>}
            <ContentJist blogs={articles} />
        </div>

    )
}

export default Allblogs;
