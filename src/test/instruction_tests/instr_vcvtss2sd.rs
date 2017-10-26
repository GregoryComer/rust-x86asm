use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 90, 192], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 918423890, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 90, 164, 86, 82, 9, 190, 54], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 90, 236], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 90, 46], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 102, 154, 90, 233], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1629157701, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 70, 143, 90, 145, 69, 249, 26, 97], OperandSize::Dword)
}

#[test]
fn vcvtss2sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 78, 148, 90, 240], OperandSize::Qword)
}

#[test]
fn vcvtss2sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1449294462, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 54, 142, 90, 172, 114, 126, 122, 98, 86], OperandSize::Qword)
}

