# Tried doing this in dagster as a "webscale" joke, this is the result.
# I am not proud of this but I also don't want to delete it so...

import os
from os import listdir
from os.path import isfile, join
from typing import List

import papermill as pm
import parse
import requests
from dagster import Out, asset, op
from parsel import Selector
from pydantic import BaseModel


class AOCDate(BaseModel):
    year: int
    day: int


class AOCDay(BaseModel):
    date: AOCDate
    started: bool
    done: bool
    level: int


class AOCDayWithInput(BaseModel):
    day: AOCDay
    problem_input: str


def parse_date(row: Selector) -> AOCDate:
    (year, day) = parse.parse("/{0}/day/{1}", row.xpath("@href").get())
    return AOCDate(day=day, year=year)


@asset
def aoc_days() -> List[AOCDay]:
    data = requests.get(
        "https://adventofcode.com/2021",
        cookies={"session": os.getenv("AOC_COOKIE")},
    )
    select = 'pre[class="calendar"]>a'
    day_rows = Selector(data.text).css(select)

    return [
        AOCDay(
            date=parse_date(row),
            started="true" != row.xpath("aria-hidden").get(),
            done="verycomplete" in row.xpath("@class").get(),
            level=2 if "complete" in row.xpath("@class").get() else 1,
        )
        for row in day_rows
    ]


# find unfinished problems
# find runs for those problems that have a successful sainity check
# submit them


@asset
def unfinished_aoc() -> List[AOCDay]:
    days = aoc_days()
    return [day for day in days if not day.done and day.started]


@asset
def notebooks() -> List[str]:
    return [
        f
        for f in listdir("solutions")
        if isfile(join("solutions", f)) and f.endswith(".ipynb")
    ]


@asset
def aoc_inputs() -> List[AOCDayWithInput]:
    problems = unfinished_aoc()
    out = []
    for problem in problems:
        with_input = AOCDayWithInput(
            day=problem,
            problem_input=problem_input(problem.date.year, problem.date.day),
        )
        out.append(with_input)
    return out


@op(out={"solution": Out(str)})
def problem_solution(year: int, day: int, day_input: str):
    file = f"solutions/{year}/day{day}.ipynb"
    output = f"solutions/{year}/day{day}.output.ipynb"
    pm.execute_notebook(file, output, parameters=dict(problem_input=day_input))
    return open(output, "r").read()


@op(out={"text": Out(str)})
def problem_input(year: int, day: int):
    data = requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input",
        cookies={"session": os.getenv("AOC_COOKIE")},
    )
    if data.status_code == 200:
        return data.text
    else:
        raise Exception(f"No Data for {year}/{day}")


@op(out=Out(str))
def problem_submission(year: int, day: int, level: int, answer: str):
    data = {
        "level": level,
        "answer": answer,
    }

    return requests.post(
        f"https://adventofcode.com/{year}/day/{day}/answer",
        cookies={"session": os.getenv("AOC_COOKIE")},
        data=data,
    )
