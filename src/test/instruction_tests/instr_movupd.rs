use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 223], OperandSize::Dword)
}

#[test]
fn movupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1305231114, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 4, 149, 10, 63, 204, 77], OperandSize::Dword)
}

#[test]
fn movupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 255], OperandSize::Qword)
}

#[test]
fn movupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 871262406, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 176, 198, 104, 238, 51], OperandSize::Qword)
}

#[test]
fn movupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 212], OperandSize::Dword)
}

#[test]
fn movupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 6], OperandSize::Dword)
}

#[test]
fn movupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 235], OperandSize::Qword)
}

#[test]
fn movupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1323685627, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 148, 78, 251, 214, 229, 78], OperandSize::Qword)
}

