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
        <div className='continue-container'>
        <div id='continue-watching'>
            <div className='title'>
                <div>CONTINUE WATCHING</div>
                <div>
                    <ion-icon name="arrow-back-outline" onClick={() => moveCarousel(-1)}></ion-icon>
                    <ion-icon name="arrow-forward-outline" onClick={() => moveCarousel(1)}></ion-icon>
                </div>
            </div>

            <div id="carousel">
                <div id="carouselbox" ref={carouselRef}>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//kOVEVeg59E0wsnXmF9nrh6OmWII.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Star Wars</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//tWqifoYuwLETmmasnGHO7xBjEtt.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Beauty and the Beast haha</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//dImWM7GJqryWJO9LHa3XQ8DD5NH.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Fate of the Furious</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//87aWrVqaVhXhblhO7sYHLC2y8TT.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Wolf of Warrior II</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//c24sv2weTHPsmDa7jEMN0m2P3RT.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Spider-man</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//y4MBh0EjBlMuOzv9axM4qJlmhzz.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Guardians of the Galaxy</span>
                        <span className="item-cat">TV series</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//pSgXKPU5h6U89ipF7HBYajvYt7j.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Jumanji</span>
                        <span className="item-cat">Cartoon</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//kOVEVeg59E0wsnXmF9nrh6OmWII.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Star Wars</span>
                        <span className="item-cat">Movie</span>
                    </div>
                    <div className="carousel-item">
                        <div className="item-poster">
                        <img src="http://image.tmdb.org/t/p/w185//tWqifoYuwLETmmasnGHO7xBjEtt.jpg" /> 
                        <progress value="40" max="100"></progress>
                        </div>
                        <span className="item-name">Beauty and the Beast haha</span>
                        <span className="item-cat">Movie</span>
                    </div>
                </div>
            </div>
        </div>
        </div>
    );
}