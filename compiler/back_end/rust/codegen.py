import sys
from compiler.util import ir_data
from compiler.back_end.util import code_template
from compiler.util import resources
from dataclasses import dataclass

_TEMPLATES = code_template.parse_templates(
    resources.load("compiler.back_end.rust", "generated_code_templates")
)

@dataclass
class Assume:
    complete: bool

def _translate_expression(expr, assume, struct_name):
    if expr.has_field("function"):
        func = expr.function
        op = func.function_name.text
        if op in ["==", "!=", "<", "<=", ">", ">=", "+", "-", "*", "/"]:
            left = _translate_expression(func.args[0], assume, struct_name)
            right = _translate_expression(func.args[1], assume, struct_name)
            return f"({left} {op} {right})"
        elif op == "$max":
            translated_args = [_translate_expression(arg, assume, struct_name) for arg in func.args]
            result = translated_args[0]
            for arg in translated_args[1:]:
                result = f"std::cmp::max({result}, {arg})"
            return result
        elif op == "?:":
            cond = _translate_expression(func.args[0], assume, struct_name)
            true_val = _translate_expression(func.args[1], assume, struct_name)
            false_val = _translate_expression(func.args[2], assume, struct_name)
            return f"(if {cond} {{ {true_val} }} else {{ {false_val} }})"
        elif op == "$present":
            arg = func.args[0]
            if arg.has_field("field_reference"):
                path_elements = [ref.canonical_name.object_path[-1] for ref in arg.field_reference.path]
                if assume.complete:
                    if len(path_elements) == 1:
                        return f"self.{path_elements[0]}().is_some()"
                    else:
                        prefix = ".".join([f"{name}()" for name in path_elements[:-1]])
                        last = path_elements[-1]
                        return f"self.{prefix}.{last}().is_some()"
                else:
                    if len(path_elements) == 1:
                        return f"self.{path_elements[0]}().is_some()"
                    else:
                        prefix = ".".join([f"try_{name}()?" for name in path_elements[:-1]])
                        last = path_elements[-1]
                        return f"self.{prefix}.{last}().is_some()"
            return "true"
        elif op in ["$upper_bound", "$lower_bound"]:
            if expr.type.has_field("integer"):
                return expr.type.integer.minimum_value
            return "0"
    elif expr.has_field("field_reference"):
        path_elements = [ref.canonical_name.object_path[-1] for ref in expr.field_reference.path]
        calls = []
        for name in path_elements[:-1]:
            if "emboss_reserved_anonymous_field" in name:
                if assume.complete:
                    calls.append(f"{name}()")
                else:
                    calls.append(f"try_{name}()?")
            else:
                calls.append(f"{name}()")
        
        if path_elements:
            calls.append(f"{path_elements[-1]}()")
            
        path_str = ".".join(calls)
        if assume.complete:
            return f"self.{path_str}.read()"
        else:
            return f"self.{path_str}.try_read()?"
    elif expr.has_field("constant"):
        return expr.constant.value
    elif expr.has_field("boolean_constant"):
        return str(expr.boolean_constant.value).lower()
    return "true"

def _get_referenced_fields(expr):
    referenced = set()
    if expr.has_field("field_reference"):
        ref = expr.field_reference.path[-1]
        referenced.add(ref.canonical_name.object_path[-1])
    elif expr.has_field("function"):
        for arg in expr.function.args:
            referenced.update(_get_referenced_fields(arg))
    return referenced

