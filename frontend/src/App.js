import Navbar from './components/Navbar'
import Allblogs from './components/Allblogs'
import AddBlog from  './components/AddBlog'
import CompleteBlog from './components/Blog'
import {Signin, Signup} from './components/Login'
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

function App() {
  return (
	 <Router>
		  <div className="App">
			<Navbar />
			<div className="content">
	  			<Switch>
	  				<Route exact path= "/"><Allblogs /></Route>
	  				<Route exact path= "/signin"><Signin /></Route>
	  				<Route exact path= "/signup"><Signup /></Route>
	  				<Route exact path= "/addblog"><AddBlog /></Route>
	  				<Route exact path= "/:creator_name/:title"><CompleteBlog /></Route>
	  			</Switch>
			</div>
		  </div>
		</Router>
  );
}

export default App;
