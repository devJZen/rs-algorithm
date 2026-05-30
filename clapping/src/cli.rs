use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "clapping", about = "Algorithm visualizer for studying sorting & searching")]
pub struct Cli {
    #[arg(short, long, value_enum)]
    pub algorithm: Option<Algorithm>,
    #[arg(short = 'n', long)]
    pub size: Option<usize>,
}

// ── 정렬 알고리즘 ──────────────────────────────────────────────────────

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
    Heap,
    Shell,
    Tim,
}

impl Algorithm {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bubble    => "Bubble Sort",
            Self::Selection => "Selection Sort",
            Self::Insertion => "Insertion Sort",
            Self::Merge     => "Merge Sort",
            Self::Quick     => "Quick Sort",
            Self::Heap      => "Heap Sort",
            Self::Shell     => "Shell Sort",
            Self::Tim       => "Tim Sort",
        }
    }

    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Bubble    => "bubble",
            Self::Selection => "selection",
            Self::Insertion => "insertion",
            Self::Merge     => "merge",
            Self::Quick     => "quick",
            Self::Heap      => "heap",
            Self::Shell     => "shell",
            Self::Tim       => "tim",
        }
    }

    pub fn complexity(&self) -> &'static str {
        match self {
            Self::Bubble    => "O(n²)",
            Self::Selection => "O(n²)",
            Self::Insertion => "O(n²)",
            Self::Merge     => "O(n log n)",
            Self::Quick     => "O(n log n)",
            Self::Heap      => "O(n log n)",
            Self::Shell     => "O(n log² n)",
            Self::Tim       => "O(n log n)",
        }
    }

    pub fn is_implemented(&self) -> bool {
        matches!(self, Self::Bubble | Self::Selection | Self::Insertion)
    }
}

// ── 자료구조 ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataStructure {
    Array,
    LinkedList,
    Stack,
    Queue,
    BinaryTree,
    Bst,
    Heap,
    HashTable,
    Graph,
}

impl DataStructure {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Array      => "Array",
            Self::LinkedList => "Linked List",
            Self::Stack      => "Stack",
            Self::Queue      => "Queue",
            Self::BinaryTree => "Binary Tree",
            Self::Bst        => "Binary Search Tree",
            Self::Heap       => "Heap",
            Self::HashTable  => "Hash Table",
            Self::Graph      => "Graph",
        }
    }

    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Array      => "array",
            Self::LinkedList => "linked-list",
            Self::Stack      => "stack",
            Self::Queue      => "queue",
            Self::BinaryTree => "binary-tree",
            Self::Bst        => "bst",
            Self::Heap       => "heap-ds",
            Self::HashTable  => "hash-table",
            Self::Graph      => "graph",
        }
    }

    pub fn complexity(&self) -> &'static str {
        match self {
            Self::Array      => "O(1) access",
            Self::LinkedList => "O(n) access",
            Self::Stack      => "O(1) push/pop",
            Self::Queue      => "O(1) enq/deq",
            Self::BinaryTree => "O(h) ops",
            Self::Bst        => "O(log n) avg",
            Self::Heap       => "O(log n) ins",
            Self::HashTable  => "O(1) avg",
            Self::Graph      => "O(V+E)",
        }
    }
}

// ── 통합 Topic ─────────────────────────────────────────────────────────
// 선택 화면에서 알고리즘과 자료구조를 함께 표시하기 위한 타입

#[derive(Clone, Copy, Debug)]
pub enum Topic {
    Algo(Algorithm),
    Ds(DataStructure),
}

impl Topic {
    pub fn keyword(&self) -> &'static str {
        match self { Self::Algo(a) => a.keyword(), Self::Ds(d) => d.keyword() }
    }
    pub fn name(&self) -> &'static str {
        match self { Self::Algo(a) => a.name(), Self::Ds(d) => d.name() }
    }
    pub fn complexity(&self) -> &'static str {
        match self { Self::Algo(a) => a.complexity(), Self::Ds(d) => d.complexity() }
    }
    pub fn is_implemented(&self) -> bool {
        match self { Self::Algo(a) => a.is_implemented(), Self::Ds(_) => false }
    }
    pub fn is_algo(&self) -> bool { matches!(self, Self::Algo(_)) }
}

pub const ALL_TOPICS: &[Topic] = &[
    Topic::Algo(Algorithm::Bubble),
    Topic::Algo(Algorithm::Selection),
    Topic::Algo(Algorithm::Insertion),
    Topic::Algo(Algorithm::Merge),
    Topic::Algo(Algorithm::Quick),
    Topic::Algo(Algorithm::Heap),
    Topic::Algo(Algorithm::Shell),
    Topic::Algo(Algorithm::Tim),
    Topic::Ds(DataStructure::Array),
    Topic::Ds(DataStructure::LinkedList),
    Topic::Ds(DataStructure::Stack),
    Topic::Ds(DataStructure::Queue),
    Topic::Ds(DataStructure::BinaryTree),
    Topic::Ds(DataStructure::Bst),
    Topic::Ds(DataStructure::Heap),
    Topic::Ds(DataStructure::HashTable),
    Topic::Ds(DataStructure::Graph),
];

// 하위 호환용 (기존 코드에서 ALL_ALGORITHMS 참조하는 경우)
pub const ALL_ALGORITHMS: &[Algorithm] = &[
    Algorithm::Bubble, Algorithm::Selection, Algorithm::Insertion,
    Algorithm::Merge,  Algorithm::Quick,     Algorithm::Heap,
    Algorithm::Shell,  Algorithm::Tim,
];
