# FastAPI Hello World

A simple FastAPI application that returns "Hello World".

## Setup

1. Install dependencies:
```bash
pip install -r requirements.txt
```

## Running the application

Option 1: Run directly with Python:
```bash
python main.py
```

Option 2: Run with uvicorn:
```bash
uvicorn main:app --reload
```

## API Endpoints

- `GET /`: Returns a "Hello World" message
- `GET /docs`: Interactive API documentation (provided by Swagger UI)
- `GET /redoc`: Alternative API documentation (provided by ReDoc) 