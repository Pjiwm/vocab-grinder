import React, { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/tauri";

const CreateForm = () => {
  const [vocabListName, setVocabListName] = useState('');
  const [vocabListContent, setVocabListContent] = useState('');
  const [isCreating, setIsCreating] = useState(false);

  const handleCreateClick = async () => {
    console.log("submit");
    setIsCreating(true);
    try {
      const response = await invoke('create_list', {
        content: vocabListContent,
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
        {isCreating && <ProgressBar vocabListName={vocabListName} />}
      </div>

    </div>
  );
};


const ProgressBar = ({ vocabListName }) => {
  const [progressStatus, setProgressStatus] = useState(0);
  const [progressDone, setProgressDone] = useState(false);
  const [listId, setListId] = useState(null);
  const [isComputingDone, setIsComputingDone] = useState(false);
  const [isSavingToDB, setIsSavingToDb] = useState(false);

  const fetchProgress = async () => {
    try {
      const progress = await invoke('request_progress');
      setProgressStatus(progress);
    } catch (error) {
      console.error('Error fetching progress:', error);
    }
  };

  const computeList = async () => {
    try {
      console.log("NAME:", vocabListName);
      const listIdResponse = await invoke('compute_list', {
        listName: vocabListName,
      });
      setListId(listIdResponse);
      console.log('Compute list response:', listIdResponse);
    } catch (error) {
      console.error('Error computing list:', error);
    }
  };

  const fetchComputingStatus = async () => {
    try {
      const progress = await invoke('is_computing_done');
      setIsComputingDone(progress);
      if (progress === true && !isSavingToDB && listId !== null) {
        console.log("Computing is done");
        saveList();
      }
    } catch (error) {
      console.error('Error fetching computing done status:', error);
    }
  };

  const saveList = async () => {
    try {
      if (listId === null) {
        console.error('List ID is null, cannot save list');
        return;
      }
      setIsSavingToDb(true);
      await invoke("save_list", { listId });
      console.log('List saved to database successfully');
    } catch (error) {
      console.error("Error saving list to database:", error);
    }
  };

  useEffect(() => {
    const progressInterval = setInterval(() => {
      fetchProgress();
    }, 300);

    return () => clearInterval(progressInterval);
  }, []);

  useEffect(() => {
    if (progressStatus >= 100 && !progressDone) {
      computeList();
      setProgressDone(true);
    }
  }, [progressStatus, progressDone]);

  useEffect(() => {
    const computingStatusInterval = setInterval(() => {
      fetchComputingStatus();
      console.log(isComputingDone, listId, isSavingToDB);
    }, 300);

    return () => clearInterval(computingStatusInterval);
  }, []);

  useEffect(() => {
    if (isComputingDone && listId !== null && !isSavingToDB) {
      console.log("Saving list with ID: ", listId);
      saveList();
    }
  }, [isComputingDone, listId, isSavingToDB]);
  return (
    <div className="flex items-center justify-center p-4 bg-gray-800 rounded-lg">
      <div className="relative flex items-center">
        <div className="inset-0 flex items-center justify-center">
          <svg className="w-10 h-10 text-gray-200 animate-spin" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path fill="none" d="M0 0h24v24H0z" />
            <path d="M4 12a8 8 0 0 1 16 0h-2a6 6 0 0 0-12 0H4z" fill="currentColor" />
          </svg>
        </div>
        <p className='text-white text-2xl font-semibold'>{progressStatus.toFixed(2)}%</p>
      </div>
    </div>
  );
}

export default CreateForm;
