## Context

当前任务列表 (`/`) 调用 `get_all_deployments()` 一次性返回所有记录，渲染到单个 HTML 页面中。无分页、无 API。

## Goals / Non-Goals

**Goals:**
- 支持分页加载，每页 20 条
- 第 1 页由服务器端渲染（快速首屏 + SEO）
- 第 2+ 页由前端 JS 滚动触发，追加到列表
- URL 随页面更新，支持分享和浏览器前进/后退

**Non-Goals:**
- 不显示总数 ("共 X 条")
- 不提供可配置的每页条数
- 不做客户端排序/筛选

## Decisions

### 1. 分页查询放在 database 层

```rust
// database/mod.rs
pub fn get_deployments_paginated(&self, page: u32, page_size: u32) -> SqliteResult<Vec<Deployment>> {
    let conn = self.conn.lock();
    let offset = (page - 1) * page_size;
    let mut stmt = conn.prepare(
        "SELECT ... FROM deployments ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
    )?;
    // ...
}
```

**理由**: SQL LIMIT/OFFSET 直接在数据库层完成，简洁高效。

### 2. Manager 层透传分页参数

```rust
// deploy/manager.rs
pub fn get_deployments_paginated(&self, page: u32, page_size: u32) -> Vec<Deployment> {
    // 合并 queue + db 结果后 slice 分页
}
```

**理由**: 保持现有合并逻辑，只在返回前做分页切片。

### 3. JSON API 返回 HTML 片段

`GET /api/deployments?page=N` 返回渲染好的 `<tr>...</tr>` 片段，前端直接 innerHTML 追加。

**理由**: 复用现有 askama 模板，无需新建 JSON 数据结构。前端处理简单。

### 4. 无限滚动实现

```javascript
// list.html JS
let currentPage = 1;
let loading = false;
let exhausted = false;

window.addEventListener('scroll', () => {
  if (exhausted || loading) return;
  if (nearBottom()) {
    loadNextPage();
  }
});

async function loadNextPage() {
  loading = true;
  currentPage++;
  const resp = await fetch(`/api/deployments?page=${currentPage}`);
  const html = await resp.text();
  if (html.trim() === '') {
    exhausted = true;
  } else {
    document.querySelector('tbody').insertAdjacentHTML('beforeend', html);
    history.replaceState({}, '', `/?page=${currentPage}`);
  }
  loading = false;
}
```

### 5. 初始页面渲染

`list_deployments` 从 `/?page=N` 读取 page 参数，默认 page=1。模板直接渲染指定页。

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 滚动加载时旧数据被覆盖 | append 而非 replace |
| 页面刷新丢失位置 | URL 包含 page 参数，刷新后回到同页 |
| 并发滚动触发多次请求 | `loading` 锁防止重复请求 |

## Open Questions

- (无)
