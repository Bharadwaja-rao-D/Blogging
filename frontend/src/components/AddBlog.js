import { useState } from "react";
const {REACT_APP_BASE_URL} = process.env;

const AddBlog = () => {

	const [title, setTitle] = useState('');
	const [content, setContent] = useState('');
	const [description, setDescription] = useState('');

	const handleSubmit = (e) => {
		e.preventDefault();

		//TODO: Here we need to use cookie to get creator_id and creator_name
		const new_blog = {title, description, body: content, creator_id: 1};

		fetch(REACT_APP_BASE_URL+'/blog', {
			method: 'POST',
			headers: {'Content-Type': 'application/json' },
			body: JSON.stringify(new_blog)
		})
		.then(() => {
			console.log('Form submitted')

			//TODO: Need to get creator_name from the cookie
			const creator_name = 'anonymous';
			window.location.href=`http://localhost:3000/${creator_name}/${title}`
		})
	}

	return (
		<div className="add-blog">
			<form className="add-blog-form" onSubmit={(e) => handleSubmit(e)}>
				<label>Title:</label>
				<input type="text" required value={title} onChange={(e) => setTitle(e.target.value)}/>
				<label>Description:</label>
				<textarea required value={description} onChange={e => setDescription(e.target.value)}></textarea>
				<label>Content:</label>
				<textarea required value={content} rows="15" onChange={e => setContent(e.target.value)}></textarea>
				<button>Add Blog</button>
			</form>
		</div>
	);
}

export default AddBlog;

