import React from 'react';

const CreateForm = () => {
  return (
    <div className="flex flex-col justify-center items-center h-screen w-screen bg-gray-800">
      <h1 className="w-screen text-3xl font-bold text-white py-12 mt-16 text-center text-4xl">Create New Vocab List</h1>
      <div className="px-32 py-16 bg-gray-800 w-screen mb-72 mx-4">
        <form>
          <div className="mb-6">
            <label className="block text-gray-300 text-xl font-bold mb-2" htmlFor="vocab-list-name">
              Vocab List Name
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-3 px-4 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="vocab-list-name"
              type="text"
              placeholder="Enter list name"
            />
          </div>
          <div className="mb-6">
            <label className="block text-gray-300 text-xl font-bold mb-2" htmlFor="vocab-list-content">
              Paste Text
            </label>
            <textarea
              className="shadow appearance-none border rounded w-full py-3 px-4 text-gray-700 leading-tight focus:outline-none focus:shadow-outline h-64 resize-none"
              id="vocab-list-content"
              placeholder="Paste your text here"
            ></textarea>
          </div>
          <div className="flex items-center justify-left">
            <button
              className="bg-red-500 hover:bg-red-700 text-white font-bold py-3 px-6 rounded focus:outline-none focus:shadow-outline"
              type="button"
            >
              Create
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default CreateForm;
