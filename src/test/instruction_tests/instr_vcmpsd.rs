use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 194, 219, 78], OperandSize::Dword)
}

#[test]
fn vcmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1280745243, Some(OperandSize::Qword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 194, 182, 27, 159, 86, 76, 11], OperandSize::Dword)
}

#[test]
fn vcmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 194, 197, 55], OperandSize::Qword)
}

#[test]
fn vcmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 903065543, Some(OperandSize::Qword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 194, 169, 199, 175, 211, 53, 102], OperandSize::Qword)
}

#[test]
fn vcmpsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 30, 194, 242, 0], OperandSize::Dword)
}

#[test]
fn vcmpsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 231, 15, 194, 56, 14], OperandSize::Dword)
}

#[test]
fn vcmpsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 143, 28, 194, 243, 57], OperandSize::Qword)
}

#[test]
fn vcmpsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 151, 13, 194, 9, 107], OperandSize::Qword)
}

