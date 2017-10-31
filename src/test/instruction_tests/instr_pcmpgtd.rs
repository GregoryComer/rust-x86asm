use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 195], OperandSize::Dword)
}

#[test]
fn pcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 30], OperandSize::Dword)
}

#[test]
fn pcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 220], OperandSize::Qword)
}

#[test]
fn pcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1600541895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 156, 118, 199, 84, 102, 95], OperandSize::Qword)
}

#[test]
fn pcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 204], OperandSize::Dword)
}

#[test]
fn pcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 38], OperandSize::Dword)
}

#[test]
fn pcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 200], OperandSize::Qword)
}

#[test]
fn pcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 626458785, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 140, 130, 161, 0, 87, 37], OperandSize::Qword)
}

