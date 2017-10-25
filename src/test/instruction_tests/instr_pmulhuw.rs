use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 252], OperandSize::Dword)
}

#[test]
fn pmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 576502160, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 44, 149, 144, 185, 92, 34], OperandSize::Dword)
}

#[test]
fn pmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 216], OperandSize::Qword)
}

#[test]
fn pmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RAX, 1721195126, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 168, 118, 90, 151, 102], OperandSize::Qword)
}

#[test]
fn pmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 193], OperandSize::Dword)
}

#[test]
fn pmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 2056230320, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 150, 176, 149, 143, 122], OperandSize::Dword)
}

#[test]
fn pmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 198], OperandSize::Qword)
}

#[test]
fn pmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 543007635, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 175, 147, 163, 93, 32], OperandSize::Qword)
}

