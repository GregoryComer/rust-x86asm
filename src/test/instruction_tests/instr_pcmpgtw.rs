use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 236], OperandSize::Dword)
}

#[test]
fn pcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(EBX, 1897478122, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 171, 234, 55, 25, 113], OperandSize::Dword)
}

#[test]
fn pcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 224], OperandSize::Qword)
}

#[test]
fn pcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 52, 94], OperandSize::Qword)
}

#[test]
fn pcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 202], OperandSize::Dword)
}

#[test]
fn pcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 942666313, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 60, 77, 73, 242, 47, 56], OperandSize::Dword)
}

#[test]
fn pcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 242], OperandSize::Qword)
}

#[test]
fn pcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1971065937, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 52, 93, 81, 20, 124, 117], OperandSize::Qword)
}

