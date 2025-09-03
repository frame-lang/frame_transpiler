use crate::frame_c::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Int,
    Float,
    String,
    Bool,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionType {
    List(Option<Box<PrimitiveType>>),
    Map(Option<(Box<PrimitiveType>, Box<PrimitiveType>)>),
    Set(Option<Box<PrimitiveType>>),
    Queue(Option<Box<PrimitiveType>>),
    Stack(Option<Box<PrimitiveType>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInType {
    Primitive(PrimitiveType),
    Collection(CollectionType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionOperation {
    ListAppend,
    ListPop,
    ListClear,
    ListInsert,
    ListRemove,
    ListLength,
    ListIsEmpty,
    
    MapGet,
    MapSet,
    MapDelete,
    MapHas,
    MapKeys,
    MapValues,
    MapLength,
    
    SetAdd,
    SetRemove,
    SetHas,
    SetClear,
    SetLength,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringOperation {
    StringLength,
    StringSubstring,
    StringSplit,
    StringContains,
    StringReplace,
    StringUpper,
    StringLower,
    StringTrim,
    StringFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionOperation {
    ToString,
    ToInt,
    ToFloat,
    ToBool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MathOperation {
    Abs,
    Min,
    Max,
    Round,
    Floor,
    Ceil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceOperation {
    Marshal,
    Unmarshal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInOperation {
    Collection(CollectionOperation),
    String(StringOperation),
    Conversion(ConversionOperation),
    Math(MathOperation),
    Persistence(PersistenceOperation),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInProperty {
    Length,
    IsEmpty,
    Keys,
    Values,
    Name,
    Value,
}

pub struct FslRegistry {
    operations: HashMap<String, BuiltInOperation>,
    properties: HashMap<String, BuiltInProperty>,
}

impl FslRegistry {
    pub fn new() -> Self {
        let mut registry = FslRegistry {
            operations: HashMap::new(),
            properties: HashMap::new(),
        };
        
        registry.register_operations();
        registry.register_properties();
        registry
    }
    
    fn register_operations(&mut self) {
        use CollectionOperation::*;
        use StringOperation::*;
        use ConversionOperation::*;
        use MathOperation::*;
        use PersistenceOperation::*;
        use BuiltInOperation::*;
        
        self.operations.insert("append".to_string(), Collection(ListAppend));
        self.operations.insert("pop".to_string(), Collection(ListPop));
        self.operations.insert("clear".to_string(), Collection(ListClear));
        self.operations.insert("insert".to_string(), Collection(ListInsert));
        self.operations.insert("remove".to_string(), Collection(ListRemove));
        
        self.operations.insert("get".to_string(), Collection(MapGet));
        self.operations.insert("set".to_string(), Collection(MapSet));
        self.operations.insert("delete".to_string(), Collection(MapDelete));
        self.operations.insert("has".to_string(), Collection(MapHas));
        
        // Commented out: 'add' conflicts with user-defined functions
        // Set operations should be methods on set objects, not standalone functions
        // self.operations.insert("add".to_string(), Collection(SetAdd));
        
        self.operations.insert("substring".to_string(), String(StringSubstring));
        self.operations.insert("split".to_string(), String(StringSplit));
        self.operations.insert("contains".to_string(), String(StringContains));
        self.operations.insert("replace".to_string(), String(StringReplace));
        self.operations.insert("upper".to_string(), String(StringUpper));
        self.operations.insert("lower".to_string(), String(StringLower));
        self.operations.insert("trim".to_string(), String(StringTrim));
        self.operations.insert("format".to_string(), String(StringFormat));
        
        self.operations.insert("str".to_string(), Conversion(ToString));
        self.operations.insert("int".to_string(), Conversion(ToInt));
        self.operations.insert("float".to_string(), Conversion(ToFloat));
        self.operations.insert("bool".to_string(), Conversion(ToBool));
        
        self.operations.insert("abs".to_string(), Math(Abs));
        self.operations.insert("min".to_string(), Math(Min));
        self.operations.insert("max".to_string(), Math(Max));
        self.operations.insert("round".to_string(), Math(Round));
        self.operations.insert("floor".to_string(), Math(Floor));
        self.operations.insert("ceil".to_string(), Math(Ceil));
        
        self.operations.insert("marshal".to_string(), Persistence(Marshal));
        self.operations.insert("unmarshal".to_string(), Persistence(Unmarshal));
    }
    
    fn register_properties(&mut self) {
        use BuiltInProperty::*;
        
        self.properties.insert("length".to_string(), Length);
        self.properties.insert("is_empty".to_string(), IsEmpty);
        self.properties.insert("keys".to_string(), Keys);
        self.properties.insert("values".to_string(), Values);
        self.properties.insert("name".to_string(), Name);
        self.properties.insert("value".to_string(), Value);
    }
    
    pub fn recognize_operation(&self, name: &str) -> Option<BuiltInOperation> {
        self.operations.get(name).cloned()
    }
    
    pub fn recognize_property(&self, name: &str) -> Option<BuiltInProperty> {
        self.properties.get(name).cloned()
    }
    
    pub fn is_fsl_operation(&self, name: &str) -> bool {
        self.operations.contains_key(name)
    }
    
    pub fn is_fsl_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
}

pub struct BuiltInCallNode {
    pub operation: BuiltInOperation,
    pub target: Box<ExprType>,
    pub arguments: Vec<ExprType>,
    pub line: usize,
}

pub struct BuiltInPropertyNode {
    pub property: BuiltInProperty,
    pub target: Box<ExprType>,
    pub line: usize,
}

pub trait FslVisitor {
    fn visit_builtin_call(&mut self, node: &BuiltInCallNode);
    fn visit_builtin_property(&mut self, node: &BuiltInPropertyNode);
}