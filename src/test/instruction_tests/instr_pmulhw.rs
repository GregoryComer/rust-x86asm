use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 230], OperandSize::Dword)
}

#[test]
fn pmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 136411445, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 60, 197, 53, 121, 33, 8], OperandSize::Dword)
}

#[test]
fn pmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 231], OperandSize::Qword)
}

#[test]
fn pmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDX, 1328826173, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 146, 61, 71, 52, 79], OperandSize::Qword)
}

#[test]
fn pmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 194], OperandSize::Dword)
}

#[test]
fn pmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 803653103, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 4, 117, 239, 197, 230, 47], OperandSize::Dword)
}

#[test]
fn pmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 194], OperandSize::Qword)
}

#[test]
fn pmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 136615968, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 44, 253, 32, 152, 36, 8], OperandSize::Qword)
}

