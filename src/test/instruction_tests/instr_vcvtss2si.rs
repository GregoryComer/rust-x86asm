use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 235], OperandSize::Dword)
}

#[test]
fn vcvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 28, 215], OperandSize::Dword)
}

#[test]
fn vcvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 240], OperandSize::Qword)
}

#[test]
fn vcvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 585350282, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 156, 211, 138, 188, 227, 34], OperandSize::Qword)
}

#[test]
fn vcvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 225], OperandSize::Qword)
}

#[test]
fn vcvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 41], OperandSize::Qword)
}

#[test]
fn vcvtss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 88, 45, 218], OperandSize::Dword)
}

#[test]
fn vcvtss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 52, 83], OperandSize::Dword)
}

#[test]
fn vcvtss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 126, 88, 45, 255], OperandSize::Qword)
}

#[test]
fn vcvtss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 46], OperandSize::Qword)
}

#[test]
fn vcvtss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 254, 56, 45, 229], OperandSize::Qword)
}

#[test]
fn vcvtss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1525180892, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 60, 141, 220, 105, 232, 90], OperandSize::Qword)
}

