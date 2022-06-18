import { useState } from 'react';
import { useParams } from 'react-router-dom';
import {Loading, ErrorComponent} from './Common';
import useFetch from './useFetch'
const {REACT_APP_BASE_URL} = process.env;

const Comments = ({comments}) => {
    return (
        <div className="all-comments">
        {
            comments &&
            comments.map(comment => (
                <div className="specific-comment" key = {comment.comment_id}>
                    <p className="comment-info">{comment.commentor_name}: {comment.comment}</p>
                </div>
            ))
        }
        </div>
    );
}

const Comment = ({blog}) => {

	const [comment, setComment] = useState('')
	const blog_content = blog.content;

	const handleSubmit = (e) => {
		e.preventDefault();

		//TODO: Here we need to use cookie to get creator_id and creator_name
		const new_comment = {commentor_id: 1, comment};

		const {creator_name, title} = blog_content;

		const url = `${REACT_APP_BASE_URL}/blog/${creator_name}/${title}`;
		fetch(url, {
			method: 'POST',
			headers: {'Content-Type': 'application/json' },
			body: JSON.stringify(new_comment)
		})
		.then(() => {
			console.log('Form submitted');
			window.location.href=`http://localhost:3000/${creator_name}/${title}`

		})
	}

	return (
		<div className="all-comments">
			<h3>Comments</h3>
			<Comments comments={blog.comments} />
			<div className="add-comment">
			<form className='new-comment' onSubmit={handleSubmit}>
				<label> Add Comment: </label>
				<textarea 
					required
					value={comment}
					onChange={(e) =>  setComment(e.target.value)}
				>
				</textarea>
				<button>Add Comment</button>
			</form>
		</div>
		</div>
	);
}

const CompleteBlog = () => {
	const { creator_name, title } = useParams();
	const url = `${REACT_APP_BASE_URL}/blog/${creator_name}/${title}`;
	const {articles:blog , isPending, error} = useFetch(url);

	return (
		<div className="complete-blog">
			{isPending && <Loading/>}
			{error && <ErrorComponent/>}
			{blog && (
				<article>
					<h2> {blog.content.title.charAt(0).toUpperCase() + blog.content.title.slice(1)}</h2>
					<p> {blog.content.creator_name}</p>
					<p> {blog.content.body}</p>
					<Comment blog={blog}/>
				</article>
			)
			}
		</div>
	);
}

export default CompleteBlog;
