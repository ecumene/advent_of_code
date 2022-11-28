import os

import requests
from parsel import Selector


def http_get_aoc_day(year: int, day: int) -> str:
    data = requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input",
        cookies={"session": os.getenv("AOC_COOKIE")},
    )
    if data.status_code == 200:
        return data.text
    else:
        message = f"Error from AOC server: {data.status_code} -- {data.text}"
        raise Exception(message)


def http_post_answer(year: int, day: int, level: int, answer: str):
    data = {
        "level": level,
        "answer": answer,
    }

    data = requests.post(
        f"https://adventofcode.com/{year}/day/{day}/answer",
        cookies={"session": os.getenv("AOC_COOKIE")},
        data=data,
    )

    return Selector(data.text).css("p").get()
