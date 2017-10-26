use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 253], OperandSize::Dword)
}

#[test]
fn pmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EAX, 1611622507, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 128, 107, 104, 15, 96], OperandSize::Dword)
}

#[test]
fn pmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 234], OperandSize::Qword)
}

#[test]
fn pmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1802693206, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 12, 125, 86, 234, 114, 107], OperandSize::Qword)
}

#[test]
fn pmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 223], OperandSize::Dword)
}

#[test]
fn pmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 1541851277, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 136, 141, 200, 230, 91], OperandSize::Dword)
}

#[test]
fn pmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 224], OperandSize::Qword)
}

#[test]
fn pmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 600956800, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 148, 182, 128, 223, 209, 35], OperandSize::Qword)
}

