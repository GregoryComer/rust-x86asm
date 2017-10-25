use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 90, 196], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 90, 60, 135], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 90, 204], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1949006689, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 90, 36, 213, 97, 123, 43, 116], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 239, 254, 90, 193], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1338330371, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 231, 140, 90, 151, 3, 77, 197, 79], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 207, 212, 90, 196], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 143, 137, 90, 52, 178], OperandSize::Qword)
}

