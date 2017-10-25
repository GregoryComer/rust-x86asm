use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 205], OperandSize::Dword)
}

#[test]
fn pand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1072806706, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 164, 155, 50, 187, 241, 63], OperandSize::Dword)
}

#[test]
fn pand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 233], OperandSize::Qword)
}

#[test]
fn pand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 28, 203], OperandSize::Qword)
}

#[test]
fn pand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 246], OperandSize::Dword)
}

#[test]
fn pand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1885108162, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 20, 77, 194, 119, 92, 112], OperandSize::Dword)
}

#[test]
fn pand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 207], OperandSize::Qword)
}

#[test]
fn pand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 49], OperandSize::Qword)
}

