use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packsswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 198], OperandSize::Dword)
}

#[test]
fn packsswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 20, 75], OperandSize::Dword)
}

#[test]
fn packsswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 217], OperandSize::Qword)
}

#[test]
fn packsswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 99, 28, 201], OperandSize::Qword)
}

#[test]
fn packsswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 252], OperandSize::Dword)
}

#[test]
fn packsswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 670395962, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 36, 85, 58, 110, 245, 39], OperandSize::Dword)
}

#[test]
fn packsswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 254], OperandSize::Qword)
}

#[test]
fn packsswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 99, 44, 130], OperandSize::Qword)
}

