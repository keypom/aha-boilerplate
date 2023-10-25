import React from 'react';
import BlueSquareSrc from './images/BlueSquare.png';
import BgTileSrc from './images/bgTile.png';

const BlueSquare: React.FC = () => {
    return (
        <>
            <img src={BlueSquareSrc} width={52} />
        </>
    );
};

const SquaresBG: React.FC = () => {
    return (
        <div
            className="fixed top-0 left-0 right-0 bottom-0 -z-10 flex w-full justify-between bg-repeat p-8"
            style={{
                backgroundImage: `url(${BgTileSrc})`,
            }}
        >
            <div className="flex h-full flex-col justify-between">
                <BlueSquare />
                <BlueSquare />
            </div>
            <div className="flex h-full flex-col justify-between">
                <BlueSquare />
                <BlueSquare />
            </div>
        </div>
    );
};

export default SquaresBG;
