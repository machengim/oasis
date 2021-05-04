import React from 'react';

export default function NewItem() {
    return (
        <div className='w-full'>
            <div className='w-full h-24 bg-light-gray shadow-lg'>
                <div className='w-4/5 h-full mx-auto flex items-center'>
                    <div className='text-white text-3xl'>
                        NEW ITEMS
                    </div>
                </div>
            </div>
            <div className='w-full pt-4'>
                <div className='w-4/5 mx-auto'>
                    <div className='max-w-md mx-auto shadow-md overflow-hidden md:max-w-2xl'>
                        <div className='md:flex'>
                            <div className='md:flex-shrink-0'>
                                <img className='h-40' />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}