import React, {useRef} from 'react';

export default function Continue() {
    const carouselRef = useRef(null);

    let carouselOffset = 0;
    function moveCarousel(direction) {
        if (!carouselRef) return;

        let offset = (direction > 0)? 300: -300;
        carouselOffset += offset;
        if (carouselOffset < 0) {
            carouselOffset = 0;
        }
        else if (carouselOffset > carouselRef.current.scrollWidth) {
            carouselOffset = carouselRef.current.scrollWidth;
        }
        console.log(carouselOffset);

        carouselRef.current.scrollTo({
            top: 0,
            left: carouselOffset,
            behavior: 'smooth'
        });
    }

    return (
        <div className='w-full bg-deer pb-24 border-b-2 border-pink-600'>
            <div className='w-4/5 mx-auto pt-24'>
                <div className='w-full flex justify-between mt-16'>
                    <div className='text-white text-3xl flex justify-between'>
                        CONTINUE WATCHING
                    </div>
                    <div className='flex text-3xl space-x-4'>
                        <div className='cursor-pointer'>
                            <ion-icon name="arrow-back-outline" 
                                onClick={() => moveCarousel(-1)}
                                />
                        </div>
                        <div className='cursor-pointer'>
                            <ion-icon name="arrow-forward-outline"
                                onClick={() => moveCarousel(1)} 
                            />
                        </div>
                    </div>
                </div>
                <div className='w-full mt-8 relative'>
                    <div className='flex space-x-8 overflow-hidden' ref={carouselRef}>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                        <div className='flex flex-col w-40'>
                            <div className='w-full h-60 opacity-70 hover:opacity-100'>
                                <img src="/images/starwar.jpg" />
                            </div>
                            <p className='text-white text-xl truncate mt-2'>
                                Star Wars Blablanblan
                            </p>
                            <p className='text-pink-600 text-md'>
                                Movie
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}