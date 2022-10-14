import pandas as pd

config = {
    "word_data_source": "unigram_freq.csv",
    "search_limit": 2000,
    "minimum_word_length": 7,
}

def main():
    word_freq_data = pd.read_csv(config["word_data_source"])
    words = word_freq_data["word"].head(config["search_limit"]) if config["search_limit"] else word_freq_data["word"]
    games_file = open("games.txt", "w+", encoding="utf-8")
    games_file.truncate(0)
    for w in words:
        if len(w) > config["minimum_word_length"]:
            print(w)
            games_file.write(f'{w},')


if __name__ == "__main__":
    main()
    