import React from 'react';
import ReactDom from 'react-dom';
import Header from './components/header';
import Continue from './components/continue';
import NewItem from './components/newitem';
import Footer from './components/footer';

ReactDom.render(
    <App />, 
    document.getElementById('root')
);

function App() {
	return (
        <div className='w-full h-screen bg-dark-gray text-gray-400'>
            <Header />
            <Continue />
            <NewItem />
            <Footer />
        </div>
    );
}