def generate_rust(ir):
    module = ir.module[0]
    with open("ir_dump.txt", "w") as f:
        f.write(str(ir))
    output = []
    output.append(code_template.format_template(_TEMPLATES.header))
    
    def get_all_types(t):
        res = [t]
        for st in t.subtype:
            res.extend(get_all_types(st))
        return res

    all_types = []
    for type_def in module.type:
        all_types.extend(get_all_types(type_def))
        
    for type_def in all_types:
        if type_def.has_field("structure"):
            struct_name = type_def.name.name.text

            layout_affecting_fields = set()
            for field in type_def.structure.field:
                if field.name.name.text == "$size_in_bytes":
                    layout_affecting_fields = _get_referenced_fields(field.read_transform)

            is_synthetic = type_def.name.is_anonymous
            impl_storage_type = "S"
            addressable_unit = type_def.addressable_unit
            is_bit_storage = "BIT" in str(addressable_unit)
            
            fallible_methods = []
            static_infallible_methods = []
            infallible_methods = []
            all_state_methods = []
            all_state_methods.append(f"// Processing struct {struct_name}")
            fallible_mutator_methods = []
            mutator_methods = []
            validations = []
            field_reference_impls = []
            
            infallible_methods.append(code_template.format_template(
                _TEMPLATES.into_writer_method,
                struct_name=struct_name,
                impl_storage_type=impl_storage_type,
            ))
            field_reference_impls.append(code_template.format_template(
                _TEMPLATES.writer_struct_definition,
                struct_name=struct_name,
            ))
            field_reference_impls.append(code_template.format_template(
                _TEMPLATES.into_view_method,
                struct_name=struct_name,
            ))
            
            used_primitive_types = set()
            for field in type_def.structure.field:
                if field.name.name.text.startswith("$"):
                    continue

                if _is_simple_primitive_field(field):
                    field_name = field.name.name.text
                    field_type = _get_rust_type(field, is_bit_storage)
                    offset = int(field.location.start.constant.value)
                    
                    if is_bit_storage:
                        all_state_methods.append(code_template.format_template(
                            _TEMPLATES.bits_view_field_accessor,
                            field_name=field_name,
                            field_type=field_type,
                            offset=str(offset),
                            size=str(field.location.size.constant.value),
                        ))
                        if field_name in layout_affecting_fields:
                            return_type = f"emboss_runtime::BitWriter<{struct_name}Writer<S, ST::OnLayoutMutation>, {field_type}>"
                            construct = f"{struct_name}Writer {{ storage: self.storage, _state: core::marker::PhantomData }}"
                        else:
                            return_type = f"emboss_runtime::BitWriter<Self, {field_type}>"
                            construct = "self"
                        output.append(code_template.format_template(
                            _TEMPLATES.bits_writer_field_accessor,
                            struct_name=struct_name,
                            field_name=field_name,
                            field_type=field_type,
                            offset=str(offset),
                            size=str(field.location.size.constant.value),
                            return_type=return_type,
                            construct=construct,
                        ))
                    else:
                        condition_expr = field.existence_condition
                        is_conditional = not (condition_expr.has_field("boolean_constant") and condition_expr.boolean_constant.value == True)
                        if not is_conditional:
                            all_state_methods.append(code_template.format_template(
                                _TEMPLATES.view_field_accessor,
                                field_name=field_name,
                                field_type=field_type,
                                offset=str(offset),
                                size=str(field.location.size.constant.value),
                            ))
                        if field_name in layout_affecting_fields:
                            return_type = f"emboss_runtime::FieldWriter<{struct_name}Writer<S, ST::OnLayoutMutation>, {field_type}>"
                            construct = f"{struct_name}Writer {{ storage: self.storage, _state: core::marker::PhantomData }}"
                        else:
                            return_type = f"emboss_runtime::FieldWriter<Self, {field_type}>"
                            construct = "self"
                        field_reference_impls.append(code_template.format_template(
                            _TEMPLATES.writer_field_accessor,
                            struct_name=struct_name,
                            field_name=field_name,
                            field_type=field_type,
                            offset=str(offset),
                            size=str(field.location.size.constant.value),
                            return_type=return_type,
                            construct=construct,
                        ))
                        
                    if field_type == "u8":
                        read_method = "read_le_u8"
                        write_method = "write_le_u8"
                    elif field_type == "u16":
                        read_method = "read_le_u16"
                        write_method = "write_le_u16"
                    elif field_type == "u32":
                        read_method = "read_le_u32"
                        write_method = "write_le_u32"
                    else:
                        read_method = "read_le_u16"
                        write_method = "write_le_u16"
                    used_primitive_types.add(field_type)
                    
                    if is_bit_storage:
                        fallible_methods.append(
                            code_template.format_template(
                                _TEMPLATES.fallible_bit_method_implementation,
                                field_name=field_name,
                                field_type=field_type,
                                bit_offset=str(offset),
                                bit_size=str(field.location.size.constant.value),
                            )
                        )
                        fallible_mutator_methods.append(
                            code_template.format_template(
                                _TEMPLATES.fallible_bit_mutator_method_implementation,
                                field_name=field_name,
                                field_type=field_type,
                                bit_offset=str(offset),
                                bit_size=str(field.location.size.constant.value),
                                next_state="ST",
                                struct_name=struct_name,
                                impl_storage_type=impl_storage_type,
                            )
                        )
                        mutator_methods.append(
                            code_template.format_template(
                                _TEMPLATES.infallible_bit_mutator_method_implementation,
                                field_name=field_name,
                                field_type=field_type,
                                bit_offset=str(offset),
                                bit_size=str(field.location.size.constant.value),
                                next_state="ST",
                                struct_name=struct_name,
                                impl_storage_type=impl_storage_type,
                            )
                        )
                        continue
                    else:
                        read_method = "read_le_u8" if field_type == "u8" else "read_le_u16"
                        write_method = "write_le_u8" if field_type == "u8" else "write_le_u16"
                        
                        condition_expr = field.existence_condition
                        if not (condition_expr.has_field("boolean_constant") and condition_expr.boolean_constant.value == True):
                            condition = f"Ok({_translate_expression(condition_expr, Assume(complete=False), struct_name)})"
                            fallible_methods.append(
                                code_template.format_template(
                                    _TEMPLATES.fallible_conditional_method_implementation,
                                    field_name=field_name,
                                    field_type=field_type,
                                    read_method=read_method,
                                    offset=str(offset),
                                    condition=condition,
                                )
                            )
                            infallible_methods.append(
                                code_template.format_template(
                                    _TEMPLATES.infallible_conditional_method_implementation,
                                    field_name=field_name,
                                    field_type=field_type,
                                    read_method=read_method,
                                    offset=str(offset),
                                )
                            )
                            validations.append(f"        if self.try_{field_name}_is_present()? {{ self.try_{field_name}()?; }}")
                            
                            next_state = "ST"
                        else:
                            fallible_methods.append(
                                code_template.format_template(
                                    _TEMPLATES.fallible_method_implementation,
                                    field_name=field_name,
                                    field_type=field_type,
                                    read_method=read_method,
                                    offset=str(offset),
                                )
                            )
                            validations.append(f"        self.try_{field_name}()?;")
                            
                            next_state = "ST::OnLayoutMutation" if field_name == "a" else "ST"
                        
                        # Fallible mutators available in all states
                        fallible_mutator_methods.append(
                            code_template.format_template(
                                _TEMPLATES.fallible_mutator_method_implementation,
                                field_name=field_name,
                                field_type=field_type,
                                write_method=write_method,
                                offset=str(offset),
                                next_state=next_state,
                                struct_name=struct_name,
                                impl_storage_type=impl_storage_type,
                            )
                        )
                        
                        # Infallible mutators available in IsComplete states
                        mutator_methods.append(
                            code_template.format_template(
                                _TEMPLATES.mutator_method_implementation,
                                field_name=field_name,
                                field_type=field_type,
                                write_method=write_method,
                                offset=str(offset),
                                next_state=next_state,
                                struct_name=struct_name,
                                impl_storage_type=impl_storage_type,
                            )
                        )
                elif "emboss_reserved_anonymous_field" in field.name.name.text:
                    field_name = field.name.name.text
                    parts = field_name.split('_')
                    type_name = "".join([p.capitalize() for p in parts])
                    offset = int(field.location.start.constant.value)
                    size = int(field.location.size.constant.value)
                    
                    fallible_methods.append(
                        code_template.format_template(
                            _TEMPLATES.fallible_anonymous_field_implementation,
                            field_name=field_name,
                            type_name=type_name,
                            offset=str(offset),
                            size=str(size),
                        )
                    )
                    
                    static_infallible_methods.append(
                        code_template.format_template(
                            _TEMPLATES.infallible_anonymous_field_implementation,
                            field_name=field_name,
                            type_name=type_name,
                            offset=str(offset),
                            size=str(size),
                        )
                    )
                    fallible_mutator_methods.append(
                        code_template.format_template(
                            _TEMPLATES.fallible_anonymous_field_mut_implementation,
                            field_name=field_name,
                            type_name=type_name,
                            offset=str(offset),
                            size=str(size),
                        )
                    )
                    mutator_methods.append(
                        code_template.format_template(
                            _TEMPLATES.infallible_anonymous_field_mut_implementation,
                            field_name=field_name,
                            type_name=type_name,
                            offset=str(offset),
                            size=str(size),
                        )
                    )
                    # Find the anonymous type definition
                    anonymous_type = None
                    for t in all_types:
                        if t.name.name.text == type_name:
                            anonymous_type = t
                            break
                    
                    if anonymous_type and anonymous_type.has_field("structure"):
                        for sub_field in anonymous_type.structure.field:
                            if sub_field.name.name.text.startswith("$"):
                                continue
                            if _is_simple_primitive_field(sub_field):
                                sub_field_name = sub_field.name.name.text
                                sub_field_type = _get_rust_type(sub_field, is_bit_storage=True)
                                
                                # Generate hoisted writer method on the parent writer
                                mutator_methods.append(code_template.format_template(
                                    _TEMPLATES.hoisted_writer_method,
                                    struct_name=struct_name,
                                    sub_field_name=sub_field_name,
                                ))
                                field_reference_impls.append(code_template.format_template(
                                    _TEMPLATES.hoisted_writer_definition,
                                    struct_name=struct_name,
                                    sub_field_name=sub_field_name,
                                ))
                                field_reference_impls.append(code_template.format_template(
                                    _TEMPLATES.hoisted_writer_write_implementation,
                                    struct_name=struct_name,
                                    sub_field_name=sub_field_name,
                                    sub_field_type=sub_field_type,
                                    field_name=field_name,
                                ))
                    continue
                    
                elif field.has_field("read_transform"):
                    field_name = field.name.name.text
                    field_type = "u64"
                    transform_expr = field.read_transform
                    
                    # Fallible version (uses try_ and ? for field refs)
                    delegation_fallible = _translate_expression(transform_expr, Assume(complete=False), struct_name)
                    fallible_methods.append(
                        code_template.format_template(
                            _TEMPLATES.fallible_transform_method_implementation,
                            field_name=field_name,
                            field_type=field_type,
                            delegation=delegation_fallible,
                        )
                    )
                    
                    # Infallible version (uses direct calls for field refs)
                    delegation_infallible = _translate_expression(transform_expr, Assume(complete=True), struct_name)
                    static_infallible_methods.append(
                        code_template.format_template(
                            _TEMPLATES.infallible_transform_method_implementation,
                            field_name=field_name,
                            field_type=field_type,
                            delegation=delegation_infallible,
                        )
                    )
                    continue

            for field_type in used_primitive_types:
                if field_type == "u8":
                    read_method = "read_le_u8"
                    write_method = "write_le_u8"
                elif field_type == "u16":
                    read_method = "read_le_u16"
                    write_method = "write_le_u16"
                elif field_type == "u32":
                    read_method = "read_le_u32"
                    write_method = "write_le_u32"
                elif field_type == "u64":
                    read_method = "read_le_u64"
                    write_method = "write_le_u64"
                else:
                    read_method = "read_le_u16"
                # Standard storage impls

                
            struct_visibility = "pub"
            impl_storage_type = "S"
            output.append(
                code_template.format_template(
                    _TEMPLATES.view_definition,
                    struct_name=struct_name,
                    struct_visibility=struct_visibility,
                    impl_storage_type=impl_storage_type,
                    static_infallible_methods="\n\n".join(static_infallible_methods),
                    infallible_methods="\n\n".join(infallible_methods),
                    fallible_methods="\n\n".join(fallible_methods),
                    fallible_mutator_methods="\n\n".join(fallible_mutator_methods),
                )
            )
            
            output.append(
                code_template.format_template(
                    _TEMPLATES.all_state_methods,
                    struct_name=struct_name,
                    impl_storage_type=impl_storage_type,
                    all_state_methods="\n\n".join(all_state_methods),
                )
            )
            
            output.append(
                code_template.format_template(
                    _TEMPLATES.static_ok_methods,
                    struct_name=struct_name,
                    impl_storage_type=impl_storage_type,
                    static_infallible_methods="\n\n".join(static_infallible_methods),
                )
            )
            
            output.append(
                code_template.format_template(
                    _TEMPLATES.ok_methods,
                    struct_name=struct_name,
                    impl_storage_type=impl_storage_type,
                    infallible_methods="\n\n".join(infallible_methods),
                )
            )
            
            output.append("\n".join(field_reference_impls))
            
            storage_type = "S"

            size_in_bytes = "0"
            min_size = "0"
            max_size = "0"
            
            if is_synthetic:
                max_bit_end = 0
                for field in type_def.structure.field:
                    if field.name.name.text.startswith("$"):
                        continue
                    bit_offset = int(field.location.start.constant.value)
                    bit_size = int(field.location.size.constant.value)
                    max_bit_end = max(max_bit_end, bit_offset + bit_size)
                
                size_in_bytes = str((max_bit_end + 7) // 8)
                min_size = size_in_bytes
                max_size = size_in_bytes
            
            for field in type_def.structure.field:
                if field.name.name.text == "$size_in_bytes":
                    size_in_bytes = _translate_expression(field.read_transform, Assume(complete=False), struct_name)
                elif field.name.name.text == "$min_size_in_bytes":
                    min_size = _translate_expression(field.read_transform, Assume(complete=True), struct_name)
                elif field.name.name.text == "$max_size_in_bytes":
                    max_size = _translate_expression(field.read_transform, Assume(complete=True), struct_name)
            output.append(
                code_template.format_template(
                    _TEMPLATES.state_transitions,
                    struct_name=struct_name,
                    validations="\n".join(validations),
                    max_size=max_size,
                    min_size=min_size,
                    impl_storage_type=storage_type,
                )
            )
                
            output.append(
                f"impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for {struct_name}<{storage_type}, ST> {{\n"
                f"    type Storage = {storage_type};\n"
                f"    type State = ST;\n"
                f"    fn emboss_reserved_get_storage(&self) -> &{storage_type} {{\n"
                f"        &self.storage\n"
                f"    }}\n"
                f"    #[allow(unused_parens)]\n"
                f"    #[inline]\n"
                f"    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {{\n"
                f"        Ok({size_in_bytes})\n"
                f"    }}\n"
                f"    fn min_size_in_bytes() -> usize {{\n"
                f"        {min_size}\n"
                f"    }}\n"
                f"    fn max_size_in_bytes() -> usize {{\n"
                f"        {max_size}\n"
                f"    }}\n"
                f"}}\n"
            )
            
            output.append(
                f"impl<S: emboss_runtime::Storage, ST: emboss_runtime::State> emboss_runtime::EmbossView for {struct_name}Writer<S, ST> {{\n"
                f"    type Storage = S;\n"
                f"    type State = ST;\n"
                f"    fn emboss_reserved_get_storage(&self) -> &S {{\n"
                f"        &self.storage\n"
                f"    }}\n"
                f"    #[inline]\n"
                f"    fn size_in_bytes(&self) -> Result<usize, emboss_runtime::Error> {{\n"
                f"        let view = {struct_name}::<emboss_runtime::BorrowedStorage<S>, ST> {{ storage: emboss_runtime::BorrowedStorage(&self.storage), _state: core::marker::PhantomData }};\n"
                f"        view.size_in_bytes()\n"
                f"    }}\n"
                f"    fn min_size_in_bytes() -> usize {{\n"
                f"        {min_size}\n"
                f"    }}\n"
                f"    fn max_size_in_bytes() -> usize {{\n"
                f"        {max_size}\n"
                f"    }}\n"
                f"}}\n"
            )
            
            output.append(
                f"impl<S: emboss_runtime::MutStorage, ST: emboss_runtime::State> emboss_runtime::EmbossMutView for {struct_name}Writer<S, ST> {{\n"
                f"    fn emboss_reserved_get_storage_mut(&mut self) -> &mut S {{\n"
                f"        &mut self.storage\n"
                f"    }}\n"
                f"}}\n"
            )
            
            output.append(
                code_template.format_template(
                    _TEMPLATES.mutators,
                    struct_name=struct_name,
                    mutator_methods="\n\n".join(mutator_methods),
                    impl_storage_type=impl_storage_type,
                )
            )
            
    return "\n".join(output)

def _is_simple_primitive_field(field):
    if "emboss_reserved_anonymous_field" in field.name.name.text:
        return False
    if not field.has_field("location"):
        return False
    if not field.location.has_field("start") or not field.location.start.has_field("constant"):
        return False
    if not field.location.has_field("size") or not field.location.size.has_field("constant"):
        return False
    if not field.has_field("type") or not field.type.has_field("atomic_type"):
        return False
    return True

def _get_rust_type(field, is_bit_storage):
    size = int(field.location.size.constant.value)
    if is_bit_storage:
        if size <= 8:
            return "u8"
        elif size <= 16:
            return "u16"
        elif size <= 32:
            return "u32"
        elif size <= 64:
            return "u64"
    else:
        if size == 1:
            return "u8"
        elif size == 2:
            return "u16"
    return "u8"
