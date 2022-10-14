# Hookman

Hookman is simply a CLI version of the classic hangman game written in the Rust programming language. The name is inspired by the idea of using a rusty hook instead of a rope to build the hangman on.

The project is based off of this awesome [tutorial series by dcode on YouTube](https://www.youtube.com/playlist?list=PLVvjrrRCBy2Igh_kCtvRr2Np4fMRawn6x). 
Check it out if you want an enjoyable intro to Rust.

## Words Data
The words data used for the game was derived from this [English Word Frequency kaggle dataset.](https://www.kaggle.com/datasets/rtatman/english-word-frequency)

You can configure and generate a different word set for your hookman games by configuring and running the python script (`write_games_file.py`).

Configuration options are:
- Word Data Source :: path of text file containing comma-separated words used for building the games text file
- Search Limit :: max rows to read through the CSV data source when adding to the games text file
- Minimum word length :: minumum character length for a word to be considered and added to the games text file

The script requires Python 3 (only tested using v3.9) to be available in your local environment with the `pandas` package installed.

To build `games.txt`, run the following from the repository's root directory:
```py
python3 write_games_file.py
```
