import React from 'react';

function Home() {
  return (
    <div className='flex flex-col items-center justify-center text-center gap-4 min-h-screen bg-black px-4'>
      <h1 className='text-white text-6xl font-bold'>Welcome to Simlog</h1>
      <p className='text-white text-lg max-w-300'>
        Simlog is a flight logging app for flight simulator pilots.
        It lets users upload and manage detailed flight records
        (aircraft, route, duration, weather, approach/landing quality, notes, etc.),
        browse individual flights or their full history,
        and ask an AI assistant questions about a specific flight for quick insights and recall.
      </p>
    </div>
  );
}

export default Home;