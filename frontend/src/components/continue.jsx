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
        <div className='w-full bg-deer pb-20 border-b-2 border-pink-600'>
            <div className='custom-container mx-auto pt-20'>
                <div className='w-full flex justify-between mt-16'>
                    <div className='text-white text-3xl flex justify-between'>
                        CONTINUE WATCHING
                    </div>
                    <div className='text-3xl flex space-x-4'>
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
                        <div className='flex flex-col w-48'>
                            <div className='relative shine-in-300 hover:opacity-100'>
                                <img src="images/starwar.jpg" className='w-full h-72' />
                                <progress value='30' max='100' className='absolute w-full bottom-0 left-0'/>
                            </div>
                            <p className='text-white text-2xl truncate mt-2'>
                                Star Wars Blablanblanblan
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