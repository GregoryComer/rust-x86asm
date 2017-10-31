use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 207], OperandSize::Dword)
}

#[test]
fn pmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1928509270, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 188, 131, 86, 183, 242, 114], OperandSize::Dword)
}

#[test]
fn pmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 239], OperandSize::Qword)
}

#[test]
fn pmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1958410942, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 60, 85, 190, 250, 186, 116], OperandSize::Qword)
}

#[test]
fn pmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 208], OperandSize::Dword)
}

#[test]
fn pmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1104298764, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 172, 191, 12, 67, 210, 65], OperandSize::Dword)
}

#[test]
fn pmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 196], OperandSize::Qword)
}

#[test]
fn pmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 41], OperandSize::Qword)
}

