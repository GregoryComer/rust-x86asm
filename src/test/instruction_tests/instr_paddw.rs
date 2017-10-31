use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 233], OperandSize::Dword)
}

#[test]
fn paddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 458272179, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 52, 141, 179, 173, 80, 27], OperandSize::Dword)
}

#[test]
fn paddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 192], OperandSize::Qword)
}

#[test]
fn paddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1041854685, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 28, 205, 221, 112, 25, 62], OperandSize::Qword)
}

#[test]
fn paddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 233], OperandSize::Dword)
}

#[test]
fn paddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 2102340684, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 164, 250, 76, 44, 79, 125], OperandSize::Dword)
}

#[test]
fn paddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 243], OperandSize::Qword)
}

#[test]
fn paddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1544315406, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 180, 88, 14, 98, 12, 92], OperandSize::Qword)
}

