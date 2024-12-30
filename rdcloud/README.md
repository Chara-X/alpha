# rdcloud

## Tables

```sql
CREATE TABLE troubles (
    标识 TEXT PRIMARY KEY NOT NULL, 
    标题 TEXT,
    工作项类型 TEXT,
    状态 TEXT,
    变更大类 TEXT,
    缺陷等级 TEXT,
    创建时间 TEXT,
    创建人部门 TEXT,
    iChange发现版本号 TEXT,
    发现活动 TEXT,
    引入活动 TEXT,
    缺陷来源 TEXT,
    版本所处阶段 TEXT,
    引入者 TEXT,
    缺陷位置 TEXT,
    故障引入年份 TEXT,
    排查标识 TEXT,
    FOREIGN KEY(排查标识) REFERENCES shoots(标识) 
)
```

```sql
CREATE TABLE shoots (
    标识 TEXT PRIMARY KEY NOT NULL,
    排查手段 TEXT,
    手段类别 TEXT
)
```
