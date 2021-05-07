import React from 'react';

export default function NewItem() {
    return (
        <div className='w-full  border-b-2 border-pink-600'>
            <div className='w-full h-24 bg-light-gray shadow-lg'>
                <div className='custom-container h-full mx-auto flex items-center'>
                    <div className='text-white text-3xl'>
                        NEW ITEMS
                    </div>
                </div>
            </div>
            <div className='w-full h-auto'>
                <div className='custom-container mx-auto py-6 grid grid-cols-none lg:grid-cols-2 gap-8'>
                    <div className='flex max-w-2xl h-60 overflow-hidden shine-in-300 hover:opacity-100'>
                        <img src='/images/starwar.jpg' className='w-40 h-60' />
                        <div className='ml-6'>
                            <p className='text-pink-600'>Movie</p>
                            <p className='text-2xl text-white w-5/6 truncate'>Star Wars Blablbsbsbsbxxxssdss</p>
                            <p className='my-2'>
                                <span className='tag-with-border mr-2'>Sci-fi</span>
                                <span className='tag-with-border mr-2'>Act</span>
                            </p>
                            <p className='mt-8 wrap w-5/6'>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.</p>
                        </div>
                    </div>
                    <div className='flex max-w-2xl h-60 overflow-hidden'>
                        <img src='/images/starwar.jpg' className='w-40 h-60' />
                        <div className='ml-6'>
                            <p className='text-pink-600'>Movie</p>
                            <p className='text-2xl text-white truncate'>Star Wars</p>
                            <p className='my-2'>
                                <span className='tag-with-border mr-2'>Sci-fi</span>
                                <span className='tag-with-border mr-2'>Act</span>
                            </p>
                            <p className='wrap'>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.</p>
                        </div>
                    </div>
                    <div className='flex max-w-2xl h-60 overflow-hidden'>
                        <img src='/images/starwar.jpg' className='w-40 h-60' />
                        <div className='ml-6'>
                            <p className='text-pink-600'>Movie</p>
                            <p className='text-2xl text-white truncate'>Star Wars</p>
                            <p className='my-2'>
                                <span className='tag-with-border mr-2'>Sci-fi</span>
                                <span className='tag-with-border mr-2'>Act</span>
                            </p>
                            <p className='wrap'>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.</p>
                        </div>
                    </div>
                    <div className='flex max-w-2xl h-60 overflow-hidden'>
                        <img src='/images/starwar.jpg' className='w-40 h-60' />
                        <div className='ml-6'>
                            <p className='text-pink-600'>Movie</p>
                            <p className='text-2xl text-white truncate'>Star Wars</p>
                            <p className='my-2'>
                                <span className='tag-with-border mr-2'>Sci-fi</span>
                                <span className='tag-with-border mr-2'>Act</span>
                            </p>
                            <p className='wrap'>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.</p>
                        </div>
                    </div>
                    <div className='flex max-w-2xl h-60 overflow-hidden'>
                        <img src='/images/starwar.jpg' className='w-40 h-60' />
                        <div className='ml-6'>
                            <p className='text-pink-600'>Movie</p>
                            <p className='text-2xl text-white truncate w-5/6'>Star Wars Blablablasbsbsbsbsb</p>
                            <p className='my-2'>
                                <span className='tag-with-border mr-2'>Sci-fi</span>
                                <span className='tag-with-border mr-2'>Act</span>
                            </p>
                            <p className='wrap'>Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}