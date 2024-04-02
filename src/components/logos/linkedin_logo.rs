use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(LinkedInLogo)]
pub fn linkedin_logo() -> Html {
    html!(
        <a href="https://www.linkedin.com/in/korntewin/" target="_blank" rel="noopener noreferrer">
           <img
               class={css!("
                        height: 50px;
                        padding: 10px;
               ")}
               src={"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAOEAAADhCAMAAAAJbSJIAAAAclBMVEUAAAD///+AgIBXV1dRUVHZ2dnd3d0mJibw8PCWlpY5OTnR0dH7+/v19fWTk5PW1tbp6em8vLwRERHNzc1KSkoeHh5EREShoaFnZ2epqamvr68zMzMsLCyIiIgWFhbCwsJ4eHhubm57e3tfX18hISGMjIyO403oAAAES0lEQVR4nO2d6ZaiMBBGjQuCgoCgLMq4v/8rTnerM62GVGySk4T+7m851D0slVSFOBgAAAAAAAAAAAAAAAAAAAAAAAD4tYwWdRbHWb3ITUeihXWdRCH7IoySYGs6HtVsMvZIWPXKcVuwV/zmj+m4lDF+voA3enMZS48vyFg6Mx2bEsatgoztVqajU8AmbhdkrDYdngJSkSALF6bj60wuFGQsdj77Cx7CK4XpCDtyoARZMjIdYzfmpCFrTMfYialPG6ZT01F2YRjShtHadJRdEKeKGyfTUXZBmO3v7E1H2YWJjKHLSf8PmQ0/qUyH2YH+G/b/Lv0Fb5qdjGFpOsouNBIZnzk9uzj3ftQm8yC6PfIe7ElB12dPEjNg16umJSHouX4JqYQRul7E+GAtvE93S9PxKWCUtAtOTAenhkOrouf6W+bOiV+PCrM+3KJX1nX0KpgU/REcDJbH51eqXzs94Oaw2dffHse4KPvQdXpiNTs1Qb1Lq2A43vTpBgUAAAAAAOCBzelwbBbBJ5djOZ6dTQeklE2T7uZe4kfhFT/x4ixLL/1YGzgYDLOEM8f+IErmF/1dg/MwnXOpD22HzCr+EdfDnhdvLHxRAyiMvNbzqEHUf0rG3ENycWf1oRGwrYS/veKVGiekB3Gw3NbakQj4+O+X571UI52x3UlXXW9JRMBdYDokwh3ef7iu5fw+qTT1D7bEeTPeWm9Zw1Ji2dx/ND2OY+K03PWlkoaNoJrOw7/oeBpHxFknvHeNnGEj0WB+JKo0VPj0GQ75GVCMhqXz2gzp7jIXzxnD0du36I3UEcOp1FokLqqXYGkyfCMPPhMeOWe0zXB/+rngxzhKbU9Ii2EgtSKwlUzpumsthj/JE99R+ihqMeyKx5/S9MiQqWw/22noK7yIdhqq/CrQUkOmrkJlq6G6NZ+2GqobgdtqyPpvOOSct1+GsWOG0dsDOV9VKVy/YRjXVXFpgqJK36lMRapmGLoNk+KY34dg09MllfnA44tQVb7QbFjkjyPM7UG2hKps/bVWQy9/HUGvZA9WVbDRaZjyWxEzubeOqpepPsOwapsCraS6NarWmOszrNuLu2uZl6qnaI6ozXAuKra0L57/ZqgoIeoyjIS9siVv2ybHDIkp7JpOGrYbUh3dgEz9lhsG1HlXZNnfckM6OnLDCrsNE/rEJdWcsttQYky5pBKG3YYyMx+3DWUKulS+sNtQpnlEtRitNoxk6rmBy4Y+DGEIQxjCEIYwhCEMYQhDGMIQhjCEIQxhCEMYwhCGMIQhDGEIQxjCEIYwhCEMYQhDGMIQhjCEIQxhCEMYwhCGv9dQy96XVhmS+5duXDekPrDifhXvlCHx53n8fcWUGC4oQ2Ubmjfte1GHLRv85cRXWVL/mbglNh5QuAlmXs0nMYdJtmj7JH604x5xo+I9u6/s5xMBldt/fwoAAAAAAAAAAAAAAAAAAAAAAOBN/gKmA2Ckdxu8TgAAAABJRU5ErkJggg=="}
           />
        </a>
    )
}
