import React from 'react';
import ReactDOM from 'react-dom';
import Header from './components/header';
import Continue from './components/continue';
import NewItem from './components/newitem';

import './css/index.css';

ReactDOM.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
    document.getElementById('root'),
);

function App() {
    return (
        <div>
            <Header />
            <Continue />
            <NewItem />
        </div>
    )
}