// Get the current file name
function getCurrentFileName() {
    const currentUrl = window.location.href;
    const fileName = currentUrl.substring(currentUrl.lastIndexOf('/') + 1);
    return fileName.replace('.html', '');
  }
  
  // Navigate to the previous or next day
  function navigateTo(direction) {
    const currentDay = parseInt(getCurrentFileName().replace('day', ''));
    let newDay;
  
    if (direction === 'prev') {
      newDay = currentDay - 1;
    } else {
      newDay = currentDay + 1;
    }

    if (newDay >= 1 && newDay <= 25) {
      window.location.href = `day${newDay}.html`;
    } else {
      alert(`You are already at the ${direction === 'prev' ? 'first' : 'last'} day.`);
    }
  }