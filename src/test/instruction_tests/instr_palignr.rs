use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn palignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 200, 101], OperandSize::Dword)
}

#[test]
fn palignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 52, 82, 17], OperandSize::Dword)
}

#[test]
fn palignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 217, 49], OperandSize::Qword)
}

#[test]
fn palignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1839619643, Some(OperandSize::Qword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 156, 199, 59, 94, 166, 109, 39], OperandSize::Qword)
}

#[test]
fn palignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 200, 50], OperandSize::Dword)
}

#[test]
fn palignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 7651018, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 167, 202, 190, 116, 0, 103], OperandSize::Dword)
}

#[test]
fn palignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 226, 107], OperandSize::Qword)
}

#[test]
fn palignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 92835312, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 172, 246, 240, 141, 136, 5, 8], OperandSize::Qword)
}

