import { useState } from "react";

const Signin = ( {signup = false} ) => {
	const [uname, setUname] = useState('');
	const [pass, setPass] = useState('');
	const [repass, setRePass] = useState('');
	const handleSubmit = (e) => {
		e.preventDefault();
	}
	return (
		<div className="signin">
		<h1>Signin</h1>
		<form className="signin-form" onSubmit={(e) => handleSubmit(e)}>
		<label>Username:</label>
		<input type="text" required value={uname} onChange={(e) => setUname(e.target.value)}/>
		<label>Password:</label>
		<input type="password" required value={pass} onChange={(e) => setPass(e.target.value)}/>
		{ signup && 
			<div className="repassword">
			<label>Password:</label>
			<input type="password" required value={repass} onChange={(e) => setRePass(e.target.value)}/>
			</div>
		}
		<button>Submit</button>
		</form>
		</div>
	);
}

const Signup = () => {
	return (
		<div className="signup">
		<h1>Signup</h1>
		<Signin signup={true} />
		</div>
	);
}

export {Signin, Signup}
