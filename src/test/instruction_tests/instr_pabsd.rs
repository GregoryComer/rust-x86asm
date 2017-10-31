use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 196], OperandSize::Dword)
}

#[test]
fn pabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1844303796, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 180, 217, 180, 215, 237, 109], OperandSize::Dword)
}

#[test]
fn pabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 201], OperandSize::Qword)
}

#[test]
fn pabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1890723807, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 28, 85, 223, 39, 178, 112], OperandSize::Qword)
}

#[test]
fn pabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 213], OperandSize::Dword)
}

#[test]
fn pabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 50003468, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 44, 69, 12, 254, 250, 2], OperandSize::Dword)
}

#[test]
fn pabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 248], OperandSize::Qword)
}

#[test]
fn pabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 20, 209], OperandSize::Qword)
}

