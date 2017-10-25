use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 227], OperandSize::Dword)
}

#[test]
fn pcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 12, 207], OperandSize::Dword)
}

#[test]
fn pcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 235], OperandSize::Qword)
}

#[test]
fn pcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 419089329, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 60, 221, 177, 203, 250, 24], OperandSize::Qword)
}

#[test]
fn pcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 208], OperandSize::Dword)
}

#[test]
fn pcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 44, 184], OperandSize::Dword)
}

#[test]
fn pcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 202], OperandSize::Qword)
}

#[test]
fn pcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 2094313473, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 28, 117, 1, 176, 212, 124], OperandSize::Qword)
}

