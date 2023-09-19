from pydantic import BaseModel


class ToDoBase(BaseModel):
    title: str
    description: str | None = None


class ToDoCreate(ToDoBase):
    title: str
    description: str
    is_done: bool


class ToDo(ToDoBase):
    id: int
    title: str
    description: str
    is_done: bool

    class Config:
        orm_mode = True
