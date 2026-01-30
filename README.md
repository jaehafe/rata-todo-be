# rata-todo-be

Rust 기반 Todo REST API 서버

> Frontend: [rata-todo-tui](https://github.com/jaehafe/rata-todo-tui)

## Stack

- Rust
- axum (웹 프레임워크)
- Diesel (ORM)
- PostgreSQL
- tokio (비동기 런타임)

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/todos` | 전체 조회 |
| POST | `/api/v1/todos` | 생성 |
| GET | `/api/v1/todos/{id}` | 단일 조회 |
| PUT | `/api/v1/todos/{id}` | 수정 |
| DELETE | `/api/v1/todos/{id}` | 삭제 |

## Architecture

[Lemmy](https://github.com/LemmyNet/lemmy) 스타일의 레이어드 아키텍처 적용

```
┌──────────────────────────────────────────┐
│              API Layer                   │  HTTP 요청/응답, 라우팅
│              (api/)                      │
├──────────────────────────────────────────┤
│            Model Layer                   │  데이터 구조 정의, DTO
│            (models/)                     │
├──────────────────────────────────────────┤
│              DB Layer                    │  CRUD 로직, 쿼리 실행
│              (db/)                       │
├──────────────────────────────────────────┤
│           Error Layer                    │  에러 타입, HTTP 상태 매핑
│           (error.rs)                     │
└──────────────────────────────────────────┘
```

```
src/
├── main.rs       # 서버 진입점
├── error.rs      # 에러 처리
├── schema.rs     # Diesel 스키마
├── api/
│   └── todo.rs   # API 핸들러
├── db/
│   └── todo.rs   # DB 작업
└── models/
    └── todo.ts   # 데이터 모델
```

**레이어 분리 원칙**
- `api/` - HTTP 레이어만 담당, 비즈니스 로직 없음
- `models/` - 순수 데이터 구조 (Todo, NewTodo, UpdateTodo)
- `db/` - Model에 impl로 CRUD 메서드 구현, DB 접근 캡슐화
- `error.rs` - `AppError` enum으로 에러를 axum Response로 변환

## Database Schema

```sql
CREATE TABLE todo (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    completed   BOOLEAN DEFAULT FALSE,
    created_at  TIMESTAMPTZ DEFAULT NOW(),
    updated_at  TIMESTAMPTZ
);
```

## Run

```bash
# .env 설정
DATABASE_URL=postgres://user@localhost:5432/rata-todo

# 마이그레이션
diesel migration run

# 서버 실행
cargo run
```

서버: `http://localhost:8080`
