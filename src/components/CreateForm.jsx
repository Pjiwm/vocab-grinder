import React, { useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";

const CreateForm = () => {
  const [vocabListName, setVocabListName] = useState('');
  const [vocabListContent, setVocabListContent] = useState('');

  const handleCreateClick = async () => {
    console.log("submit")
    try {
      const response = await invoke('create_list', {
        name: vocabListName,
        content: vocabListContent
      });
      alert(response);  // Show the response from the Rust command
      console.log(response)
    } catch (error) {
      console.error('Error creating vocab list:', error);
      alert('Failed to create vocab list.');
    }
  };

  return (
    <div className="flex flex-col justify-center items-center h-screen w-screen bg-gray-800">
      <h1 className="w-screen text-3xl font-bold text-white py-12 mt-16 text-center text-4xl">Create New Vocab List</h1>
      <div className="px-32 py-16 bg-gray-800 w-screen mb-72 mx-4">
        <form onSubmit={(e) => e.preventDefault()}>
          <div className="mb-6">
            <label className="block text-gray-300 text-xl font-bold mb-2" htmlFor="vocab-list-name">
              Vocab List Name
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-3 px-4 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="vocab-list-name"
              type="text"
              placeholder="Enter list name"
              value={vocabListName}
              onChange={(e) => setVocabListName(e.target.value)}
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
              value={vocabListContent}
              onChange={(e) => setVocabListContent(e.target.value)}
            ></textarea>
          </div>
          <div className="flex items-center justify-left">
            <button
              className="bg-red-500 hover:bg-red-700 text-white font-bold py-3 px-6 rounded focus:outline-none focus:shadow-outline"
              type="button"
              onClick={handleCreateClick}
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
