import React, { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";

const DetailListView = ({ listId }) => {
  const [wordsArray, setWordsArray] = useState([]);

  const handleObtainListWords = async () => {
    try {
      const words = await invoke('show_list_items', { listId });
      console.log("Obtained words from list:", words.length, "words");
      setWordsArray(words);
    } catch (error) {
      console.error('Error obtaining words from list:', error);
    }
  };
  useEffect(() => {
    handleObtainListWords();
  }, []);

  return (
    <div className="w-full">
      <table className="table-auto w-full text-left text-white">
        <thead className="bg-gray-700 border">
          <tr>
            <th className="px-4 py-2">Word</th>
            <th className="px-4 py-2">Reading</th>
            <th className="px-4 py-2">Translation</th>
          </tr>
        </thead>
        <tbody>
          {wordsArray.map(entry => (
            <tr className="bg-gray-800">
              <td className="border px-4 py-2">{entry.word}</td>
              <td className="border px-4 py-2">{entry.reading}</td>
              <td className="border px-4 py-2">{entry.translation}</td>
            </tr>

          ))}
        </tbody>
      </table>
    </div>
  );
};

export default DetailListView;
