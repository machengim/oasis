import React from 'react';

export default function Header() {
    console.log('header loaded');

    return (
        <div className='header-container'>
        <div id='header'>
            <div id='logo'>
                OASIS
            </div>
            <div id='nav-items'>
                <ul>
                    <li>
                        <a href='#'>Home</a>
                    </li>
                    <li>
                        <a href='#'>Movie</a>
                    </li>
                    <li>
                        <a href='#'>TV Shows</a>
                    </li>
                    <li>
                        <a href='#'>Photos</a>
                    </li>
                    <li>
                        <div className='dropdown'>
                            ...
                            <div className='dropdown-content'>
                                <p>Cartoon</p>
                                <p>Pictures</p>
                            </div>
                        </div>
                    </li>
                </ul>
            </div>
            <div id='header-right'>
                <ion-icon name="search"></ion-icon>
                <div className='dropdown'>
                    EN
                    <div className='dropdown-content'>
                        <p>中文</p>
                        <p>日本語</p>
                    </div>
                </div>
                <ion-icon name="person-outline"></ion-icon>
                <div id='mobile-btn'>
                    <div className='mobile-btn-line'></div>
                </div>
            </div>
        </div>
        </div>
    );
}