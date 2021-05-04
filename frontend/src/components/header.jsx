import React from 'react';

export default function Header() {
    return (
        <div className='w-full fixed shadow-lg bg-light-gray'>
            <div className='w-full h-24 md:w-4/5 flex mx-auto items-center justify-between'>
                <div className='text-4xl text-white my-auto font-bold font-sans tracking-wide'>
                    OASIS
                </div>
                <div className='text-xl h-full hidden md:block'>
                    <ul className='flex space-x-8 h-full'>
                        <li className='h-full flex items-center ' >
                            <span className='text-pink-600'>
                                Home
                            </span>
                        </li>
                        <li className='h-full flex items-center ' >
                            <a href='#'>
                                Movie
                            </a>
                        </li>
                        <li className='h-full flex items-center relative' >
                            <a href='#'>
                                ...
                            </a>
                            <div className='dropdown-menu hidden'>
                                <p><a href='#'>Photos</a></p>
                                <p><a href='#'>Music</a></p>
                            </div>
                        </li>
                    </ul>
                </div>
                <div className='flex space-x-4 h-full items-center'>
                    <div className='h-full flex items-center'>
                        <a href='#'>EN</a>
                        <div className='dropdown-menu hidden'>
                            <p><a href='#'>中文</a></p>
                        </div>
                    </div>
                    <div className='nav-icon'>
                        <ion-icon name="search"/>
                    </div>
                    <div className='nav-icon'>
                        <ion-icon name="person-circle-outline"/>
                    </div>
                    <div className='nav-icon md:hidden'>
                        <ion-icon name="grid-outline"/>
                    </div>
                </div>
            </div>
        </div>
    );
}