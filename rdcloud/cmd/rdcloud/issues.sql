-- database: rdcloud.db
-- Create table.
CREATE TABLE IF NOT EXISTS
    "issues" (
        "工作项类型" TEXT NOT NULL,
        "标识" TEXT NOT NULL,
        "标题" TEXT NOT NULL,
        "状态" TEXT NOT NULL,
        "领域" TEXT NOT NULL,
        "缺陷位置" TEXT NOT NULL,
        "描述" TEXT NOT NULL,
        "iChange发现版本号" TEXT NOT NULL,
        "引入者" TEXT,
        "创建人" TEXT NOT NULL,
        "创建时间" TEXT NOT NULL,
        "发现活动" TEXT NOT NULL,
        "版本所处阶段" TEXT NOT NULL,
        "缺陷等级" TEXT NOT NULL,
        "研究结论" TEXT NOT NULL,
        "测试目标" TEXT,
        "测试类型" TEXT,
        "测试行为" TEXT,
        PRIMARY KEY ("标识"),
        FOREIGN KEY ("测试目标", "测试类型", "测试行为") REFERENCES "tests" ("目标", "类型", "行为") ON DELETE SET NULL
    );

--Rows count.
SELECT
    COUNT(*)
FROM
    "issues";

SELECT
    COUNT(*)
FROM
    "tests";

-- Unclosed issues.
SELECT
    *
FROM
    "issues"
WHERE
    "状态" NOT IN ('已关闭', '已确认重复', '已确认拒绝', '已取消');

-- Uncovered Issues.
SELECT
    *
FROM
    "issues"
WHERE
    "测试标识" IS NULL;

-- Number of issues per "缺陷位置".
SELECT
    "缺陷位置",
    COUNT(*)
FROM
    "issues"
GROUP BY
    "缺陷位置";

-- Number of issues per "版本所处阶段".
SELECT
    "版本所处阶段",
    COUNT(*)
FROM
    "issues"
GROUP BY
    "版本所处阶段";

-- Number of issues per "引入者".
SELECT
    "引入者",
    COUNT(*)
FROM
    "issues"
GROUP BY
    "引入者";

-- Number of issues per "月份".
SELECT
    strftime('%Y-%m', "创建时间") AS "月份",
    COUNT(*)
FROM
    "issues"
GROUP BY
    strftime('%Y-%m', "创建时间");

-- Number of issues per "iChange发现版本号"
SELECT
    "iChange发现版本号",
    strftime('%Y-%m', MIN("创建时间")) AS "起始时间",
    COUNT(*)
FROM
    "issues"
GROUP BY
    "iChange发现版本号"
ORDER BY
    "创建时间";
