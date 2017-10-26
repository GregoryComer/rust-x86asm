use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 198], OperandSize::Dword)
}

#[test]
fn pmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 36, 143], OperandSize::Dword)
}

#[test]
fn pmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 204], OperandSize::Qword)
}

#[test]
fn pmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDI, 1225953863, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 151, 71, 146, 18, 73], OperandSize::Qword)
}

#[test]
fn pmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 222], OperandSize::Dword)
}

#[test]
fn pmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 322989896, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 52, 133, 72, 111, 64, 19], OperandSize::Dword)
}

#[test]
fn pmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 213], OperandSize::Qword)
}

#[test]
fn pmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 20, 80], OperandSize::Qword)
}

