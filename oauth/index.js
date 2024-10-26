const extractInfoFromEmail = require('./gmailChecker.js'); // path to the script

async function getInformation() {
  try {
    // Specify the exact "From" header and the regex pattern to identify the necessary content.
    const content = await extractInfoFromEmail('Robin <robin@abstract.money>', '/^\\s*(\\d+)\\s*$/gm', '/Users/robin/Programming/External/gmail-email-checker/client_secret.json', true);
    if (content) {
      console.log('Extracted Content:', content);
    } else {
      console.log('No content matches the provided criteria.');
    }
  } catch (error) {
    console.error('An error occurred:', error.message);
  }
}

getInformation();