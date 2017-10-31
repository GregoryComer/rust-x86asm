use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 194], OperandSize::Dword)
}

#[test]
fn pabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 15], OperandSize::Dword)
}

#[test]
fn pabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 226], OperandSize::Qword)
}

#[test]
fn pabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 39], OperandSize::Qword)
}

#[test]
fn pabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 235], OperandSize::Dword)
}

#[test]
fn pabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 1259379614, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 151, 158, 155, 16, 75], OperandSize::Dword)
}

#[test]
fn pabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 196], OperandSize::Qword)
}

#[test]
fn pabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 36, 95], OperandSize::Qword)
}

