use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 213], OperandSize::Dword)
}

#[test]
fn paddusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 44642869, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 132, 190, 53, 50, 169, 2], OperandSize::Dword)
}

#[test]
fn paddusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 229], OperandSize::Qword)
}

#[test]
fn paddusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 5859897, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 221, 148, 222, 57, 106, 89, 0], OperandSize::Qword)
}

#[test]
fn paddusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 227], OperandSize::Dword)
}

#[test]
fn paddusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 52, 177], OperandSize::Dword)
}

#[test]
fn paddusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 217], OperandSize::Qword)
}

#[test]
fn paddusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 221, 39], OperandSize::Qword)
}

