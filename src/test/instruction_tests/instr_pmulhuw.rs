use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 237], OperandSize::Dword)
}

#[test]
fn pmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 40], OperandSize::Dword)
}

#[test]
fn pmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 239], OperandSize::Qword)
}

#[test]
fn pmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 59], OperandSize::Qword)
}

#[test]
fn pmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 226], OperandSize::Dword)
}

#[test]
fn pmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 16], OperandSize::Dword)
}

#[test]
fn pmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 249], OperandSize::Qword)
}

#[test]
fn pmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 20, 207], OperandSize::Qword)
}

