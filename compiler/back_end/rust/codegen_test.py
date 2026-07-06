import unittest
from compiler.back_end.rust import codegen
from compiler.util import ir_data

class CodegenTest(unittest.TestCase):
    def test_generate_minimal_struct(self):
        field = ir_data.Field(
            name=ir_data.NameDefinition(name=ir_data.Word(text="foo")),
            location=ir_data.FieldLocation(
                start=ir_data.Expression(constant=ir_data.NumericConstant(value="0")),
                size=ir_data.Expression(constant=ir_data.NumericConstant(value="8"))
            ),
            type=ir_data.Type(atomic_type=ir_data.AtomicType(
                reference=ir_data.Reference(canonical_name=ir_data.CanonicalName(object_path=["UInt"]))
            ))
        )
        
        struct_def = ir_data.TypeDefinition(
            name=ir_data.NameDefinition(name=ir_data.Word(text="MyStruct")),
            structure=ir_data.Structure(field=[field])
        )
        
        module = ir_data.Module(type=[struct_def])
        ir = ir_data.EmbossIr(module=[module])
        
        output = codegen.generate_rust(ir)
        
        self.assertIn("pub trait MyStructView", output)
        self.assertIn("fn foo(&self) -> Result<u8, Error>;", output)
        self.assertIn("impl MyStructView for [u8]", output)
        self.assertIn("self.read_le_u8(0)", output)

if __name__ == "__main__":
    unittest.main()
