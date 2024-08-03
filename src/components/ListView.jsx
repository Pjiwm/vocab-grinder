import { MdOutlineDeleteForever } from 'react-icons/md';
import React from 'react';

const ListView = () => {
  return (
    <div className="flex flex-col pt-12 px-4 items-center h-full w-full bg-gray-800 min-h-screen">
      <div className="flex flex-col items-center w-full space-y-5">
        <ListItem noMarginTop={true} />
        <ListItem />
        <ListItem />
        <ListItem />
        {/* Add more ListItem components as needed */}
      </div>
    </div>
  )
};

export default ListView;

const ListItem = ({ noMarginTop }) => {
  return (
    <div className={`w-full md:w-7/12 xl:w-9/12 2xl:w-9/12 rounded shadow-lg bg-gray-900 border-solid border-2 border-red-400
      transition-transform duration-300 hover:scale-105 hover:cursor-pointer ${noMarginTop ? '' : 'my-5'}`}>
      <div className="py-2">
        <div className="grid grid-cols-12 items-center mb-2 w-full">
          <div className="font-bold text-4xl text-white col-span-11 md:px-3">Sample list</div>
          <div className="button-icon text-3xl">
            <MdOutlineDeleteForever />
          </div>
        </div>
        <p className="text-xl text-base text-white px-3">
          Lorem ipsum dolor sit amet, consectetur adipisicing elit. Voluptatibus quia, nulla!
          Maiores et perferendis eaque, exercitationem praesentium nihil.
        </p>
      </div>
    </div>
  )
}
